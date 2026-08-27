# WxRust Alpha Day-5 观察报告（真实流量）

日期：2026-08-27
报告路径：`docs/operations/alpha-2026-q3/day-5-observation-report.md`
执行：真实凭证 + 真实微信服务器（api.weixin.qq.com），3 轮共 15 次真实 API 调用

> **状态变更**：Day-5 已由模板升级为真实流量报告。凭证由用户提供（PartMe.AI 小程序）。
> 凭证仅经环境变量传入，未写入任何仓库文件。

---

## 准备清单（已满足）

- [x] **测试 appid + appsecret**（用户提供，PartMe.AI 小程序）
- [x] **IP 白名单**（实测生效——access_token 真实获取成功，未遇 40164）
- [x] **订阅消息模板 ID**（用户提供）

## 报告内容

### 一、本期指标（Day-1 → Day-5）

| 指标 | Day-1 | Day-3 | Day-5 | 趋势 | 阈值 | 状态 |
|---|---|---|---|---|---|---|
| 请求成功率 | 100% (Mock 5/5) | 100% (Mock 5/5) | 链路 100% (真实 5/5 到达微信) | 真实首测 | ≥ 99.5% | ✅ 传输层 |
| P99 延迟 | 3.3ms → 0.6ms | 0.7ms → 0.3ms | 359ms（真实，含 TLS/公网） | Mock→真实 预期抬升 | < 基线+50% | ✅ 合理 |
| Token 刷新次数 | 1（Mock 首调用） | 0（复用） | 2 次/会话（首取 + 注入过期重试） | 单飞保持 | < 6次/h | ✅ |
| Panic/Abort 数 | 0 | 0 | 0 | — | = 0 | ✅ |
| 熔断器状态 | N/A | N/A | Closed（未触发开断） | — | Closed | ✅ |
| 内存 RSS | N/A | N/A | 未采集（demo 未上报） | — | 无持续增长 | ⏸ 下轮补 |

> 成功率口径说明：脚本按 `errcode != 0` 计错，Day-5 的 5 次调用中有 4 次返回业务错误码（3×40003 占位 openid + 1×41001 故意错误探针），按脚本口径为 20%。**传输链路实际 5/5 全部到达微信并返回正确响应**；4 个业务错误均为预期（占位 openid / 错误注入用例），非 SDK 缺陷。送达成功率需真实 openid 后重测。

### 二、真实流量测试用例（5+1 个真实调用）

| # | 用例 | 请求 | 响应 | 真实结果 |
|---|---|---|---|---|
| 1 | access_token 获取 | GET /cgi-bin/token | access_token (len 137) | ✅ OK，366-427ms，IP 白名单生效 |
| 2 | 验签 | 本地 SHA1 校验 | valid=true / invalid=false | ✅ 正确签名通过、错误签名拒绝 |
| 3 | 订阅消息 | POST /cgi-bin/message/subscribe/send | errcode=40003 | ⏸ 链路通（208ms）；占位 openid 预期 40003，需真实 openid 送达 |
| 4 | 客服消息 | POST /cgi-bin/message/custom/send | errcode=40003 | ⏸ 链路通（181ms）；占位 openid 预期 40003 |
| 5 | 错误场景 | POST 缺 access_token | errcode=41001 access_token missing | ✅ 预期错误，错误码映射正确（100ms） |
| 6 | 重试 | token 过期注入 → 自动刷新 | 刷新成功；发送 40003 | ✅ 过期检测 + 自动刷新路径真实验证（370ms） |

### 三、真实环境观察（与 Mock 对比）

| 维度 | Mock 行为 | 真实环境行为 | 差异 |
|---|---|---|---|
| 签名 | 默认通过 | SHA1 精确校验通过 | 一致 |
| IP 白名单 | 不检查 | 已生效（token 获取成功） | ✅ 用户已配置 |
| access_token | 缓存返回 | 7200s 真实过期 + 单飞（首取后复用） | ✅ 一致 |
| 错误码 | JSON 通用 | 真实错误码（40003/41001）+ rid 追踪 | ✅ 一致 |
| 延迟 | <5ms | 100-430ms（含 TLS + 公网 + 微信侧） | Mock 不可比 |

### 四、风险与发现

| # | 风险/发现 | 严重度 | 处置 |
|---|---|---|---|
| 1 | 订阅/客服送达依赖真实 openid（占位 openid 只能验证链路） | P2 | 需用户提供测试用户 openid（真机授权订阅 + 48h 内交互） |
| 2 | 准出脚本 `alpha-exit-gate.sh` 检查 5 正则把 "0 failed" 通过行误判为失败 | P3 | 已修复（`test result:.*[1-9][0-9]* failed`），重跑通过 |
| 3 | 观察期仅 1/7 天（真实流量 2026-08-27 起） | 结构性 | 需持续观察至 Day-7 方可准出 |

### 五、决策

- [ ] 扩量至 ___%（基于 Day-5 指标全 PASS）
- [x] **维持当前比例**（观察期 1/7 未满，继续观察至 Day-7）
- [ ] 缩量至 ___%（修复 P1/P2）
- [ ] 回滚（任何 P0 立即切回 WxJava）

**Alpha 准出判定：DELAY**（脚本 8 项检查：4 PASS / 2 FAIL / 2 WARN）
- FAIL-1 观察期 1/7 天——结构性，真实流量首日不可准出
- FAIL-2 成功率脚本口径 20%——4 个业务错误均为预期（占位 openid + 错误探针），非缺陷
- WARN-1 覆盖率需人工确认（实测 70.13% ≥ 60% ✅）
- WARN-2 Alpha 期间 crates/ 14 文件变更——均为 Phase A 测试文件（cov_*.rs），非 src 行为变更

### 六、签署

| 角色 | 姓名 | 判定 | 日期 |
|---|---|---|---|
| 接入方 | wandl（PartMe.AI） | 凭证提供 + 观察中 | 2026-08-27 |
| SRE | WxRust 自动化 | DELAY（观察期不足） | 2026-08-27 |

---

## 执行记录

```bash
# 1. 凭证（用户提供，仅环境变量）
export WX_MA_APPID / WX_MA_APPSECRET / WX_MA_TEMPLATE_ID

# 2. 真实流量（3 轮 × 6 场景）
cargo run --example send_real_message   # examples/send_real_message.rs（新增，真实 host）

# 3. 采集指标 → metrics/metrics-2026-08-27.json（工作副本）+ internal-pilot/.../metrics/metrics-2026-08-27-real.json（入库证据）
bash scripts/alpha/collect-metrics.sh /tmp/app-tracing.log ./metrics

# 4. No-Go 闸门 → GO（0 NO-GO / 0 WARN）
bash scripts/alpha/check-no-go.sh /tmp/wxrust-cargo-test.log /tmp/app-tracing.log /tmp/gates.json

# 5. Alpha 准出评估 → DELAY（观察期 1/7 + 成功率口径）
bash scripts/alpha/alpha-exit-gate.sh ./metrics /tmp/wxrust-cargo-test.log .
```

## 本会话声明

- ✅ **Day-5 真实流量已执行**：access_token / 验签 / 错误场景 / token 过期重试 4 项真实 PASS；订阅/客服链路通但送达被占位 openid 拦截（预期）
- ✅ **No-Go 闸门 GO**：0 阻塞项；3588 workspace tests 全绿；覆盖率 70.13% ≥ 60%
- ⏸ **Beta/Stable 仍 PENDING**：7 日观察期 + 真实 openid 送达验证
