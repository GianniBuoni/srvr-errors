use super::*;

impl Arguments {
    pub fn try_check_empty_args(&self) -> Result<&Self, ClientError> {
        if self.args.is_empty() {
            return Err(ClientError::EmptyArgs(self.task.clone()));
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

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
                let got = ArgumentsBuilder::new(args)
                    .with_task("user_create")
                    .with_column("namen")
                    .with_table("users")
                    .try_build()?;

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
}
