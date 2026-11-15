# WP-03 第二轮：WxJava / WxRust WxError 真实差分与 API 兼容性验收

日期：2026-11-12。范围：WxJava 4.8.4.B 固定提交 `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`；WxRust 工作分支 `feature/wxjava-1to1-phase1`（基线 HEAD `bc5dbb7`，工作树含尚未提交的前序改动）。

## 结论（证据等级：真实执行）

固定版本 Java Oracle 与当前 Rust Adapter 重新编译，在同一份 `tools/compat/fixtures.jsonl` 上执行，最终得到 **30 个样本：27 个 EQUIVALENT，3 个 DIFFERENT**；整体差分退出码为 1，不能宣称完整兼容。此处 27 个 EQUIVALENT 必须细分为 24 个完整 JSON 结果比较一致、3 个仅错误类别比较一致，后者不验证 Java/Rust 异常类型和异常消息完全相等。

| 执行阶段 | 样本 | 一致 | 差异 |
|---|---:|---:|---:|
| 2026-11-11 Pay UTF-16 修复后 | 27 | 21 | 6 |
| 本轮 Common 字段转换修复后 | 27 | 24 | 3 |
| 本轮新增 fallible 入口与错误类别样本后 | 30 | 27 | 3 |

## 1. 代码修复：字段转换遵循 GsonHelper

直接对照 WxJava `WxErrorAdapter.deserialize`、`GsonHelper.getAsPrimitiveInt` 与 `getAsString`，在 Rust 的 `WxError` 反序列化字段上加入显式规则。

| Fixture | WxJava 固定版本的可观察行为 | Rust 修复后 | 结果 |
|---|---|---|---|
| `error-code-string` | `"40001"` → `40001` | 相同 | EQUIVALENT |
| `error-code-null` | JSON null → int 0 | 相同 | EQUIVALENT |
| `error-message-number` | JSON number 123 → string `"123"` | 相同 | EQUIVALENT |

新增 Rust 测试 `phase1_wx_error_coercion.rs`：首次实际运行 3 failed / 2 passed（RED）；修改后 5 passed / 0 failed（GREEN）。保留缺失/null 与非基本类型拒绝的回归用例。没有在 Adapter 中人为重写这些业务解析规则。

## 2. API 合同：异常与 -99 错误对象不是同一行为

原 Rust `WxError::from_json` 和 `from_json_with_type` 返回 `Self`，解析异常转成 `-99` 对象。这与 Java `WxError.fromJson` 的抛异常语义不同。为避免一次性破坏现有生产调用方，保留旧入口的历史行为，新增以下可选 API：

- `WxError::try_from_json(&str) -> Result<WxError, WxErrorParseError>`。
- `WxError::try_from_json_with_type(&str, Option<WxType>) -> Result<WxError, WxErrorParseError>`。
- 结构化错误类型分别标记 `MalformedJson`、`NullRoot`、`UnexpectedRootType`、`InvalidField`，并提供错误类别。此处 Rust 不伪造 Java 异常类名。

新增六项 Rust fallible 测试首次因接口缺失编译失败（RED），实现后 6 passed（GREEN）。原 `from_json()` 对 `{not-json`、`null` 和 `[]` 仍返回 -99，这是刻意保留的向后兼容行为，不属于原方法的行为等价。

另新增 3 条 `common.error.try` 真实双运行时样本，显式选用 `error_category` 比较模式。Java Adapter 仍调用真实 `WxError.fromJson`，按捕获的实际 Java 异常类别分类；Rust Adapter 调用真实 `try_from_json_with_type`，记录实际 Rust 类型、变体和消息。比较器仅在样本明确选择该模式且两侧都产生真实 exception 时按类别比对，不能把值与异常判为一致，类别缺失则基础设施失败。

| 原始 Java 方法样本（仍有差异） | Java 结果 | Rust 历史入口结果 |
|---|---|---|
| `error-malformed-json` | `JsonSyntaxException` | `WxError(-99)` |
| `error-json-null` | `NullPointerException` | `WxError(-99)` |
| `error-json-array` | `JsonSyntaxException` | `WxError(-99)` |

**关键约束：** 新增三个错误类别一致的样本不能冲销上述三个旧 API 差异，也不能升级 `METHOD_COMPATIBILITY_MATRIX.csv` 中任何方法为 VERIFIED。若要达到原方法级行为兼容，必须先明确调用方迁移到 fallible API 的传播策略，并执行端到端差分。

## 3. 回归：发现并修复 MiniApp Mock Token 测试边界

全 Workspace 首次重跑暴露 `g4_complaint_query_errcode_nonzero` 失败：Mock 只配置业务 API，未将 Token 刷新 URL 指向本地 Mock；P0-01 真正刷新后返回环境/HTTP 错误，掩盖了原来的 40001。修复仅限测试：增加本地 Token endpoint、请求次数断言，验证初次取 Token 与刷新各一次、业务 API 初次调用与单次重放各一次，并防止测试误触真实网络。定向测试 1/1 通过；未弱化原 `Some(40001)` 断言。

## 4. 质量门禁（证据等级：实际命令）

| 项目 | 结果 |
|---|---|
| Python Comparator/Runner 自测 | 21 passed |
| Rust Common Gson 转换定向测试 | 5 passed |
| Rust Common fallible 解析定向测试 | 6 passed |
| MiniApp Token Mock 定向测试 | 1 passed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo check --workspace --all-targets --offline --locked` | exit 0 |
| `cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings` | exit 0 |
| `cargo test --workspace --offline --locked` | **5,018 passed，0 failed，1 ignored**；完整日志的 214 组 summary 统计 |
| `cargo llvm-cov --package wx-rust-common --summary-only --offline --locked` | Common：Region 91.08%，Function 72.30%，Line 93.20% |

覆盖率是 **wx-rust-common 单包**，不是整个 WxRust Workspace。`wx_error.rs` 的 Line 覆盖率约 88.57%。第一次全量回归失败的日志保留于 `artifacts/phase1-diff/common-validation-20261112-01/`，修复后的全部成功日志与 coverage 输出保留于 `common-validation-20261112-02/`，不会把失败记录删除。

## 5. 可复现证据

- 机器摘要：`docs/verification/wp03-common-error-2026-11-12.json`，包含源集合 Hash、Java class/Rust binary SHA256、两轮差分结果、原 API 差异、fallible 新入口结果与全量测试/覆盖率日志哈希。
- 原始共享输入及两侧 JSONL 输出：`docs/verification/wp03-evidence-2026-11-12/`。
- 实际双运行时日志：`artifacts/phase1-diff/common-fallible-20261112-01/`（本地留存，不入 Git）。
- 实际工程验证日志：`artifacts/phase1-diff/common-validation-20261112-02/`（本地留存，不入 Git）。

复现：

```powershell
python -m unittest discover -s tools/compat -p 'test_*.py'
python tools/compat/run_diff.py --java-root D:\wxjava-phase1-oracle-20261111 --jdk-home D:\Java\jdk-17.0.12 --out artifacts/phase1-diff/a-new-empty-directory
cargo test --workspace --offline --locked
cargo llvm-cov --package wx-rust-common --summary-only --offline --locked
```

注意差分命令目前**预期因三个旧入口差异返回退出码 1**，这是正确的行为差异证据，不是构建失败。

## 6. 下一步

1. P0 Common：审计 `WxError::from_json_with_type` 的约 30 个以上调用方，决定旧 `Self` 返回与新 `Result` 在 RequestExecutor/HTTP Pipeline 的传播和兼容策略；必须增加相同 Mock 请求/错误/重试/副作用的双运行时 Fixture。
2. P0 Pay：V3 RSA/证书/回调验签与 AES-GCM 官方样本差分；本轮只认证了有限的 Pay Map 签名输入。
3. P0 CP：Finance Native SDK 私有聊天/媒体拉取仍 UNSUPPORTED，不得因解密适配而标记完整。
4. P1/P2 与 WxJava 4.8.6 的增量账本仍未完成；不将对象映射、方法候选或测试覆盖率等同于行为一比一完成。

本轮未提交、未推送、未发布。
