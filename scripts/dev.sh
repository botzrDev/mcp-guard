#!/bin/bash
# =============================================================================
# MCP Guard Unified Development Script
# Manages all services: backend (Rust), frontend (Angular)
# Usage: ./scripts/dev.sh [command] [service]
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LANDING_DIR="$PROJECT_ROOT/landing"

# PID files
BACKEND_PID_FILE="$PROJECT_ROOT/.backend.pid"
FRONTEND_PID_FILE="$LANDING_DIR/.ng-serve.pid"

# Log files
LOG_DIR="$PROJECT_ROOT/.dev-logs"
BACKEND_LOG="$LOG_DIR/backend.log"
FRONTEND_LOG="$LOG_DIR/frontend.log"

# Ports
BACKEND_PORT=3000
FRONTEND_PORT=4200

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# =============================================================================
# Utility Functions
# =============================================================================

print_header() {
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${CYAN}║${NC}  ${BOLD}MCP Guard Development Environment${NC}                            ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════╝${NC}"
}

print_status() {
    echo -e "${BLUE}[dev]${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_service() {
    local service=$1
    local status=$2
    local port=$3
    local pid=$4
    
    if [ "$status" = "running" ]; then
        echo -e "  ${GREEN}●${NC} ${BOLD}$service${NC} running on port $port (PID: $pid)"
    else
        echo -e "  ${RED}○${NC} ${BOLD}$service${NC} not running (port $port)"
    fi
}

ensure_log_dir() {
    mkdir -p "$LOG_DIR"
}

show_help() {
    print_header
    echo ""
    echo -e "${BOLD}Usage:${NC} ./scripts/dev.sh [command] [service]"
    echo ""
    echo -e "${BOLD}Commands:${NC}"
    echo "  start [service]     Start services (all if no service specified)"
    echo "  stop [service]      Stop services gracefully"
    echo "  restart [service]   Restart services"
    echo "  status              Show status of all services"
    echo "  logs [service]      Tail logs (combined if no service specified)"
    echo "  clean               Stop all and clean build artifacts"
    echo "  help                Show this help message"
    echo ""
    echo -e "${BOLD}Services:${NC}"
    echo "  backend             Rust backend server (port $BACKEND_PORT)"
    echo "  frontend            Angular frontend (port $FRONTEND_PORT)"
    echo "  all                 All services (default)"
    echo ""
    echo -e "${BOLD}Examples:${NC}"
    echo "  ./scripts/dev.sh start              # Start all services"
    echo "  ./scripts/dev.sh start frontend     # Start only frontend"
    echo "  ./scripts/dev.sh stop               # Stop all services"
    echo "  ./scripts/dev.sh logs backend       # Tail backend logs"
    echo "  ./scripts/dev.sh status             # Show service status"
    echo ""
}

# =============================================================================
# Process Management
# =============================================================================

get_pid_from_file() {
    local pid_file=$1
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            echo "$pid"
            return
        else
            rm -f "$pid_file"
        fi
    fi
    echo ""
}

get_port_pid() {
    local port=$1
    local port_pid=""
    
    if command -v lsof &> /dev/null; then
        port_pid=$(lsof -ti:$port 2>/dev/null | head -1)
    fi
    
    if [ -z "$port_pid" ] && command -v ss &> /dev/null; then
        port_pid=$(ss -tlnp "sport = :$port" 2>/dev/null | grep -oP 'pid=\K[0-9]+' | head -1)
    fi
    
    echo "$port_pid"
}

kill_process() {
    local pid=$1
    local name=$2
    
    if [ -z "$pid" ]; then
        return 0
    fi
    
    if ! kill -0 "$pid" 2>/dev/null; then
        return 0
    fi
    
    print_status "Stopping $name (PID: $pid)..."
    
    # Graceful shutdown
    kill "$pid" 2>/dev/null || true
    
    # Wait for graceful shutdown
    for i in {1..5}; do
        if ! kill -0 "$pid" 2>/dev/null; then
            print_success "$name stopped"
            return 0
        fi
        sleep 1
    done
    
    # Force kill
    print_warning "Force killing $name..."
    kill -9 "$pid" 2>/dev/null || true
    sleep 1
    
    if ! kill -0 "$pid" 2>/dev/null; then
        print_success "$name stopped"
        return 0
    else
        print_error "Failed to stop $name"
        return 1
    fi
}

# =============================================================================
# Backend Service
# =============================================================================

backend_start() {
    local pid=$(get_pid_from_file "$BACKEND_PID_FILE")
    if [ -n "$pid" ]; then
        print_warning "Backend already running (PID: $pid)"
        return 0
    fi
    
    # Check if port is in use
    local port_pid=$(get_port_pid $BACKEND_PORT)
    if [ -n "$port_pid" ]; then
        print_warning "Port $BACKEND_PORT already in use (PID: $port_pid)"
        print_status "Stopping existing process..."
        kill_process "$port_pid" "existing backend"
    fi
    
    ensure_log_dir
    
    print_status "Starting backend server..."
    cd "$PROJECT_ROOT"
    
    # Start backend in background
    RUST_LOG=info cargo run --release -- run > "$BACKEND_LOG" 2>&1 &
    local server_pid=$!
    echo "$server_pid" > "$BACKEND_PID_FILE"
    
    # Wait for server to start
    print_status "Waiting for backend to start..."
    for i in {1..30}; do
        if curl -s "http://localhost:$BACKEND_PORT/health" > /dev/null 2>&1; then
            print_success "Backend started at http://localhost:$BACKEND_PORT (PID: $server_pid)"
            return 0
        fi
        
        if ! kill -0 "$server_pid" 2>/dev/null; then
            print_error "Backend failed to start. Check logs: $BACKEND_LOG"
            rm -f "$BACKEND_PID_FILE"
            return 1
        fi
        sleep 1
    done
    
    print_warning "Backend may still be starting. Check status later."
    return 0
}

backend_stop() {
    local pid=$(get_pid_from_file "$BACKEND_PID_FILE")
    
    if [ -z "$pid" ]; then
        local port_pid=$(get_port_pid $BACKEND_PORT)
        if [ -n "$port_pid" ]; then
            kill_process "$port_pid" "backend"
        else
            print_warning "Backend is not running"
        fi
        return 0
    fi
    
    kill_process "$pid" "backend"
    rm -f "$BACKEND_PID_FILE"
}

backend_status() {
    local pid=$(get_pid_from_file "$BACKEND_PID_FILE")
    if [ -z "$pid" ]; then
        pid=$(get_port_pid $BACKEND_PORT)
    fi
    
    if [ -n "$pid" ]; then
        print_service "Backend" "running" "$BACKEND_PORT" "$pid"
        return 0
    else
        print_service "Backend" "stopped" "$BACKEND_PORT" ""
        return 1
    fi
}

# =============================================================================
# Frontend Service
# =============================================================================

frontend_start() {
    local pid=$(get_pid_from_file "$FRONTEND_PID_FILE")
    if [ -n "$pid" ]; then
        print_warning "Frontend already running (PID: $pid)"
        return 0
    fi
    
    # Check if port is in use
    local port_pid=$(get_port_pid $FRONTEND_PORT)
    if [ -n "$port_pid" ]; then
        print_warning "Port $FRONTEND_PORT already in use (PID: $port_pid)"
        print_status "Stopping existing process..."
        kill_process "$port_pid" "existing frontend"
    fi
    
    # Check dependencies
    if [ ! -d "$LANDING_DIR/node_modules" ]; then
        print_status "Installing frontend dependencies..."
        cd "$LANDING_DIR" && npm install
    fi
    
    ensure_log_dir
    
    print_status "Starting frontend server..."
    cd "$LANDING_DIR"
    
    # Start frontend in background
    npm start > "$FRONTEND_LOG" 2>&1 &
    local server_pid=$!
    echo "$server_pid" > "$FRONTEND_PID_FILE"
    
    # Wait for server to start
    print_status "Waiting for frontend to start..."
    for i in {1..30}; do
        if curl -s "http://localhost:$FRONTEND_PORT" > /dev/null 2>&1; then
            print_success "Frontend started at http://localhost:$FRONTEND_PORT (PID: $server_pid)"
            return 0
        fi
        
        if ! kill -0 "$server_pid" 2>/dev/null; then
            print_error "Frontend failed to start. Check logs: $FRONTEND_LOG"
            rm -f "$FRONTEND_PID_FILE"
            return 1
        fi
        sleep 1
    done
    
    print_warning "Frontend may still be starting. Check status later."
    return 0
}

frontend_stop() {
    local pid=$(get_pid_from_file "$FRONTEND_PID_FILE")
    
    if [ -z "$pid" ]; then
        local port_pid=$(get_port_pid $FRONTEND_PORT)
        if [ -n "$port_pid" ]; then
            kill_process "$port_pid" "frontend"
        else
            print_warning "Frontend is not running"
        fi
        return 0
    fi
    
    kill_process "$pid" "frontend"
    rm -f "$FRONTEND_PID_FILE"
}

frontend_status() {
    local pid=$(get_pid_from_file "$FRONTEND_PID_FILE")
    if [ -z "$pid" ]; then
        pid=$(get_port_pid $FRONTEND_PORT)
    fi
    
    if [ -n "$pid" ]; then
        print_service "Frontend" "running" "$FRONTEND_PORT" "$pid"
        return 0
    else
        print_service "Frontend" "stopped" "$FRONTEND_PORT" ""
        return 1
    fi
}

# =============================================================================
# Combined Commands
# =============================================================================

cmd_start() {
    local service=${1:-all}
    
    print_header
    echo ""
    
    case $service in
        backend)
            backend_start
            ;;
        frontend)
            frontend_start
            ;;
        all)
            print_status "Starting all services..."
            echo ""
            backend_start
            echo ""
            frontend_start
            echo ""
            print_success "All services started!"
            echo ""
            echo -e "${BOLD}Access points:${NC}"
            echo "  Frontend:  http://localhost:$FRONTEND_PORT"
            echo "  Backend:   http://localhost:$BACKEND_PORT"
            echo "  Health:    http://localhost:$BACKEND_PORT/health"
            ;;
        *)
            print_error "Unknown service: $service"
            exit 1
            ;;
    esac
}

cmd_stop() {
    local service=${1:-all}
    
    print_header
    echo ""
    
    case $service in
        backend)
            backend_stop
            ;;
        frontend)
            frontend_stop
            ;;
        all)
            print_status "Stopping all services..."
            echo ""
            frontend_stop
            backend_stop
            echo ""
            print_success "All services stopped"
            ;;
        *)
            print_error "Unknown service: $service"
            exit 1
            ;;
    esac
}

cmd_restart() {
    local service=${1:-all}
    cmd_stop "$service"
    sleep 2
    cmd_start "$service"
}

cmd_status() {
    print_header
    echo ""
    echo -e "${BOLD}Service Status:${NC}"
    echo ""
    
    backend_status || true
    frontend_status || true
    
    echo ""
}

cmd_logs() {
    local service=${1:-all}
    ensure_log_dir
    
    case $service in
        backend)
            if [ -f "$BACKEND_LOG" ]; then
                tail -f "$BACKEND_LOG"
            else
                print_warning "No backend logs found"
            fi
            ;;
        frontend)
            if [ -f "$FRONTEND_LOG" ]; then
                tail -f "$FRONTEND_LOG"
            else
                print_warning "No frontend logs found"
            fi
            ;;
        all)
            if [ -f "$BACKEND_LOG" ] || [ -f "$FRONTEND_LOG" ]; then
                tail -f "$BACKEND_LOG" "$FRONTEND_LOG" 2>/dev/null
            else
                print_warning "No logs found"
            fi
            ;;
        *)
            print_error "Unknown service: $service"
            exit 1
            ;;
    esac
}

cmd_clean() {
    print_header
    echo ""
    
    cmd_stop all
    
    echo ""
    print_status "Cleaning build artifacts..."
    
    # Clean backend
    cd "$PROJECT_ROOT"
    cargo clean 2>/dev/null || true
    print_success "Backend artifacts cleaned"
    
    # Clean frontend
    cd "$LANDING_DIR"
    rm -rf dist .angular 2>/dev/null || true
    print_success "Frontend artifacts cleaned"
    
    # Clean logs
    rm -rf "$LOG_DIR" 2>/dev/null || true
    print_success "Logs cleaned"
    
    echo ""
    print_success "All artifacts cleaned"
}

# =============================================================================
# Main
# =============================================================================

case "${1:-help}" in
    start)
        cmd_start "$2"
        ;;
    stop)
        cmd_stop "$2"
        ;;
    restart)
        cmd_restart "$2"
        ;;
    status)
        cmd_status
        ;;
    logs)
        cmd_logs "$2"
        ;;
    clean)
        cmd_clean
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
