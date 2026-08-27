#!/usr/bin/env bash
# ============================================================================
# WxRust Alpha Day-1 自动仪表采集脚本
# 用途：从测试日志/tracing 自动 grep + 汇总到 JSON
# 依赖：bash + grep + awk（零 Python）
# 用法：bash scripts/alpha/collect-metrics.sh [tracing_log] [output_dir]
# 输出：metrics-YYYY-MM-DD.json
# ============================================================================

set -euo pipefail

# 参数解析
TRACING_LOG="${1:-}"
OUTPUT_DIR="${2:-./metrics}"
DATE_STR=$(date +%Y-%m-%d)
OUTPUT_FILE="$OUTPUT_DIR/metrics-$DATE_STR.json"

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# ============================================================================
# 工具函数
# ============================================================================

safe_grep_count() {
    local pattern="$1"
    local file="$2"
    if [ -f "$file" ]; then
        local result
        result=$(grep -ciE "$pattern" "$file" 2>/dev/null || true)
        result=${result:-0}
        echo "$result" | tr -d '[:space:]'
    else
        echo 0
    fi
}

safe_grep_value() {
    local pattern="$1"
    local file="$2"
    if [ -f "$file" ]; then
        local result
        result=$(grep -oE "$pattern" "$file" 2>/dev/null | head -1 | grep -oE '[0-9]+\.?[0-9]*' || true)
        result=${result:-0}
        echo "$result" | tr -d '[:space:]'
    else
        echo "0"
    fi
}

# ============================================================================
# 数据采集
# ============================================================================

collect_metrics() {
    echo "================================================================"
    echo "  WxRust Alpha 指标采集"
    echo "  时间: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "  日志: ${TRACING_LOG:-未提供}"
    echo "================================================================"
    echo ""

    # --- 基础请求指标 ---
    local total_requests total_errors success_rate
    total_requests=$(safe_grep_count 'http_request|api_call|send_request|wx_rust' "$TRACING_LOG")
    total_errors=$(safe_grep_count 'level=ERROR|error.*wx_rust|errcode[^:]*:[^0]' "$TRACING_LOG")

    if [ "$total_requests" -gt 0 ]; then
        success_rate=$(awk "BEGIN {printf \"%.2f\", ($total_requests - $total_errors) * 100 / $total_requests}")
    else
        success_rate="0.00"
    fi

    echo "  总请求数: $total_requests"
    echo "  错误数: $total_errors"
    echo "  成功率: ${success_rate}%"

    # --- P99 延迟（从日志中提取 duration 字段） ---
    local p99_latency avg_latency
    if [ -f "$TRACING_LOG" ]; then
        # 提取所有 duration 值并计算 P99
        p99_latency=$(grep -oE 'duration[":=[:space:]]*[0-9]+' "$TRACING_LOG" 2>/dev/null \
            | grep -oE '[0-9]+$' \
            | sort -n \
            | awk '{a[NR]=$1} END {if(NR>0) print a[int(NR*0.99)]; else print 0}')
        avg_latency=$(grep -oE 'duration[":=[:space:]]*[0-9]+' "$TRACING_LOG" 2>/dev/null \
            | grep -oE '[0-9]+$' \
            | awk '{s+=$1; n++} END {if(n>0) printf "%.0f", s/n; else print 0}')
    else
        p99_latency="0"
        avg_latency="0"
    fi

    echo "  P99 延迟: ${p99_latency}ms"
    echo "  平均延迟: ${avg_latency}ms"

    # --- Token 刷新指标 ---
    local token_refreshes
    token_refreshes=$(safe_grep_count 'token.*refresh|refresh.*token|get_access_token|set_access_token' "$TRACING_LOG")
    local token_errors
    token_errors=$(safe_grep_count 'token.*error|token.*fail|token.*expired' "$TRACING_LOG")

    echo "  Token 刷新次数: $token_refreshes"
    echo "  Token 错误数: $token_errors"

    # --- Panic / 异常指标 ---
    local panic_count unwrap_failures abort_count
    panic_count=$(safe_grep_count 'panicked at|thread.*panicked|PANIC' "$TRACING_LOG")
    unwrap_failures=$(safe_grep_count 'unwrap.*None|unwrap.*Err|called.*unwrap.*on' "$TRACING_LOG")
    abort_count=$(safe_grep_count 'process.*abort|SIGABRT|abort' "$TRACING_LOG")

    echo "  Panic 数: $panic_count"
    echo "  Unwrap 失败: $unwrap_failures"
    echo "  Abort 数: $abort_count"

    # --- 内存指标 ---
    local memory_mb
    memory_mb=$(safe_grep_value 'memory_mb[":=[:space:]]*[0-9]+' "$TRACING_LOG")
    if [ "$memory_mb" = "0" ]; then
        memory_mb=$(safe_grep_value 'rss[":=[:space:]]*[0-9]+' "$TRACING_LOG")
    fi

    echo "  内存使用: ${memory_mb}MB"

    # --- 熔断器状态 ---
    local circuit_state="unknown"
    if [ -f "$TRACING_LOG" ]; then
        if grep -qi 'circuit.*open\|breaker.*open' "$TRACING_LOG" 2>/dev/null; then
            circuit_state="open"
        elif grep -qi 'circuit.*closed\|breaker.*closed' "$TRACING_LOG" 2>/dev/null; then
            circuit_state="closed"
        elif grep -qi 'circuit.*half\|breaker.*half' "$TRACING_LOG" 2>/dev/null; then
            circuit_state="half_open"
        fi
    fi

    echo "  熔断器状态: $circuit_state"

    # --- 微信错误码分布 ---
    local err_40001 err_40002 err_40003 err_40014 err_40029 err_42001 err_45009 err_other=0
    err_40001=$(safe_grep_count '"errcode":\s*40001' "$TRACING_LOG")
    err_40002=$(safe_grep_count '"errcode":\s*40002' "$TRACING_LOG")
    err_40003=$(safe_grep_count '"errcode":\s*40003' "$TRACING_LOG")
    err_40014=$(safe_grep_count '"errcode":\s*40014' "$TRACING_LOG")
    err_40029=$(safe_grep_count '"errcode":\s*40029' "$TRACING_LOG")
    err_42001=$(safe_grep_count '"errcode":\s*42001' "$TRACING_LOG")
    err_45009=$(safe_grep_count '"errcode":\s*45009' "$TRACING_LOG")

    echo "  微信错误码分布:"
    echo "    40001(凭证无效): $err_40001"
    echo "    40002(凭证过期): $err_40002"
    echo "    40003(OpenID): $err_40003"
    echo "    40014(签名): $err_40014"
    echo "    40029(票据): $err_40029"
    echo "    42001(超时): $err_42001"
    echo "    45009(频率): $err_45009"

    # ============================================================================
    # 输出 JSON
    # ============================================================================

    mkdir -p "$OUTPUT_DIR"

    cat > "$OUTPUT_FILE" << EOF
{
  "date": "$DATE_STR",
  "collected_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "source_log": "${TRACING_LOG:-N/A}",
  "requests": {
    "total": $total_requests,
    "errors": $total_errors,
    "success_rate": $success_rate
  },
  "latency": {
    "p99_ms": $p99_latency,
    "avg_ms": $avg_latency
  },
  "token": {
    "refreshes": $token_refreshes,
    "errors": $token_errors
  },
  "stability": {
    "panic_count": $panic_count,
    "unwrap_failures": $unwrap_failures,
    "abort_count": $abort_count
  },
  "memory": {
    "current_mb": $memory_mb
  },
  "circuit_breaker": {
    "state": "$circuit_state"
  },
  "wx_error_codes": {
    "40001_invalid_credential": $err_40001,
    "40002_expired_credential": $err_40002,
    "40003_invalid_openid": $err_40003,
    "40014_invalid_sign": $err_40014,
    "40029_invalid_ticket": $err_40029,
    "42001_timeout": $err_42001,
    "45009_rate_limit": $err_45009
  }
}
EOF

    echo ""
    echo -e "${GREEN}  指标已写入: $OUTPUT_FILE${NC}"
    echo ""
}

# ============================================================================
# 主流程
# ============================================================================

main() {
    collect_metrics

    echo "================================================================"
    echo "  采集完成"
    echo "================================================================"
}

main
