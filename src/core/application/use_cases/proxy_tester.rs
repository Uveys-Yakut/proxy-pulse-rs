use crate::core::application::Error as AppError;
use crate::core::application::ports::ProxyRepository;

pub struct ProxyTester<'a> {
    repo: &'a dyn ProxyRepository,
}

impl<'a> ProxyTester<'a> {
    pub fn new(repo: &'a dyn ProxyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, max_concurrent: usize) -> Result<(), AppError> {
        let mut proxy_rx = self.repo.stream_proxies(max_concurrent).await?;

        while let Some(proxy) = proxy_rx.recv().await {
            println!("Proxy: {:?}", proxy);
        }

        Ok(())
    }
}
