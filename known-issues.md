# Known Issues / 已知问题

本文档记录 WxRust 项目当前已知的限制、风险与约束。每项附状态、影响范围与解除条件。

---

## 1. RSA 0.9.10 RUSTSEC-2023-0071（Marvin Attack）

| 字段 | 内容 |
|------|------|
| **漏洞** | [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071) |
| **严重度** | Medium (CVSS 5.9) |
| **影响版本** | `rsa 0.9.10`（当前锁定版本） |
| **影响面** | `wx-rust-common`、`wx-rust-pay`、`wx-rust-cp` 中的 RSA-OAEP 加密与 PKCS#1 v1.5 签名 |
| **攻击方式** | 计时侧信道（Marvin Attack），通过测量解密/验签耗时差异可能恢复 RSA 私钥 |
| **修复版本** | 无。rsa 0.10 尚为 RC（0.10.0-rc.18），非稳定版 |

### 当前 Mitigation 策略

1. **RSA-OAEP 盲化**：rsa 0.9 的 OAEP 实现内置随机填充，单次解密的时间差异被随机性掩盖（相比 PKCS#1 v1.5 解密，OAEP 受 Marvin Attack 影响更小）。
2. **固定消息加密**：微信支付 v3 敏感信息加密场景中，相同明文每次加密产生不同密文（OAEP 随机性），攻击者难以构造精确的计时 oracle。
3. **PKCS#1 v1.5 签名**：签名操作（非解密）使用 `sign_with_rng`，内部盲化步骤使计时差异不泄露私钥信息。
4. **验证测试**：`security_rsa_mitigation_test`（14 项）覆盖加解密往返、签名验签往返、错误输入行为，确认 RSA 基本原语在当前版本下正确工作。

### 何时可解除

- **条件**：`rsa` crate 发布 0.10.x 稳定版（非 RC），且与 `rsa::sha2`、`Oaep`、`Pkcs1v15Sign` API 兼容。
- **操作**：升级 workspace `rsa` 依赖，运行 `cargo test --workspace` + `cargo clippy`，移除 `deny.toml` 中的 `RUSTSEC-2023-0071` 例外。
- **追踪**：关注 [RustCrypto/rsa releases](https://github.com/RustCrypto/rsa/releases)。

---

## 2. 代码覆盖率边界

| 字段 | 内容 |
|------|------|
| **当前值** | 61.57% 行覆盖率（`cargo llvm-cov`） |
| **门禁** | 60% 行覆盖率（`--fail-under-lines 60`） |
| **余量** | 1.57 个百分点 |
| **函数覆盖率** | 41.80%（未设门禁） |
| **未覆盖行数** | 约 25,523 行 |

### 风险

余量极小，新增代码若覆盖率不足可能跌破门禁。

### 改进方向

- 优先覆盖 pay 子域固定域名路径分支、并发锁竞争分支。
- 函数覆盖率 41.80% 表明大量函数未被直接调用（通过 trait 默认方法/枚举派生间接覆盖），可逐步提升。

---

## 3. 发布约束（Workspace 内部依赖顺序）

| 字段 | 内容 |
|------|------|
| **约束** | workspace 内部 crate 依赖未上 crates.io，`cargo publish --dry-run` 仅对独立 crate 有效 |
| **受影响 crate** | Layer-1（aispeech/channel/cp/miniapp/mp/pay/qidian）和 Layer-2（open）依赖 `wx-rust-common` |

### 当前处理

- 独立 crate（`wx-rust-common`、`wx-rust`）：`cargo publish --dry-run` 完整验证。
- 依赖 crate：`cargo package --list --allow-dirty` 验证打包正确性（文件列表、manifest 有效）。
- 发布顺序：`wx-rust-common` -> Layer-1 -> Layer-2 -> `wx-rust`（见 `docs/verification/publish-pipeline-2026-08-25.md`）。

### 解除条件

首次发布 `wx-rust-common` 至 crates.io 后，后续 crate 的 `--dry-run` 可完整执行。

---

## 4. Ignored 测试

| 字段 | 内容 |
|------|------|
| **位置** | `crates/wx-rust-cp/src/bean/external/msg/attachment_builder.rs`（doctest） |
| **类型** | ````ignore` 代码块 |
| **原因** | 使用示例（`AttachmentBuilder::image_builder()...`）需要外部 import 上下文，在 doctest 环境中无法独立编译。此为文档示例，非功能测试。 |
| **影响** | 无功能影响。1991 项功能测试全部通过，0 failed。 |

### 是否需要修复

当前为文档级 ignore，不影响测试覆盖与 CI 门禁。如需消除 ignored 计数，可将示例改为 `no_run` 或补充完整 import。
