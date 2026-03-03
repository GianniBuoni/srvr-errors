#![cfg(feature = "postgres")]
use super::*;

impl Arguments {
    /// Check database if given args are already in the configured table column.
    /// Returns a ClientError if any are found along with offending values.
    pub async fn try_check_unique_constraint(
        &self,
        conn: impl sqlx::PgExecutor<'_>,
    ) -> Result<(), ClientError> {
        let found = match self.select_statement(conn).await {
            Ok(f) => f,
            Err(_) => return Ok(()),
        };
        match found.is_empty() {
            true => Ok(()),
            false => Err(ClientError::UniqueConstraint(found.join(", ").into())),
        }
    }
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use super::{super::tests::*, *};

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_unique_constraint_errors(conn: PgPool) -> anyhow::Result<()> {
        let test_cases = [
            (entry_exists(), false, "test non-unique error"),
            (
                entry_not_exists(),
                true,
                "test arguments unique to database",
            ),
            (repeating_args(), false, "test repeating invalid args"),
        ];
        for case in test_cases {
            let ((args, out), should_pass, desc) = case;
            let got = ArgumentsBuilder::new(args)
                .with_table("users")
                .with_column("name")
                .with_task("user_create")
                .try_build()?;
            let got = got.try_check_unique_constraint(&conn).await;

            if should_pass {
                assert!(got.is_ok(), "{desc}");
                break;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                Err(e) => {
                    let want = ClientError::UniqueConstraint(out.into()).to_string();
                    assert_eq!(want, e.to_string());
                }
            }
        }
        anyhow::Ok(())
    }
}
