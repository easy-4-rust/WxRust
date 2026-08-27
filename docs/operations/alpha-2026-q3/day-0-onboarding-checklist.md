# WxRust Alpha Day-0 接入清单

日期：2026-08-27
适用版本：`wx-rust-*` v0.1.0
适用对象：内部项目接入方（首个真实项目）

---

## 总览

6 步骤完成从零到灰度上线。每步含：具体命令、参考文档、完成判定（Done = 可勾选）。

---

## Step 1: 选型与范围确认

### 目标
确定接入的 crate、API 范围、灰度策略。

### 操作

```bash
# 查看可用 crate
cargo search wx-rust --limit 10

# 确认目标 crate（按业务域选一个）
# wx-rust-mp       公众号
# wx-rust-miniapp  小程序
# wx-rust-pay      微信支付（Alpha 避开资金主链路）
# wx-rust-cp       企业微信
# wx-rust-open     第三方平台
# wx-rust-channel  视频号小店
```

### 决策项
- [ ] 选定接入 crate：`wx-rust-________`
- [ ] 圈定 API 清单（<= 10 个方法）：列出方法名
  - 示例：`get_user_info`, `send_template_message`, `js_code_to_session`
- [ ] 确认非资金主链路（Alpha 首批避开 `wx-rust-pay` 大额场景）
- [ ] 配置 fallback：保留 WxJava 旧路径，feature flag 一键回切

### 完成判定
- [ ] 已选定 crate 和 API 范围
- [ ] 已确认 fallback 方案

### 参考
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第二节

---

## Step 2: Cargo.toml 引入

### 目标
在接入项目中添加 WxRust 依赖并确认编译通过。

### 操作

```toml
# 接入项目 Cargo.toml
[dependencies]
# 方式 A：按需引入单个 crate
wx-rust-miniapp = "0.1.0"

# 方式 B：伞形引入（含全部子 crate）
# wx-rust = "0.1.0"

# 可选：启用同步特性（仅门面场景）
# wx-rust-miniapp = { version = "0.1.0", features = ["sync"] }
```

```bash
# 解析依赖
cargo update

# 编译检查（不含测试）
cargo check

# 确认版本
cargo tree | grep wx-rust
```

### 完成判定
- [ ] `cargo check` 通过，无编译错误
- [ ] `cargo tree` 显示 wx-rust-* 0.1.0 已解析

### 参考
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第二节

---

## Step 3: WxMaConfigStorage / WxMpConfigStorage 实现

### 目标
实现配置存储 trait，将 token 持久化到接入方现有存储（Redis/DB/文件）。

### 操作

```rust
use wx_rust_miniapp::config::WxMaConfigStorage;
use wx_rust_common::error::WxError;
use async_trait::async_trait;

struct MyConfigStorage {
    // 接入方的存储客户端
    redis: redis::Client,
}

#[async_trait]
impl WxMaConfigStorage for MyConfigStorage {
    async fn get_access_token(&self) -> Result<Option<String>, WxError> {
        // 从 Redis 读取 token
        // key 格式建议: wx:ma:{appid}:access_token
        todo!()
    }

    async fn set_access_token(&self, token: &str, expires_in: u64) -> Result<(), WxError> {
        // 写入 Redis，TTL = expires_in - 300s（提前 5 分钟过期）
        todo!()
    }

    async fn get_jsapi_ticket(&self) -> Result<Option<String>, WxError> {
        todo!()
    }

    async fn set_jsapi_ticket(&self, ticket: &str, expires_in: u64) -> Result<(), WxError> {
        todo!()
    }
}
```

### 关键注意事项
- Token TTL 设为 `expires_in - 300`（提前 5 分钟过期），避免用到临界 token
- 存储 key 加 appid 前缀，支持多 appid 隔离
- 读写操作加超时（建议 500ms），避免存储故障拖垮业务

### 完成判定
- [ ] ConfigStorage trait 实现编译通过
- [ ] 单元测试覆盖 get/set 基本路径
- [ ] Token TTL 逻辑正确（提前过期）

### 参考
- docs.rs 上 `wx-rust-miniapp` 的 `WxMaConfigStorage` trait 文档
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第三节

---

## Step 4: Mock 测试

### 目标
用 mock HTTP server 验证核心 API 调用路径，不依赖真实微信环境。

### 操作

```toml
# 接入项目 Cargo.toml
[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
# 可选：wiremock 或 mockito 用于 HTTP mock
```

```rust
#[tokio::test]
async fn test_get_user_info_with_mock() {
    // 1. 启动 mock server，返回预设的用户信息 JSON
    // 2. 构造 WxMaServiceImpl，注入 mock base_url
    // 3. 调用 get_user_info
    // 4. 断言返回字段正确
}

#[tokio::test]
async fn test_token_refresh_flow() {
    // 1. Mock token 接口返回新 token
    // 2. 验证 ConfigStorage.set_access_token 被调用
    // 3. 验证后续请求使用新 token
}

#[tokio::test]
async fn test_error_response_handling() {
    // 1. Mock 返回 errcode != 0 的响应
    // 2. 验证 WxErrorException 正确抛出
    // 3. 验证错误码和错误消息正确传递
}
```

### 完成判定
- [ ] 至少 3 个 mock 测试通过（正常路径 + token 刷新 + 错误路径）
- [ ] 错误处理走 `WxErrorException`，不 panic
- [ ] `cargo test` 输出 0 failed

### 参考
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第三节 3.2

---

## Step 5: 真环境接入

### 目标
接入真实微信环境，验证端到端调用。

### 操作

```bash
# 1. 准备测试用 appid/appsecret（非生产环境）
# 2. 设置环境变量（不要硬编码）
export WX_APPID="wx_test_xxxx"
export WX_SECRET="test_secret_xxxx"

# 3. 运行集成测试
cargo test --features integration -- --nocapture

# 4. 观察 tracing 输出
RUST_LOG=debug cargo test --features integration -- --nocapture 2>&1 | tee /tmp/wx-integration.log
```

### 观察点
- [ ] 首次 token 获取成功（日志中出现 `access_token` 获取记录）
- [ ] Token 自动续期（等待 2 小时后观察）
- [ ] 错误响应正确处理（用错误 appid 测试）
- [ ] 无 panic / unwrap 失败
- [ ] tracing span 结构正确（每个请求有独立 span）

### 完成判定
- [ ] 至少 1 个真实 API 调用成功
- [ ] Token 自动续期验证通过
- [ ] 错误路径验证通过
- [ ] 日志中无 panic / unwrap

### 参考
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第三节 3.3

---

## Step 6: 灰度开关上线

### 目标
通过 feature flag 控制流量比例，实现可控灰度。

### 操作

```rust
// 方式 A：编译时 feature flag
// Cargo.toml
[features]
wx-rust-alpha = []  // 启用时走 WxRust 路径

// 业务代码
#[cfg(feature = "wx-rust-alpha")]
let wx_service = WxMaServiceImpl::new_arc(config);

#[cfg(not(feature = "wx-rust-alpha"))]
let wx_service = WxJavaLegacyService::new(config);
```

```rust
// 方式 B：运行时配置开关（推荐）
let use_wxrust = std::env::var("WXRUST_ENABLED")
    .map(|v| v == "true")
    .unwrap_or(false);

if use_wxrust {
    // 走 WxRust 路径
} else {
    // 走 WxJava 旧路径
}
```

### 灰度策略
- Day 0-1: 5% 流量（或 1 个非核心业务模块）
- Day 2-3: 20% 流量
- Day 4-7: 50% 流量
- Day 7+: 100% 或进入 Beta

### 完成判定
- [ ] 灰度开关实现并测试通过
- [ ] 回切到 WxJava 路径验证通过（断开 WxRust 依赖仍可运行）
- [ ] 已在测试环境完成至少 1 次灰度开关切换

### 参考
- `docs/verification/alpha-onboarding-guide-2026-08-27.md` 第四节

---

## 接入完成确认

| 步骤 | 状态 | 日期 | 备注 |
|------|------|------|------|
| Step 1: 选型 | [x] | 2026-08-27 | wx-rust-miniapp, API: send_subscribe_msg/send_kefu_msg/check_signature |
| Step 2: Cargo.toml | [x] | 2026-08-27 | cargo check 通过, cargo tree 显示 0.1.0 |
| Step 3: ConfigStorage | [x] | 2026-08-27 | WxMaDefaultConfig 内存实现, token TTL 7200s |
| Step 4: Mock 测试 | [x] | 2026-08-27 | 5/5 测试通过: 正常+错误+token单飞+签名+客服 |
| Step 5: 真环境 | [ ] | | 待测试 appid/appsecret |
| Step 6: 灰度开关 | [ ] | | 待接入真实服务后实现 |

**全部勾选后，进入 7 日观察期。**

---

## 附录：快速故障排除

| 症状 | 可能原因 | 处理 |
|------|---------|------|
| `cargo check` 报 wx-rust 版本冲突 | workspace 依赖版本不匹配 | `cargo update -p wx-rust-*` |
| Token 获取失败 | appid/secret 错误或 IP 白名单 | 检查微信公众平台配置 |
| `async_trait` 编译错误 | 缺少 `async-trait` 依赖 | 添加 `async-trait = "0.1"` |
| tracing 无输出 | 未初始化 subscriber | 添加 `tracing_subscriber::init()` |
