use std::collections::HashSet;

use super::*;

impl Arguments {
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
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

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
                let got = ArgumentsBuilder::new(args)
                    .with_task("user_create")
                    .with_column("name")
                    .with_table("users")
                    .try_build()?;

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
}
