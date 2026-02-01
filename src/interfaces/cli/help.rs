use clap::Arg;
use clap::CommandFactory;
use colored::*;
use heck::{AsSnakeCase, AsTitleCase};
use rand::seq::IndexedRandom;
use std::fmt::Write;

use crate::interfaces::cli::Cli;

const ASCII_HEADER: &str = r#"

$$$$$$$\                                                $$$$$$$\            $$\                     
$$  __$$\                                               $$  __$$\           $$ |                    
$$ |  $$ | $$$$$$\   $$$$$$\  $$\   $$\ $$\   $$\       $$ |  $$ |$$\   $$\ $$ | $$$$$$$\  $$$$$$\  
$$$$$$$  |$$  __$$\ $$  __$$\ \$$\ $$  |$$ |  $$ |      $$$$$$$  |$$ |  $$ |$$ |$$  _____|$$  __$$\ 
$$  ____/ $$ |  \__|$$ /  $$ | \$$$$  / $$ |  $$ |      $$  ____/ $$ |  $$ |$$ |\$$$$$$\  $$$$$$$$ |
$$ |      $$ |      $$ |  $$ | $$  $$<  $$ |  $$ |      $$ |      $$ |  $$ |$$ | \____$$\ $$   ____|
$$ |      $$ |      \$$$$$$  |$$  /\$$\ \$$$$$$$ |      $$ |      \$$$$$$  |$$ |$$$$$$$  |\$$$$$$$\ 
\__|      \__|       \______/ \__/  \__| \____$$ |      \__|       \______/ \__|\_______/  \_______|
                                        $$\   $$ |                                                  
                                        \$$$$$$  |                                                  
                                         \______/                                    
    "#;

fn placeholder_format(arg: &Arg, has_value_arg: bool) -> String {
    let default = if has_value_arg { "" } else { "  " };

    let placeholders = match arg.get_value_names() {
        Some(p) if arg.get_action().takes_values() => p,
        _ => return default.to_string(),
    };

    let multiple = placeholders.len() > 1;

    let mut out = String::new();

    for (i, placeholder) in placeholders.iter().enumerate() {
        out.push('<');
        out.push_str(placeholder);
        out.push('>');

        if multiple && i + 1 < placeholders.len() {
            out.push(' ');
        }
    }

    out
}

pub fn cli_help() {
    let mut custom_help_buff = String::new();
    let mut cmd = Cli::command();
    let args: Vec<_> = cmd
        .get_arguments()
        .filter(|a| a.get_short().is_some() || a.get_long().is_some())
        .collect();
    let has_value_arg = args.iter().any(|a| a.get_action().takes_values());
    let max_len_long_flag_col = args.iter().fold(0, |max_len, arg| {
        let placeholder_len = placeholder_format(arg, has_value_arg).len();
        let long_flag_len = arg.get_long().map_or(0, |l| l.len());

        max_len.max(placeholder_len + long_flag_len)
    });

    writeln!(custom_help_buff, "{}", random_color_ascii_header()).unwrap();
    writeln!(custom_help_buff, "{}", app_info_banner()).unwrap();

    writeln!(
        custom_help_buff,
        "\n{} {}",
        "Usage:".blue().bold(),
        cmd.render_usage().to_string().replace("Usage: ", "").cyan()
    )
    .unwrap();
    writeln!(custom_help_buff, "\n{}:", "Options".blue().bold()).unwrap();

    for arg in cmd
        .get_arguments()
        .filter(|a| a.get_short().is_some() || a.get_long().is_some())
    {
        let placeholder_format = placeholder_format(arg, has_value_arg);
        let is_flag_type = arg.get_action().takes_values();
        let required = arg.is_required_set().then_some(" (required)").unwrap_or("");

        arg.get_help().inspect(|help| {
            let mut flags = String::new();

            let short_flag = arg
                .get_short()
                .map_or_else(|| "    ".to_string(), |short| format!("-{}", short));
            flags.push_str(&short_flag);
            arg.get_long().inspect(|long| {
                let placeholder_pad = is_flag_type
                    .then(|| {
                        " ".repeat(max_len_long_flag_col - (long.len() + placeholder_format.len()))
                    })
                    .unwrap_or_else(|| " ".repeat(max_len_long_flag_col - long.len()));

                if !flags.is_empty() && arg.get_short().is_some() {
                    flags.push_str(", ")
                };

                flags.push_str(&format!(
                    "--{} {} {}",
                    long, placeholder_format, placeholder_pad
                ));
            });

            let arg_default = arg
                .get_default_values()
                .get(0)
                .map_or("".to_string(), |default| {
                    format!(
                        " {} {} {}",
                        "[default:".dimmed(),
                        default.to_string_lossy().yellow(),
                        "]".dimmed()
                    )
                });

            writeln!(
                custom_help_buff,
                "{} {}{}{}",
                flags.cyan(),
                help.to_string().white(),
                arg_default,
                required.red()
            )
            .unwrap();
        });
    }

    println!("{}", custom_help_buff);
}

fn random_color_ascii_header() -> String {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let mut rng = rand::rng();

    ASCII_HEADER
        .chars()
        .map(|c| {
            let color = colors.as_slice().choose(&mut rng).unwrap();
            c.to_string().color(*color).to_string()
        })
        .collect()
}

fn app_info_banner() -> String {
    let banner_padding = 5;
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");
    let info_text = format!(
        "{}Welcome to {} [v{}] Proxy Testing Tool",
        " ".repeat(banner_padding),
        AsSnakeCase(AsTitleCase(app_name).to_string()),
        app_version
    );
    let banner_frame = "=".repeat(info_text.len() + banner_padding);

    format!(
        "{frame}\n{info_text}\n{frame}",
        frame = banner_frame,
        info_text = info_text
    )
    .color(Color::Cyan)
    .to_string()
}
