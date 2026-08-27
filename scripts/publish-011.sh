#!/usr/bin/env bash
# publish-011.sh -- Publish all WxRust crates v0.1.1 in dependency order (background task)
set -u
REPO_ROOT="/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
cd "$REPO_ROOT"
export PATH="$HOME/.cargo/bin:$PATH"

# Dependency-sorted publish order (Layer 0 -> 3)
CRATES=(
  wx-rust-common
  wx-rust-aispeech
  wx-rust-channel
  wx-rust-cp
  wx-rust-miniapp
  wx-rust-mp
  wx-rust-pay
  wx-rust-qidian
  wx-rust-open
  wx-rust
)

LOG=/tmp/publish-011.log
: > "$LOG"

for c in "${CRATES[@]}"; do
  published=0
  for attempt in 1 2 3 4 5 6; do
    out=$(cargo publish -p "$c" --allow-dirty 2>&1)
    if echo "$out" | grep -qE "Published ${c} v0\.1\.1|already exists"; then
      echo "[OK] $c (attempt $attempt)" | tee -a "$LOG"
      published=1
      break
    fi
    if echo "$out" | grep -qE "429|too many requests|rate limit|api rate limit"; then
      echo "[WAIT] $c attempt $attempt -- rate limit, sleeping 600s" | tee -a "$LOG"
      sleep 600
      continue
    fi
    echo "[FAIL] $c attempt $attempt: $(echo "$out" | tail -4 | tr '\n' ' ')" | tee -a "$LOG"
    break
  done
  if [ "$published" -eq 0 ]; then
    echo "[ABORT] publishing $c failed; stopping." | tee -a "$LOG"
    exit 1
  fi
done

echo "ALL DONE" | tee -a "$LOG"
