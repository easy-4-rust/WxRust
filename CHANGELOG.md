# Changelog

本文件记录 WxRust 项目的关键里程碑。当前版本为 `0.1.3`（workspace 统一版本）。

## [0.1.3] - 2026-08-28

### 依赖升级（issue #15，本轮实际升级 5/6）

- **MSRV：`rust-version` 1.85 → 1.89**——解锁此前被传递依赖 MSRV 卡住的升级路径
- **quick-xml 0.41 → 0.42**：API 适配（`QName::as_ref()` 返回 `&str`、`BytesText::decode()` → `xml_content(XmlVersion::Implicit1_0)`），波及 common/mp/cp/miniapp/pay 5 个 XML 解析文件
- **x509-cert 0.2 → 0.3**：`tbs_certificate`/`serial_number`/`validity` 字段转 getter，`wx_pay_cert_utils` 适配
- **md-5 0.10 → 0.11**（pay 声明放宽）
- **redis 0.29 → 1.6**（common，源码编译兼容，零改动）
- **criterion 0.5 → 0.8**：`criterion::black_box` → `std::hint::black_box`
- **chacha20 0.10.1 → 0.10.2**（修复 yanked 传递依赖）
- clippy 1.89 新 lint 适配：`collapsible_if`/`manual_is_multiple_of` 加入 workspace allow（保持 Java 镜像代码风格）

### 关键修复

- **pipeline/mod.rs**：折叠嵌套 if 时误把 `break Err` 移入 `on_token_invalid` 块，导致 errcode≠0 且无 token 失效回调时不再上抛（channel post 路径回归）——已还原正确结构

### 验证

- workspace tests：**3589 passed / 0 failed**
- clippy 0 error、fmt clean、audit/deny 干净
- CI validation full profile：除 msrv job（1.85→1.89 后同步更新）外全绿

## [0.1.2] - 2026-08-28

### CI/CD（仓库元数据对齐 easyexcel-rust）

- **toolchain 固定为 1.97.1**（5 个 workflows `RUST_TOOLCHAIN` env + `dtolnay/rust-toolchain@stable` + `toolchain` 参数）：消除 latest stable 漂移——根因：8/26 新版 clippy 1.97 `io_other_error` lint 命中既有测试导致 CI 红，本地固定 1.97.1
- **concurrency group**（5 个 workflows）：ci/coverage/security/sync-feature=`cancel-in-progress: true`（PR 重复触发自动取消）；release=`false`（发布期不能取消）
- **ci.yml 新增 msrv job**：固定 Rust 1.85，对应 Cargo.toml `rust-version = "1.85"`
- **CODEOWNERS**：默认 `@wandl`；pay/deny/Cargo.lock/workflows 关键路径单独标注
- **PULL_REQUEST_TEMPLATE.md**：关联 issue / 改动类型 / 测试清单 / 发布影响 / 风险评估 5 段
- **dependabot 限流**：cargo PR `10 → 3` + cooldown（major 30天/其他 7天）+ `commit prefix: "deps"`；actions PR `10 → 2`

### 依赖升级（issue #15 决策）

- **本次 0/6 实际升级**：在 workspace `rust-version = "1.85"` 与 rsa 0.9.x 锁链双重约束下，`cargo update` 拒绝 6 个依赖（md-5 0.11、quick-xml 0.42、rand_core 0.10、x509-cert 0.3、redis 1.6、criterion 0.8）——原因是它们的传递依赖 MSRV > 1.85，或会与 rsa 0.9 锁链产生 workspace 双版本（rand_core 0.6+0.10）
- **v0.1.2 不强行突破**：详见 issue #15 与已知问题（cargo.toml 注释）
- **下一周期处理方向**（待用户决策）：
  1. 上调 workspace MSRV（如 1.85 → 1.89）解锁 md-5/quick-xml/x509-cert 等传递依赖
  2. rand_core 与 rsa 0.10 stable 合并升级（已跟踪 Phase D）
  3. redis 1.6、criterion 0.8 单独特性分支验证

## [0.1.1] - 2026-08-28

### 功能增强（v0.1.0 以来 54 个 src 文件）

- **channel**：售后保障单（guarantee 全流程 bean + impl）、订单（补偿发货 / 预售改 SKU / 赠品 / 隐私号收发货）、商品（第三方货源 / 审核策略 / 类目分类 / 方案 scheme）、店铺（H5 链接 / 二维码 / 标签链接）+ URL 枚举扩展
- **miniapp**：g3 小店 / g4 能力 URL 枚举补全、OCR 服务 impl 增强
- **open**：component service impl 增强
- **common**：OcrService 增强
- **aispeech**：dialog `async_task_result` bean

### 测试体系

- 镜像率提升至 **100.8%**（Batch-C/D/E，383/380 unique，V2 复测 #7）
- Phase A 深度覆盖：pay 114 + cp 70 + common/mp 103 = **287 个新测试**
- **RSA-OAEP Marvin 缓解证据测试**（`wx_pay_v3_crypto_test.rs::rsa_oaep_roundtrip_marvin_mitigation_evidence`）：随机填充、往返解密、篡改密文拒绝，与 `wx-rust-common` 的 `security_rsa_mitigation_test`（14 项）构成双重证据链
- workspace tests：**3588 passed / 0 failed**；覆盖率 **70.06%** line（门禁 ≥60%）

### 真实环境验证（Alpha Day-5 闭环，2026-08-27）

- **订阅消息 + 客服消息双通道真实送达**（微信 errcode=0 + 接入方人工确认）
- access_token / 验签 / 错误码映射 / token 过期自动刷新重试全部实测通过
- No-Go 闸门 GO（0 阻塞）；准出判定 DELAY（观察期 7 日，Day-7 = 2026-09-02）

### 其他

- 新增 `docs/known-issues.md`（已知风险与已接受风险清单）
- 新增最小原生小程序测试工程 `partme-miniapp-test`（openid 获取 + 订阅授权 + 客服会话）
- 修复 `scripts/alpha/alpha-exit-gate.sh` 检查 5 正则假阳性（`test result:.*[1-9][0-9]* failed`）

## [0.1.0] - 工程基线（2026-08-25 发布 crates.io，10/10 crate）

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

- **rsa 0.9.10 RUSTSEC-2023-0071**：Marvin Attack（medium），无修复版本可用（0.10 尚为 RC）。当前 mitigation：RSA-OAEP 盲化 + 固定消息加密。已配置 `deny.toml` 例外。待 rsa 0.10 稳定后升级并移除例外。（详见 `docs/known-issues.md`）
