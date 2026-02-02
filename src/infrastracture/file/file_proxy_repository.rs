use crate::core::application::{Error as AppError, ports::ProxyRepository};
use crate::core::domain::Proxy;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Semaphore;
use tokio::sync::mpsc;

pub struct FileProxyRepository {
    path: PathBuf,
}

impl FileProxyRepository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait]
impl ProxyRepository for FileProxyRepository {
    async fn stream_proxies(
        &self,
        max_concurrent: usize,
    ) -> Result<mpsc::Receiver<Proxy>, AppError> {
        let (tx, rx) = mpsc::channel::<Proxy>(100);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let path = self.path.clone();

        tokio::spawn(async move {
            let file = match File::open(&path).await {
                Ok(f) => f,
                Err(_) => return,
            };
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok(proxy) = Proxy::from_str(line) {
                    let tx_clone = tx.clone();
                    let sem_clone = semaphore.clone();

                    tokio::spawn(async move {
                        let permit = sem_clone.acquire_owned().await.unwrap();
                        if tx_clone.send(proxy).await.is_err() {
                            return;
                        }
                        drop(permit);
                    });
                }
            }
        });

        Ok(rx)
    }
}
