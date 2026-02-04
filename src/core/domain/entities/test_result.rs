use crate::core::domain::{IpAdress, Port, ProxyAnonymity, ProxyScheme};

#[derive(Debug)]
pub struct TestResult {
    ip: IpAdress,
    port: Port,
    scheme: ProxyScheme,
    latency_ms: u128,
    retries: u8,
    anonymity: ProxyAnonymity,
    score: u8,
}

impl TestResult {
    pub fn new(
        ip: IpAdress,
        port: Port,
        scheme: ProxyScheme,
        latency_ms: u128,
        retries: u8,
        anonymity: ProxyAnonymity,
        score: u8,
    ) -> Self {
        Self {
            ip,
            port,
            scheme,
            latency_ms,
            retries,
            anonymity,
            score,
        }
    }
}
