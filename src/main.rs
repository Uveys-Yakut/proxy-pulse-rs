mod core;
mod infrastracture;
mod interfaces;

use core::application::dto::AppInputDTO;
use interfaces::cli::Cli;
use std::sync::Arc;

use crate::{
    core::application::use_cases::{self, ProxyTester},
    infrastracture::{
        file::FileProxyRepository, proxy_test::ReqwestProxyTestService,
        string::StringProxyRepository,
    },
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse_and_validate();
    let app_dto = AppInputDTO::from_cli(cli);
    let tester = Arc::new(ReqwestProxyTestService::new(app_dto.timeout));

    if let Some(file) = app_dto.file {
        let file_repo = Arc::new(FileProxyRepository::new(file));
        let use_case = ProxyTester::new(file_repo, tester.clone(), app_dto.max_concurrent);

        let _ = use_case.execute().await;
    }

    if let Some(proxies) = app_dto.proxies {
        let string_repo = Arc::new(StringProxyRepository::new(&proxies));
        let use_cases = ProxyTester::new(string_repo, tester.clone(), app_dto.max_concurrent);

        let _ = use_cases.execute().await;
    }
}
