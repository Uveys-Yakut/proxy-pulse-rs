use crate::core::application::Error;
use crate::core::domain::Proxy;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait ProxyRepository {
    async fn stream_proxies(&self, max_concurrent: usize) -> Result<mpsc::Receiver<Proxy>, Error>;
}
