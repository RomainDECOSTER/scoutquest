//! HTTPS server implementation with Rustls

use super::utils::{log_tls_info, sanitize_path_for_logging};
use super::{ensure_certificates, get_certificate_paths, validate_tls_config, TlsError};
use crate::{AppConfig, ScoutQuestTlsConfig, ServerConfig};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

/// Starts the HTTPS server with TLS configuration
pub async fn start_https_server(
    app: Router,
    server_config: &ServerConfig,
    tls_config: &ScoutQuestTlsConfig,
) -> anyhow::Result<()> {
    // Validate TLS configuration
    validate_tls_config(tls_config)?;

    // Log TLS configuration
    log_tls_info(
        tls_config.enabled,
        &tls_config.cert_dir,
        tls_config.auto_generate,
    );

    // Get certificate paths
    let (cert_path, key_path) = get_certificate_paths(tls_config);

    // Ensure certificates exist (generate if needed)
    ensure_certificates(&cert_path, &key_path, tls_config.auto_generate).await?;

    // Load TLS configuration
    let rustls_config = load_rustls_config(&cert_path, &key_path, tls_config).await?;

    // Create server address
    let addr = SocketAddr::from((
        server_config.host.parse::<std::net::IpAddr>()?,
        server_config.port,
    ));

    tracing::info!("🔒 Starting HTTPS server on https://{}", addr);
    tracing::info!("📋 TLS Configuration:");
    tracing::info!("   Auto-generate: {}", tls_config.auto_generate);
    tracing::info!("   Certificate: {}", sanitize_path_for_logging(&cert_path));
    tracing::info!("   Private key: {}", sanitize_path_for_logging(&key_path));
    tracing::info!("   Verify peer: {}", tls_config.verify_peer);

    // Start HTTP redirect server if enabled
    if tls_config.redirect_http.unwrap_or(false) {
        let http_port = tls_config.http_port.unwrap_or(3001);
        start_http_redirect_server(&server_config.host, http_port, server_config.port).await?;
    }

    // Start HTTPS server
    let listener = std::net::TcpListener::bind(addr)?;
    axum_server::from_tcp_rustls(listener, rustls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

/// Loads Rustls configuration from certificate files
async fn load_rustls_config(
    cert_path: &std::path::Path,
    key_path: &std::path::Path,
    _tls_config: &ScoutQuestTlsConfig,
) -> Result<RustlsConfig, TlsError> {
    tracing::info!("🔐 Loading TLS certificates...");

    let rustls_config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .map_err(|e| TlsError::CertificateLoad(format!("Failed to load TLS config: {}", e)))?;

    tracing::info!("✅ TLS certificates loaded successfully");
    Ok(rustls_config)
}

/// Starts an HTTP redirect server that redirects all traffic to HTTPS
async fn start_http_redirect_server(
    host: &str,
    http_port: u16,
    https_port: u16,
) -> anyhow::Result<()> {
    use axum::{http::Uri, response::Redirect, routing::any};

    let redirect_app = Router::new().route(
        "/*path",
        any(move |uri: Uri| async move {
            let https_uri = if https_port == 443 {
                format!("https://localhost{}", uri.path())
            } else {
                format!("https://localhost:{}{}", https_port, uri.path())
            };

            tracing::debug!("🔄 Redirecting HTTP request to: {}", https_uri);
            Redirect::permanent(&https_uri)
        }),
    );

    let http_addr = SocketAddr::from((host.parse::<std::net::IpAddr>()?, http_port));

    tracing::info!("🔄 Starting HTTP redirect server on http://{}", http_addr);
    tracing::info!("   Redirecting to HTTPS port: {}", https_port);

    // Start HTTP redirect server in background
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, redirect_app).await {
            tracing::error!("HTTP redirect server error: {}", e);
        }
    });

    Ok(())
}

/// Starts the regular HTTP server (fallback when TLS is disabled)
pub async fn start_http_server(app: Router, server_config: &ServerConfig) -> anyhow::Result<()> {
    let addr = SocketAddr::from((
        server_config.host.parse::<std::net::IpAddr>()?,
        server_config.port,
    ));

    tracing::info!("🌐 Starting HTTP server on http://{}", addr);
    tracing::warn!("⚠️ Server is running in HTTP mode - consider enabling TLS for production");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

/// Main server startup function that decides between HTTP and HTTPS
pub async fn start_server(app: Router, config: &AppConfig) -> anyhow::Result<()> {
    // Check if TLS is enabled
    if let Some(scoutquest_config) = &config.scoutquest {
        if let Some(tls_config) = &scoutquest_config.tls {
            if tls_config.enabled {
                return start_https_server(app, &config.server, tls_config).await;
            }
        }
    }

    // Fallback to HTTP server
    start_http_server(app, &config.server).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ScoutQuestConfig;

    fn create_test_config(tls_enabled: bool) -> AppConfig {
        AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8443,
                enable_cors: true,
                cors_origins: vec!["*".to_string()],
            },
            scoutquest: Some(ScoutQuestConfig {
                tls: Some(ScoutQuestTlsConfig {
                    enabled: tls_enabled,
                    cert_dir: "/tmp/test-certs".to_string(),
                    auto_generate: true,
                    ..Default::default()
                }),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_config_creation() {
        let config = create_test_config(true);
        assert!(config.scoutquest.is_some());

        let tls_config = config.scoutquest.unwrap().tls.unwrap();
        assert!(tls_config.enabled);
        assert_eq!(tls_config.cert_dir, "/tmp/test-certs");
    }
}
