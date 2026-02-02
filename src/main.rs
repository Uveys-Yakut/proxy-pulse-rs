mod core;
mod infrastracture;
mod interfaces;

use core::application::dto::AppInputDTO;
use interfaces::cli::Cli;

use crate::{
    core::application::use_cases::ProxyTester,
    infrastracture::{file::FileProxyRepository, string::StringProxyRepository},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse_and_validate();
    let app_dto = AppInputDTO::from_cli(cli);

    if let Some(file) = app_dto.file {
        let file_repo = FileProxyRepository::new(file);
        let tester = ProxyTester::new(&file_repo);

        let _ = tester.execute(app_dto.max_concurrent).await;
    }

    if let Some(proxies) = app_dto.proxies {
        let string_repo = StringProxyRepository::new(&proxies);
        let tester = ProxyTester::new(&string_repo);

        let _ = tester.execute(app_dto.max_concurrent).await;
    }
}
