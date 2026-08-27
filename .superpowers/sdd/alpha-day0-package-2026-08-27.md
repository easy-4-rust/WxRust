# WxRust Alpha Day-0 启动包交付报告

日期：2026-08-27
版本：WxRust v0.1.0
交付物：Alpha 灰度运营装备（模板 + 脚本 + 闸门）

---

## 一、交付文件清单

### 文档模板（docs/operations/alpha-2026-q3/）

| 文件 | 用途 |
|------|------|
| `day-0-onboarding-checklist.md` | 6 步接入清单：选型 → Cargo.toml → ConfigStorage → Mock 测试 → 真环境 → 灰度开关 |
| `day-1-observation-report.md` | Day-1 观察报告模板（5 张表：基础指标/失败案例/已知限制/Next-24h/准入准出） |
| `day-3-observation-report.md` | Day-3 观察报告模板（含 3 日趋势分析、错误模式识别） |
| `day-7-observation-report.md` | Day-7 准出报告模板（含 7 日趋势、Alpha→Beta 决策、经验教训） |

### 自动化脚本（scripts/alpha/）

| 文件 | 用途 |
|------|------|
| `check-no-go.sh` | No-Go 闸门检查（6 项检查，任一命中即 exit 1） |
| `collect-metrics.sh` | 指标采集（从 tracing 日志自动 grep 汇总到 JSON） |
| `alpha-exit-gate.sh` | Alpha 准出评估（综合 7 日数据给出 GO/NO-GO/DELAY 判定） |

---

## 二、脚本运行 Demo

### 2.1 check-no-go.sh — 正常场景（GO）

```bash
bash scripts/alpha/check-no-go.sh /tmp/wxrust-alpha-demo/tracing.log /tmp/wxrust-alpha-demo/tracing.log /tmp/wxrust-alpha-demo/gates.json
```

输出：
```
================================================================
  WxRust Alpha No-Go 闸门检查
================================================================

检查 1: 资金类接口 errcode!=0 且 data 为空
  [PASS] 未发现资金类接口异常空响应

检查 2: panic / unwrap 失败 / abort
  [PASS] 未发现 panic/unwrap 失败/abort

检查 3: Token 单飞失效（并发刷新检测）
  Token 刷新次数: 3
  API 请求数: 49
  [PASS] Token 刷新频率正常（3 次，峰值 1 次/分钟）

检查 4: 内存持续增长检测
  首次内存: 45MB
  末次内存: 46MB
  峰值内存: 48MB
  增长: 1MB (2%)
  [PASS] 内存稳定（增长 2%，45MB -> 46MB）

检查 5: 覆盖率门禁 (< 60%)
  当前覆盖率: 69.05%
  [PASS] 覆盖率 69.05% 满足 >= 60% 门禁

检查 6: 镜像率门禁 (< 30%)
  当前镜像率: 45.2%
  [PASS] 镜像率 45.2% 满足 >= 30% 门禁

结果: GO — 未发现阻塞性问题
```

### 2.2 check-no-go.sh — 异常场景（NO-GO）

```bash
bash scripts/alpha/check-no-go.sh /tmp/wxrust-alpha-demo/bad-tracing.log /tmp/wxrust-alpha-demo/bad-tracing.log /tmp/wxrust-alpha-demo/gates.json
```

输出：
```
检查 1: 资金类接口 errcode!=0 且 data 为空
  发现 2 个资金类接口 errcode!=0 且 data 为空的响应
  [NO-GO]

检查 2: panic / unwrap 失败 / abort
  发现 2 个 panic/unwrap 失败/abort
  [NO-GO]

检查 3: Token 单飞失效（并发刷新检测）
  Token 刷新次数(4) > API 请求数(3)，疑似并发刷新风暴
  [NO-GO]

结果: NO-GO — 发现 3 个阻塞性问题
```

### 2.3 collect-metrics.sh — 指标采集

```bash
bash scripts/alpha/collect-metrics.sh /tmp/wxrust-alpha-demo/tracing.log /tmp/wxrust-alpha-demo/metrics
```

输出（JSON）：
```json
{
  "date": "2026-08-27",
  "requests": { "total": 55, "errors": 0, "success_rate": 100.00 },
  "latency": { "p99_ms": 150, "avg_ms": 107 },
  "token": { "refreshes": 3, "errors": 0 },
  "stability": { "panic_count": 0, "unwrap_failures": 0, "abort_count": 0 },
  "memory": { "current_mb": 45 },
  "circuit_breaker": { "state": "unknown" },
  "wx_error_codes": { ... }
}
```

### 2.4 alpha-exit-gate.sh — 准出评估

```bash
bash scripts/alpha/alpha-exit-gate.sh /tmp/wxrust-alpha-demo/metrics
```

输出：
```
检查 1: 7 日观察期完成度
  [FAIL] 观察期严重不足（1/7 天），无法准出

检查 2: 请求成功率（7 日均值 >= 99.5%）
  [PASS] 成功率 100.00% 满足 >= 99.5% 门禁

检查 3: P99 延迟
  [PASS] P99 延迟正常（均值 150ms，峰值 150ms）

检查 4: Token 刷新稳定性（< 6 次/h/appid）
  [PASS] Token 刷新频率正常（0.1 次/h）

检查 5: 稳定性（Panic = 0）
  [PASS] 7 日零 panic/abort

检查 6: 覆盖率（>= 60%）
  [WARN] 未找到覆盖率数据（需手动确认）

检查 7: 代码变更风险
  [WARN] Alpha 期间 crates/ 下有 70 个文件变更，需评估稳定性

判定: NO-GO
原因: 1 个检查项未通过
行动: 修复问题后重新评估
```

---

## 三、No-Go 条件说明

| # | 条件 | 检测方式 | 命中逻辑 |
|---|------|---------|---------|
| 1 | 资金类接口 errcode!=0 且 data 为空 | grep tracing 日志 | 任何 1 次即 NO-GO |
| 2 | panic / unwrap 失败 / abort | grep cargo test + tracing | 任何 1 次即 NO-GO |
| 3 | Token 并发刷新风暴 | 刷新次数 > 请求数 | 任何 1 次即 NO-GO |
| 4 | 内存持续增长 | 3 日趋势分析 | 增长 > 50% 即 NO-GO |
| 5 | 覆盖率 < 60% | 从 gates JSON 读取 | 低于门禁即 NO-GO |
| 6 | 镜像率 < 30% | 从 gates JSON 读取 | 低于门禁即 NO-GO |

---

## 四、使用方式

### Day-0 接入
```bash
# 参照清单逐步执行
cat docs/operations/alpha-2026-q3/day-0-onboarding-checklist.md
```

### 每日指标采集
```bash
# 采集当日指标
bash scripts/alpha/collect-metrics.sh /path/to/tracing.log ./metrics

# 检查 No-Go 条件
bash scripts/alpha/check-no-go.sh /path/to/cargo-test.log /path/to/tracing.log /path/to/gates.json
```

### Day-7 准出评估
```bash
# 综合评估
bash scripts/alpha/alpha-exit-gate.sh ./metrics /path/to/cargo-test.log /path/to/project
```

---

## 五、约束遵守

- 全部使用 bash + grep + awk，零 Python 依赖
- 脚本可直接运行，含 demo 数据验证
- 未修改任何 crate 业务代码
- 模板基于真实项目结构（known-issues.md、覆盖率 61.57%、镜像率等）
