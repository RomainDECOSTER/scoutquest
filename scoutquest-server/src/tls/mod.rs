//! TLS module for ScoutQuest Server
//!
//! This module provides TLS/SSL support for the ScoutQuest server including:
//! - Automatic certificate generation
//! - HTTPS server with Rustls
//! - Certificate management utilities
//! - TLS configuration handling

pub mod cert_gen;
pub mod config;
pub mod server;
pub mod utils;

pub use cert_gen::*;
pub use config::*;
pub use server::*;

use std::fmt;

/// TLS-related errors
#[derive(Debug)]
pub enum TlsError {
    CertificateGeneration(String),
    CertificateLoad(String),
    InvalidConfiguration(String),
    IoError(std::io::Error),
    RustlsError(rustls::Error),
}

impl fmt::Display for TlsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsError::CertificateGeneration(msg) => {
                write!(f, "Certificate generation error: {}", msg)
            }
            TlsError::CertificateLoad(msg) => write!(f, "Certificate load error: {}", msg),
            TlsError::InvalidConfiguration(msg) => write!(f, "Invalid TLS configuration: {}", msg),
            TlsError::IoError(err) => write!(f, "IO error: {}", err),
            TlsError::RustlsError(err) => write!(f, "Rustls error: {}", err),
        }
    }
}

impl std::error::Error for TlsError {}

impl From<std::io::Error> for TlsError {
    fn from(err: std::io::Error) -> Self {
        TlsError::IoError(err)
    }
}

impl From<rustls::Error> for TlsError {
    fn from(err: rustls::Error) -> Self {
        TlsError::RustlsError(err)
    }
}
