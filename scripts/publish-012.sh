#!/usr/bin/env bash
# publish-012.sh -- Publish all WxRust crates v0.1.2 in dependency order
# 修复 publish-011.sh 的判断问题：误把 cargo publish 输出中"note: waiting for X to be
# available at registry" 的 "available" 当作错误匹配，正则改为只匹配明确的失败信号。
set -u
REPO_ROOT="/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
cd "$REPO_ROOT"
export PATH="$HOME/.cargo/bin:$PATH"

CRATES=(
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

LOG=/tmp/publish-012.log
: > "$LOG"

for c in "${CRATES[@]}"; do
  published=0
  for attempt in 1 2 3 4 5 6; do
    out=$(cargo publish -p "$c" --allow-dirty 2>&1)
    # 成功标志：明确的 "Published <crate> v<...> at registry" 或 "already ... exists"
    if echo "$out" | grep -qE "^[[:space:]]*Published ${c} v[0-9]+\.[0-9]+\.[0-9]+ at registry"; then
      echo "[OK] $c (attempt $attempt)" | tee -a "$LOG"
      published=1
      break
    fi
    if echo "$out" | grep -qiE "already (uploaded|exists)"; then
      echo "[OK] $c already published" | tee -a "$LOG"
      published=1
      break
    fi
    # 失败：429 限流 → 等
    if echo "$out" | grep -qiE "429|too many requests|rate limit"; then
      echo "[WAIT] $c attempt $attempt -- rate limit, sleeping 600s" | tee -a "$LOG"
      sleep 600
      continue
    fi
    # 其它失败：终止前 dump 末尾几行
    echo "[FAIL] $c attempt $attempt: $(echo "$out" | tail -4 | tr '\n' ' ')" | tee -a "$LOG"
    break
  done
  if [ "$published" -eq 0 ]; then
    echo "[ABORT] publishing $c failed; stopping." | tee -a "$LOG"
    exit 1
  fi
done

echo "ALL DONE" | tee -a "$LOG"