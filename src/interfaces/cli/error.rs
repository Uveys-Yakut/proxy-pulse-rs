use super::help::cli_help;
use clap::error::ErrorKind;
use colored::*;
use std::fmt;
use std::fmt::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ValidationError {
    FileNotFound(PathBuf),
    NotAFile(PathBuf),
    NotADirectory(PathBuf),
    DirectoryCreationFailed(PathBuf, std::io::Error),
    UrlSchemeInvalid(String),
}

#[derive(Debug)]
pub enum Error {
    Parse(clap::Error),
    Validation(ValidationError),
}

pub fn cli_error(err: Error) -> ! {
    match err {
        Error::Parse(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                cli_help();
            }
            ErrorKind::DisplayVersion => {
                println!(
                    "{} {}",
                    env!("CARGO_PKG_NAME").blue().bold(),
                    env!("CARGO_PKG_VERSION").cyan()
                );
            }
            _ => {
                cli_error_formatter(e.to_string());
            }
        },
        Error::Validation(e) => eprintln!("{}", e),
    }

    std::process::exit(1);
}

fn cli_error_formatter(err_str: String) {
    let mut format_buff = String::new();

    err_str
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .for_each(|error_line| {
            if let Some((head, content)) = error_line.split_once(' ') {
                let _ = match head.to_lowercase().as_str() {
                    "error:" => {
                        writeln!(format_buff, "{} {}\n", "Error:".red(), content.bright_red())
                    }
                    "usage:" => writeln!(format_buff, "{} {}\n", "Usage:".yellow(), content.cyan()),
                    _ => writeln!(
                        format_buff,
                        "{}",
                        "For more information, try '--help'.".blue()
                    ),
                };
            }
        });

    println!("{}", format_buff);
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_head = "Error:".red();

        match self {
            ValidationError::FileNotFound(path) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("The file '{}' was not found.", path.display()).bright_red()
                )
            }
            ValidationError::NotAFile(path) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("The path '{}' exists but is not a file.", path.display()).bright_red()
                )
            }
            ValidationError::NotADirectory(path) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("'{}' exists but is not a directory", path.display()).bright_red()
                )
            }
            ValidationError::DirectoryCreationFailed(path, err) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("Failed to create directory '{}': {}", path.display(), err)
                        .bright_red()
                )
            }
            ValidationError::UrlSchemeInvalid(url) => {
                write!(
                    f,
                    "{} {}",
                    error_head,
                    format!("URL must use http or https scheme, got '{}'", url).bright_red()
                )
            }
        }
    }
}
