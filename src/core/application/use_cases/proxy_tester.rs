use std::sync::Arc;

use futures::StreamExt;
use futures::stream::FuturesUnordered;
use tokio::sync::Semaphore;

use crate::core::application::Error as AppError;
use crate::core::application::ports::{ProxyRepository, ProxyTestPort};

pub struct ProxyTester {
    repo: Arc<dyn ProxyRepository>,
    tester: Arc<dyn ProxyTestPort>,
    max_concurrent: usize,
}

impl ProxyTester {
    pub fn new(
        repo: Arc<dyn ProxyRepository>,
        tester: Arc<dyn ProxyTestPort>,
        max_concurrent: usize,
    ) -> Self {
        Self {
            repo,
            tester,
            max_concurrent,
        }
    }

    pub async fn execute(&self) -> Result<(), AppError> {
        let mut proxy_rx = self.repo.stream_proxies(self.max_concurrent).await?;

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let mut tasks = FuturesUnordered::new();

        while let Some(proxy) = proxy_rx.recv().await {
            let permit = semaphore.clone();
            let tester = self.tester.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = permit.acquire().await.ok();
                tester.test(proxy).await.ok()
            }));
        }

        while let Some(res) = tasks.next().await {
            if let Ok(Some(result)) = res {
                println!("✅ {:?}", result);
            }
        }

        Ok(())
    }
}
