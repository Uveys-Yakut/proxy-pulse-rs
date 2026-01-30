use super::help::cli_help;
use clap::error::ErrorKind;
use colored::*;
use std::fmt::Write;

pub fn cli_error(err: clap::Error) {
    match err.kind() {
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
            error_formatter(err.to_string());
        }
    }
}

fn error_formatter(err_str: String) {
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
