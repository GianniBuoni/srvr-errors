#![cfg(feature = "postgres")]
use super::*;

use sqlx::{PgExecutor, Postgres};
use sqlx::{QueryBuilder, types::Uuid};

impl Arguments {
    /// Select statement used for all sqlx related vaidations.
    pub(super) async fn select_statement(
        &self,
        conn: impl PgExecutor<'_>,
    ) -> Result<Vec<String>, sqlx::Error> {
        let mut q = QueryBuilder::<Postgres>::new("SELECT ");
        q.push(&self.column);
        q.push(" FROM ");
        q.push(&self.table);
        q.push(" WHERE ");
        q.push(&self.column);
        q.push(" IN");
        q.push_tuples(self.args.iter().take(PG_BIND_LIMIT), |mut b, col| {
            if self.uuid {
                b.push_bind(Uuid::parse_str(col).expect("UUID's should be validated first."));
            } else {
                b.push_bind(col);
            }
        });
        let res = if self.uuid {
            let res = q.build_query_scalar::<Uuid>().fetch_all(conn).await?;
            res.iter().map(|f| f.to_string()).collect()
        } else {
            q.build_query_scalar::<String>().fetch_all(conn).await?
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::{super::tests::*, *};

    async fn get_uuids(conn: &PgPool) -> anyhow::Result<Vec<Uuid>> {
        Ok(sqlx::query_scalar!("SELECT id FROM users")
            .fetch_all(conn)
            .await?)
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_select_statement(conn: PgPool) -> anyhow::Result<()> {
        let test_cases = [
            (
                repeating_args(),
                true,
                vec!["john".to_string(), "paul".into()],
                "test repeating args",
            ),
            (
                entry_exists(),
                true,
                vec![
                    "john".into(),
                    "paul".into(),
                    "ringo".into(),
                    "george".into(),
                ],
                "test args that exist",
            ),
            (empty_args(), false, vec![], "test empty args"),
        ];

        for case in test_cases {
            let ((args, _), should_pass, want, desc) = case;
            let got = ArgumentsBuilder::new(args)
                .with_table("users")
                .with_column("name")
                .with_task("user_get")
                .try_build()?;
            let got = got.select_statement(&conn).await;

            if should_pass {
                assert_eq!(want, got?, "{desc}");
                continue;
            }
            assert!(got.is_err(), "{desc}")
        }
        anyhow::Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_select_uuid(conn: PgPool) -> anyhow::Result<()> {
        let uuids = get_uuids(&conn).await?;
        let args: Arc<[String]> = uuids.iter().map(|f| f.to_string()).collect();
        let got = ArgumentsBuilder::new(args.clone())
            .with_table("users")
            .with_column("id")
            .with_task("user_select")
            .args_are_uuid()
            .try_build()?;
        let got = got.select_statement(&conn).await?;

        assert_eq!(args, got.into(), "Test uuid for batch select statement");
        anyhow::Ok(())
    }
}
