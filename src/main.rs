mod cli;

use cli::cli_parse;

fn main() {
    let cli = cli_parse();

    println!("Input: {:#?}", cli);
}
