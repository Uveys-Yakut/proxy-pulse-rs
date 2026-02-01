mod core;
mod interfaces;

use core::application::dto::AppInputDTO;
use interfaces::cli::Cli;

fn main() {
    let cli = Cli::parse_and_validate();
    let app_dto = AppInputDTO::from_cli(cli);

    println!("Input: {:#?}", app_dto);
}
