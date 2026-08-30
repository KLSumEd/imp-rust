use std::env::Args;
use std::error::Error;

#[allow(dead_code)]
mod config {
    use super::Args;
    use std::path::{Path, PathBuf};

    enum FlagType {
        Flagged(&'static str, char),
        LongFlagged(&'static str),
        Nil,
    }

    struct ConfigField<T> {
        value: T,
        optional: bool,
        flag: FlagType,
    }

    impl<T> ConfigField<T> {
        pub fn is_optional(&self) -> &bool {
            &self.optional
        }

        pub fn get_flags(&self) -> (Option<&str>, Option<char>) {
            match self.flag {
                FlagType::Flagged(long_flag, short_flag) => {
                    (Some(long_flag), Some(short_flag))
                }
                FlagType::LongFlagged(long_flag) => (Some(long_flag), None),
                FlagType::Nil => (None, None),
            }
        }

        pub fn get_value(&self) -> &T {
            &self.value
        }
    }

    pub struct Config {
        filepath: ConfigField<PathBuf>,
    }

    impl Config {
        pub fn new(mut args: Args) -> Config {
            // skip first arg — binary executable path
            args.next()
                .expect("found no initial argument! what happened?!");

            let filepath_str: String =
                args.next().expect("imp binary found no args");
            let filepath: ConfigField<PathBuf> = ConfigField {
                value: PathBuf::from(filepath_str),
                optional: false,
                flag: FlagType::Nil,
            };

            Config { filepath }
        }

        pub fn get_filepath(&self) -> &Path {
            Path::new(&self.filepath.value)
        }
    }
}

use config::Config;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let config: Config = Config::new(args);
    if let Some(fp) = config.get_filepath().to_str() {
        println!("{fp}");
    }
    Ok(())
}
