#![cfg(feature = "postgres")]
use sqlx::PgExecutor;

use super::*;

impl ArgumentsCheckedRepeating {
    /// Checks if the arguments already exist in the current database.
    /// Returns a Client error with the offending items and table checked.
    pub async fn try_check_entry_exists(
        &self,
        conn: impl PgExecutor<'_>,
    ) -> Result<(), ClientError> {
        let found = match self.0.select_statement(conn).await {
            Ok(f) => f,
            Err(_) => todo!(),
        };
        match found.len() == self.0.args.len() {
            true => Ok(()),
            false => {
                let not_found = self
                    .0
                    .args
                    .iter()
                    .filter(|f| !found.iter().any(|d| *f == d))
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ");

                Err(ClientError::EntryNotFound(
                    self.0.table.clone(),
                    not_found.into(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use super::{super::tests::*, *};

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn test_entry_exists(conn: PgPool) -> anyhow::Result<()> {
        let test_cases = [
            (entry_exists(), true, "test entries that exists"),
            (
                entry_not_exists(),
                false,
                "test arguments unique to database",
            ),
        ];
        for case in test_cases {
            let ((args, out), should_pass, desc) = case;
            let got = ArgumentsBuilder::new(args)
                .with_table("users")
                .with_column("name")
                .with_task("user_edit")
                .try_build()?;

            let got = got
                .try_check_empty_args()?
                .try_check_repeated_args()?
                .try_check_entry_exists(&conn)
                .await;

            if should_pass {
                assert!(got.is_ok(), "{desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                Err(e) => {
                    let want = ClientError::EntryNotFound("users".into(), out.into()).to_string();
                    assert_eq!(want, e.to_string());
                }
            }
        }
        anyhow::Ok(())
    }
}
