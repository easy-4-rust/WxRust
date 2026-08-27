# WxRust Alpha Day-5 报告模板（凭证到位即可用）

日期：2026-08-27
模板路径：`docs/operations/alpha-2026-q3/day-5-observation-report.md`（本日仅留模板，**真实数据待用户提供 test appid + appsecret 后由用户/AI 填充**）

> **诚实声明**：本会话无法独立完成 Day-5 真实流量验证（缺 test appid + appsecret）。模板已就位，凭证到位即可执行 + 填数。

---

## 准备清单（凭证到位前用户需提供）

按 `day-0-onboarding-checklist.md` Step 5，需用户提供的最小集合：

- [ ] **测试 appid + appsecret**（最小集合）
- [ ] （可选）IP 白名单
- [ ] （可选）测试 access_token 端点权限确认
- [ ] （可选）对接偏好（Maven/Gradle）

## 报告模板内容（凭证到位后逐项填数）

### 一、本期指标（Day-1 → Day-5）

| 指标 | Day-1 | Day-3 | Day-5 | 趋势 | 阈值 | 状态 |
|---|---|---|---|---|---|---|
| 请求成功率 | 100% (Mock 5/5) | 100% (Mock 5/5) | ___% (真实 ___/___) | ___ | ≥ 99.5% | |
| P99 延迟 | 3.3ms → 0.6ms | 0.7ms → 0.3ms | ___ms | ___ | < 基线+50% | |
| Token 刷新次数 | 1（Mock 首调用） | 0（复用） | ___次/h | ___ | < 6次/h | |
| Panic/Abort 数 | 0 | 0 | ___ | ___ | = 0 | |
| 熔断器状态 | N/A | N/A | ___ | ___ | Closed | |
| 内存 RSS | N/A | N/A | ___MB | ___ | 无持续增长 | |

### 二、真实流量测试用例（5 个真实发送）

| # | 用例 | 请求 | 响应 | 真实结果 |
|---|---|---|---|---|
| 1 | 订阅消息 | POST /cgi-bin/message/subscribe/send | errcode/errmsg | ___ |
| 2 | 客服消息 | POST /cgi-bin/message/custom/send | errcode/errmsg | ___ |
| 3 | 验签 | GET callback | 200/signature 校验 | ___ |
| 4 | 错误场景 | POST 缺 access_token | errcode=40014 | ___ |
| 5 | 重试 | token 过期 | 自动刷新 + 重试成功 | ___ |

### 三、真实环境观察（与 Mock 对比）

| 维度 | Mock 行为 | 真实环境行为 | 差异 |
|---|---|---|---|
| 签名 | 默认通过 | 需精确 SHA1 + 时间戳 + token | |
| IP 白名单 | 不检查 | 40165（不在白名单） | |
| access_token | 缓存返回 | 7200s 过期 + 单飞 | |
| 数据格式 | JSON 通用 | 严格 JSON 字段顺序/类型 | |

### 四、风险与发现

| # | 风险/发现 | 严重度 | 处置 |
|---|---|---|---|
| 1 | ___ | P0/P1/P2 | |
| 2 | ___ | | |

### 五、决策

- [ ] 扩量至 ___%（基于 Day-5 指标全 PASS）
- [ ] 维持当前比例
- [ ] 缩量至 ___%（修复 P1/P2）
- [ ] 回滚（任何 P0 立即切回 WxJava）

### 六、签署

| 角色 | 姓名 | 判定 | 日期 |
|---|---|---|---|
| 接入方 | | | |
| SRE | | | |

---

## 凭证到位后执行步骤

```bash
# 1. 用户填入真实 appid/appsecret 到 .env（或 export 到 shell）
export WX_MA_APPID=wx_test_xxxxx
export WX_MA_APPSECRET=xxxxxxxx

# 2. 启动真实 demo
cargo run --example send_real_message -- --target miniapp

# 3. 采集指标
bash scripts/alpha/collect-metrics.sh > metrics/metrics-day5.json

# 4. 跑 No-Go 闸门
bash scripts/alpha/check-no-go.sh

# 5. 填报告
# 编辑 docs/operations/alpha-2026-q3/day-5-observation-report.md
git add docs/operations/alpha-2026-q3/day-5-observation-report.md metrics/metrics-day5.json
git commit -m "docs: Day-5 真实流量观察报告"
```

---

## 七、本会话声明

本会话**主动放弃以下结论**（避免虚假完成）：
- ❌ 不声称"Day-5 已完成"
- ❌ 不伪造真实流量数字
- ❌ 不假设"如果凭证到位就能跑通"——Day-5 必须基于真实流量结果填数

诚实状态：
- ✅ Day-5 模板就位（凭证到位即用）
- ⏸ Day-5 真实数据 PENDING 用户凭证
- ⏸ Beta/Stable GA PENDING 真实灰度验证

**会话目标达成进度**：代码侧 100% 就绪 + 完整运营装备 + 真实 demo 接入 + Day-1/3 报告。  
**仍需用户介入**：test appid + appsecret 才能完成 Day-5/Step 5-6/Beta/Stable。
