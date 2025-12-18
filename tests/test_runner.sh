#!/usr/bin/env bash
# Test runner script for cascolor
set -e

echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                    cascolor Test Suite                                 ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test categories
TESTS_PASSED=0
TESTS_FAILED=0

echo "Running tests with Docker (rust:latest)..."
echo ""

# Run tests in Docker
docker run --rm \
    -v "$(pwd)":/workspace \
    -w /workspace \
    rust:latest \
    bash -c "
        echo '→ Installing dependencies...' && \
        cargo fetch --quiet && \
        echo '' && \
        echo '→ Running unit tests...' && \
        cargo test --lib --verbose 2>&1 | tee test_output.txt && \
        echo '' && \
        echo '→ Running integration tests...' && \
        cargo test --test '*' --verbose 2>&1 | tee -a test_output.txt && \
        echo '' && \
        echo '→ Running doc tests...' && \
        cargo test --doc --verbose 2>&1 | tee -a test_output.txt
    "

# Check results
if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ All tests passed!${NC}"
    echo ""
    
    # Count test results
    if [ -f test_output.txt ]; then
        PASSED=$(grep -c "test result: ok" test_output.txt || echo "0")
        echo "Test Summary:"
        echo "  ✓ Test suites passed: $PASSED"
        grep "test result: ok" test_output.txt || true
        rm -f test_output.txt
    fi
else
    echo ""
    echo -e "${RED}✗ Some tests failed!${NC}"
    echo ""
    
    if [ -f test_output.txt ]; then
        echo "Failed tests:"
        grep "FAILED" test_output.txt || true
        rm -f test_output.txt
    fi
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════════════════╗"
echo "║                    Test Suite Complete                                 ║"
echo "╚════════════════════════════════════════════════════════════════════════╝"
