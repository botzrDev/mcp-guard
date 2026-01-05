---
description: Start, stop, and manage all development services
---

# Development Environment Workflow

This workflow manages the MCP Guard development environment with backend (Rust) and frontend (Angular) services.

## Starting Development

// turbo
1. Start all services:
```bash
./scripts/dev.sh start
```

2. Verify services are running:
```bash
./scripts/dev.sh status
```

Expected output shows:
- Backend running on port 3000
- Frontend running on port 4200

## Access Points

- **Frontend**: http://localhost:4200
- **Backend API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health

## Stopping Development

// turbo
1. Stop all services gracefully:
```bash
./scripts/dev.sh stop
```

## Common Commands

| Command | Description |
|---------|-------------|
| `./scripts/dev.sh start` | Start all services |
| `./scripts/dev.sh stop` | Stop all services |
| `./scripts/dev.sh restart` | Restart all services |
| `./scripts/dev.sh status` | Show service status |
| `./scripts/dev.sh logs` | Tail all logs |
| `./scripts/dev.sh start frontend` | Start only frontend |
| `./scripts/dev.sh start backend` | Start only backend |
| `make dev` | Shortcut to start all |
| `make dev-stop` | Shortcut to stop all |

## Troubleshooting

If a port is already in use:
// turbo
```bash
./scripts/dev.sh stop
./scripts/dev.sh start
```

To check what's using a port:
```bash
lsof -i:4200
lsof -i:3000
```
