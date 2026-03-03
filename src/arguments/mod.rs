use std::{fmt::Display, sync::Arc};

use crate::prelude::*;

pub mod prelude {
    pub use super::{Arguments, ArgumentsBuilder};
}

mod try_check_empty_args;
mod try_check_repeat_args;
mod try_check_uuid;

/// Configures argument validation checks.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Arguments {
    args: Arc<[String]>,
    column: Arc<str>,
    table: Arc<str>,
    task: Arc<str>,
    uuid: bool,
}

/// Builder type for Arguments.
#[derive(Default, Debug)]
pub struct ArgumentsBuilder {
    args: Arc<[String]>,
    column: Option<Arc<str>>,
    table: Option<Arc<str>>,
    task: Option<Arc<str>>,
    uuid: bool,
}

impl ArgumentsBuilder {
    pub fn new(args: Arc<[String]>) -> Self {
        Self {
            args,
            ..Default::default()
        }
    }
    pub fn with_column(mut self, column: impl Display) -> Self {
        self.column = Some(Arc::from(column.to_string()));
        self
    }
    pub fn with_table(mut self, table: impl Display) -> Self {
        self.table = Some(Arc::from(table.to_string()));
        self
    }
    pub fn with_task(mut self, task: impl Display) -> Self {
        self.task = Some(Arc::from(task.to_string()));
        self
    }
    pub fn args_are_uuid(mut self) -> Self {
        self.uuid = true;
        self
    }
    pub fn try_build(self) -> Result<Arguments, ValConfigError> {
        let args = Arguments {
            args: self.args,
            column: self.column.ok_or(ValConfigError::Arguments("column"))?,
            table: self.table.ok_or(ValConfigError::Arguments("table"))?,
            task: self.task.ok_or(ValConfigError::Arguments("task"))?,
            uuid: self.uuid,
        };
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn empty_args() -> (Arc<[String]>, String) {
        (Arc::new([]), "user_create".into())
    }
    pub fn repeating_args() -> (Arc<[String]>, String) {
        (
            Arc::new(["john".into(), "john".into(), "paul".into(), "paul".into()]),
            "john, john, paul, paul".into(),
        )
    }
    /// Test case where arguments violate a table's unique constraint
    pub fn entry_exists() -> (Arc<[String]>, String) {
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
    fn test_errors() -> anyhow::Result<()> {
        let test_cases = [
            (
                ArgumentsBuilder::new(empty_args().0),
                false,
                "column",
                "test unconfigured builder",
            ),
            (
                ArgumentsBuilder::new(empty_args().0).with_column("names"),
                false,
                "table",
                "test configured column",
            ),
            (
                ArgumentsBuilder::new(empty_args().0)
                    .with_column("names")
                    .with_table("users"),
                false,
                "task",
                "test configured column and table",
            ),
            (
                ArgumentsBuilder::new(empty_args().0)
                    .with_column("names")
                    .with_table("users")
                    .with_task("user_create"),
                true,
                "",
                "test fully configrued argument validations",
            ),
            (
                ArgumentsBuilder::new(empty_args().0)
                    .with_column("names")
                    .with_table("users")
                    .with_task("user_create")
                    .args_are_uuid(),
                true,
                "",
                "test fully configrued argument validation w/ optional uuid",
            ),
        ];
        test_cases
            .into_iter()
            .try_for_each(|(builder, should_pass, out, desc)| {
                let got = builder.try_build();

                if should_pass {
                    assert!(got.is_ok(), "{desc}");
                    return anyhow::Ok(());
                }
                match got {
                    Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                    Err(e) => {
                        let want = ValConfigError::Arguments(out).to_string();
                        assert_eq!(want, e.to_string(), "{desc}")
                    }
                }
                anyhow::Ok(())
            })
    }
}
