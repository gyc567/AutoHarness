#!/bin/bash
# Dev Local Setup Script Template
# Customize this for your project

set -e

COMMAND="${1:-up}"

echo "=== Dev Local Setup ==="
echo "Command: $COMMAND"
echo ""

case "$COMMAND" in
    up)
        echo "Starting development environment..."
        
        # Example: Build the project
        # cargo build
        
        # Example: Run database migrations
        # ./scripts/migrate.sh
        
        echo ""
        echo "✓ Development environment ready!"
        echo "  Run 'cargo run' to start the application."
        ;;
        
    down)
        echo "Stopping development environment..."
        
        # Example: Stop services
        # pkill -f 'my-service'
        
        echo "✓ Development environment stopped."
        ;;
        
    status)
        echo "Development environment status:"
        
        # Example: Check running services
        # pg_isready && echo "Database: ✓" || echo "Database: ✗"
        
        echo ""
        echo "Build status:"
        cargo build 2>/dev/null && echo "  Build: ✓" || echo "  Build: ✗"
        ;;
        
    test)
        echo "Running tests..."
        cargo test
        echo ""
        echo "✓ All tests passed!"
        ;;
        
    lint)
        echo "Running lints..."
        cargo fmt
        cargo clippy
        echo ""
        echo "✓ Code is clean!"
        ;;
        
    all)
        echo "Running full check..."
        cargo fmt && cargo clippy && cargo test
        echo ""
        echo "✓ All checks passed!"
        ;;
        
    *)
        echo "Usage: $0 {up|down|status|test|lint|all}"
        echo ""
        echo "Commands:"
        echo "  up     - Start development environment"
        echo "  down   - Stop development environment"
        echo "  status - Show environment status"
        echo "  test   - Run tests"
        echo "  lint   - Run linters"
        echo "  all    - Run full check (fmt + clippy + test)"
        exit 1
        ;;
esac
