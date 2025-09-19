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

## Production Configuration

For production environments, consider:

1. **Security**:
   - Set `enable_auth = true`
   - Use a strong `api_key`
   - Restrict `cors_origins` to specific domains

2. **Performance**:
   - Use `format = "json"` for better log aggregation
   - Set appropriate `rate_limit_per_minute`
   - Tune health check intervals

3. **Monitoring**:
   - Set `level = "warn"` or `"error"` to reduce log noise
   - Use structured logging with `format = "json"`

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
