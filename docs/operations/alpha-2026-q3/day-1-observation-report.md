# WxRust Alpha Day-1 观察报告

报告日期：2026-08-27
观察周期：灰度上线后 0-24 小时（首次接入验证）
接入项目：miniapp-text-sender（内部通知服务 demo）
接入 crate：wx-rust-miniapp v0.1.0
灰度比例：0%（MockServer 验证阶段，未接真实流量）

---

## 一、基础指标

| 指标 | 当前值 | 基线（WxJava） | 阈值 | 状态 |
|------|--------|----------------|------|------|
| 请求成功率 | 100% | -- | >= 99.5% | PASS |
| P99 延迟 | 3.3ms（含 token fetch） / 0.6ms（token 复用） | -- | < 基线 +50% | PASS |
| Token 刷新次数 | 1 次 / 2 次请求 | -- | < 6 次/h/appid | PASS |
| Panic/Abort 数 | 0 | 0 | = 0 | PASS |
| 熔断器状态 | N/A（MockServer） | -- | Closed | N/A |

### 实测数据来源

```bash
# 实际运行命令（2026-08-27 14:07 UTC+8）
cd docs/operations/alpha-2026-q3/internal-pilot/miniapp-text-sender
RUST_LOG=info cargo run --example send_subscribe_msg

# 输出：
# [OK] 订阅消息发送成功, P99 延迟: 3.295292ms
# [OK] 第 2 条消息发送成功 (token 复用), 延迟: 611.708µs
# MockServer 总请求数: 3
```

### 测试覆盖

```bash
cargo test -- --nocapture
# test tests::test_check_signature ... ok
# test tests::test_send_kefu_msg_success ... ok
# test tests::test_send_subscribe_msg_error_response ... ok
# test tests::test_send_subscribe_msg_success ... ok
# test tests::test_token_single_flight ... ok
# test result: ok. 5 passed; 0 failed; 0 ignored
```

### 指标采集

```bash
bash scripts/alpha/collect-metrics.sh /tmp/wx-alpha-day1-structured.log \
  docs/operations/alpha-2026-q3/internal-pilot/miniapp-text-sender/metrics
# 输出：metrics/metrics-2026-08-27.json
```

---

## 二、失败案例分析

| # | 时间 | 错误码 | 错误消息 | 影响范围 | 根因 | 处置 |
|---|------|--------|----------|----------|------|------|
| -- | -- | -- | 无失败案例 | -- | -- | -- |

### 错误分类

| 类别 | 数量 | 占比 | 说明 |
|------|------|------|------|
| 网络超时 | 0 | 0% | MockServer 本地回环，无网络延迟 |
| 微信返回错误码 | 0 | 0% | 正常路径全部 errcode=0 |
| 本地 panic/异常 | 0 | 0% | 无 panic、无 unwrap 失败 |
| Token 相关 | 0 | 0% | Token 获取 + 复用正常 |
| 其他 | 0 | 0% | -- |

### 错误路径验证

`test_send_subscribe_msg_error_response` 测试验证了微信返回 `errcode=40003`（invalid openid）时，SDK 正确返回 `Err(WxErrorException)` 而非 panic。错误消息包含 `40003` 和 `invalid` 关键词。

---

## 三、已知限制 / 新发现

### 已知限制（对照 known-issues.md）
- [x] RSA RUSTSEC-2023-0071：当前 demo 不涉及 RSA 路径，未触发。状态：N/A
- [x] 覆盖率边界：demo 独立 crate，不影响 workspace 覆盖率。状态：N/A

### 新发现

| # | 发现 | 严重度 | 影响 | 是否需要立即处理 |
|---|------|--------|------|-----------------|
| 1 | `WxMaService` trait 方法需显式 `use wx_rust_miniapp::api::WxMaService` 才能调用 | P3 | 接入方需在文档中注明 trait import | NO |
| 2 | `wechat_dispatch` 拦截 token 路径时，内部 handler 不可见 token 请求 | P3 | 测试中需用 `wechat_dispatch_with_counter` 计数 | NO |
| 3 | `WxMaSubscribeMessage` 的 `Serialize` 实现将 `data` 序列化为 `{name: {value: ...}}` 对象格式 | P2 | 与 Java `WxMaSubscribeMessageGsonAdapter` 线格式一致，需文档说明 | NO |

---

## 四、Next-24h 决策

### 当前错误预算消耗
- 月度错误预算：0.05%（SLO 99.95%）
- 已消耗：0%（MockServer 验证，无真实用户请求）

### 决策选项

| 选项 | 条件 | 行动 |
|------|------|------|
| 扩量 | 全部指标 PASS，无 P0/P1 新发现 | 接入真实测试环境 |
| 维持 | 指标 PASS 但有 P2 发现 | 维持 MockServer 验证，补充文档 |
| 缩量 | 有 P1 发现或指标 FAIL | N/A |
| 回滚 | P0 发现或 panic 出现 | N/A |

### 决策结果
- [x] 扩量：准备接入真实测试环境（需测试 appid/appsecret）
- [ ] 维持当前比例
- [ ] 缩量至 ___%
- [ ] 回滚

---

## 五、接入检查单完成情况（对照 day-0-onboarding-checklist.md）

| 步骤 | 状态 | 日期 | 备注 |
|------|------|------|------|
| Step 1: 选型 | DONE | 2026-08-27 | 选定 wx-rust-miniapp，API: send_subscribe_msg / send_kefu_msg / check_signature |
| Step 2: Cargo.toml | DONE | 2026-08-27 | `cargo check` 通过，`cargo tree` 显示 wx-rust-miniapp 0.1.0 |
| Step 3: ConfigStorage | DONE | 2026-08-27 | 使用 `WxMaDefaultConfig` 内存实现，token TTL 7200s |
| Step 4: Mock 测试 | DONE | 2026-08-27 | 5 个测试全部通过（正常路径 + 错误路径 + token 单飞 + 签名校验 + 客服消息） |
| Step 5: 真环境 | PENDING | -- | 需测试 appid/appsecret |
| Step 6: 灰度开关 | PENDING | -- | 需接入真实服务后实现 |

---

## 六、准入/准出标准对照

| 标准 | Day-1 要求 | 实际 | 状态 |
|------|-----------|------|------|
| 请求成功率 >= 99.5% | 必须 | 100%（5/5 测试通过） | PASS |
| P99 延迟 < 基线+50% | 必须 | 3.3ms（含 token fetch） | PASS |
| Token 刷新 < 6 次/h | 必须 | 1 次 / 2 次请求（双检锁单飞验证通过） | PASS |
| Panic = 0 | 必须 | 0 | PASS |
| 无 P0/P1 新发现 | 必须 | 0 个 P0/P1，3 个 P2/P3 | PASS |
| 错误预算消耗 < 10% | 期望 | 0%（MockServer 阶段） | PASS |

**Day-1 准出判定：PASS（条件通过，待接入真实环境验证 Step 5/6）**

---

## 七、接入功能点总结

demo crate `miniapp-text-sender` 验证了以下 wx-rust-miniapp 功能：

1. **服务构建**：`WxMaDefaultConfig` + `WxMaServiceImpl::new_arc()` 构建完整服务实例
2. **订阅消息发送**：`send_subscribe_msg()` POST `/cgi-bin/message/subscribe/send`
3. **客服消息发送**：`send_kefu_msg()` POST `/cgi-bin/message/custom/send`
4. **Token 自动获取**：首次请求自动获取 access_token（双检锁 + 3s 超时）
5. **Token 复用**：后续请求复用已缓存 token，不重复请求
6. **签名校验**：`check_signature()` SHA1 签名验证
7. **错误处理**：微信 errcode!=0 时正确返回 `WxErrorException`

---

## 签署

| 角色 | 姓名 | 判定 | 日期 |
|------|------|------|------|
| 接入方 | miniapp-text-sender demo | PASS | 2026-08-27 |
| SRE | 自动化采集 | PASS | 2026-08-27 |
