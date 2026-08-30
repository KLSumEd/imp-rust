use std::env::Args;
use std::error::Error;
use std::path::PathBuf;

#[allow(dead_code)]
enum FlagType {
    Flagged(&'static str, char),
    LongFlagged(&'static str),
    Nil,
}

#[allow(dead_code)]
struct ConfigField<T> {
    value: T,
    optional: bool,
    flag: FlagType,
}

#[allow(dead_code)]
struct Config {
    filepath: ConfigField<PathBuf>,
}

impl Config {
    fn new(mut args: Args) -> Config {
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
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let config: Config = Config::new(args);
    Ok(())
}
