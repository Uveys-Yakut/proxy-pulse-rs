use crate::core::domain::Error as DomainError;
use crate::core::domain::Proxy;
use crate::interfaces::cli::Cli;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone)]
pub struct AppInputDTO {
    pub file: Option<PathBuf>,
    pub proxies: Option<Vec<Proxy>>,
    pub url: Option<Url>,
    pub timeout: u8,
    pub workers: u16,
    pub out_dir: Option<PathBuf>,
}

impl AppInputDTO {
    pub fn from_cli(cli: Cli) -> Self {
        let proxies = match cli.proxies {
            Some(raw) => {
                let parsed: Vec<Proxy> = raw
                    .split(',')
                    .map(str::trim)
                    .filter_map(|s| Proxy::from_str(s).ok())
                    .collect();

                if parsed.is_empty() {
                    eprintln!("{}", DomainError::EmptyProxyList);
                    std::process::exit(1);
                }

                Some(parsed)
            }
            None => None,
        };

        Self {
            file: cli.file,
            url: cli.url,
            proxies: proxies,
            timeout: cli.timeout,
            workers: cli.workers,
            out_dir: cli.out_dir,
        }
    }
}
