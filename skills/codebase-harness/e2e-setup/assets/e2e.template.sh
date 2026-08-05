#!/bin/bash
# E2E Test Runner Script Template
# Customize for your project

set -e

echo "=== E2E Test Suite ==="
echo ""

COMMAND="${1:-run}"

# Detect project type
detect_project_type() {
    if [[ -f "Cargo.toml" ]]; then
        echo "rust"
    elif [[ -f "package.json" ]]; then
        echo "node"
    elif [[ -f "requirements.txt" ]] || [[ -f "pyproject.toml" ]]; then
        echo "python"
    else
        echo "unknown"
    fi
}

PROJECT_TYPE=$(detect_project_type)
echo "Detected project type: $PROJECT_TYPE"
echo ""

case "$COMMAND" in
    run)
        echo "Running E2E tests..."
        
        # Ensure dev stack is ready
        if [[ -f "../scripts/dev-local.sh" ]]; then
            echo "Checking dev stack..."
            ../scripts/dev-local.sh status || ../scripts/dev-local.sh up
        fi
        
        # Run based on project type
        case "$PROJECT_TYPE" in
            rust)
                echo "Running Rust E2E tests..."
                cargo test --test e2e
                ;;
            node)
                echo "Running Node.js E2E tests..."
                npx playwright test
                ;;
            python)
                echo "Running Python E2E tests..."
                pytest tests/e2e/
                ;;
            *)
                echo "Running default E2E tests..."
                ./e2e-runner
                ;;
        esac
        
        echo ""
        echo "✓ E2E tests passed!"
        ;;
    
    setup)
        echo "Setting up E2E environment..."
        
        case "$PROJECT_TYPE" in
            rust)
                mkdir -p tests/e2e
                echo "// E2E tests" > tests/e2e/mod.rs
                ;;
            node)
                npm install -D @playwright/test
                npx playwright install --with-deps
                mkdir -p e2e/specs
                ;;
            python)
                pip install pytest playwright
                playwright install --with-deps
                mkdir -p tests/e2e
                ;;
        esac
        
        echo "✓ E2E environment ready!"
        ;;
    
    record)
        echo "Recording new test..."
        npx playwright test --ui
        ;;
    
    *)
        echo "Usage: $0 {run|setup|record}"
        echo ""
        echo "Commands:"
        echo "  run    - Run E2E tests"
        echo "  setup  - Set up E2E environment"
        echo "  record - Record new test (interactive)"
        exit 1
        ;;
esac
