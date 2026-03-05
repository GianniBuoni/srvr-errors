use sqlx::types::Uuid;

use super::*;

impl ArgumentsCheckedRepeating {
    pub fn try_check_uuid(self) -> Result<Self, ClientError> {
        let bad = self
            .0
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
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    pub fn valid_uuids() -> (Arc<[String]>, String) {
        let ids = Arc::new([Uuid::max().to_string(), Uuid::nil().to_string()]);
        (ids, "".into())
    }

    #[test]
    fn test_uuid() -> anyhow::Result<()> {
        let tests_cases = [
            (entry_exists(), false, "test completely unique args"),
            (valid_uuids(), true, "test valid uuids"),
        ];

        tests_cases
            .into_iter()
            .try_for_each(|((args, out), should_pass, desc)| {
                let got = ArgumentsBuilder::new(args)
                    .with_task("user_get")
                    .with_table("users")
                    .with_column("uuid")
                    .args_are_uuid()
                    .try_build()?;

                let got = got
                    .try_check_empty_args()?
                    .try_check_repeated_args()?
                    .try_check_uuid();

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
