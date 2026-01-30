use crate::cli::Cli;
use crate::interface::cli_error;
use clap::Parser;

pub fn cli_parse() -> Cli {
    match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            cli_error(err);
            std::process::exit(1);
        }
    }
}
