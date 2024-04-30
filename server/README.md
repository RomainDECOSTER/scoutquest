## Features

- [x] Service registration: Route POST /api/services

Body:
```
{
    "name": "service-name",
    "ip_addr": "1.2.3.4",
    "port": 8080
    "hostname: "service-hostname",
}
```
Response:
```
{
  "uuid": "cbb20e91-fbcd-4f4f-9ff5-90ce753a7ac0"
}
```
- [x] Service discovery UI: Route GET /services
- [x] Service status update: Route PUT /api/services/:uuid?status=Up

`PUT /api/services/cbb20e91-fbcd-4f4f-9ff5-90ce753a7ac0?status=Up`
- [x] Service unregistration: Route DELETE /api/services/:service_name/:service_id

`DELETE /api/services/cbb20e91-fbcd-4f4f-9ff5-90ce753a7ac0`

## Installation

```bash
cargo install
pnpm install
```

## Development

```bash
cargo run --package scoutquest-server --bin scoutquest-server
```

With cargo watch:

```bash
cargo install cargo-watch
cargo watch -x "run --package scoutquest-server --bin scoutquest-server"
```

If you want update ui generated code, you can run:

```bash
pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch
```