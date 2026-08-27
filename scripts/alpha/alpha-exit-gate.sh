#!/usr/bin/env bash
# ============================================================================
# WxRust Alpha 准出自动评估脚本
# 用途：综合 Day-7 报告 + 测试数据 + 代码变更 → 给出 GO/NO-GO/DELAY 判定
# 依赖：bash + grep + awk + jq（可选）
# 用法：bash scripts/alpha/alpha-exit-gate.sh [metrics_dir] [cargo_test_log] [project_root]
# 输出：GO / NO-GO / DELAY 判定 + 详细评估
# ============================================================================

set -euo pipefail

# 参数解析
METRICS_DIR="${1:-./metrics}"
CARGO_TEST_LOG="${2:-}"
PROJECT_ROOT="${3:-.}"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0
TOTAL_CHECKS=0

# ============================================================================
# 工具函数
# ============================================================================

log_header() {
    echo ""
    echo -e "${CYAN}================================================================${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}================================================================${NC}"
}

log_subheader() {
    echo ""
    echo "  --- $1 ---"
}

log_pass() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    PASS_COUNT=$((PASS_COUNT + 1))
    echo -e "  ${GREEN}[PASS]${NC} $1"
}

log_fail() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    FAIL_COUNT=$((FAIL_COUNT + 1))
    echo -e "  ${RED}[FAIL]${NC} $1"
}

log_warn() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    WARN_COUNT=$((WARN_COUNT + 1))
    echo -e "  ${YELLOW}[WARN]${NC} $1"
}

log_info() {
    echo -e "  ${CYAN}[INFO]${NC} $1"
}

# 从 metrics JSON 读取值（兼容无 jq 环境）
get_metric() {
    local file="$1"
    local key="$2"
    if [ -f "$file" ]; then
        local result
        result=$(grep -oE "\"$key\":\s*[0-9.]+" "$file" 2>/dev/null | head -1 | grep -oE '[0-9.]+$' || true)
        result=${result:-0}
        echo "$result" | tr -d '[:space:]'
    else
        echo "0"
    fi
}

# ============================================================================
# 检查 1: 7 日观察期完成度
# ============================================================================

check_observation_period() {
    log_header "检查 1: 7 日观察期完成度"

    local metrics_files=0
    local expected_days=7

    # 统计 metrics 文件数
    if [ -d "$METRICS_DIR" ]; then
        metrics_files=$(ls "$METRICS_DIR"/metrics-*.json 2>/dev/null | wc -l | tr -d ' ')
    fi

    echo "  期望天数: $expected_days"
    echo "  实际天数: $metrics_files"

    if [ "$metrics_files" -ge 7 ]; then
        log_pass "7 日观察期完整覆盖（$metrics_files 天数据）"
    elif [ "$metrics_files" -ge 5 ]; then
        log_warn "观察期不完整（$metrics_files/7 天），建议补充观察"
    else
        log_fail "观察期严重不足（$metrics_files/7 天），无法准出"
    fi
}

# ============================================================================
# 检查 2: 请求成功率（7 日均值）
# ============================================================================

check_success_rate() {
    log_header "检查 2: 请求成功率（7 日均值 >= 99.5%）"

    if [ ! -d "$METRICS_DIR" ]; then
        log_fail "无 metrics 数据目录"
        return
    fi

    local total_sum=0
    local error_sum=0
    local days=0

    for f in "$METRICS_DIR"/metrics-*.json; do
        if [ -f "$f" ]; then
            local total errors
            total=$(get_metric "$f" "total")
            errors=$(get_metric "$f" "errors")
            total_sum=$((total_sum + total))
            error_sum=$((error_sum + errors))
            days=$((days + 1))
        fi
    done

    if [ "$total_sum" -eq 0 ]; then
        log_fail "无请求数据"
        return
    fi

    local success_rate
    success_rate=$(awk "BEGIN {printf \"%.2f\", ($total_sum - $error_sum) * 100 / $total_sum}")

    echo "  7 日总请求: $total_sum"
    echo "  7 日总错误: $error_sum"
    echo "  7 日成功率: ${success_rate}%"

    local below_threshold
    below_threshold=$(echo "$success_rate" | awk '{if ($1 < 99.5) print 1; else print 0}')

    if [ "$below_threshold" -eq 1 ]; then
        log_fail "成功率 ${success_rate}% 低于 99.5% 门禁"
    else
        log_pass "成功率 ${success_rate}% 满足 >= 99.5% 门禁"
    fi
}

# ============================================================================
# 检查 3: P99 延迟（7 日均值 < 基线 + 50%）
# ============================================================================

check_latency() {
    log_header "检查 3: P99 延迟"

    if [ ! -d "$METRICS_DIR" ]; then
        log_fail "无 metrics 数据目录"
        return
    fi

    local max_p99=0
    local sum_p99=0
    local days=0

    for f in "$METRICS_DIR"/metrics-*.json; do
        if [ -f "$f" ]; then
            local p99
            p99=$(get_metric "$f" "p99_ms")
            sum_p99=$((sum_p99 + p99))
            days=$((days + 1))
            if [ "$p99" -gt "$max_p99" ]; then
                max_p99=$p99
            fi
        fi
    done

    if [ "$days" -eq 0 ]; then
        log_fail "无延迟数据"
        return
    fi

    local avg_p99=$((sum_p99 / days))

    echo "  7 日 P99 均值: ${avg_p99}ms"
    echo "  7 日 P99 峰值: ${max_p99}ms"

    # 阈值：500ms（假设基线 300ms + 50% = 450ms，取 500ms 余量）
    if [ "$max_p99" -gt 1000 ]; then
        log_fail "P99 峰值 ${max_p99}ms 超过 1000ms"
    elif [ "$max_p99" -gt 500 ]; then
        log_warn "P99 峰值 ${max_p99}ms 超过 500ms 阈值"
    else
        log_pass "P99 延迟正常（均值 ${avg_p99}ms，峰值 ${max_p99}ms）"
    fi
}

# ============================================================================
# 检查 4: Token 刷新稳定性
# ============================================================================

check_token_stability() {
    log_header "检查 4: Token 刷新稳定性（< 6 次/h/appid）"

    if [ ! -d "$METRICS_DIR" ]; then
        log_fail "无 metrics 数据目录"
        return
    fi

    local max_refreshes=0
    local total_refreshes=0
    local days=0

    for f in "$METRICS_DIR"/metrics-*.json; do
        if [ -f "$f" ]; then
            local refreshes
            refreshes=$(get_metric "$f" "refreshes")
            total_refreshes=$((total_refreshes + refreshes))
            days=$((days + 1))
            if [ "$refreshes" -gt "$max_refreshes" ]; then
                max_refreshes=$refreshes
            fi
        fi
    done

    if [ "$days" -eq 0 ]; then
        log_fail "无 Token 数据"
        return
    fi

    # 按每天 24 小时计算平均
    local avg_per_hour
    avg_per_hour=$(awk "BEGIN {printf \"%.1f\", $total_refreshes / ($days * 24)}")

    echo "  7 日总刷新: $total_refreshes"
    echo "  每小时平均: ${avg_per_hour} 次"

    # 检查是否有 Token 错误
    local total_token_errors=0
    for f in "$METRICS_DIR"/metrics-*.json; do
        if [ -f "$f" ]; then
            local errs
            errs=$(grep -oE '"errors":\s*[0-9]+' "$f" 2>/dev/null | head -1 | grep -oE '[0-9]+' || echo 0)
            # 这里简化处理，实际应该从 token.errors 读取
        fi
    done

    local over_threshold
    over_threshold=$(echo "$avg_per_hour" | awk '{if ($1 > 6) print 1; else print 0}')

    if [ "$over_threshold" -eq 1 ]; then
        log_fail "Token 刷新频率 ${avg_per_hour} 次/h 超过 6 次/h 阈值"
    else
        log_pass "Token 刷新频率正常（${avg_per_hour} 次/h）"
    fi
}

# ============================================================================
# 检查 5: Panic / 异常
# ============================================================================

check_stability() {
    log_header "检查 5: 稳定性（Panic = 0）"

    if [ ! -d "$METRICS_DIR" ]; then
        log_fail "无 metrics 数据目录"
        return
    fi

    local total_panics=0
    local total_unwraps=0
    local total_aborts=0

    for f in "$METRICS_DIR"/metrics-*.json; do
        if [ -f "$f" ]; then
            local panics unwraps aborts
            panics=$(get_metric "$f" "panic_count")
            unwraps=$(get_metric "$f" "unwrap_failures")
            aborts=$(get_metric "$f" "abort_count")
            total_panics=$((total_panics + panics))
            total_unwraps=$((total_unwraps + unwraps))
            total_aborts=$((total_aborts + aborts))
        fi
    done

    echo "  7 日 Panic 总数: $total_panics"
    echo "  7 日 Unwrap 失败: $total_unwraps"
    echo "  7 日 Abort 总数: $total_aborts"

    if [ "$total_panics" -gt 0 ] || [ "$total_aborts" -gt 0 ]; then
        log_fail "发现 $total_panics 个 panic 和 $total_aborts 个 abort"
    elif [ "$total_unwraps" -gt 5 ]; then
        log_warn "发现 $total_unwraps 个 unwrap 失败（虽未 panic，但数量偏高）"
    else
        log_pass "7 日零 panic/abort"
    fi

    # 同时检查 cargo test 日志
    if [ -n "$CARGO_TEST_LOG" ] && [ -f "$CARGO_TEST_LOG" ]; then
        local test_failed
        test_failed=$(grep -c 'FAILED\|test result:.*failed' "$CARGO_TEST_LOG" 2>/dev/null || echo 0)
        if [ "$test_failed" -gt 0 ]; then
            log_fail "cargo test 存在失败用例"
        else
            log_pass "cargo test 全部通过"
        fi
    fi
}

# ============================================================================
# 检查 6: 覆盖率
# ============================================================================

check_coverage() {
    log_header "检查 6: 覆盖率（>= 60%）"

    local coverage=""

    # 从 gates JSON 或 cargo test 日志读取
    if [ -d "$METRICS_DIR" ]; then
        local latest_metrics
        latest_metrics=$(ls -t "$METRICS_DIR"/metrics-*.json 2>/dev/null | head -1)
        if [ -n "$latest_metrics" ]; then
            coverage=$(grep -oE '"coverage":\s*[0-9.]+' "$latest_metrics" 2>/dev/null | head -1 | grep -oE '[0-9.]+$' || echo "")
        fi
    fi

    if [ -z "$coverage" ] && [ -n "$CARGO_TEST_LOG" ] && [ -f "$CARGO_TEST_LOG" ]; then
        coverage=$(grep -oE 'coverage[:=[:space:]]*[0-9]+\.?[0-9]*%?' "$CARGO_TEST_LOG" 2>/dev/null | head -1 | grep -oE '[0-9]+\.?[0-9]*' || echo "")
    fi

    if [ -z "$coverage" ]; then
        log_warn "未找到覆盖率数据（需手动确认）"
        return
    fi

    echo "  当前覆盖率: ${coverage}%"

    local below_threshold
    below_threshold=$(echo "$coverage" | awk '{if ($1 < 60.0) print 1; else print 0}')

    if [ "$below_threshold" -eq 1 ]; then
        log_fail "覆盖率 ${coverage}% 低于 60% 门禁"
    else
        log_pass "覆盖率 ${coverage}% 满足 >= 60% 门禁"
    fi
}

# ============================================================================
# 检查 7: 代码变更风险
# ============================================================================

check_code_changes() {
    log_header "检查 7: 代码变更风险"

    if [ ! -d "$PROJECT_ROOT/.git" ] && [ ! -f "$PROJECT_ROOT/.git" ]; then
        log_info "非 git 仓库（或 .git 不存在），跳过代码变更检查"
        return
    fi

    # 检查 Alpha 期间是否有代码变更
    local changes
    changes=$(cd "$PROJECT_ROOT" && git diff --stat HEAD~10..HEAD 2>/dev/null | tail -1 || echo "无变更或提交不足 10 次")

    echo "  最近 10 次提交变更: $changes"

    # 检查是否有 crates/ 下的业务代码变更
    local crate_changes
    crate_changes=$(cd "$PROJECT_ROOT" && git diff --stat HEAD~10..HEAD -- crates/ 2>/dev/null | wc -l | tr -d ' ' || echo 0)

    echo "  crates/ 下变更文件数: $crate_changes"

    if [ "$crate_changes" -gt 10 ]; then
        log_warn "Alpha 期间 crates/ 下有 $crate_changes 个文件变更，需评估稳定性"
    else
        log_pass "代码变更风险可控（$crate_changes 个文件）"
    fi
}

# ============================================================================
# 综合判定
# ============================================================================

make_verdict() {
    log_header "综合判定"

    echo ""
    echo "  检查项: $TOTAL_CHECKS"
    echo -e "  PASS:   ${GREEN}$PASS_COUNT${NC}"
    echo -e "  FAIL:   ${RED}$FAIL_COUNT${NC}"
    echo -e "  WARN:   ${YELLOW}$WARN_COUNT${NC}"
    echo ""

    # 判定逻辑
    # NO-GO: 任何 FAIL
    # DELAY: 有 WARN 但无 FAIL
    # GO: 全部 PASS

    if [ "$FAIL_COUNT" -gt 0 ]; then
        echo -e "${RED}================================================================${NC}"
        echo -e "${RED}  判定: NO-GO${NC}"
        echo -e "${RED}  原因: $FAIL_COUNT 个检查项未通过${NC}"
        echo -e "${RED}  行动: 修复问题后重新评估${NC}"
        echo -e "${RED}================================================================${NC}"
        exit 1
    elif [ "$WARN_COUNT" -gt 0 ]; then
        echo -e "${YELLOW}================================================================${NC}"
        echo -e "${YELLOW}  判定: DELAY${NC}"
        echo -e "${YELLOW}  原因: $WARN_COUNT 个警告项需关注${NC}"
        echo -e "${YELLOW}  行动: 延长观察期 3-5 天，修复警告项后重新评估${NC}"
        echo -e "${YELLOW}================================================================${NC}"
        exit 2
    else
        echo -e "${GREEN}================================================================${NC}"
        echo -e "${GREEN}  判定: GO → Beta${NC}"
        echo -e "${GREEN}  原因: 全部 $PASS_COUNT 项检查通过${NC}"
        echo -e "${GREEN}  行动: 进入 Beta 阶段，扩展接入范围${NC}"
        echo -e "${GREEN}================================================================${NC}"
        exit 0
    fi
}

# ============================================================================
# 主流程
# ============================================================================

main() {
    echo "================================================================"
    echo "  WxRust Alpha 准出自动评估"
    echo "  时间: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "================================================================"
    echo ""
    echo "输入:"
    echo "  metrics 目录: ${METRICS_DIR}"
    echo "  cargo test 日志: ${CARGO_TEST_LOG:-未提供}"
    echo "  项目根目录: ${PROJECT_ROOT}"

    # 执行所有检查
    check_observation_period
    check_success_rate
    check_latency
    check_token_stability
    check_stability
    check_coverage
    check_code_changes

    # 综合判定
    make_verdict
}

main
