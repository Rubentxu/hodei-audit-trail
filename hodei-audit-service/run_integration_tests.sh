#!/bin/bash

# Integration Tests Runner Script
# This script runs the integration tests with real ClickHouse and MinIO containers

set -e

echo "========================================"
echo "  Hodei Audit - Integration Tests"
echo "========================================"
echo ""

# Check if Docker is running
if ! docker ps > /dev/null 2>&1; then
    echo "❌ Error: Docker is not running"
    echo "Please start Docker and try again"
    exit 1
fi

echo "✅ Docker is running"
echo ""

# Navigate to service directory
cd "$(dirname "$0")"

echo "🚀 Running integration tests with Testcontainers..."
echo "This will spin up real ClickHouse and MinIO instances"
echo ""

# Run all integration tests
cargo test integration_tests::test -- --nocapture

echo ""
echo "========================================"
echo "  Integration Tests Completed!"
echo "========================================"
echo ""
echo "📊 Test Summary:"
echo "  - ClickHouse integration: ✅"
echo "  - MinIO/S3 integration: ✅"
echo "  - Tiered storage: ✅"
echo "  - End-to-end workflow: ✅"
echo ""
echo "🎉 All tests passed successfully!"
