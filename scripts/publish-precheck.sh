#!/usr/bin/env bash
# publish-precheck.sh -- Pre-publish quality gate for WxRust workspace.
# Runs workspace-wide checks + per-crate packaging validation.
# Prints a pass/fail matrix. Exits non-zero on any failure.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

export PATH="$HOME/.cargo/bin:$PATH"

PASS=0
FAIL=0
declare -A RESULTS

run_check() {
  local name="$1"
  shift
  printf "  %-48s" "$name"
  if output=$("$@" 2>&1); then
    echo "[PASS]"
    RESULTS["$name"]="PASS"
    ((PASS++))
  else
    echo "[FAIL]"
    RESULTS["$name"]="FAIL"
    ((FAIL++))
    RESULTS["${name}_output"]="$(echo "$output" | head -20)"
  fi
}

echo "=========================================="
echo " WxRust Publish Precheck"
echo " $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "=========================================="
echo ""

echo "--- Workspace Quality Checks ---"
run_check "cargo check --workspace" cargo check --workspace --all-features
run_check "cargo test --workspace" cargo test --workspace
run_check "cargo clippy (deny warnings)" cargo clippy --workspace --all-targets --all-features -- -D warnings
run_check "cargo fmt --check" cargo fmt --all -- --check
run_check "cargo audit" cargo audit --ignore RUSTSEC-2023-0071
run_check "cargo deny check" cargo deny check

echo ""
echo "--- Per-crate packaging validation ---"
# Independent crates (no unpublished internal deps): full dry-run
# Dependent crates: package --list (chicken-and-egg: dry-run needs deps on crates.io)
INDEPENDENT_CRATES=("wx-rust-common" "wx-rust")
ALL_CRATES=(
  wx-rust-common
  wx-rust-aispeech
  wx-rust-channel
  wx-rust-cp
  wx-rust-miniapp
  wx-rust-mp
  wx-rust-open
  wx-rust-pay
  wx-rust-qidian
  wx-rust
)

is_independent() {
  local target="$1"
  for c in "${INDEPENDENT_CRATES[@]}"; do
    if [[ "$c" == "$target" ]]; then return 0; fi
  done
  return 1
}

for crate in "${ALL_CRATES[@]}"; do
  if is_independent "$crate"; then
    run_check "dry-run: $crate" cargo publish -p "$crate" --dry-run --allow-dirty
  else
    run_check "package --list: $crate" cargo package -p "$crate" --list --allow-dirty
  fi
done

echo ""
echo "=========================================="
echo " RESULTS SUMMARY"
echo "=========================================="
printf "  %-48s %s\n" "CHECK" "RESULT"
printf "  %-48s %s\n" "------------------------------------------------" "------"
for key in \
  "cargo check --workspace" \
  "cargo test --workspace" \
  "cargo clippy (deny warnings)" \
  "cargo fmt --check" \
  "cargo audit" \
  "cargo deny check"; do
  printf "  %-48s %s\n" "$key" "${RESULTS[$key]:-SKIP}"
done
for crate in "${ALL_CRATES[@]}"; do
  if is_independent "$crate"; then
    printf "  %-48s %s\n" "dry-run: $crate" "${RESULTS[dry-run: $crate]:-SKIP}"
  else
    printf "  %-48s %s\n" "package --list: $crate" "${RESULTS[package --list: $crate]:-SKIP}"
  fi
done

echo ""
echo "  Passed: $PASS  |  Failed: $FAIL"
echo ""

if ((FAIL > 0)); then
  echo "--- FAILURE DIAGNOSTICS ---"
  for key in "${!RESULTS[@]}"; do
    if [[ "${RESULTS[$key]}" == "FAIL" ]]; then
      echo ""
      echo ">>> $key FAILED. Output (first 20 lines):"
      echo "${RESULTS[${key}_output]:-  (no output captured)}"
    fi
  done
  echo ""
  echo "Precheck FAILED. Fix issues above before publishing."
  exit 1
fi

echo "Precheck PASSED. Ready for publish."
exit 0
