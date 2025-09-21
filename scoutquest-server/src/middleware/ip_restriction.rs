use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use ipnet::IpNet;
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use crate::NetworkConfig;

#[derive(Debug, Clone)]
pub struct IpRestrictionMiddleware {
    enabled: bool,
    allowed_cidrs: Vec<IpNet>,
    denied_cidrs: Vec<IpNet>,
    deny_action: DenyAction,
    trust_proxy_headers: bool,
}

#[derive(Debug, Clone)]
pub enum DenyAction {
    Reject,
    LogOnly,
}

impl FromStr for DenyAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "reject" => Ok(DenyAction::Reject),
            "log_only" => Ok(DenyAction::LogOnly),
            _ => Err(anyhow::anyhow!("Invalid deny_action: {}", s)),
        }
    }
}

impl IpRestrictionMiddleware {
    pub fn new(config: &NetworkConfig) -> anyhow::Result<Self> {
        if !config.enabled {
            return Ok(Self {
                enabled: false,
                allowed_cidrs: vec![],
                denied_cidrs: vec![],
                deny_action: DenyAction::Reject,
                trust_proxy_headers: config.trust_proxy_headers,
            });
        }

        let allowed_cidrs: Result<Vec<_>, _> = config
            .allowed_cidrs
            .iter()
            .map(|s| s.parse::<IpNet>())
            .collect();

        let allowed_cidrs =
            allowed_cidrs.map_err(|e| anyhow::anyhow!("Invalid CIDR in allowed_cidrs: {}", e))?;

        let denied_cidrs = if let Some(denied) = &config.denied_cidrs {
            let denied_cidrs: Result<Vec<_>, _> =
                denied.iter().map(|s| s.parse::<IpNet>()).collect();
            denied_cidrs.map_err(|e| anyhow::anyhow!("Invalid CIDR in denied_cidrs: {}", e))?
        } else {
            vec![]
        };

        let deny_action = config.deny_action.parse()?;

        if allowed_cidrs.is_empty() {
            return Err(anyhow::anyhow!(
                "allowed_cidrs cannot be empty when network restrictions are enabled"
            ));
        }

        Ok(Self {
            enabled: true,
            allowed_cidrs,
            denied_cidrs,
            deny_action,
            trust_proxy_headers: config.trust_proxy_headers,
        })
    }

    fn extract_client_ip(
        &self,
        headers: &HeaderMap,
        connect_info: &ConnectInfo<SocketAddr>,
    ) -> IpAddr {
        if self.trust_proxy_headers {
            // Try X-Forwarded-For first
            if let Some(xff) = headers.get("x-forwarded-for") {
                if let Ok(xff_str) = xff.to_str() {
                    // Take the first IP in the chain (original client)
                    if let Some(first_ip) = xff_str.split(',').next() {
                        if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                            tracing::debug!("Using X-Forwarded-For IP: {}", ip);
                            return ip;
                        }
                    }
                }
            }

            // Try X-Real-IP
            if let Some(real_ip) = headers.get("x-real-ip") {
                if let Ok(real_ip_str) = real_ip.to_str() {
                    if let Ok(ip) = real_ip_str.parse::<IpAddr>() {
                        tracing::debug!("Using X-Real-IP: {}", ip);
                        return ip;
                    }
                }
            }
        }

        // Fallback to connection info
        let ip = connect_info.0.ip();
        tracing::debug!("Using connection IP: {}", ip);
        ip
    }

    fn is_ip_allowed(&self, ip: IpAddr) -> bool {
        // First check denied CIDRs (blacklist has priority)
        for denied_cidr in &self.denied_cidrs {
            if denied_cidr.contains(&ip) {
                tracing::warn!("IP {} is explicitly denied by CIDR {}", ip, denied_cidr);
                return false;
            }
        }

        // Then check allowed CIDRs (whitelist)
        for allowed_cidr in &self.allowed_cidrs {
            if allowed_cidr.contains(&ip) {
                tracing::debug!("IP {} is allowed by CIDR {}", ip, allowed_cidr);
                return true;
            }
        }

        tracing::warn!("IP {} is not in any allowed CIDR range", ip);
        false
    }
}

pub async fn ip_restriction_layer(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(restriction): State<Arc<IpRestrictionMiddleware>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip if middleware is disabled
    if !restriction.enabled {
        return Ok(next.run(req).await);
    }

    let headers = req.headers();
    let client_ip = restriction.extract_client_ip(headers, &ConnectInfo(addr));

    if !restriction.is_ip_allowed(client_ip) {
        match restriction.deny_action {
            DenyAction::Reject => {
                tracing::warn!(
                    "Access denied for IP {} - returning 403 Forbidden",
                    client_ip
                );
                return Err(StatusCode::FORBIDDEN);
            }
            DenyAction::LogOnly => {
                tracing::warn!(
                    "Access would be denied for IP {} (log_only mode - allowing request)",
                    client_ip
                );
                // Continue with the request in log-only mode
            }
        }
    } else {
        tracing::debug!("Access granted for IP {}", client_ip);
    }

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_cidr_matching() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec!["10.42.0.0/16".to_string()],
            denied_cidrs: None,
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        let middleware = IpRestrictionMiddleware::new(&config).unwrap();

        // Should be allowed
        assert!(middleware.is_ip_allowed("10.42.1.100".parse().unwrap()));
        assert!(middleware.is_ip_allowed("10.42.255.255".parse().unwrap()));
        assert!(middleware.is_ip_allowed("10.42.0.1".parse().unwrap()));

        // Should be denied
        assert!(!middleware.is_ip_allowed("10.43.1.100".parse().unwrap()));
        assert!(!middleware.is_ip_allowed("192.168.1.100".parse().unwrap()));
        assert!(!middleware.is_ip_allowed("127.0.0.1".parse().unwrap()));
    }

    #[test]
    fn test_ipv6_support() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec!["fe80::/64".to_string()],
            denied_cidrs: None,
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        let middleware = IpRestrictionMiddleware::new(&config).unwrap();

        // Should be allowed
        assert!(middleware.is_ip_allowed("fe80::1".parse().unwrap()));
        assert!(middleware.is_ip_allowed("fe80::ffff:ffff:ffff:ffff".parse().unwrap()));

        // Should be denied
        assert!(!middleware.is_ip_allowed("fe81::1".parse().unwrap()));
        assert!(!middleware.is_ip_allowed("::1".parse().unwrap()));
    }

    #[test]
    fn test_denied_cidrs_priority() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec!["10.0.0.0/8".to_string()],
            denied_cidrs: Some(vec!["10.42.0.0/16".to_string()]),
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        let middleware = IpRestrictionMiddleware::new(&config).unwrap();

        // Should be allowed (in 10.0.0.0/8 but not in denied range)
        assert!(middleware.is_ip_allowed("10.1.1.1".parse().unwrap()));
        assert!(middleware.is_ip_allowed("10.43.1.1".parse().unwrap()));

        // Should be denied (in denied range even though in allowed range)
        assert!(!middleware.is_ip_allowed("10.42.1.1".parse().unwrap()));
        assert!(!middleware.is_ip_allowed("10.42.255.255".parse().unwrap()));
    }

    #[test]
    fn test_log_only_mode() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec!["127.0.0.0/8".to_string()],
            denied_cidrs: None,
            deny_action: "log_only".to_string(),
            trust_proxy_headers: true,
        };

        let middleware = IpRestrictionMiddleware::new(&config).unwrap();

        match middleware.deny_action {
            DenyAction::LogOnly => assert!(true),
            DenyAction::Reject => panic!("Expected LogOnly mode"),
        }
    }

    #[test]
    fn test_disabled_middleware() {
        let config = NetworkConfig {
            enabled: false,
            allowed_cidrs: vec!["127.0.0.0/8".to_string()],
            denied_cidrs: None,
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        let middleware = IpRestrictionMiddleware::new(&config).unwrap();
        assert!(!middleware.enabled);
    }

    #[test]
    fn test_invalid_cidr_config() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec!["invalid-cidr".to_string()],
            denied_cidrs: None,
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        assert!(IpRestrictionMiddleware::new(&config).is_err());
    }

    #[test]
    fn test_empty_allowed_cidrs() {
        let config = NetworkConfig {
            enabled: true,
            allowed_cidrs: vec![],
            denied_cidrs: None,
            deny_action: "reject".to_string(),
            trust_proxy_headers: true,
        };

        assert!(IpRestrictionMiddleware::new(&config).is_err());
    }
}
