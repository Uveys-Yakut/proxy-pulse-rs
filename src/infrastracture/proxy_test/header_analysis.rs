use crate::core::domain::ProxyAnonymity;

pub fn analyze_headers(body: &str) -> (bool, bool) {
    let b = body.to_lowercase();

    let transparent_headers = ["x-forwarded-for", "x-real-ip", "client-ip"];

    let proxy_headers = ["via", "forwarded", "proxy-connection"];

    let has_transparent = transparent_headers.iter().any(|h| b.contains(h));
    let has_proxy = proxy_headers.iter().any(|h| b.contains(h));

    (has_transparent, has_proxy)
}

pub fn classify_proxy(
    real_ip: &str,
    proxy_ip: &str,
    has_transparent_hdr: bool,
    has_proxy_hdr: bool,
) -> ProxyAnonymity {
    if proxy_ip == real_ip || has_transparent_hdr {
        ProxyAnonymity::Transparent
    } else if has_proxy_hdr {
        ProxyAnonymity::Anonymous
    } else {
        ProxyAnonymity::Elite
    }
}
