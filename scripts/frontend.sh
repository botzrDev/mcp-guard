#!/bin/bash
# Frontend management script for MCP Guard landing page
# Usage: ./scripts/frontend.sh [command]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LANDING_DIR="$PROJECT_ROOT/landing"
PID_FILE="$LANDING_DIR/.ng-serve.pid"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[frontend]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[frontend]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[frontend]${NC} $1"
}

print_error() {
    echo -e "${RED}[frontend]${NC} $1"
}

show_help() {
    echo "MCP Guard Frontend Management"
    echo ""
    echo "Usage: ./scripts/frontend.sh [command]"
    echo ""
    echo "Commands:"
    echo "  start       Start development server (port 4200)"
    echo "  dev         Start dev server and open browser"
    echo "  stop        Stop running dev server"
    echo "  restart     Restart development server"
    echo "  build       Build for production"
    echo "  preview     Build and serve production build"
    echo "  clean       Clean build artifacts"
    echo "  install     Install npm dependencies"
    echo "  update      Update npm dependencies"
    echo "  status      Check if dev server is running"
    echo "  logs        Show recent build output"
    echo "  help        Show this help message"
    echo ""
}

check_deps() {
    if ! command -v node &> /dev/null; then
        print_error "Node.js is not installed"
        exit 1
    fi

    if ! command -v npm &> /dev/null; then
        print_error "npm is not installed"
        exit 1
    fi

    if [ ! -d "$LANDING_DIR/node_modules" ]; then
        print_warning "Dependencies not installed. Running npm install..."
        cd "$LANDING_DIR" && npm install
    fi
}

get_pid() {
    # Get PID from our PID file if it exists and process is still running
    if [ -f "$PID_FILE" ]; then
        local pid=$(cat "$PID_FILE")
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            echo "$pid"
            return
        else
            # Stale PID file, remove it
            rm -f "$PID_FILE"
        fi
    fi
    echo ""
}

cmd_start() {
    check_deps

    local pid=$(get_pid)
    if [ -n "$pid" ]; then
        print_warning "Dev server already running (PID: $pid)"
        print_status "Access at http://localhost:4200"
        return
    fi

    print_status "Starting development server..."
    cd "$LANDING_DIR"
    npm start &
    local server_pid=$!
    echo "$server_pid" > "$PID_FILE"

    # Wait for server to start
    sleep 3
    if kill -0 "$server_pid" 2>/dev/null; then
        print_success "Development server started at http://localhost:4200 (PID: $server_pid)"
    else
        print_error "Failed to start development server"
        rm -f "$PID_FILE"
        exit 1
    fi
}

cmd_dev() {
    check_deps

    local pid=$(get_pid)
    if [ -n "$pid" ]; then
        print_warning "Dev server already running (PID: $pid)"
        print_status "Opening browser..."
        xdg-open "http://localhost:4200" 2>/dev/null || open "http://localhost:4200" 2>/dev/null || true
        return
    fi

    print_status "Starting development server with browser..."
    cd "$LANDING_DIR"
    npm run dev
}

cmd_stop() {
    local pid=$(get_pid)
    if [ -z "$pid" ]; then
        print_warning "Dev server is not running (no PID file found)"
        return
    fi

    print_status "Stopping dev server (PID: $pid)..."

    # Send SIGTERM first for graceful shutdown
    kill "$pid" 2>/dev/null || true

    # Wait up to 5 seconds for graceful shutdown
    for i in {1..5}; do
        if ! kill -0 "$pid" 2>/dev/null; then
            break
        fi
        sleep 1
    done

    # Force kill if still running
    if kill -0 "$pid" 2>/dev/null; then
        print_warning "Process didn't stop gracefully, forcing..."
        kill -9 "$pid" 2>/dev/null || true
    fi

    rm -f "$PID_FILE"
    print_success "Dev server stopped"
}

cmd_restart() {
    cmd_stop
    sleep 1
    cmd_start
}

cmd_build() {
    check_deps
    print_status "Building for production..."
    cd "$LANDING_DIR"
    npm run build
    print_success "Build complete! Output in landing/dist/landing/browser"
}

cmd_preview() {
    check_deps
    print_status "Building and serving production build..."
    cd "$LANDING_DIR"
    npm run preview
}

cmd_clean() {
    print_status "Cleaning build artifacts..."
    cd "$LANDING_DIR"
    rm -rf dist .angular
    print_success "Clean complete"
}

cmd_install() {
    print_status "Installing dependencies..."
    cd "$LANDING_DIR"
    npm install
    print_success "Dependencies installed"
}

cmd_update() {
    print_status "Updating dependencies..."
    cd "$LANDING_DIR"
    npm update
    print_success "Dependencies updated"
}

cmd_status() {
    local pid=$(get_pid)
    if [ -n "$pid" ]; then
        print_success "Dev server is running (PID: $pid)"
        print_status "Access at http://localhost:4200"
    else
        print_warning "Dev server is not running"
    fi
}

cmd_logs() {
    print_status "Recent Angular build cache info:"
    if [ -d "$LANDING_DIR/.angular/cache" ]; then
        ls -la "$LANDING_DIR/.angular/cache" 2>/dev/null || echo "No cache directory"
    else
        echo "No cache found"
    fi

    echo ""
    print_status "Last build output:"
    if [ -d "$LANDING_DIR/dist" ]; then
        ls -la "$LANDING_DIR/dist/landing/browser" 2>/dev/null || echo "No build output"
    else
        echo "No build found. Run './scripts/frontend.sh build' first."
    fi
}

# Main command handler
case "${1:-help}" in
    start)
        cmd_start
        ;;
    dev)
        cmd_dev
        ;;
    stop)
        cmd_stop
        ;;
    restart)
        cmd_restart
        ;;
    build)
        cmd_build
        ;;
    preview)
        cmd_preview
        ;;
    clean)
        cmd_clean
        ;;
    install)
        cmd_install
        ;;
    update)
        cmd_update
        ;;
    status)
        cmd_status
        ;;
    logs)
        cmd_logs
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
