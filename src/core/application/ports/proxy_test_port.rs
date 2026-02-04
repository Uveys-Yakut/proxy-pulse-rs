use crate::core::domain::error::ProxyTestError;
use crate::core::domain::{Proxy, TestResult};
use async_trait::async_trait;

#[async_trait]
pub trait ProxyTestPort: Send + Sync {
    async fn test(&self, proxy: Proxy) -> Result<TestResult, ProxyTestError>;
}
