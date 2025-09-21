//! TLS utilities and helper functions

use std::path::Path;

/// Logs TLS configuration information
pub fn log_tls_info(enabled: bool, cert_dir: &str, auto_generate: bool) {
    if enabled {
        tracing::info!("🔒 TLS Configuration:");
        tracing::info!("   Status: Enabled");
        tracing::info!("   Certificate directory: {}", cert_dir);
        tracing::info!(
            "   Auto-generation: {}",
            if auto_generate { "Enabled" } else { "Disabled" }
        );
    } else {
        tracing::info!("🌐 TLS Configuration: Disabled (HTTP mode)");
    }
}

/// Sanitizes file paths for logging (removes sensitive information)
pub fn sanitize_path_for_logging(path: &Path) -> String {
    // For security, don't log full paths in production
    // Just show the filename and parent directory
    if let (Some(file_name), Some(parent)) = (path.file_name(), path.parent()) {
        if let Some(parent_name) = parent.file_name() {
            format!(
                "{}/{}",
                parent_name.to_string_lossy(),
                file_name.to_string_lossy()
            )
        } else {
            file_name.to_string_lossy().to_string()
        }
    } else {
        path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sanitize_path_for_logging() {
        let path = PathBuf::from("/etc/ssl/certs/server.crt");
        let sanitized = sanitize_path_for_logging(&path);
        assert_eq!(sanitized, "certs/server.crt");
    }
}
