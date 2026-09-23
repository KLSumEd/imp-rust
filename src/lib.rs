use std::error::Error;

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

    use std::env::{self, Args};

    pub fn parse() {
        let mut args: Args = env::args();

        let Some(command) = args.next() else {
            CliError::handle_error(CliError::NoCmd)
        };

        unimplemented!()
    }

    fn parse_command() {}

    struct CliError {
        code: u16,
        name: &'static str,
        description: &'static str,
    }

    impl CliError {
        fn handle_error(error: CliError) -> ! {
            todo!()
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    cli::parse();
    Ok(())
}
