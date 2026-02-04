use url::Url;

use super::value_objects::{IpAdress, Port};
use crate::core::domain::Error as DomainError;
use std::{fmt, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProxyScheme {
    Http,
    Https,
    Socks5h,
    Socks5,
    Socks4,
}

#[derive(Debug)]
pub enum ProxyAnonymity {
    Elite,
    Anonymous,
    Transparent,
}

#[derive(Debug, Clone)]
pub struct Proxy {
    ip: IpAdress,
    port: Port,
    scheme: Option<ProxyScheme>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProxyCacheKey {
    ip: IpAdress,
    port: Port,
    scheme: ProxyScheme,
}

impl ProxyScheme {
    pub fn get_all_scheme() -> Vec<ProxyScheme> {
        vec![
            Self::Http,
            Self::Https,
            Self::Socks5h,
            Self::Socks5,
            Self::Socks4,
        ]
    }

    pub fn get_test_url(&self) -> Url {
        match self {
            Self::Http => Url::parse("http://example.com").unwrap(),
            _ => Url::parse("https://ipinfo.io/ip").unwrap(),
        }
    }

    pub fn get_header_test_url(&self) -> Url {
        match self {
            ProxyScheme::Http => Url::parse("http://httpbin.org/headers").unwrap(),
            _ => Url::parse("https://httpbin.org/headers").unwrap(),
        }
    }

    pub fn get_ip_check_url(&self) -> Url {
        match self {
            ProxyScheme::Http => Url::parse("http://httpbin.org/ip").unwrap(),
            _ => Url::parse("https://httpbin.org/ip").unwrap(),
        }
    }
    /// string → enum
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "http" => Some(Self::Http),
            "https" => Some(Self::Https),
            "socks4" => Some(Self::Socks4),
            "socks5" => Some(Self::Socks5),
            "socks5h" => Some(Self::Socks5h),
            _ => None,
        }
    }
}

impl fmt::Display for ProxyScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Socks5h => "socks5h",
            Self::Socks5 => "socks5",
            Self::Socks4 => "socks4",
        };
        write!(f, "{}", s)
    }
}

impl ProxyCacheKey {
    pub fn new(ip: IpAdress, port: Port, scheme: ProxyScheme) -> Self {
        Self { ip, port, scheme }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.ip.0, self.port.0)
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
