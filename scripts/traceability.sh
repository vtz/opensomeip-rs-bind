#!/usr/bin/env bash
# Copyright (c) 2025 Vinicius Tadeu Zein
# SPDX-License-Identifier: Apache-2.0
#
# Extract @implements and @tests annotations from Rust source files
# and produce a traceability matrix.
#
# Usage: ./scripts/traceability.sh [crates/...]
#
# @implements REQ_RUST_007

set -euo pipefail

SEARCH_DIR="${1:-crates}"

echo "========================================="
echo " Traceability Matrix — opensomeip-rs-bind"
echo "========================================="
echo ""

echo "── @implements annotations ──"
echo ""
grep -rn '@implements' "$SEARCH_DIR" --include='*.rs' | \
    sed 's|//[!/]*\s*@implements\s*||; s|///\s*@implements\s*||' | \
    sort || echo "(none found)"

echo ""
echo "── @tests annotations ──"
echo ""
grep -rn '@tests' "$SEARCH_DIR" --include='*.rs' | \
    sed 's|//[!/]*\s*@tests\s*||; s|///\s*@tests\s*||' | \
    sort || echo "(none found)"

echo ""
echo "── Requirements coverage summary ──"
echo ""

for req in REQ_RUST_001 REQ_RUST_002 REQ_RUST_003 REQ_RUST_004 REQ_RUST_005 REQ_RUST_006 REQ_RUST_007; do
    impl_count=$({ grep -r "@implements.*$req" "$SEARCH_DIR" --include='*.rs' 2>/dev/null || true; } | wc -l | tr -d ' ')
    test_count=$({ grep -r "@tests.*$req" "$SEARCH_DIR" --include='*.rs' 2>/dev/null || true; } | wc -l | tr -d ' ')
    printf "  %-15s  impl: %2s  tests: %2s\n" "$req" "$impl_count" "$test_count"
done

echo ""
echo "Done."
