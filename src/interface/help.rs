use clap::CommandFactory;
use colored::*;
use heck::{AsSnakeCase, AsTitleCase};
use rand::seq::IndexedRandom;
use std::fmt::Write;

use crate::cli::Cli;

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

pub fn cli_help() {
    let mut custom_help_buff = String::new();
    let mut cmd = Cli::command();
    let max_len_long_flag_col = cmd
        .get_arguments()
        .filter(|a| a.get_short().is_some())
        .fold(0, |max_len, arg| {
            let arg_id_len = arg.get_id().to_string().len();
            let long_flag_len = arg.get_long().map_or(0, |l| l.len());

            max_len.max(arg_id_len + long_flag_len)
        });
    let has_value_arg = cmd
        .get_arguments()
        .filter(|a| a.get_action().takes_values())
        .next()
        .is_some();

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
        let is_flag_type = arg.get_action().takes_values();
        let placeholder = arg.get_id().to_string().to_uppercase();
        let placeholder_format = is_flag_type
            .then(|| format!("<{}>", placeholder))
            .unwrap_or_else(|| {
                has_value_arg
                    .then_some("  ".to_string())
                    .unwrap_or("".to_string())
            });
        let required = arg.is_required_set().then_some(" (required)").unwrap_or("");

        arg.get_help().inspect(|help| {
            let mut flags = String::new();

            arg.get_short()
                .inspect(|short| flags.push_str(&format!("-{}", short)));
            arg.get_long().inspect(|long| {
                let placeholder_pad = is_flag_type
                    .then(|| " ".repeat(max_len_long_flag_col - (long.len() + placeholder.len())))
                    .unwrap_or_else(|| " ".repeat(max_len_long_flag_col - long.len()));

                (!flags.is_empty()).then(|| flags.push_str(", "));
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
