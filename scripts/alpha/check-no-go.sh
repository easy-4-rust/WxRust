#!/usr/bin/env bash
# ============================================================================
# WxRust Alpha No-Go 闸门脚本
# 用途：检查是否存在阻塞性问题，任一 No-Go 条件命中即 exit 1
# 依赖：bash + grep + awk + wc（零 Python）
# 用法：bash scripts/alpha/check-no-go.sh [cargo_test_log] [tracing_log] [gates_json]
# 示例：bash scripts/alpha/check-no-go.sh /tmp/cargo-test.log /tmp/app.log /tmp/gates.json
# ============================================================================

set -euo pipefail

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 参数解析
CARGO_TEST_LOG="${1:-}"
TRACING_LOG="${2:-}"
GATES_JSON="${3:-}"

NO_GO_COUNT=0
WARN_COUNT=0
CHECKS_RUN=0

# ============================================================================
# 工具函数
# ============================================================================

log_header() {
    echo ""
    echo "================================================================"
    echo "  $1"
    echo "================================================================"
}

log_pass() {
    CHECKS_RUN=$((CHECKS_RUN + 1))
    echo -e "  ${GREEN}[PASS]${NC} $1"
}

log_fail() {
    CHECKS_RUN=$((CHECKS_RUN + 1))
    NO_GO_COUNT=$((NO_GO_COUNT + 1))
    echo -e "  ${RED}[NO-GO]${NC} $1"
}

log_warn() {
    CHECKS_RUN=$((CHECKS_RUN + 1))
    WARN_COUNT=$((WARN_COUNT + 1))
    echo -e "  ${YELLOW}[WARN]${NC} $1"
}

log_skip() {
    echo -e "  ${YELLOW}[SKIP]${NC} $1（未提供输入文件）"
}

# ============================================================================
# 检查 1: 资金类接口 errcode != 0 但 data 为空
# ============================================================================

check_finance_empty_data() {
    log_header "检查 1: 资金类接口 errcode!=0 且 data 为空"

    if [ -z "$TRACING_LOG" ] || [ ! -f "$TRACING_LOG" ]; then
        log_skip "tracing 日志"
        return
    fi

    # 查找 errcode 非 0 且 data 为空或 null 的响应
    # 模式：errcode 不为 0 且 data 字段为空/null
    local finance_patterns="payment|pay|refund|transfer|redpack|merchant|settle"
    local bad_responses=0

    # 方式 1：查找同时包含 errcode!=0 和 data 为空的行
    if grep -qiE "$finance_patterns" "$TRACING_LOG" 2>/dev/null; then
        # 查找 errcode 非 0 的行（支持 JSON 和 key=value 两种格式）
        local err_lines
        err_lines=$(grep -nE '"errcode":\s*[^0]|errcode=[^0]' "$TRACING_LOG" 2>/dev/null | grep -iE "$finance_patterns" || true)

        if [ -n "$err_lines" ]; then
            # 检查这些行中 data 是否为空
            while IFS= read -r line; do
                if echo "$line" | grep -qE '"data":\s*(null|""|\{\}|\[\])|data=null|data=""'; then
                    bad_responses=$((bad_responses + 1))
                    echo "    发现: $line" | head -c 200
                    echo ""
                fi
            done <<< "$err_lines"
        fi
    fi

    if [ "$bad_responses" -gt 0 ]; then
        log_fail "发现 $bad_responses 个资金类接口 errcode!=0 且 data 为空的响应"
    else
        log_pass "未发现资金类接口异常空响应"
    fi
}

# ============================================================================
# 检查 2: panic / unwrap 失败 / abort
# ============================================================================

check_panics() {
    log_header "检查 2: panic / unwrap 失败 / abort"

    local panic_count=0

    # 检查 cargo test 输出
    if [ -n "$CARGO_TEST_LOG" ] && [ -f "$CARGO_TEST_LOG" ]; then
        local test_panics
        test_panics=$(grep -ciE 'panicked at|thread.*panicked|unwrap.*None|unwrap.*Err|process.*abort' "$CARGO_TEST_LOG" 2>/dev/null || true)
        test_panics=${test_panics:-0}
        test_panics=$(echo "$test_panics" | tr -d '[:space:]')
        panic_count=$((panic_count + test_panics))
        if [ "$test_panics" -gt 0 ]; then
            echo "    cargo test 中发现 $test_panics 个 panic/unwrap 失败"
            grep -nE 'panicked at|thread.*panicked|unwrap.*None|unwrap.*Err|process.*abort' "$CARGO_TEST_LOG" 2>/dev/null | head -5
        fi
    else
        log_skip "cargo test 日志"
    fi

    # 检查 tracing 日志
    if [ -n "$TRACING_LOG" ] && [ -f "$TRACING_LOG" ]; then
        local trace_panics
        trace_panics=$(grep -ciE 'panicked at|thread.*panicked|unwrap.*None|unwrap.*Err|process.*abort|FATAL' "$TRACING_LOG" 2>/dev/null || true)
        trace_panics=${trace_panics:-0}
        trace_panics=$(echo "$trace_panics" | tr -d '[:space:]')
        panic_count=$((panic_count + trace_panics))
        if [ "$trace_panics" -gt 0 ]; then
            echo "    tracing 日志中发现 $trace_panics 个 panic/abort 记录"
            grep -nE 'panicked at|thread.*panicked|FATAL|panic' "$TRACING_LOG" 2>/dev/null | head -5
        fi
    else
        log_skip "tracing 日志"
    fi

    if [ "$panic_count" -gt 0 ]; then
        log_fail "发现 $panic_count 个 panic/unwrap 失败/abort"
    else
        log_pass "未发现 panic/unwrap 失败/abort"
    fi
}

# ============================================================================
# 检查 3: Token 单飞失效（并发刷新风暴）
# ============================================================================

check_token_storm() {
    log_header "检查 3: Token 单飞失效（并发刷新检测）"

    if [ -z "$TRACING_LOG" ] || [ ! -f "$TRACING_LOG" ]; then
        log_skip "tracing 日志"
        return
    fi

    # 统计 token 刷新次数
    local refresh_count
    refresh_count=$(grep -ciE 'token.*refresh|refresh.*token|get_access_token' "$TRACING_LOG" 2>/dev/null || true)
    refresh_count=${refresh_count:-0}
    refresh_count=$(echo "$refresh_count" | tr -d '[:space:]')

    # 统计实际 API 请求数
    local request_count
    request_count=$(grep -ciE 'http_request|api_call|send_request' "$TRACING_LOG" 2>/dev/null || true)
    request_count=${request_count:-0}
    request_count=$(echo "$request_count" | tr -d '[:space:]')

    echo "    Token 刷新次数: $refresh_count"
    echo "    API 请求数: $request_count"

    # 检测：刷新次数 > 请求数（异常：应该是一次刷新服务多次请求）
    if [ "$request_count" -gt 0 ] && [ "$refresh_count" -gt "$request_count" ]; then
        log_fail "Token 刷新次数($refresh_count) > API 请求数($request_count)，疑似并发刷新风暴"
    elif [ "$refresh_count" -gt 0 ]; then
        # 检查短时间内大量刷新（1 分钟内 > 3 次）
        local burst_count=0
        if grep -qE 'token.*refresh|refresh.*token' "$TRACING_LOG" 2>/dev/null; then
            # 统计每分钟的刷新次数，找最大值
            burst_count=$(grep -E 'token.*refresh|refresh.*token' "$TRACING_LOG" 2>/dev/null \
                | awk '{print substr($0,1,16)}' \
                | sort | uniq -c | sort -rn | head -1 | awk '{print $1}' || true)
            burst_count=${burst_count:-0}
            burst_count=$(echo "$burst_count" | tr -d '[:space:]')
        fi

        if [ "$burst_count" -gt 3 ]; then
            log_fail "1 分钟内 Token 刷新 $burst_count 次（阈值: 3），疑似并发刷新风暴"
        else
            log_pass "Token 刷新频率正常（$refresh_count 次，峰值 $burst_count 次/分钟）"
        fi
    else
        log_pass "未检测到 Token 刷新记录（可能未触发 token 路径）"
    fi
}

# ============================================================================
# 检查 4: 内存持续增长（无合理释放路径）
# ============================================================================

check_memory_growth() {
    log_header "检查 4: 内存持续增长检测"

    if [ -z "$TRACING_LOG" ] || [ ! -f "$TRACING_LOG" ]; then
        log_skip "tracing 日志"
        return
    fi

    # 从日志中提取内存使用数据（如果有 memory 相关指标）
    local memory_entries
    memory_entries=$(grep -cE 'memory_mb|rss_mb|heap_mb|alloc.*mb' "$TRACING_LOG" 2>/dev/null || true)
    memory_entries=${memory_entries:-0}
    memory_entries=$(echo "$memory_entries" | tr -d '[:space:]')

    if [ "$memory_entries" -lt 3 ]; then
        log_skip "内存指标（日志中内存数据不足 3 条，无法判断趋势）"
        return
    fi

    # 提取内存值并计算趋势
    local first_mem last_mem max_mem
    first_mem=$(grep -oE 'memory_mb[":=[:space:]]*[0-9]+' "$TRACING_LOG" 2>/dev/null | head -1 | grep -oE '[0-9]+$' || true)
    first_mem=${first_mem:-0}
    first_mem=$(echo "$first_mem" | tr -d '[:space:]')
    last_mem=$(grep -oE 'memory_mb[":=[:space:]]*[0-9]+' "$TRACING_LOG" 2>/dev/null | tail -1 | grep -oE '[0-9]+$' || true)
    last_mem=${last_mem:-0}
    last_mem=$(echo "$last_mem" | tr -d '[:space:]')
    max_mem=$(grep -oE 'memory_mb[":=[:space:]]*[0-9]+' "$TRACING_LOG" 2>/dev/null | grep -oE '[0-9]+$' | sort -rn | head -1 || true)
    max_mem=${max_mem:-0}
    max_mem=$(echo "$max_mem" | tr -d '[:space:]')

    echo "    首次内存: ${first_mem}MB"
    echo "    末次内存: ${last_mem}MB"
    echo "    峰值内存: ${max_mem}MB"

    if [ "$first_mem" -gt 0 ] && [ "$last_mem" -gt 0 ]; then
        local growth=$((last_mem - first_mem))
        local growth_pct=$((growth * 100 / first_mem))

        echo "    增长: ${growth}MB (${growth_pct}%)"

        # 增长超过 50% 视为异常
        if [ "$growth_pct" -gt 50 ]; then
            log_fail "内存增长 ${growth_pct}%（${first_mem}MB -> ${last_mem}MB），疑似内存泄漏"
        elif [ "$growth_pct" -gt 20 ]; then
            log_warn "内存增长 ${growth_pct}%（${first_mem}MB -> ${last_mem}MB），需持续观察"
        else
            log_pass "内存稳定（增长 ${growth_pct}%，${first_mem}MB -> ${last_mem}MB）"
        fi
    else
        log_pass "内存数据正常（无法解析具体数值，未检测到异常模式）"
    fi
}

# ============================================================================
# 检查 5: 覆盖率 < 60%
# ============================================================================

check_coverage() {
    log_header "检查 5: 覆盖率门禁 (< 60%)"

    local coverage=""

    # 从 gates JSON 读取
    if [ -n "$GATES_JSON" ] && [ -f "$GATES_JSON" ]; then
        coverage=$(grep -oE '"coverage":\s*[0-9.]+' "$GATES_JSON" 2>/dev/null | head -1 | grep -oE '[0-9.]+' || echo "")
    fi

    # 从 cargo test 日志读取
    if [ -z "$coverage" ] && [ -n "$CARGO_TEST_LOG" ] && [ -f "$CARGO_TEST_LOG" ]; then
        coverage=$(grep -oE 'coverage[:=[:space:]]*[0-9]+\.?[0-9]*%?' "$CARGO_TEST_LOG" 2>/dev/null | head -1 | grep -oE '[0-9]+\.?[0-9]*' || echo "")
    fi

    if [ -z "$coverage" ]; then
        log_skip "覆盖率（未找到覆盖率数据）"
        return
    fi

    echo "    当前覆盖率: ${coverage}%"

    # 比较（bash 只支持整数比较，用 awk 处理浮点）
    local below_threshold
    below_threshold=$(echo "$coverage" | awk '{if ($1 < 60.0) print 1; else print 0}')

    if [ "$below_threshold" -eq 1 ]; then
        log_fail "覆盖率 ${coverage}% 低于 60% 门禁"
    else
        log_pass "覆盖率 ${coverage}% 满足 >= 60% 门禁"
    fi
}

# ============================================================================
# 检查 6: 镜像率 < 30%
# ============================================================================

check_mirror_rate() {
    log_header "检查 6: 镜像率门禁 (< 30%)"

    local mirror_rate=""

    # 从 gates JSON 读取
    if [ -n "$GATES_JSON" ] && [ -f "$GATES_JSON" ]; then
        mirror_rate=$(grep -oE '"mirror_rate":\s*[0-9.]+' "$GATES_JSON" 2>/dev/null | head -1 | grep -oE '[0-9.]+' || echo "")
    fi

    if [ -z "$mirror_rate" ]; then
        log_skip "镜像率（未找到镜像率数据）"
        return
    fi

    echo "    当前镜像率: ${mirror_rate}%"

    local below_threshold
    below_threshold=$(echo "$mirror_rate" | awk '{if ($1 < 30.0) print 1; else print 0}')

    if [ "$below_threshold" -eq 1 ]; then
        log_fail "镜像率 ${mirror_rate}% 低于 30% 门禁"
    else
        log_pass "镜像率 ${mirror_rate}% 满足 >= 30% 门禁"
    fi
}

# ============================================================================
# 主流程
# ============================================================================

main() {
    echo "================================================================"
    echo "  WxRust Alpha No-Go 闸门检查"
    echo "  时间: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "================================================================"
    echo ""
    echo "输入文件:"
    echo "  cargo test 日志: ${CARGO_TEST_LOG:-未提供}"
    echo "  tracing 日志:    ${TRACING_LOG:-未提供}"
    echo "  gates JSON:      ${GATES_JSON:-未提供}"

    # 执行所有检查
    check_finance_empty_data
    check_panics
    check_token_storm
    check_memory_growth
    check_coverage
    check_mirror_rate

    # 汇总
    log_header "检查汇总"
    echo ""
    echo "  检查项: $CHECKS_RUN"
    echo -e "  NO-GO:  ${RED}$NO_GO_COUNT${NC}"
    echo -e "  WARN:   ${YELLOW}$WARN_COUNT${NC}"
    echo ""

    if [ "$NO_GO_COUNT" -gt 0 ]; then
        echo -e "${RED}================================================================${NC}"
        echo -e "${RED}  结果: NO-GO — 发现 $NO_GO_COUNT 个阻塞性问题${NC}"
        echo -e "${RED}================================================================${NC}"
        exit 1
    else
        echo -e "${GREEN}================================================================${NC}"
        echo -e "${GREEN}  结果: GO — 未发现阻塞性问题${NC}"
        echo -e "${GREEN}================================================================${NC}"
        exit 0
    fi
}

main
