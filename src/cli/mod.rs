mod interface;

use clap::{ArgGroup, Parser};
use std::path::PathBuf;

use crate::cli::interface::cli_error;

#[derive(Debug, Parser)]
#[command(version)]
#[command(group(ArgGroup::new("input").required(true).multiple(false).args(["file", "proxies"])))]
pub struct Cli {
    /// Path to the file containing the list of proxies
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// List of proxies to test (comma-separated if more than one)
    #[arg(short, long)]
    proxies: Option<String>,
    /// Url to test against
    #[arg(short, long, default_value = "http://httpbin.org/ip")]
    url: String,
    /// Timeout duration in second
    #[arg(short, long, default_value_t = 5)]
    timeout: u8,
    /// Number of proxies to test simultaneously
    #[arg(short, long, default_value_t = 100)]
    workers: u16,
    /// Enable this option to test SOCKS5 proxies
    #[arg(short, long)]
    socks: bool,
    /// Path to the file where succesful proxies will be saved
    #[arg(short, long, default_value = "working_proxies.txt")]
    output: PathBuf,
}

pub fn cli_parse() -> Cli {
    match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            cli_error(err);
            std::process::exit(1);
        }
    }
}
