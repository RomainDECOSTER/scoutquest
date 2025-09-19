# SquoutQuest Server

Universal Service Discovery for microservices architectures.

## Installation

```bash
cargo run
```

## Configuration

See `config/README.md` for detailed configuration documentation.

Available configuration files:
- `config/default.toml` - Default configuration for development
- `config/production.toml` - Production-ready configuration
- `config/example.toml` - Example configuration with detailed comments

## Usage

```bash
# Start the server
cargo run

# With custom configuration
cargo run -- --config config/production.toml --port 9090

# With environment variables
SCOUTQUEST_SERVER_PORT=8080 cargo run
```

## Dashboard

Once started, the dashboard is available at: http://localhost:8080/dashboard
