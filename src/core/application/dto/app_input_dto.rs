use crate::interfaces::cli::Cli;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppInputDTO {
    pub file: Option<PathBuf>,
    pub proxies: Option<String>,
    pub timeout: u8,
    pub max_concurrent: usize,
    pub out_dir: Option<PathBuf>,
}

impl AppInputDTO {
    pub fn from_cli(cli: Cli) -> Self {
        Self {
            file: cli.file,
            proxies: cli.proxies,
            timeout: cli.timeout,
            max_concurrent: cli.max_concurrent as usize,
            out_dir: cli.out_dir,
        }
    }
}
