use super::value_objects::{IpAdress, Port};
use crate::core::domain::Error as DomainError;

#[derive(Debug, Clone)]
pub enum ProxyScheme {
    HTTP,
    HTTPS,
    SOCKS4,
    SOCKS5,
}

#[derive(Debug, Clone)]
pub struct Proxy {
    ip: IpAdress,
    port: Port,
    scheme: Option<ProxyScheme>,
}

impl ProxyScheme {
    /// string → enum
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "http" => Some(Self::HTTP),
            "https" => Some(Self::HTTPS),
            "socks4" => Some(Self::SOCKS4),
            "socks5" => Some(Self::SOCKS5),
            _ => None,
        }
    }
}

impl Proxy {
    pub fn ip(&self) -> &IpAdress {
        &self.ip
    }

    pub fn port(&self) -> &Port {
        &self.port
    }

    pub fn scheme(&self) -> &Option<ProxyScheme> {
        &self.scheme
    }

    pub fn new(ip: IpAdress, port: Port, scheme: Option<ProxyScheme>) -> Self {
        Self {
            ip: ip,
            port: port,
            scheme: scheme,
        }
    }

    pub fn from_str(s: &str) -> Result<Proxy, DomainError> {
        let mut scheme = None;
        let mut remainder = s;

        if let Some(pos) = s.find("://") {
            scheme = Some(
                ProxyScheme::parse(&s[..pos])
                    .ok_or(DomainError::InvalidScheme(s[..pos].to_string()))?,
            );
            remainder = &s[pos + 3..];
        }

        let parts: Vec<&str> = remainder.split(':').collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidIp(s.to_string()));
        }

        let ip = parts[0].to_string();
        if ip.is_empty() {
            return Err(DomainError::InvalidIp(s.to_string()));
        }

        let port: u16 = parts[1]
            .parse()
            .map_err(|_| DomainError::InvalidPort(s.to_string()))?;

        if port == 0 || port > 65534 {
            return Err(DomainError::InvalidPort(s.to_string()));
        }

        Ok(Self::new(IpAdress(ip), Port(port.to_string()), scheme))
    }
}
