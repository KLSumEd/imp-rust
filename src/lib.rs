use anyhow::{self, Result};

mod cli {
    // TODO: Implement CLI -- Kai
    //      1. Must be extensible
    //      2. Must be constant (i.e. processed at compile-time)
    //      3. Attempt to follow Open-Closed principle (i.e. should be able
    //          to add new commands/options without modifying existing structure)
    //
    //  Consider using imperative approach
    //      - Match command
    //      - command_name => handler_fn()
    //      - fn handler_fn() { /* do custom arg parsing */ }
    //      - For fundamental parsing, use shared parse_arg() function(s)

    use super::Result;
    use std::env::{self, Args};
    use thiserror::Error;

    pub fn parse() -> Result<()> {
        let mut args: Args = env::args();

        if args.next() == None {
            return Err(CliError::NoExePath.into());
        };

        let Some(command) = args.next() else {
            return Err(CliError::NoCommand.into());
        };

        parse_command(&command)
    }

    fn parse_command(command: &str) -> Result<()> {
        match command {
            "build" => parse_cmd_build(),
            "test" => parse_cmd_test(),
            "lint" => parse_cmd_lint(),
            "format" => parse_cmd_format(),
            _other => Err(CliError::UnknownCommand(String::from(_other)).into()),
        }
    }

    fn parse_cmd_build() -> Result<()> {
        todo!();
    }

    fn parse_cmd_test() -> Result<()> {
        todo!();
    }

    fn parse_cmd_lint() -> Result<()> {
        todo!();
    }

    fn parse_cmd_format() -> Result<()> {
        todo!();
    }

    #[allow(unused)]
    #[derive(Error, Debug)]
    enum CliError {
        #[error("initial argument was not the path to this executable")]
        NoExePath,

        #[error("no command was given")]
        NoCommand,

        #[error("unrecognised command `{0:?}`")]
        UnknownCommand(String),
    }
}

pub fn run() -> Result<()> {
    cli::parse()
}
