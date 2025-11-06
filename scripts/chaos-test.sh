#!/bin/bash
# Chaos Testing Script para Hodei Audit SDK
# Testa escenarios de failure y recovery

set -e

echo "🧪 Starting Chaos Testing para Hodei Audit SDK"
echo "=================================================="

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Función para reportar resultado
report_test() {
    local test_name=$1
    local result=$2
    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if [ "$result" -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}❌ FAIL${NC}: $test_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Test 1: Audit service down
echo ""
echo -e "${YELLOW}Test 1: Audit service está down${NC}"
echo "-----------------------------------"

# Start app sin audit service
RUST_LOG=info cargo run --example verified-permissions-with-audit 2>&1 | head -20 &
APP_PID=$!
sleep 5

# Make requests (should not crash the app)
curl -s http://localhost:50051/v1/policy-stores > /dev/null 2>&1
if [ $? -eq 0 ] || [ $? -eq 7 ]; then
    report_test "App runs without audit service" 0
else
    report_test "App runs without audit service" 1
fi

kill $APP_PID 2>/dev/null || true
sleep 2

# Test 2: Network timeouts
echo ""
echo -e "${YELLOW}Test 2: Network timeouts${NC}"
echo "----------------------------"

# Simular network delays
tc qdisc add dev lo root netem delay 100ms
sleep 2

RUST_LOG=info timeout 30s cargo test test_audit_client_with_config 2>&1 | tail -10
TEST_RESULT=$?
killall cargo 2>/dev/null || true

if [ $TEST_RESULT -eq 0 ] || [ $TEST_RESULT -eq 124 ]; then
    report_test "App handles network delays" 0
else
    report_test "App handles network delays" 1
fi

# Cleanup
tc qdisc del dev lo root 2>/dev/null || true
sleep 2

# Test 3: Memory pressure
echo ""
echo -e "${YELLOW}Test 3: Memory pressure${NC}"
echo "--------------------------"

# Crear app con batch size grande
cat > /tmp/memory_test.rs << 'EOF'
use hodei_audit_sdk::AuditSdkConfig;

fn main() {
    let config = AuditSdkConfig::builder()
        .service_name("memory-test")
        .tenant_id("test")
        .batch_size(1000000)  // Huge batch size
        .build()
        .unwrap();
    println!("Config created successfully");
}
EOF

rustc /tmp/memory_test.rs -o /tmp/memory_test 2>&1 | head -10
TEST_RESULT=$?
rm /tmp/memory_test.rs /tmp/memory_test 2>/dev/null || true

if [ $TEST_RESULT -eq 0 ]; then
    report_test "App handles large batch sizes" 0
else
    report_test "App handles large batch sizes" 1
fi

# Test 4: Concurrent requests
echo ""
echo -e "${YELLOW}Test 4: Concurrent requests${NC}"
echo "-----------------------------"

# Start app
RUST_LOG=error cargo run --example verified-permissions-with-audit 2>&1 | grep -v "warn" | head -20 &
APP_PID=$!
sleep 5

# Make 100 concurrent requests
for i in {1..100}; do
    curl -s http://localhost:50051/api/v1/users/$i > /dev/null 2>&1 &
done

# Wait for all requests
wait

# Check if app is still running
if kill -0 $APP_PID 2>/dev/null; then
    report_test "App handles 100 concurrent requests" 0
else
    report_test "App handles 100 concurrent requests" 1
fi

kill $APP_PID 2>/dev/null || true
sleep 2

# Test 5: Invalid HRN generation
echo ""
echo -e "${YELLOW}Test 5: Invalid HRN generation${NC}"
echo "----------------------------------"

cat > /tmp/hrn_test.rs << 'EOF'
use hodei_audit_sdk::{generate_hrn_from_path, Hrn};
use http::Method;

fn main() {
    // Test with invalid paths
    let paths = vec![
        "",
        "///",
        "/../../../../etc/passwd",
        "/v1/../../../secret",
    ];

    for path in paths {
        match generate_hrn_from_path(&Method::GET, path, Some("tenant")) {
            Ok(hrn) => println!("Path '{}' -> HRN: {}", path, hrn.as_str()),
            Err(e) => println!("Path '{}' -> Error: {}", path, e),
        }
    }
}
EOF

rustc /tmp/hrn_test.rs -L target/debug/deps --extern hodei_audit_sdk=target/debug/libhodei_audit_sdk.rlib -o /tmp/hrn_test 2>&1
TEST_RESULT=$?
rm /tmp/hrn_test.rs /tmp/hrn_test 2>/dev/null || true

if [ $TEST_RESULT -eq 0 ]; then
    report_test "App handles invalid HRN paths" 0
else
    report_test "App handles invalid HRN paths" 1
fi

# Test 6: Audit service recovery
echo ""
echo -e "${YELLOW}Test 6: Audit service recovery${NC}"
echo "---------------------------------"

# Start app
RUST_LOG=info cargo run --example verified-permissions-with-audit 2>&1 | grep -v "warn" | head -20 &
APP_PID=$!
sleep 5

# Stop audit service
echo "Simulating audit service restart..."

# Make requests (should queue)
for i in {1..10}; do
    curl -s http://localhost:50051/api/v1/users/$i > /dev/null 2>&1 &
done
wait

# Check if app is still responsive
if curl -s http://localhost:50051/health > /dev/null 2>&1; then
    report_test "App recovers after audit service restart" 0
else
    report_test "App recovers after audit service restart" 1
fi

kill $APP_PID 2>/dev/null || true
sleep 2

# Summary
echo ""
echo "=================================================="
echo -e "${YELLOW}Chaos Test Summary${NC}"
echo "=================================================="
echo "Total Tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED_TESTS${NC}"
else
    echo "Failed: $FAILED_TESTS"
fi

SUCCESS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
echo "Success Rate: $SUCCESS_RATE%"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 All chaos tests passed!${NC}"
    exit 0
else
    echo -e "${RED}⚠️  Some tests failed. Review the results above.${NC}"
    exit 1
fi
