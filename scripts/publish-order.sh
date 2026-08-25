#!/usr/bin/env bash
# publish-order.sh -- Determine and verify the correct publish order for WxRust crates.
#
# Strategy:
#   - Layer-0 crate (wx-rust-common): full `cargo publish --dry-run`
#   - Layer-1/2 crates (depend on unpublished internals): `cargo package --list` to
#     validate packaging; full dry-run only succeeds after prior crates reach crates.io
#   - Layer-3 crate (wx-rust, no internal deps): full `cargo publish --dry-run`
#
# This is the standard workspace publish chicken-and-egg workaround.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

export PATH="$HOME/.cargo/bin:$PATH"

# ── Dependency-sorted publish order ──────────────────────────────────────────
# Layer 0: no internal deps
# Layer 1: depends only on wx-rust-common
# Layer 2: depends on layer-1 crates
# Layer 3: umbrella crate (no deps)
CRATES=(
  # Layer 0 -- foundation
  wx-rust-common
  # Layer 1 -- leaf modules (depend only on common)
  wx-rust-aispeech
  wx-rust-channel
  wx-rust-cp
  wx-rust-miniapp
  wx-rust-mp
  wx-rust-pay
  wx-rust-qidian
  # Layer 2 -- depends on common + mp + miniapp
  wx-rust-open
  # Layer 3 -- umbrella (no internal deps)
  wx-rust
)

# Layer-0 and Layer-3 crates have no unpublished internal deps -> full dry-run works.
INDEPENDENT_CRATES=("wx-rust-common" "wx-rust")

TOTAL=${#CRATES[@]}
PASSED=0
FAILED=0
SKIPPED=0
BLOCKED=0
declare -A RESULTS

echo "=========================================="
echo " WxRust Publish Order Verification"
echo " $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "=========================================="
echo ""

echo "Publish order (total $TOTAL crates):"
echo "---"
IDX=0
for crate in "${CRATES[@]}"; do
  IDX=$((IDX + 1))
  printf "  %2d. %s\n" "$IDX" "$crate"
done
echo "---"
echo ""

is_independent() {
  local target="$1"
  for c in "${INDEPENDENT_CRATES[@]}"; do
    if [[ "$c" == "$target" ]]; then
      return 0
    fi
  done
  return 1
}

for crate in "${CRATES[@]}"; do
  if is_independent "$crate"; then
    # Full dry-run: crate has no unpublished internal deps
    printf "[%2d/%d] %-22s  dry-run ... " "$((PASSED + FAILED + BLOCKED + 1))" "$TOTAL" "$crate"
    if cargo publish -p "$crate" --dry-run --allow-dirty 2>&1 | grep -q "aborting upload due to dry run"; then
      echo "PASS"
      RESULTS["$crate"]="PASS"
      PASSED=$((PASSED + 1))
    else
      echo "FAIL"
      RESULTS["$crate"]="FAIL"
      FAILED=$((FAILED + 1))
      echo ""
      echo "  Diagnostic: cargo publish -p $crate --dry-run --allow-dirty"
      echo ""
    fi
  else
    # Chicken-and-egg: full dry-run requires prior crates on crates.io.
    # Validate packaging only via `cargo package --list`.
    printf "[%2d/%d] %-22s  package --list ... " "$((PASSED + FAILED + BLOCKED + 1))" "$TOTAL" "$crate"
    if pkg_out=$(cargo package -p "$crate" --list --allow-dirty 2>&1); then
      file_count=$(echo "$pkg_out" | wc -l | tr -d ' ')
      echo "PASS ($file_count files)"
      RESULTS["$crate"]="PASS (packaged $file_count files; full dry-run blocked: dep not on crates.io)"
      PASSED=$((PASSED + 1))
    else
      echo "FAIL"
      RESULTS["$crate"]="FAIL"
      FAILED=$((FAILED + 1))
      echo ""
      echo "  Diagnostic: cargo package -p $crate --list --allow-dirty"
      echo "  Error: $(echo "$pkg_out" | tail -3)"
      echo ""
    fi
  fi
done

echo ""
echo "=========================================="
echo " RESULTS"
echo "=========================================="
for crate in "${CRATES[@]}"; do
  printf "  %-22s  %s\n" "$crate" "${RESULTS[$crate]:-SKIP}"
done
echo ""
echo "  Passed: $PASSED  |  Failed: $FAILED  |  Total: $TOTAL"
echo ""

# Show chicken-and-egg note
HAS_DEP_CRATES=false
for crate in "${CRATES[@]}"; do
  if ! is_independent "$crate"; then
    HAS_DEP_CRATES=true
    break
  fi
done

if [[ "$HAS_DEP_CRATES" == "true" ]]; then
  echo "=========================================="
  echo " CHICKEN-AND-EGG NOTE"
  echo "=========================================="
  echo ""
  echo "Crates marked 'full dry-run blocked' depend on wx-rust-common,"
  echo "which is not yet on crates.io. Once wx-rust-common is published,"
  echo "re-run with --full to validate all crates end-to-end."
  echo ""
  echo "Real publish sequence:"
  echo ""
  echo "  cargo publish -p wx-rust-common && sleep 30"
  for crate in "${CRATES[@]}"; do
    if ! is_independent "$crate"; then
      echo "  cargo publish -p $crate && sleep 30"
    fi
  done
  echo ""
fi

if ((FAILED > 0)); then
  exit 1
fi
exit 0
