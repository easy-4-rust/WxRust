# Changelog

本文件记录 WxRust 项目的关键里程碑。当前版本为 `0.1.0`（workspace 统一版本），尚未正式发布至 crates.io。

## [Unreleased]

### 安全

- **RSA mitigation 验证测试**：在 `wx-rust-common` 新增 `security_rsa_mitigation_test`（14 项），覆盖 RSA-OAEP 加解密往返、PKCS#1 v1.5 签名/验签往返、错误输入行为（不 panic），作为 RUSTSEC-2023-0071 mitigation 证据链的一环。（2026-08-25）

## [0.1.0] - 工程基线（未发布）

### 架构与工程基线

- **初始迁移**：从 WxJava（Java）迁移 9 个模块、3287 个对象、3406 个 `.rs` 文件至 Rust。（commit `11c5c61`）
- **测试基线**：workspace 全量 1977 项测试通过（0 failed, 1 ignored），覆盖 121 个测试目标。
- **Clippy/Fmt 门禁**：`cargo clippy --workspace --all-targets -- -D warnings` 0 warnings；`cargo fmt --all -- --check` exit 0。
- **覆盖率门禁**：`cargo llvm-cov --workspace --fail-under-lines 60` 达 61.57%（行覆盖率），通过 60% 门禁。
- **供应链治理**：`cargo deny check` 全通过（license/ban/advisory/source）；`cargo audit` 1 medium 已登记为已知风险例外（rsa 0.9.10 RUSTSEC-2023-0071）。
- **MSRV 声明**：`rust-version = "1.85"`，`edition = "2024"`，`resolver = "3"`。
- **CI 门禁**：fmt + clippy + test + deny + audit + redis + criterion 基准，全部纳入 GitHub Actions。

### 并发原生架构改造

- **HttpTransport trait**：抽象 HTTP 传输层，提供 `ReqwestTransport`（生产）与 `MockTransport`（测试）双实现。（commit `4051076`）
- **execute_pipeline 统一执行管线**：token 失效单次重放，miniapp/mp/cp/qidian/channel 五个子域逐一接入，等价性验证全通过（测试通过数不变）。（commits `c744509`..`108cce8`）
- **CircuitBreaker 熔断器**：管线可选接入，闭合/半开/开三态，1000 并发基准验证。（commit `cf287d2`）
- **execute_stream 流式执行**：pay 子域 `download_bill_stream` 流式下载 + common `execute_stream` 支持。（commit `f38950f`）
- **WxClock 时钟注入**：token 过期测试去除 `sleep` 依赖，确定性测试。（commit `e71efd0`）
- **feature=sync 同步门面**：miniapp 提供 `block_on` 同步调用，CI 门禁确保编译通过。（commit `a5f24ea`）
- **并发基准**：1000 并发单飞/共享 token/熔断基准 + CI 接入（Criterion）。（commit `6d9c395`）

### 覆盖率提升

- **Phase 2 测试补齐**：121 新测试（pay/channel 子域）。（commit `9aa94c5`）
- **Phase 3 扩展测试**：210 新测试（6 文件）。（commit `8f040ee`）
- **HTTP mock 测试**：pay 92 tests（30.54%->40.63%）、channel 35+ tests、open 58 tests（44.13%->56.96%）、mp/cp/miniapp 批量 mock。
- **错误码枚举全覆盖**：common `find_msg_by_code` 1832 分支 100%。
- **V0 缺口补齐**：v3 认证/加密模块（Signer/Verifier + Credentials/Validator + AesUtils/PemUtils/RsaCryptoUtil）、config/exception/zip 工具，72 项逐项处置，MISSING=0。（commits `47466f9`, `1827aa9`, `f9d57bb`）

### 发布链路

- **内部 crate 依赖改用 workspace 声明**：`version + path`，publish 时携带 crates.io 版本号。（commit `6728cc7`）
- **Publish pipeline 验证**：10 crate 依赖排序，dry-run / package 验证全通过。Layer-1/2 crate 因 workspace 内部依赖未上 crates.io 而使用 `cargo package --list` 验证。（2026-08-25）

### 已知风险

- **rsa 0.9.10 RUSTSEC-2023-0071**：Marvin Attack（medium），无修复版本可用（0.10 尚为 RC）。当前 mitigation：RSA-OAEP 盲化 + 固定消息加密。已配置 `deny.toml` 例外。待 rsa 0.10 稳定后升级并移除例外。
