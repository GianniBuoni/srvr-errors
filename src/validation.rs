use std::{collections::HashSet, fmt::Display, sync::Arc};

use sqlx::{PgPool, types::Uuid};

use crate::prelude::*;

pub mod prelude {
    pub use super::Validation;
}

/// Struct to configure argument validation checks.
#[derive(Debug)]
pub struct Validation {
    args: Arc<[String]>,
    #[allow(dead_code)]
    table: Arc<str>,
    task: Arc<str>,
}

impl Validation {
    pub fn try_check_empty_args(&self) -> Result<&Self, ClientError> {
        if self.args.is_empty() {
            return Err(ClientError::EmptyArgs(self.task.clone()));
        }
        Ok(self)
    }
    pub fn try_check_repeated_args(&self) -> Result<&Self, ClientError> {
        let mut unique = HashSet::new();
        let mut repeat = HashSet::new();

        self.args.iter().for_each(|f| match unique.insert(f) {
            true => (),
            false => {
                repeat.insert(f);
            }
        });

        match repeat.is_empty() {
            true => Ok(self),
            false => Err(ClientError::RepeatArgs(
                repeat
                    .into_iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", "),
            )),
        }
    }
    pub async fn try_check_unique_constraint(&self, _conn: &PgPool) -> Result<&Self, ClientError> {
        todo!()
    }
    pub fn try_check_uuid(&self) -> Result<&Self, ClientError> {
        let bad = self
            .args
            .iter()
            .filter(|f| Uuid::try_parse(f).is_err())
            .cloned()
            .collect::<Vec<String>>();

        match bad.is_empty() {
            true => Ok(self),
            false => Err(ClientError::Uuid(bad.join(", "))),
        }
    }
    pub async fn try_check_if_entry_exists(&self, _conn: &PgPool) -> Result<&Self, ClientError> {
        todo!()
    }
    pub fn new(args: Arc<[String]>, table: impl Display, task: impl Display) -> Self {
        Self {
            args,
            table: Arc::from(table.to_string()),
            task: Arc::from(task.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::types::Uuid;

    use super::*;

    fn empty_args() -> (Arc<[String]>, String) {
        (Arc::new([]), "user_create".into())
    }
    fn repeating_args() -> (Arc<[String]>, String) {
        (
            Arc::new(["john".into(), "john".into(), "paul".into(), "paul".into()]),
            "john, john, paul, paul".into(),
        )
    }
    fn valid_uuids() -> (Arc<[String]>, String) {
        let ids = Arc::new([Uuid::max().to_string(), Uuid::nil().to_string()]);
        (ids, "".into())
    }
    /// Test case where arguments are expected to exist
    #[allow(dead_code)]
    fn entry_not_found() -> (Arc<[String]>, String) {
        (
            Arc::new(["balto".into(), "air bud".into()]),
            "balto, air bud".into(),
        )
    }
    /// Test case where arguments violate a table's unique constraint
    fn entry_exists() -> (Arc<[String]>, String) {
        (
            Arc::new([
                "john".into(),
                "paul".into(),
                "ringo".into(),
                "george".into(),
            ]),
            "john, paul, ringo, george".into(),
        )
    }

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        let test_cases = [
            (empty_args(), false, "test failing args"),
            (entry_exists(), true, "test completely unique args"),
            (repeating_args(), true, "test repeating args"),
        ];

        test_cases
            .into_iter()
            .try_for_each(|((args, out), should_pass, desc)| {
                let got = Validation::new(args.clone(), "users", "user_create");
                let got = got.try_check_empty_args();

                if should_pass {
                    assert!(got.is_ok(), "{desc}");
                    return anyhow::Ok(());
                }

                match got {
                    Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                    Err(e) => {
                        assert_eq!(
                            ClientError::EmptyArgs(out.into()).to_string(),
                            e.to_string(),
                            "{desc}"
                        )
                    }
                }
                anyhow::Ok(())
            })
    }

    #[test]
    fn test_repeat_args() -> anyhow::Result<()> {
        let tests_cases = [
            (empty_args(), true, "test empty args"),
            (entry_exists(), true, "test completely unique args"),
            (repeating_args(), false, "test repeating args"),
        ];

        tests_cases
            .into_iter()
            .try_for_each(|((args, _), should_pass, desc)| {
                let got = Validation::new(args.clone(), "users", "user_create");
                let got = got.try_check_repeated_args();

                if should_pass {
                    assert!(got.is_ok(), "{desc}");
                    return anyhow::Ok(());
                }

                match got {
                    Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                    Err(e) => {
                        let got = e.to_string();
                        // since hash sets do not care about order, the outputs
                        // can be variable
                        assert!(got.contains("john"), "john: {desc}");
                        assert!(got.contains("paul"), "paul: {desc}");
                        assert!(got.contains(", "), "',': {desc}");
                    }
                }
                anyhow::Ok(())
            })
    }

    #[test]
    fn test_uuid() -> anyhow::Result<()> {
        let tests_cases = [
            (empty_args(), true, "test empty args"),
            (entry_exists(), false, "test completely unique args"),
            (repeating_args(), false, "test repeating args"),
            (valid_uuids(), true, "test valid uuids"),
        ];

        tests_cases
            .into_iter()
            .try_for_each(|((args, out), should_pass, desc)| {
                let got = Validation::new(args, "users", "user_create");
                let got = got.try_check_uuid();

                if should_pass {
                    assert!(got.is_ok(), "{desc}");
                    return anyhow::Ok(());
                }

                match got {
                    Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                    Err(e) => {
                        let want = ClientError::Uuid(out).to_string();
                        assert_eq!(want, e.to_string(), "{desc}");
                    }
                }
                anyhow::Ok(())
            })
    }
}
