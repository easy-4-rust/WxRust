# WP-03 Phase 1：动态拦截器、Multipart 与媒体下载差分（2026-11-15）

## 基线与证据等级

- Java Oracle：WxJava 4.8.4.B / `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`。
- Rust 工作区：`feature/wxjava-1to1-phase1` / `bc5dbb7`（存在先前未提交改动；没有 reset、clean、commit 或 push）。
- 测试方式：真实 SDK 经各自独立 127.0.0.1 Loopback Server 发起 Token 与业务请求；合成数据。
- 方法矩阵 `VERIFIED=0`。场景实测通过不代表模块或全 SDK 已一比一迁移完成。

## 1. 动态 Request Interceptor

- RED：`artifacts/phase1-diff/http-dynamic-interceptor-red-20261114-03/report.json`，`DIFFERENT=1`。Java 在 Token/API 请求动态添加 `X-WxRust-Dynamic`，而未启用 hook 的 Rust 未添加。
- GREEN：`artifacts/phase1-diff/http-dynamic-interceptor-green-20261114-03/report.json`，`EQUIVALENT=1`。Token GET 的值为 `token-hook-v1`；API GET 的值为 `api-hook-v1`。响应和完整服务端请求轨迹严格一致。
- 已确认能力边界：静态客户端默认 Header 与按最终请求变化的 Header；尚未涵盖动态请求签名、拦截器冲突、所有业务请求路径和多并发重试。

## 2. Multipart 两类真实 SDK 上传

- Media Upload：`artifacts/phase1-diff/http-multipart-media-final-20261114-03/report.json`，**严格 Wire 比较仍 DIFFERENT=1**。
- Common Upload（含 description 文本字段）：`artifacts/phase1-diff/http-multipart-common-upload-green-20261114-01/report.json`，**严格 Wire 比较仍 DIFFERENT=1**。
- 两类场景中，Java/Rust 真实请求目标一致、响应 JSON 相同，素材字段名 `media`、文件名 `phase1-binary-probe.bin`、二进制文件 MIME `application/octet-stream` 均一致。
- 直接从两侧 `body_hex` 提取 multipart 原始报文，以 MIME parser 独立解码，媒体文件内容均为 14 bytes，SHA256 均为 `eb2e42101eb44509ef7c39418c6e7dfa36a3926e8e401bcf25fe158f4b763b24`；Common Upload 的 `description` 字段内容 22 bytes，SHA256 均为 `b489ee2a1a8f0eb2f6daf68868c7e34e86b88486a5f744eb002272ac347ab333`。
- 语义比较下字段序列、值、媒体类型、文件名和字段内容一致；**原始字节不同不可隐去**：Apache HttpComponents 与 Reqwest 使用不同随机 boundary，Common Upload 的 `Content-Type` Header 大小写也不同。
- 此处「MIME 语义一致」与「Raw Wire 一比一」是两种不同结论；严格差分仍不通过，未把随机 boundary 归一化成 PASS。
- 迁移中新增了 Rust MiniApp `upload` 通用入口、Multipart Content-Type/字段属性对齐，以及 `WxMediaUploadResult` 可空字段与 camelCase/snake_case 别名支持。

## 3. Binary 下载

- 修复 `wx-rust-miniapp/src/api/impl/wx_ma_media_service_impl.rs` 中 `get_media` 未应用自定义 apiHost 的问题。
- RED `artifacts/phase1-diff/http-media-download-red-20261114-01/report.json`：Rust 未进入预期 Loopback API（0 API calls），不可宣称成功。
- 后续 `artifacts/phase1-diff/http-media-download-green-20261114-01/report.json`：Java/Rust 均真实请求本地 Token/API，响应 Content-Type、Content-Disposition 和文件内容一致。
- 仍为 **DIFFERENT=1**：Java 返回本地临时文件（并具有动态文件名），Rust 返回 `Vec<u8>`。在 Rust 示例适配器里 `file_name=null`，未伪装成相同 File 语义。
- 数据 `bytes_hex=000102030d0a4142437f` 两侧一致；完整文件路径/临时文件生命周期仍未迁移。

## 4. 新增回归与构建记录

- Python Harness：40/40 passed。
- D 盘独立 Target `D:\WxRust-Phase1-verify-target-20261115`，不清理 E 盘已有构建产物。
- Common bean 测试 `phase1_media_upload_result`：2/2 passed。
- MiniApp `media_upload_and_get`：1/1 passed。
- CP `media_upload_parse_result_and_json_error_download`：1/1 passed。
- CP `test_ec_upload_attachment`：1/1 passed。
- Common HTTP Pipeline：13/13 passed。
- 受影响定向测试总计 **18 passed，0 failed**。
- 2026-11-15 `cargo check --workspace --all-targets --offline --locked`：修复 CP TP 返回值类型与 MiniApp/CP 断言后通过。
- 原 E 盘 `cargo test --workspace` **失败于 rustc-LLVM no space on device**，不应记录成测试失败或测试通过；E 盘可用空间约 0.01 GiB，D 盘在开始验证时约 58.35 GiB。
- **Workspace 全量测试从 D 盘独立 Target 重新执行成功**：`cargo test --workspace --offline --locked --quiet`，`5025 passed / 0 failed / 1 ignored`，215 组测试汇总，退出码 0（耗时 355.7 秒）。完整日志：`D:\\WxRust-Phase1-validation-20261115\\workspace-tests.log`，SHA256 `6F851EE1EA4BD32CBAD5009AF5BC29C1C92E9DD9DA973D56B527EFFC3AB043A8`。严格 Clippy 独立核验，未提前标记。
- 最后仅整理 Rust 示例适配器的类型别名，随后 **再次执行全量 Workspace 回归**：`5025 passed / 0 failed / 1 ignored`（215 组，退出码 0），最终日志 `D:\\WxRust-Phase1-validation-20261115\\workspace-tests-final-02.log`，SHA256 `CD60919E6801F4E067A02EEDEED24E7F7320D6D6DF942C2A56A7908152C9B425`。
- **严格 Clippy 通过**：`cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings`，退出码 0，日志 `D:\\WxRust-Phase1-validation-20261115\\clippy-all-targets-03.log`，SHA256 `7D6CB2B487DFA5AEFB562E48F61D27A3BCF486C78A5C2B6423374C99B2ED497B`。
- 最终 `cargo fmt --all -- --check`、`git diff --check` 通过，Cargo.lock 未改变；Python Harness 40/40 通过。

## 5. 未完成

- Multipart Raw Wire 字节级一致性（随机 boundary 与部分 Header 大小写差异）。
- Binary 下载的 Java File / Rust Vec<u8> 返回契约，临时文件及下载元数据语义。
- HTTP 超时取消、Token 并发刷新、Redis 跨进程一致性、压缩/流式响应、CP Native 与 Pay V3 等。
- 逐方法矩阵的实测 `VERIFIED` 仍为 0；单场景的等价不构成整个接口的正式验收。

没有 commit、push、merge 或 release。所有以前的本地修改与独立原始证据均保留。
