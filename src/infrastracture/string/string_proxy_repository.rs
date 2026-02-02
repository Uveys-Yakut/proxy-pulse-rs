use crate::core::application::{Error as AppError, ports::ProxyRepository};
use crate::core::domain::Proxy;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

pub struct StringProxyRepository {
    proxies: Vec<String>,
}

impl StringProxyRepository {
    pub fn new(proxies_str: &str) -> Self {
        let proxies = proxies_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Self { proxies }
    }
}

#[async_trait]
impl ProxyRepository for StringProxyRepository {
    async fn stream_proxies(&self, max_worker: usize) -> Result<mpsc::Receiver<Proxy>, AppError> {
        let (tx, rx) = mpsc::channel::<Proxy>(self.proxies.len());
        let semaphore = Arc::new(Semaphore::new(max_worker));

        for line in self.proxies.clone() {
            if let Ok(proxy) = Proxy::from_str(&line) {
                let tx_clone = tx.clone();
                let sem_clone = semaphore.clone();

                tokio::spawn(async move {
                    let permit = sem_clone.acquire_owned().await.unwrap();
                    let _ = tx_clone.send(proxy).await;
                    drop(permit);
                });
            }
        }

        Ok(rx)
    }
}
