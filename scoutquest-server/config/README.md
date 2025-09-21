# SquoutQuest Server Configuration

This directory contains configuration files for the SquoutQuest server.

## Available Configuration Files

- **`default.toml`** - Default configuration for development
- **`production.toml`** - Production-ready configuration
- **`example.toml`** - Example configuration with detailed comments

## Quick Start

1. **Copy the example file**:
   ```bash
   cp example.toml my-config.toml
   ```

2. **Edit the configuration**:
   ```bash
   # Edit with your preferred editor
   nano my-config.toml
   # or
   vim my-config.toml
   ```

3. **Start the server with your config**:
   ```bash
   cargo run -- --config my-config.toml
   ```

## Configuration Sections

### [server]
Server HTTP configuration.

| Setting | Default | Description |
|---------|---------|-------------|
| `host` | `"0.0.0.0"` | Host address to bind to |
| `port` | `8080` | Port number |
| `enable_cors` | `true` | Enable CORS support |
| `cors_origins` | `["*"]` | Allowed CORS origins |

### [logging]
Logging configuration.

| Setting | Default | Description |
|---------|---------|-------------|
| `level` | `"info"` | Log level (trace, debug, info, warn, error) |
| `format` | `"pretty"` | Log format (pretty, json) |

### [health_check]
Health check configuration.

| Setting | Default | Description |
|---------|---------|-------------|
| `interval_seconds` | `30` | Interval between health checks |
| `timeout_seconds` | `10` | Health check timeout |
| `max_failures` | `3` | Max consecutive failures |

### [security]
Security configuration.

| Setting | Default | Description |
|---------|---------|-------------|
| `enable_auth` | `false` | Enable API authentication |
| `api_key` | `""` | API key for authentication |
| `rate_limit_per_minute` | `1000` | Rate limiting per IP |

### [network]
Network access restrictions by CIDR ranges.

| Setting | Default | Description |
|---------|---------|-------------|
| `enabled` | `false` | Enable network access restrictions |
| `allowed_cidrs` | `["0.0.0.0/0"]` | Whitelist CIDR ranges (IPv4/IPv6) |
| `denied_cidrs` | `[]` | Blacklist CIDR ranges (takes priority) |
| `deny_action` | `"reject"` | Action for denied IPs: "reject" or "log_only" |
| `trust_proxy_headers` | `true` | Trust X-Forwarded-For and X-Real-IP headers |

## Environment Variables

You can override configuration using environment variables:

```bash
# Override server settings
SCOUTQUEST_SERVER_HOST=127.0.0.1
SCOUTQUEST_SERVER_PORT=9090

# Override logging
SCOUTQUEST_LOGGING_LEVEL=debug
SCOUTQUEST_LOGGING_FORMAT=json

# Override health check
SCOUTQUEST_HEALTH_CHECK_INTERVAL_SECONDS=15
SCOUTQUEST_HEALTH_CHECK_TIMEOUT_SECONDS=5

# Override security
SCOUTQUEST_SECURITY_ENABLE_AUTH=true
SCOUTQUEST_SECURITY_API_KEY=your-secret-key
SCOUTQUEST_SECURITY_RATE_LIMIT_PER_MINUTE=500
```

## Command Line Arguments

You can also override settings via command line:

```bash
# Override port
cargo run -- --port 9090

# Override host
cargo run -- --host 127.0.0.1

# Override log level
cargo run -- --log-level debug

# Use custom config file
cargo run -- --config my-config.toml
```

## Configuration Priority

Settings are loaded in this order (later overrides earlier):

1. **Default values** (hardcoded in the application)
2. **Configuration file** (e.g., `config.toml`)
3. **Environment variables** (prefixed with `SCOUTQUEST_`)
4. **Command line arguments** (highest priority)

## Network Security Examples

### Kubernetes Deployment
```toml
[network]
enabled = true
allowed_cidrs = ["10.42.0.0/16"]  # Only cluster pods
deny_action = "reject"
```

### Docker Compose
```toml
[network]
enabled = true
allowed_cidrs = ["172.17.0.0/16", "172.20.0.0/16"]  # Docker networks
deny_action = "reject"
```

### Development Mode
```toml
[network]
enabled = true
allowed_cidrs = ["0.0.0.0/0"]  # Allow all
deny_action = "log_only"       # Just log, don't block
```

### High Security Production
```toml
[network]
enabled = true
allowed_cidrs = [
    "10.42.0.0/16",    # Kubernetes cluster
    "172.20.0.0/16",   # Docker bridge network
    "127.0.0.1/32"     # Localhost for health checks
]
denied_cidrs = [
    "10.42.99.0/24"    # Block specific suspicious subnet
]
deny_action = "reject"
trust_proxy_headers = true
```

## Production Configuration

For production environments, consider:

1. **Security**:
   - Set `enable_auth = true`
   - Use a strong `api_key`
   - Restrict `cors_origins` to specific domains
   - **Enable network restrictions** with `[network]` section
   - Use specific CIDR ranges, avoid `0.0.0.0/0`

2. **Performance**:
   - Use `format = "json"` for better log aggregation
   - Set appropriate `rate_limit_per_minute`
   - Tune health check intervals

3. **Monitoring**:
   - Set `level = "warn"` or `"error"` to reduce log noise
   - Use structured logging with `format = "json"`
   - Monitor network access logs for security

4. **Network Security**:
   - Always enable network restrictions in production
   - Use specific CIDR ranges for your infrastructure
   - Consider using `deny_action = "reject"` for strict security
   - Set `trust_proxy_headers = true` if behind a load balancer

## Example Production Configuration

```toml
[server]
host = "0.0.0.0"
port = 8080
enable_cors = true
cors_origins = ["https://dashboard.mycompany.com"]

[logging]
level = "warn"
format = "json"

[health_check]
interval_seconds = 15
timeout_seconds = 5
max_failures = 2

[security]
enable_auth = true
api_key = "your-secure-api-key-here"
rate_limit_per_minute = 500
```

## 🔒 TLS/HTTPS Support (NEW!)

ScoutQuest Server now supports native TLS/HTTPS with automatic certificate generation and management.

### [scoutquest.tls]
TLS/SSL configuration for HTTPS support.

| Setting | Default | Description |
|---------|---------|-------------|
| `enabled` | `false` | Enable TLS/HTTPS support |
| `cert_dir` | `"/etc/certs"` | Certificate directory |
| `auto_generate` | `true` | Auto-generate self-signed certificates |
| `verify_peer` | `true` | Verify client certificates |
| `cert_path` | `None` | Custom certificate file path |
| `key_path` | `None` | Custom private key file path |
| `min_version` | `"1.2"` | Minimum TLS version |
| `max_version` | `"1.3"` | Maximum TLS version |
| `redirect_http` | `false` | Redirect HTTP to HTTPS |
| `http_port` | `3001` | HTTP redirect server port |

### Zero-Configuration TLS (Development)

```toml
[server]
port = 8443

[scoutquest.tls]
enabled = true
cert_dir = "./certs"
auto_generate = true
verify_peer = false  # Disable for development
```

### Production TLS with Custom Certificates

```toml
[server]
port = 443

[scoutquest.tls]
enabled = true
auto_generate = false
cert_path = "/etc/ssl/certs/scoutquest.crt"
key_path = "/etc/ssl/private/scoutquest.key"
verify_peer = true
redirect_http = true
http_port = 80
```

### TLS Environment Variables

```bash
# Enable TLS
SCOUTQUEST_SCOUTQUEST_TLS_ENABLED=true
SCOUTQUEST_SCOUTQUEST_TLS_CERT_DIR=/etc/certs
SCOUTQUEST_SCOUTQUEST_TLS_AUTO_GENERATE=true

# Custom certificates
SCOUTQUEST_SCOUTQUEST_TLS_CERT_PATH=/path/to/cert.pem
SCOUTQUEST_SCOUTQUEST_TLS_KEY_PATH=/path/to/key.pem

# Security settings
SCOUTQUEST_SCOUTQUEST_TLS_VERIFY_PEER=true
SCOUTQUEST_SCOUTQUEST_TLS_MIN_VERSION=1.2
SCOUTQUEST_SCOUTQUEST_TLS_REDIRECT_HTTP=true
```

### TLS Configuration Examples

The following TLS configuration files are available:

- **`tls-auto.toml`** - Zero-config TLS with auto-generation
- **`tls-development.toml`** - Development-friendly TLS setup
- **`tls-production.toml`** - Production TLS with custom certificates

### Certificate Management

#### Automatic Certificate Generation
ScoutQuest automatically generates self-signed certificates when `auto_generate = true`:

```bash
# Certificates are created in cert_dir
./certs/
├── scoutquest.crt  # Self-signed certificate
└── scoutquest.key  # Private key (600 permissions)
```

#### Manual Certificate Setup
```bash
# Generate development certificates
openssl req -x509 -newkey rsa:4096 -keyout scoutquest.key -out scoutquest.crt \
    -days 365 -nodes -subj "/CN=localhost"

# Set proper permissions
chmod 600 scoutquest.key
chmod 644 scoutquest.crt
```

### TLS Usage Examples

```bash
# Start with auto-generated TLS
cargo run -- --config config/tls-auto.toml

# Development with TLS
cargo run -- --config config/tls-development.toml

# Production with custom certificates
cargo run -- --config config/tls-production.toml

# Enable TLS via environment variable
SCOUTQUEST_SCOUTQUEST_TLS_ENABLED=true cargo run
```
