use clap::{Arg, ArgMatches};
use std::error::Error;

// mod cli {
//     pub use clap::Parser;
//
//     /// A Command Line Interface struct deriving `clap::Parser`
//     #[derive(Parser, Debug)]
//     #[command(name = "imp")]
//     #[command(version, about, long_about = None)]
//     pub struct Cli {
//         #[arg()]
//         source_file: String,
//         #[arg()]
//         output_file: String,
//     }
// }
//
// use cli::{Cli, Parser};

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli_matches: ArgMatches = clap::command!()
        .arg(Arg::new("source_file").required(true))
        .arg(Arg::new("output_file").required(true))
        .get_matches();

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn verify_cli() {
//         use clap::CommandFactory;
//         Cli::command().debug_assert();
//     }
// }
