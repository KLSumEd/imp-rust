use std::error::Error;

mod cli {
    use std::path::PathBuf;

    // TODO: Implement CLI -- Kai
    //      1. Must be extensible
    //      2. Must be constant (i.e. processed at compile-time)
    //      3. Attempt to follow Open-Closed principle (i.e. should be able
    //          to add new commands/options without modifying existing structure)

    /// A Command Line Interface struct deriving `clap::Parser`
    #[derive(Debug)]
    pub struct Cli {
        command: &'static Command,
    }

    impl Cli {
        const CMD_BUILD: &'static Command<1> = &Command {
            name: "build",
            args: [&Arg {
                name: "source_file(s)",
                arg_type: &ArgType::PATHS,
            }],
        };
        const CMD_TEST: &'static Command<1> = &Command {
            name: "test",
            args: [&Arg {
                name: "test_file(s)",
                arg_type: &ArgType::PATHS,
            }],
        };
        const CMD_LINT: &'static Command<1> = todo!();
        const CMD_FORMAT: &'static Command<1> = todo!();
    }

    impl Cli {
        pub fn parse() -> Self {
            unimplemented!();
        }
    }

    #[derive(Debug)]
    struct Command<const N: usize> {
        name: &'static str,
        args: [&'static Arg; N],
    }

    #[derive(Debug)]
    struct Arg {
        name: &'static str,
        arg_type: &'static ArgType,
    }

    #[derive(Debug)]
    enum ArgType {
        PATH,
        BOOL,
        PATHS,
        STRINGS,
    }

    #[derive(Debug)]
    enum Opt<T> {
        Short {
            id: &'static str,
            desc: &'static str,
            value: T,
        },
        Long {
            id: &'static str,
            desc: &'static str,
            value: T,
        },
        Full {
            short_id: &'static str,
            desc: &'static str,
            value: T,
        },
    }

    impl<T> Opt<T> {
        pub fn get_value(self: Self) -> T {
            match self {
                Opt::Short { value: val, .. } => val,
                Opt::Long { value: val, .. } => val,
                Opt::Full { value: val, .. } => val,
            }
        }
    }
}

use cli::Cli;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Cli = Cli::parse();
    dbg!(args);
    Ok(())
}
