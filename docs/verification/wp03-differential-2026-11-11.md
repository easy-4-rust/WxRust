# WP-03 双运行时差分：首批执行与差异记录

日期：2026-11-11。工作分支：`feature/wxjava-1to1-phase1`。基础 HEAD：`bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`，保留先前未提交的 WP-01/WP-02/P0-01 改动。

## 本轮结论

真实执行固定 WxJava 与当前 WxRust 的同一组 27 个样本。第一次 19 一致 / 8 不同；修复 Pay UTF-16 键排序后，第二次 21 一致 / 6 不同。两轮 Java 与 Rust Adapter 均正常编译和执行，返回码 1 来自真实差分，不是基础设施错误。**总体兼容验收仍未通过。**

| 范围 | 样本数 | 最终一致 | 最终不同 |
|---|---:|---:|---:|
| Common WxError 解析/翻译 | 16 | 10 | 6 |
| Pay Map 签名 | 11 | 11 | 0 |
| 合计 | 27 | 21 | 6 |

## 已修复：PAY-SIGN-UTF16

根因：Java `TreeMap<String,...>` 使用 UTF-16 单元顺序，Rust `String::Ord` 按 UTF-8/Unicode 标量顺序。U+10000 与 U+E000 同时作为 key 时排序相反，改变签名输入。通常的 ASCII 协议参数不受这个差异影响；这是一项精确迁移边界修复，不是已证实的线上支付事故。

变更：`crates/wx-rust-pay/src/util/sign_utils.rs` 改用 `encode_utf16().cmp(...)` 比较 key，不修改过滤字段、拼接规则、散列算法或密钥。

测试证据：`phase1_sign_utf16` 的 MD5/HMAC 两项测试先真实失败（与 Java 产生的 golden 不同），修改后 2/2 通过。随后重新 clean 构建固定 Java SDK、重编 Rust Adapter并执行共享 fixtures，两个场景均从 DIFFERENT 转为 EQUIVALENT。

## 尚未修复的六项 Common 差异

| 场景 | Java 实测 | Rust 实测 |
|---|---|---|
| `errcode` 为字符串 `"40001"` | 解析为 40001 | 返回 -99 解析失败对象 |
| `errcode` 为 null | 默认 0，保留消息 | 返回 -99 解析失败对象 |
| `errmsg` 为数字 123 | 转换为字符串 `"123"` | 返回 -99 解析失败对象 |
| 非法 JSON `{not-json` | JsonSyntaxException | 返回 -99 解析失败对象 |
| 整体 JSON null | NullPointerException | 返回 -99 解析失败对象 |
| 整体 JSON array | JsonSyntaxException | 返回 -99 解析失败对象 |

前面三项是 Gson primitive coercion 与直接 serde 结构体解析的差异。后面三项涉及 Java 异常与 Rust 当前返回 `Self` 的 API 合同，不应在 Adapter 内转换为相同结果来掩盖。后续应独立测试并决定 fallible 解析入口及旧 API 兼容方式，不能在没有完整调用链证据时批量更改。

## 方法与环境边界

- Oracle SHA：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`，`D:\wxjava-phase1-oracle-20261111`；原 WxJava develop 不切换。
- Java 17 路径显式指定，避免 PATH 指向 Java 8。
- Maven Common/Pay clean package 成功；上游 Surefire 测试跳过。运行的是随后编译的真实 SDK Adapter，不是静态推导。
- Common 可观察字段限定为 errorCode/errorMsg/errorMsgEn；不包括原始报文字符串与 toString。
- XML 语义比较器已自测，不等于完成 Java/Rust XML SDK 差分。
- HTTP、并发/取消/Redis Token、Pay V3、CP Native 与 P1/P2 尚未完成。
- 不更新方法矩阵 VERIFIED；本报告仅作场景证据。

## 证据文件

- `tools/compat/README.md`：协议、退出码、复现命令和未覆盖项。
- `tools/compat/fixtures.jsonl`：27 个合成样本，固定输入，两侧共用。
- `artifacts/phase1-diff/baseline-20261111-01/`：第一轮原始日志和报告。
- `artifacts/phase1-diff/after-pay-fix-20261111-01/`：修复后原始日志和报告。
- `artifacts/phase1-diff/validation-20261111-01/`：完整工程验证日志。
- `docs/verification/wp03-differential-2026-11-11.json`：可携带的机器证据摘要，含源码/fixture 哈希和测试统计。

## 工程验收

| 命令/验证 | 本轮结果 |
|---|---|
| Python comparator/runner 自测 | 18 passed |
| Pay UTF-16 定向回归 | 修复前 2 failed；修复后 2 passed |
| cargo fmt --all -- --check | exit 0 |
| cargo check --workspace --all-targets --offline --locked | exit 0 |
| cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings | exit 0 |
| cargo test --workspace --offline --locked | 5007 passed、0 failed、1 ignored（完整日志 212 个 summary 汇总） |
| 双运行时最终差分 | exit 1；21 EQUIVALENT、6 DIFFERENT，不能称整体通过 |

工程验证前后源集合哈希一致，并与最终差分报告的源集合哈希相同：`30fd420383f94bdd562ed3cc54ca50573962e826d0ef2c90e3a2f1ce9ddfb629`。

比较器与 Runner 的 18 项自测通过，包含对空样本、重复输出、缺少操作名、异常缺少类型、类加载错误来源、子进程失败、覆盖旧证据等的拒绝测试。最终全工作区统计以完整 `workspace-tests.log` 汇总及机器摘要为准，不再从工具返回的 tail 推断总数。

WP-03 已交付可运行首批框架；完整 T017/T018 和全部 P0 行为收口仍未完成。没有提交、推送或发布。
