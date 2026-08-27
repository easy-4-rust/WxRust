# P3 impl 层缺口补齐报告

日期：2026-08-27
范围：cp / miniapp / open 三个 crate（不碰 channel/pay）

## 门禁结果

- `cargo test -p wx-rust-cp`: 521 passed (+3 新增)
- `cargo test -p wx-rust-miniapp`: 329 passed (+2 测试修正)
- `cargo test -p wx-rust-open`: 190 passed (无变化)
- `cargo clippy -D warnings`: clean
- `cargo fmt --check`: clean

## 逐项处置表

| # | 任务 | 状态 | 说明 |
|---|------|------|------|
| 1 | cp 智能机器人 API 模式（#4100） | **修复补齐** | Java 有 10 个方法，Rust 原有 8 个。补齐 `parseEncryptedCallbackMessage` + `replyMessage` 两个 default 方法。同时在 `WxCpIntelligentRobotCryptUtil` 中新增 `from_params` 构造器、`decrypt`（验签+解密）、`encrypt_json`（JSON 格式加密，含 msg_signature/timestamp/nonce）。`WxCryptUtil::gen_random_str` 改为 pub。新增 3 个测试（encrypt_json_roundtrip、parse_encrypted_callback_message、reply_message）。 |
| 2 | cp 待办 API（#4092） | **已对齐** | Java 2 个方法（`get`/`update`），Rust 已有 2 个方法 + 完整 impl + 3 个测试。无需改动。 |
| 3 | cp 长整型应用 ID（#4079） | **修复补齐** | Java `WxCpAgent.agentId` 从 `int` 改 `Long`。Rust `WxCpAgent.agent_id` 已从 `i32` 改为 `i64`。编译通过，无级联影响（serde 自动处理）。 |
| 4 | miniapp 手机号 openid 校验（#4078） | **修复补齐** | Java 有 `default getPhoneNumber(String code, String openid)` 方法。Rust 缺失。已在 `WxMaUserService` trait 中补齐 `get_phone_number_with_openid` default 方法，委托 `get_phone_number(code)`。 |
| 5 | miniapp NFC scheme（4.8.5） | **已对齐** | `WxMaSchemeService` 已有 `generate` + `generate_nfc` 两个方法 + 完整 impl。无需改动。 |
| 6 | miniapp 消息推送安全模式 JSON 验签（#4069） | **已对齐** | `WxMaMessage::from_encrypted_json` 已实现：解析加密 JSON → 取 Encrypt 字段 → AES 解析加密 JSON → 取 Encrypt 字段 → AES 解密 → 重解析。无需改动。 |
| 7 | open 商家客服（#4105） | **无需改** | KfService 在 channel 模块（由另一智能体负责），open 侧 `WxOpenComponentService` 无相关桥接方法。无需改动。 |
| 8 | miniapp getUserEncryptKey 签名修复（4.8.5 #3be78af） | **修复补齐** | Java 修复：`sessionKey.getBytes(UTF-8)` 替代 `Base64.decodeBase64(sessionKey)`。Rust 已同步修复：`wx_ma_service.rs` 中 `get_user_encrypt_key` 改用 `session_key.as_bytes()`，移除 base64 解码。同步更新 trait 文档注释。修正 2 个集成测试（`coverage_boost_ma_service_trait.rs` 和 `sub_domain_g2_content.rs`）以匹配新签名算法。移除未使用的 `base64::Engine` import。 |

## 新增/修改文件清单

### cp crate
- `crates/wx-rust-cp/src/bean/wx_cp_agent.rs`: `agent_id: i32` → `i64`
- `crates/wx-rust-cp/src/api/wx_cp_intelligent_robot_service.rs`: 补齐 2 个 trait 方法
- `crates/wx-rust-cp/src/api/impl/wx_cp_intelligent_robot_service_impl.rs`: 补齐 2 个 impl + 3 个测试
- `crates/wx-rust-cp/src/util/crypto/wx_cp_intelligent_robot_crypt_util.rs`: 新增 `from_params`/`decrypt`/`encrypt_json` + 1 个测试

### miniapp crate
- `crates/wx-rust-miniapp/src/api/wx_ma_user_service.rs`: 新增 `get_phone_number_with_openid` default 方法
- `crates/wx-rust-miniapp/src/api/wx_ma_service.rs`: 修复 `get_user_encrypt_key` 签名算法
- `crates/wx-rust-miniapp/src/api/wx_ma_internet_service.rs`: 更新 trait 文档
- `crates/wx-rust-miniapp/tests/coverage_boost_ma_service_trait.rs`: 修正 encrypt key 测试
- `crates/wx-rust-miniapp/tests/sub_domain_g2_content.rs`: 修正签名黄金值测试

### 公共 crate
- `crates/wx-rust-common/src/util/crypto/wx_crypt_util.rs`: `gen_random_str` 改为 pub

### open crate
- 无改动

## 统计

- 8 项处置：3 已对齐 / 4 修复补齐 / 1 无需改
- 新增测试：3 个（cp intelligent robot）
- 修正测试：2 个（miniapp encrypt key 签名）
