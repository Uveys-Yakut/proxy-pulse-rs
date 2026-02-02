use clap::{ArgGroup, Parser, value_parser};
use std::{path::PathBuf, u16};
use url::Url;

use super::error::{Error, cli_error};

#[derive(Debug, Parser)]
#[command(version)]
#[command(
    group(ArgGroup::new("input")
        .required(true)
        .multiple(false)
        .args(["file", "proxies"]))
)]
pub struct Cli {
    /// Path to the file containing the list of proxies
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// List of proxies to test (comma-separated if more than one)
    #[arg(short, long)]
    pub proxies: Option<String>,
    /// Url to test against
    #[arg(short, long, default_value = "http://httpbin.org/ip")]
    pub url: Option<Url>,
    /// Timeout duration in second (1 - 10)
    #[arg(
        short, 
        long, 
        value_name="SEC", 
        default_value_t = 3, 
        value_parser=value_parser!(u8).range(1..=10)
    )]
    pub timeout: u8,
    /// Maximum number of concurrent proxy tests (1 - 500)
    #[arg(
        short='c', 
        long="max-concurrent", 
        value_name="NUM", 
        default_value_t = 100, 
        value_parser=value_parser!(u16).range(1..=500)
    )]
    pub max_concurrent: u16,
    /// Output directory for results
    #[arg(
        long = "out-dir",
        value_name = "DIR",
        default_value = "./proxy-results"
    )]
    pub out_dir: Option<PathBuf>,
}

impl Cli {
    pub fn parse_and_validate() -> Self {
        match Self::try_parse()
            .map_err(Error::Parse)
            .and_then(|cli| cli.validate().map(|_| cli))
        {
            Ok(cli) => cli,
            Err(err) => cli_error(err),
        }
    }
}
