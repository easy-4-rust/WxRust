# WxRust Phase 1 / P0-11：MiniApp 素材下载 Java File 语义兼容验证

验证日期：2026-11-15。对照源码固定为 WxJava **4.8.4.B**，commit `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`；Rust 位于 `feature/wxjava-1to1-phase1`、HEAD `bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`，工作区的原有未提交改动保持不变。

## 结论与证据等级

这轮直接调用真实 WxJava `WxMaMediaService.getMedia(String)` 和 Rust 新增 `WxMaMediaService.get_media_file(&str)`，两端只向各自的 `127.0.0.1` Loopback HTTP Server 请求，使用合成 Token、媒体 ID 与字节数据。

**三个明确的真实场景最终均通过：**

| 场景 | 首次/严格结果 | 本轮最终结果 | 证据范围 |
|---|---|---|---|
| 成功下载并返回临时文件 | 严格结果 `DIFFERENT 1/1` | 显式 `file_semantics`：`EQUIVALENT 1/1` | 请求/响应轨迹严格一致、文件真实存在、字节一致、文件名与临时目录符合规则；随机文件名原样保留且不相等 |
| 缺失 `Content-Disposition` | Java/Rust 实际异常 | `http_error`：`EQUIVALENT 1/1` | 两侧 `-99`、`missing_content_disposition` 类别和完整 HTTP 轨迹一致 |
| HTTP 503，Body 内 `errcode=0` | 修复前 Java 异常、Rust 错误地保存文件，`DIFFERENT` | `http_error`：`EQUIVALENT 1/1` | 两端在 HTTP 503 拒绝，类别 `http_status` 与完整轨迹一致 |

文件成功场景明确报告 `raw_value_equal=false`。**本结论是文件契约语义对齐，不是随机临时文件名或返回值逐字节一致。** 两侧异常类型与消息也保留原始证据，不声称类名完全相同。

## 1. 固定 WxJava 源码依据

- `weixin-java-miniapp/.../WxMaMediaServiceImpl.java`：`getMedia` 建立 `wxma` 前缀临时目录，经 `BaseMediaDownloadRequestExecutor.create` 请求素材并返回 `File`。
- `weixin-java-common/.../hc/HttpComponentsMediaDownloadRequestExecutor.java`：HTTP 状态异常按 Apache 响应处理；`Content-Type` 为 `application/json` 时按微信错误报文处理；其他响应通过 `HttpResponseProxy.getFileName` 获取文件名，采用 `FilenameUtils.getBaseName/getExtension` 和 `FileUtils.createTmpFile` 创建文件。
- `weixin-java-common/.../HttpResponseProxy.java`：优先解析 `filename*=utf-8''...` 并 URL 解码，其次解析 `filename="..."`，缺少有效文件名或响应头时抛异常。
- `weixin-java-common/.../fs/FileUtils.java`：使用 `File.createTempFile` 生成随机文件名，并调用 `deleteOnExit()`。

以上行为直接来自固定源码与实际双运行时，不使用自行构造的 Java SDK 语义替代代码。

## 2. Rust 实现

在 `wx-rust-miniapp`：

- 保留原 `get_media(&str) -> Result<Vec<u8>, WxErrorException>`，供已有字节调用方使用；它仍为 **ADAPTED** 而非 Java File 同签名 API。
- 新增 `get_media_file(&str) -> Result<PathBuf, WxErrorException>`，实际调用已有 SDK 的 Token 获取和 HTTP Client 发送链，应用自定义 API Host、Request Interceptors，按 Java 的 Content-Disposition 文件名逻辑返回临时文件路径。
- 非 JSON 成功内容通过响应分块写文件；先验证缺失/无效文件名，避免错误情况下创建无效文件。遇到 HTTP status >= 300 先拒绝，不会把 503 误保存成媒体文件。
- 成功结果由 Rust 调用方负责删除临时文件和目录；下载中出错会尽力清理当前临时目录/文件。
- 新文件名采用不同的随机/时钟后缀，保留扩展名和 `wxma` 临时目录规则。没有伪造 Java 的随机名称。

代码主要位于 `crates/wx-rust-miniapp/src/api/wx_ma_media_service.rs` 与 `.../impl/wx_ma_media_service_impl.rs`；Java/Rust 真实 Adapter 与 Fixture 在 `tools/compat/http/` 和 `crates/wx-rust-miniapp/examples/phase1_http_oracle.rs`。

## 3. 显式差分和失败关闭

新增比较器 `file_semantics`（`tools/compat/compare.py`），只在 Fixture 主动声明时执行。验证：

1. 两侧 SDK 均返回真实文件结果、字段齐全且 `file_exists=true`；
2. 文件名有指定前缀、扩展名，临时目录以 `wxma` 开头且名称非空；
3. 下载文件 **原始字节** 与 Fixture 的预期字节精确相等；
4. 实际 HTTP Token/API 请求、Headers、Status、响应 Body、请求顺序和次数全部严格相等；
5. 随机文件名和目录名原样保留，`raw_value_equal=false` 作为独立证据。

负向测试覆盖文件不存在、文件内容/扩展名/前缀错误、临时目录无效、HTTP 轨迹不匹配以及缺少必要输入证据，均不会被错误判为 EQUIVALENT。

`http_error` 仍要求双方真实返回 exception，保留实际 Java/Rust 异常类型、消息、错误类别、错误码和完整 Wire Trace；不把异常强行归一为成功值。

## 4. 执行证据

- 严格文件结果原始 RED：`artifacts/phase1-diff/http-media-download-file-red-20261115-01/report.json`。
- HTTP 503 修复前 RED：`artifacts/phase1-diff/http-media-download-file-503-red-20261115-01/report.json`。
- **最后一版代码真实三项 GREEN**：
  - `artifacts/phase1-diff/http-media-final-file-semantic-20261115-01/report.json`
  - `artifacts/phase1-diff/http-media-final-file-missing-disposition-20261115-01/report.json`
  - `artifacts/phase1-diff/http-media-final-file-503-20261115-01/report.json`
- 机器报告（包括每轮源、Fixture 和报告 SHA256）：`docs/verification/wp03-media-file-compat-2026-11-15.json`。
- 可携带的原始 JSONL、RED/GREEN 报告及日志：`docs/verification/wp03-media-file-evidence-2026-11-15/`（34 项，均核对 SHA256）。
- 同一套最后源码集合 SHA256：`ffa7e9f108812c52f737936e395504022f9f25c98f5730bb679f1575c17cf5ea`。

## 5. 完整工程验证

在 D 盘既有独立 `CARGO_TARGET_DIR=D:\WxRust-Phase1-verify-target-20261115` 上重新执行，避免 E 盘先前无空间的构建问题；没有删除旧缓存。

| 检查 | 当前结果 |
|---|---|
| Python Harness | **44 passed / 0 failed** |
| MiniApp 文件名解析定向测试 | **3 passed / 0 failed** |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo check --workspace --all-targets --offline --locked` | exit 0 |
| 严格 Clippy：Workspace / All Targets / All Features / -D warnings | exit 0 |
| `cargo test --workspace --offline --locked --quiet` | **5,028 passed / 0 failed / 1 ignored**，215 组 |

完整日志与验证哈希：`D:\WxRust-Phase1-media-file-validation-20261115/`，并已纳入上述可携带证据。不存在根据旧日志推断本轮回归的情况。

## 6. 仍未达到一比一的边界

- **Java 返回 `File`，Rust 文件接口返回 `PathBuf`；仍为跨语言 ADAPTED。** Rust 原有 `Vec<u8>` 便利接口也仍存在。新增 trait 方法对第三方 `WxMaMediaService` 自定义实现者可能构成源码兼容性影响，需在后续公共 API/semver 评审中处理。
- **临时文件生命周期不同**：Java `deleteOnExit` 依托 JVM，Rust 返回 `PathBuf` 要求调用方显式清理；不是完全相同。
- 文件名编码和恶意响应头的全部边界、Windows/macOS/Linux 文件系统差异、非常大的流式文件/取消行为还未完成全量差分。
- 两类 **Multipart Raw Wire** 仍 `DIFFERENT`（Apache / Reqwest 独立随机 boundary、部分 Header 字节大小写不同），之前 MIME 语义解析相同不等于字节级通过。
- Token 并发刷新、请求超时/取消、Redis 多实例一致性、Pay V3 和 CP Finance Native 未完成。

方法矩阵继续 `VERIFIED=0`；本轮仅证明三条真实场景的已声明语义。未 commit / push / merge / release。
