use colored::*;
use std::fmt;

#[derive(Debug)]
pub enum ProxyTestError {
    ConnectionFailed,
    Timeout,
    InvalidResponse,
    TestFailed,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidScheme(String),
    InvalidIp(String),
    InvalidPort(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_head = "Error:".red();

        match self {
            Error::InvalidScheme(s) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("Unknown or unsupported proxy scheme in '{}'. Supported schemes: http, https, socks4, socks5.", s).bright_red()
                )
            }
            Error::InvalidIp(s) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("Invalid proxy format '{}'. Expected format: host:port or scheme://host:port", s).bright_red()
                )
            }
            Error::InvalidPort(s) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!(
                        "Invalid port in proxy '{}'. Port must be a number between 1 and 65535",
                        s
                    )
                    .bright_red()
                )
            }
        }
    }
}
