use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};

pub fn get_client_ip(behind_proxy: bool, headers: &HeaderMap, socket_addr: SocketAddr) -> IpAddr {
    if behind_proxy {
        return get_proxy_client_ip(&headers).unwrap_or_else(|| socket_addr.ip());
    } else {
        // Use direct connection ip address
        return socket_addr.ip();
    }
}

pub fn get_proxy_client_ip(headers: &HeaderMap) -> Option<std::net::IpAddr> {
    // Try different proxy headers in order of preference
    let header_names = [
        "x-forwarded-for",
        "x-real-ip",
        "cf-connecting-ip", // Cloudflare
        "x-client-ip",
    ];

    for header_name in &header_names {
        if let Some(value) = headers.get(*header_name) {
            if let Ok(ip_str) = value.to_str() {
                // X-Forwarded-For can have multiple IPs, take the first one
                let first_ip = ip_str.split(',').next().unwrap_or(ip_str).trim();
                if let Ok(ip) = first_ip.parse() {
                    return Some(ip);
                }
            }
        }
    }

    None
}
