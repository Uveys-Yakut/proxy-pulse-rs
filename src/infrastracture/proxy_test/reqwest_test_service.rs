use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use reqwest::Client;
use tokio::time::Instant;

use crate::core::{
    application::ports::ProxyTestPort,
    domain::{
        Proxy, ProxyAnonymity, ProxyCacheKey, ProxyScheme, TestResult, error::ProxyTestError,
    },
};

use super::{
    header_analysis::{analyze_headers, classify_proxy},
    scoring::calculate_score,
};

static REAL_IP: OnceCell<String> = OnceCell::new();
type ClientCache = Arc<DashMap<ProxyCacheKey, Client>>;

pub struct ReqwestProxyTestService {
    timeout: Duration,
    cache: ClientCache,
}

impl ReqwestProxyTestService {
    pub fn new(timeout: u8) -> Self {
        Self {
            timeout: Duration::from_secs(timeout as u64),
            cache: Arc::new(DashMap::new()),
        }
    }

    fn get_or_create_client(
        &self,
        key: &ProxyCacheKey,
        scheme: ProxyScheme,
    ) -> Result<Client, ProxyTestError> {
        if let Some(client) = self.cache.get(key) {
            return Ok(client.clone());
        }

        let proxy = reqwest::Proxy::all(format!("{}://{}", scheme, key.url()))
            .map_err(|_| ProxyTestError::ConnectionFailed)?;

        let client = Client::builder()
            .proxy(proxy)
            .timeout(self.timeout)
            .pool_max_idle_per_host(2)
            .tcp_keepalive(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| ProxyTestError::ConnectionFailed)?;

        self.cache.insert(key.clone(), client.clone());

        Ok(client)
    }

    async fn get_real_ip(&self) -> Result<String, ProxyTestError> {
        if let Some(ip) = REAL_IP.get() {
            return Ok(ip.clone());
        }

        let ip = Client::new()
            .get("https://api.ipify.org")
            .send()
            .await
            .map_err(|_| ProxyTestError::TestFailed)?
            .text()
            .await
            .map_err(|_| ProxyTestError::TestFailed)?
            .trim()
            .to_string();

        let _ = REAL_IP.set(ip.clone());
        Ok(ip)
    }

    async fn get_proxy_ip(
        &self,
        client: &Client,
        scheme: ProxyScheme,
    ) -> Result<String, ProxyTestError> {
        let ip = client
            .get(scheme.get_test_url())
            .send()
            .await
            .map_err(|_| ProxyTestError::TestFailed)?
            .text()
            .await
            .map_err(|_| ProxyTestError::TestFailed)?;

        Ok(ip.trim().to_string())
    }

    async fn get_headers(
        &self,
        client: &Client,
        scheme: ProxyScheme,
    ) -> Result<String, ProxyTestError> {
        client
            .get(scheme.get_header_test_url())
            .send()
            .await
            .map_err(|_| ProxyTestError::TestFailed)?
            .text()
            .await
            .map_err(|_| ProxyTestError::TestFailed)
    }

    async fn try_scheme(
        &self,
        proxy: &Proxy,
        scheme: ProxyScheme,
    ) -> Result<ProxyAnonymity, ProxyTestError> {
        let key = ProxyCacheKey::new(proxy.ip().clone(), proxy.port().clone(), scheme.clone());

        let client = self.get_or_create_client(&key, scheme.clone())?;

        let real_ip = self.get_real_ip().await?;

        let (proxy_ip, headers_body) = tokio::try_join!(
            self.get_proxy_ip(&client, scheme.clone()),
            self.get_headers(&client, scheme.clone())
        )
        .map_err(|_| ProxyTestError::TestFailed)?;

        let (transparent_hdr, proxy_hdr) = analyze_headers(&headers_body);

        Ok(classify_proxy(
            &real_ip,
            &proxy_ip,
            transparent_hdr,
            proxy_hdr,
        ))
    }
}

#[async_trait]
impl ProxyTestPort for ReqwestProxyTestService {
    async fn test(&self, proxy: Proxy) -> Result<TestResult, ProxyTestError> {
        let schemes = proxy
            .scheme()
            .clone()
            .map(|s| vec![s])
            .unwrap_or_else(ProxyScheme::get_all_scheme);

        for scheme in schemes {
            let mut retries: u8 = 0;

            for attempt in 0..=2 {
                let start = Instant::now();

                match self.try_scheme(&proxy, scheme.clone()).await {
                    Ok(anonymity) => {
                        let latency = start.elapsed().as_millis();
                        let score = calculate_score(latency, retries, &anonymity);

                        return Ok(TestResult::new(
                            proxy.ip().clone(),
                            proxy.port().clone(),
                            scheme,
                            latency,
                            retries,
                            anonymity,
                            score,
                        ));
                    }
                    Err(_) => {
                        retries += 1;
                        tokio::time::sleep(Duration::from_millis(100 * (attempt + 1))).await;
                    }
                }
            }
        }

        Err(ProxyTestError::TestFailed)
    }
}
