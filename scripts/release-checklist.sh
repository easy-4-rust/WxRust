#!/usr/bin/env bash
# release-checklist.sh -- Release gate checklist for WxRust workspace.
# Runs all pre-release checks, collects PASS/FAIL/WARN results,
# and prints a summary matrix. Exits non-zero ONLY when FAIL > 0.
#
# Usage:
#   bash scripts/release-checklist.sh          # text output
#   bash scripts/release-checklist.sh --json   # machine-readable JSON to stdout
#
# Environment:
#   export PATH="$HOME/.cargo/bin:$PATH"

set -uo pipefail
# NOTE: no set -e -- we collect failures manually

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

export PATH="$HOME/.cargo/bin:$PATH"

# ---------- result storage ----------
PASS=0
FAIL=0
WARN=0
CHECK_NAMES=()
declare -A CHECK_STATUS   # name -> PASS|FAIL|WARN
declare -A CHECK_DETAIL   # name -> short detail string

JSON_MODE=false
if [[ "${1:-}" == "--json" ]]; then
  JSON_MODE=true
fi

# ---------- helpers ----------
log() {
  if ! $JSON_MODE; then
    echo "$@"
  fi
}

record() {
  local name="$1" status="$2" detail="${3:-}"
  CHECK_NAMES+=("$name")
  CHECK_STATUS["$name"]="$status"
  CHECK_DETAIL["$name"]="$detail"
  case "$status" in
    PASS) ((PASS++)) ;;
    FAIL) ((FAIL++)) ;;
    WARN) ((WARN++)) ;;
  esac
  if ! $JSON_MODE; then
    local tag
    case "$status" in
      PASS) tag="[PASS]" ;;
      FAIL) tag="[FAIL]" ;;
      WARN) tag="[WARN]" ;;
    esac
    printf "  %-52s %s" "$name" "$tag"
    if [[ -n "$detail" ]]; then
      printf "  (%s)" "$detail"
    fi
    echo ""
  fi
}

# Run a command; record PASS if exit 0, FAIL otherwise.
# Captures first 10 lines of output on failure.
run_check() {
  local name="$1"; shift
  local output
  if output=$("$@" 2>&1); then
    record "$name" "PASS"
  else
    local snippet
    snippet=$(echo "$output" | tail -5 | tr '\n' ' ' | cut -c1-120)
    record "$name" "FAIL" "$snippet"
  fi
}

# Run a command; record PASS/WARN/FAIL based on exit code.
# 0 -> PASS, non-zero -> FAIL.  Used when absence is a warning.
run_check_warn() {
  local name="$1"; shift
  local output
  if output=$("$@" 2>&1); then
    record "$name" "PASS"
  else
    record "$name" "WARN" "tool not available or check skipped"
  fi
}

# Check file existence; PASS if exists, FAIL otherwise.
check_file() {
  local name="$1" path="$2"
  if [[ -f "$path" ]]; then
    record "$name" "PASS" "$path"
  else
    record "$name" "FAIL" "$path not found"
  fi
}

# Check file existence; PASS if exists, WARN otherwise.
check_file_warn() {
  local name="$1" path="$2"
  if [[ -f "$path" ]]; then
    record "$name" "PASS" "$path"
  else
    record "$name" "WARN" "$path not found"
  fi
}

# ---------- banner ----------
log "============================================================"
log " WxRust Release Checklist"
log " $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
log " Repository: $REPO_ROOT"
log "============================================================"
log ""

# ============================================================
# 1. Git Status
# ============================================================
log "--- Git Status ---"

# 1a. Uncommitted changes
if git diff --quiet && git diff --cached --quiet; then
  record "git: no uncommitted changes" "PASS"
else
  dirty_files=$(git status --short | wc -l | tr -d ' ')
  record "git: no uncommitted changes" "FAIL" "${dirty_files} file(s) with changes"
fi

# 1b. Untracked files
untracked=$(git ls-files --others --exclude-standard | wc -l | tr -d ' ')
if [[ "$untracked" -eq 0 ]]; then
  record "git: no untracked files" "PASS"
else
  record "git: no untracked files" "WARN" "${untracked} untracked file(s)"
fi

# 1c. On a branch (not detached HEAD)
if git symbolic-ref -q HEAD >/dev/null 2>&1; then
  branch=$(git symbolic-ref --short HEAD)
  record "git: on branch" "PASS" "$branch"
else
  record "git: on branch" "WARN" "detached HEAD"
fi

log ""

# ============================================================
# 2. Cargo Quality Gates
# ============================================================
log "--- Cargo Quality Gates ---"

run_check "cargo check --workspace" cargo check --workspace --all-features
run_check "cargo fmt --check" cargo fmt --all -- --check
run_check "cargo clippy (deny warnings)" cargo clippy --workspace --all-targets --all-features -- -D warnings
run_check "cargo test --workspace" cargo test --workspace

log ""

# ============================================================
# 3. Security & Dependency Audit
# ============================================================
log "--- Security & Dependency Audit ---"

if command -v cargo-audit >/dev/null 2>&1 || cargo audit --version >/dev/null 2>&1; then
  run_check "cargo audit" cargo audit --ignore RUSTSEC-2023-0071
else
  record "cargo audit" "WARN" "cargo-audit not installed"
fi

if command -v cargo-deny >/dev/null 2>&1 || cargo deny --version >/dev/null 2>&1; then
  run_check "cargo deny check" cargo deny check
else
  record "cargo deny check" "WARN" "cargo-deny not installed"
fi

log ""

# ============================================================
# 4. Code Coverage
# ============================================================
log "--- Code Coverage ---"

if command -v cargo-llvm-cov >/dev/null 2>&1 || cargo llvm-cov --version >/dev/null 2>&1; then
  # Use a subshell to capture the output; --fail-under-lines 60 gates the exit code
  cov_output=$(cargo llvm-cov --workspace --fail-under-lines 60 --summary-only 2>&1) && cov_exit=0 || cov_exit=$?
  if [[ $cov_exit -eq 0 ]]; then
    # Extract the total line coverage percentage from the summary
    cov_pct=$(echo "$cov_output" | grep -E '^TOTAL' | awk '{print $NF}' | tr -d '%')
    record "coverage >= 60% line" "PASS" "${cov_pct:-?}%"
  else
    cov_pct=$(echo "$cov_output" | grep -E '^TOTAL' | awk '{print $NF}' | tr -d '%')
    record "coverage >= 60% line" "FAIL" "${cov_pct:-below threshold}%"
  fi
else
  record "coverage >= 60% line" "WARN" "cargo-llvm-cov not installed"
fi

log ""

# ============================================================
# 5. Release Documentation
# ============================================================
log "--- Release Documentation ---"

check_file     "CHANGELOG.md exists"         "CHANGELOG.md"
check_file     "known-issues.md exists"      "known-issues.md"

log ""

# ============================================================
# 6. Publish Precheck (delegates to scripts/publish-precheck.sh)
# ============================================================
log "--- Publish Precheck ---"

if [[ -x "scripts/publish-precheck.sh" ]]; then
  # Run the publish precheck; it has its own pass/fail matrix.
  # We capture output and only record the final exit code.
  precheck_output=$(bash scripts/publish-precheck.sh 2>&1) && precheck_exit=0 || precheck_exit=$?
  if [[ $precheck_exit -eq 0 ]]; then
    record "publish-precheck.sh" "PASS"
  else
    snippet=$(echo "$precheck_output" | grep -E '\[FAIL\]' | head -3 | tr '\n' '; ' | cut -c1-120)
    record "publish-precheck.sh" "FAIL" "$snippet"
  fi
else
  record "publish-precheck.sh" "WARN" "script not found or not executable"
fi

log ""

# ============================================================
# 7. Issue Tracking
# ============================================================
log "--- Issue Tracking ---"

check_file_warn "issues-github-ready doc" "docs/verification/issues-github-ready-2026-08-25.md"

log ""

# ============================================================
# Summary
# ============================================================
TOTAL=$((PASS + FAIL + WARN))

if $JSON_MODE; then
  # Machine-readable JSON output
  echo "{"
  echo "  \"timestamp\": \"$(date -u '+%Y-%m-%dT%H:%M:%SZ')\","
  echo "  \"summary\": { \"total\": $TOTAL, \"pass\": $PASS, \"fail\": $FAIL, \"warn\": $WARN },"
  echo "  \"checks\": ["
  first=true
  for name in "${CHECK_NAMES[@]}"; do
    if ! $first; then echo ","; fi
    first=false
    detail="${CHECK_DETAIL[$name]}"
    # Escape quotes in detail
    detail_escaped=$(echo "$detail" | sed 's/"/\\"/g')
    printf '    { "name": "%s", "status": "%s", "detail": "%s" }' "$name" "${CHECK_STATUS[$name]}" "$detail_escaped"
  done
  echo ""
  echo "  ]"
  echo "}"
else
  log "============================================================"
  log " RESULTS SUMMARY"
  log "============================================================"
  printf "  %-52s %s\n" "CHECK" "RESULT"
  printf "  %-52s %s\n" "----------------------------------------------------" "------"
  for name in "${CHECK_NAMES[@]}"; do
    printf "  %-52s %s\n" "$name" "${CHECK_STATUS[$name]}"
  done
  log ""
  log "------------------------------------------------------------"
  printf "  TOTAL: %d  |  PASS: %d  |  FAIL: %d  |  WARN: %d\n" "$TOTAL" "$PASS" "$FAIL" "$WARN"
  log "------------------------------------------------------------"
  log ""

  if ((FAIL > 0)); then
    log "Result: FAILED -- $FAIL check(s) did not pass. Fix issues before release."
    exit 1
  elif ((WARN > 0)); then
    log "Result: PASSED with WARNINGS -- $WARN item(s) need attention."
    exit 0
  else
    log "Result: ALL PASSED -- ready for release."
    exit 0
  fi
fi

# JSON mode exit code
if ((FAIL > 0)); then
  exit 1
else
  exit 0
fi
