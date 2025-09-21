//! TLS configuration utilities and validation

use super::TlsError;
use crate::models::ScoutQuestTlsConfig;
use std::path::{Path, PathBuf};

/// Validates TLS configuration and returns normalized paths
pub fn validate_tls_config(config: &ScoutQuestTlsConfig) -> Result<(), TlsError> {
    if !config.enabled {
        return Ok(());
    }

    // Validate certificate directory
    if config.cert_dir.is_empty() {
        return Err(TlsError::InvalidConfiguration(
            "cert_dir cannot be empty when TLS is enabled".to_string(),
        ));
    }

    // Validate TLS versions if specified
    if let Some(ref min_version) = config.min_version {
        validate_tls_version(min_version)?;
    }

    if let Some(ref max_version) = config.max_version {
        validate_tls_version(max_version)?;
    }

    // Validate custom certificate paths if provided
    if let (Some(ref cert_path), Some(ref key_path)) = (&config.cert_path, &config.key_path) {
        if !config.auto_generate {
            let cert_path = Path::new(cert_path);
            let key_path = Path::new(key_path);

            if !cert_path.exists() {
                return Err(TlsError::InvalidConfiguration(format!(
                    "Certificate file not found: {}",
                    cert_path.display()
                )));
            }

            if !key_path.exists() {
                return Err(TlsError::InvalidConfiguration(format!(
                    "Private key file not found: {}",
                    key_path.display()
                )));
            }
        }
    }

    Ok(())
}

/// Validates TLS version string
fn validate_tls_version(version: &str) -> Result<(), TlsError> {
    match version {
        "1.0" | "1.1" | "1.2" | "1.3" => Ok(()),
        _ => Err(TlsError::InvalidConfiguration(format!(
            "Invalid TLS version: {}. Supported versions: 1.0, 1.1, 1.2, 1.3",
            version
        ))),
    }
}

/// Returns the certificate and key paths to use
pub fn get_certificate_paths(config: &ScoutQuestTlsConfig) -> (PathBuf, PathBuf) {
    match (&config.cert_path, &config.key_path) {
        (Some(cert_path), Some(key_path)) => {
            // Use custom paths
            (PathBuf::from(cert_path), PathBuf::from(key_path))
        }
        _ => {
            // Use auto-generated paths in cert_dir
            let cert_dir = Path::new(&config.cert_dir);
            (
                cert_dir.join("scoutquest.crt"),
                cert_dir.join("scoutquest.key"),
            )
        }
    }
}

/// Creates the certificate directory if it doesn't exist
pub async fn ensure_cert_directory(cert_dir: &str) -> Result<(), TlsError> {
    let path = Path::new(cert_dir);
    if !path.exists() {
        tracing::info!("📁 Creating certificate directory: {}", cert_dir);
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}

/// Checks if certificates exist at the given paths
pub fn certificates_exist(cert_path: &Path, key_path: &Path) -> bool {
    cert_path.exists() && key_path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tls_version() {
        assert!(validate_tls_version("1.2").is_ok());
        assert!(validate_tls_version("1.3").is_ok());
        assert!(validate_tls_version("2.0").is_err());
        assert!(validate_tls_version("invalid").is_err());
    }

    #[test]
    fn test_get_certificate_paths() {
        let config_auto = ScoutQuestTlsConfig {
            enabled: true,
            cert_dir: "/etc/certs".to_string(),
            auto_generate: true,
            cert_path: None,
            key_path: None,
            ..Default::default()
        };

        let (cert_path, key_path) = get_certificate_paths(&config_auto);
        assert_eq!(cert_path, PathBuf::from("/etc/certs/scoutquest.crt"));
        assert_eq!(key_path, PathBuf::from("/etc/certs/scoutquest.key"));

        let config_custom = ScoutQuestTlsConfig {
            enabled: true,
            cert_dir: "/etc/certs".to_string(),
            auto_generate: false,
            cert_path: Some("/custom/cert.pem".to_string()),
            key_path: Some("/custom/key.pem".to_string()),
            ..Default::default()
        };

        let (cert_path, key_path) = get_certificate_paths(&config_custom);
        assert_eq!(cert_path, PathBuf::from("/custom/cert.pem"));
        assert_eq!(key_path, PathBuf::from("/custom/key.pem"));
    }
}
