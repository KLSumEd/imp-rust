use std::env::Args;
use std::error::Error;
use std::path::Path;

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

struct Config<'a> {
    filepath: ConfigField<&'a Path>,
}

impl Config<'static> {
    fn new<'b>(args: &mut Args) -> Config<'b> {
        let args: Vec<String> = args.collect();

        Config {
            filepath: ConfigField {
                value: Path::new("test.bmp"),
                optional: false,
                flag: FlagType::Nil,
            },
        }
    }
}

pub fn run(mut args: Args) -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");
    let _config: Config = Config::new(&mut args);
    Ok(())
}
