//! Automatic certificate generation for ScoutQuest Server

use super::{certificates_exist, ensure_cert_directory, TlsError};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use std::path::Path;
use tokio::fs;

/// Generates a self-signed certificate and private key
pub async fn generate_self_signed_cert(cert_path: &Path, key_path: &Path) -> Result<(), TlsError> {
    tracing::info!("🔐 Generating self-signed certificate...");
    tracing::info!("   Certificate: {}", cert_path.display());
    tracing::info!("   Private key: {}", key_path.display());

    // Create certificate directory if it doesn't exist
    if let Some(parent) = cert_path.parent() {
        ensure_cert_directory(&parent.to_string_lossy()).await?;
    }

    // Create certificate parameters
    let mut params = CertificateParams::new(vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "scoutquest".to_string(),
        "scoutquest-server".to_string(),
    ]);

    // Set certificate distinguished name
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, "ScoutQuest Server");
    params
        .distinguished_name
        .push(DnType::OrganizationName, "ScoutQuest");
    params.distinguished_name.push(DnType::CountryName, "US");

    // Set certificate validity period (1 year)
    let not_before = time::OffsetDateTime::now_utc();
    let not_after = not_before + time::Duration::days(365);
    params.not_before = not_before;
    params.not_after = not_after;

    // Generate the certificate
    let cert = Certificate::from_params(params)
        .map_err(|e| TlsError::CertificateGeneration(e.to_string()))?;

    // Serialize certificate and private key
    let cert_pem = cert
        .serialize_pem()
        .map_err(|e| TlsError::CertificateGeneration(e.to_string()))?;
    let key_pem = cert.serialize_private_key_pem();

    // Write certificate file
    fs::write(cert_path, cert_pem).await?;
    tracing::info!("✅ Certificate written to: {}", cert_path.display());

    // Write private key file with restricted permissions
    fs::write(key_path, key_pem).await?;

    // Set restrictive permissions on private key (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(key_path).await?.permissions();
        perms.set_mode(0o600); // Read/write for owner only
        fs::set_permissions(key_path, perms).await?;
    }

    tracing::info!("✅ Private key written to: {}", key_path.display());
    tracing::info!("🔐 Self-signed certificate generation completed successfully");

    Ok(())
}

/// Validates an existing certificate file
pub async fn validate_certificate(cert_path: &Path) -> Result<(), TlsError> {
    if !cert_path.exists() {
        return Err(TlsError::CertificateLoad(format!(
            "Certificate file not found: {}",
            cert_path.display()
        )));
    }

    let cert_content = fs::read_to_string(cert_path).await?;

    // Basic PEM format validation
    if !cert_content.contains("-----BEGIN CERTIFICATE-----")
        || !cert_content.contains("-----END CERTIFICATE-----")
    {
        return Err(TlsError::CertificateLoad(
            "Invalid certificate format: not a valid PEM certificate".to_string(),
        ));
    }

    tracing::info!("✅ Certificate validation passed: {}", cert_path.display());
    Ok(())
}

/// Validates an existing private key file
pub async fn validate_private_key(key_path: &Path) -> Result<(), TlsError> {
    if !key_path.exists() {
        return Err(TlsError::CertificateLoad(format!(
            "Private key file not found: {}",
            key_path.display()
        )));
    }

    let key_content = fs::read_to_string(key_path).await?;

    // Basic PEM format validation for private keys
    let is_valid_key = key_content.contains("-----BEGIN PRIVATE KEY-----")
        || key_content.contains("-----BEGIN RSA PRIVATE KEY-----")
        || key_content.contains("-----BEGIN EC PRIVATE KEY-----");

    if !is_valid_key {
        return Err(TlsError::CertificateLoad(
            "Invalid private key format: not a valid PEM private key".to_string(),
        ));
    }

    tracing::info!("✅ Private key validation passed: {}", key_path.display());
    Ok(())
}

/// Ensures certificates exist, generating them if necessary
pub async fn ensure_certificates(
    cert_path: &Path,
    key_path: &Path,
    auto_generate: bool,
) -> Result<(), TlsError> {
    if certificates_exist(cert_path, key_path) {
        // Validate existing certificates
        validate_certificate(cert_path).await?;
        validate_private_key(key_path).await?;
        tracing::info!("🔐 Using existing TLS certificates");
        return Ok(());
    }

    if !auto_generate {
        return Err(TlsError::InvalidConfiguration(
            "TLS certificates not found and auto_generate is disabled".to_string(),
        ));
    }

    // Generate new certificates
    generate_self_signed_cert(cert_path, key_path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_self_signed_cert() {
        let temp_dir = TempDir::new().unwrap();
        let cert_path = temp_dir.path().join("test.crt");
        let key_path = temp_dir.path().join("test.key");

        let result = generate_self_signed_cert(&cert_path, &key_path).await;
        assert!(result.is_ok());
        assert!(cert_path.exists());
        assert!(key_path.exists());

        // Validate generated files
        let cert_content = fs::read_to_string(&cert_path).await.unwrap();
        let key_content = fs::read_to_string(&key_path).await.unwrap();

        assert!(cert_content.contains("-----BEGIN CERTIFICATE-----"));
        assert!(cert_content.contains("-----END CERTIFICATE-----"));
        assert!(key_content.contains("-----BEGIN PRIVATE KEY-----"));
        assert!(key_content.contains("-----END PRIVATE KEY-----"));
    }

    #[tokio::test]
    async fn test_ensure_certificates_auto_generate() {
        let temp_dir = TempDir::new().unwrap();
        let cert_path = temp_dir.path().join("auto.crt");
        let key_path = temp_dir.path().join("auto.key");

        let result = ensure_certificates(&cert_path, &key_path, true).await;
        assert!(result.is_ok());
        assert!(cert_path.exists());
        assert!(key_path.exists());
    }

    #[tokio::test]
    async fn test_ensure_certificates_no_auto_generate() {
        let temp_dir = TempDir::new().unwrap();
        let cert_path = temp_dir.path().join("missing.crt");
        let key_path = temp_dir.path().join("missing.key");

        let result = ensure_certificates(&cert_path, &key_path, false).await;
        assert!(result.is_err());
    }
}
