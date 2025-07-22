#!/usr/bin/env bash
# Copyright (c) 2025 Cowboy AI, LLC.
# Script to run tests and save results for README display

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Run tests and capture output
echo "Running all tests..."
TEST_OUTPUT=$(cargo test --all-features 2>&1)
TEST_EXIT_CODE=$?

# Extract all test results
ALL_RESULTS=$(echo "$TEST_OUTPUT" | grep -E "test result:" | grep -v "Doc-tests")
SUMMARY=$(echo "$ALL_RESULTS" | tail -1)

# Count all tests across all suites
TOTAL=0
FAILED=0
while IFS= read -r line; do
    if [[ -n "$line" ]]; then
        PASSED=$(echo "$line" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" || echo "0")
        FAIL=$(echo "$line" | grep -oE "[0-9]+ failed" | grep -oE "[0-9]+" || echo "0")
        TOTAL=$((TOTAL + PASSED + FAIL))
        FAILED=$((FAILED + FAIL))
    fi
done <<< "$ALL_RESULTS"

# Save to file
cat > test-results.json << EOF
{
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "total": ${TOTAL:-0},
  "passed": ${TOTAL:-0},
  "failed": ${FAILED:-0},
  "exit_code": $TEST_EXIT_CODE,
  "summary": "$SUMMARY"
}
EOF

# Also save a markdown summary
cat > test-results.md << EOF
## Test Results

Last run: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

### Summary
- **Total Tests**: ${TOTAL:-0}
- **Passed**: $((TOTAL - FAILED))
- **Failed**: ${FAILED:-0}

### Test Suites
\`\`\`
$ALL_RESULTS
\`\`\`
EOF

# Display results
if [ $TEST_EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
else
    echo -e "${RED}✗ Some tests failed${NC}"
fi

echo "$SUMMARY"
exit $TEST_EXIT_CODE