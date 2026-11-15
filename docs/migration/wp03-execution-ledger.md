# SDD ledger — plan: docs/superpowers/plans/2026-11-11-wp03-differential-harness.md

## 2026-11-11 首批执行

- 执行位置：既有 feature/wxjava-1to1-phase1；HEAD bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc。保护先前未提交改动，不切换到缺少这些改动的新 worktree。
- Pre-flight: T016协议→Java/Rust Adapter→T018比较器，三者共用 id/operation/result.kind。输出缺失/重复/非法字段视为INFRA_ERROR，不能作为SDK行为一致。
- Ruling: 原计划范围大，本轮只完成 Common错误解析和Pay Map签名的可运行闭环；HTTP/CP Native真实差分仍留待验证，不以模拟实现凑完成数。
- Task A complete: comparator 12项初始RED（未实现）→12项GREEN；协议/来源审查再增加测试，观察3项RED后修复，最终18项GREEN。
- Task B complete: Java固定SHA clean package成功（Surefire跳过），两侧SDK实际执行27条共享输入；基线19一致/8差异。
- Task C complete (bounded): Pay UTF-16排序回归2失败→2通过；重新运行两侧SDK后21一致/6差异。Common未更改，6失败样本保留。
- Task D complete: cargo test --workspace --offline --locked从完整日志统计5007通过、0失败、1忽略；fmt/check/all-feature Clippy通过。源集合前后哈希一致。
- Task E complete: 协议README、机器证据JSON、四份原始输出JSONL、样本快照与验证报告已保存；方法矩阵未升级VERIFIED。
- Ruling: 历史1298测试数是截断摘要，不当作全量总数；README/P0报告已注明更正。
- No commit / push / publish performed.

## 2026-11-11 待办快照（历史）

当时的 Common WxError 差异已在 2026-11-12 第二轮分阶段处理；HTTP POST / Token Replay 已在 2026-11-13 第三轮完成一个真实差分场景。后续未完成项以最新技术报告为准。
## 2026-11-12 第二轮：Common WxError 行为差分与错误模型

- Oracle 仍固定 WxJava 4.8.4.B `a49d6e1`，测试工作区仍为 `feature/wxjava-1to1-phase1`；保留全部既有未提交改动。
- 直接对照 Java `WxErrorAdapter` 与 `GsonHelper`，新增 Rust serde `errcode`/`errmsg` 基本类型转换；3 个 Rust 测试实际 RED → GREEN，真实 Java/Rust 原始 27 fixtures 从 21/6 提升为 24 一致/3 差异。
- 对于 malformed JSON、root null、root array 的 Java 抛异常与 Rust 老入口返回 -99 不等价，决定保留现有 `Self` 返回接口，新增 `try_from_json*` 返回 `Result<WxError, WxErrorParseError>`。6 个 Rust fallible 测试先编译 RED 后 6/6 GREEN。
- 扩展 Fixture 至 30 条，其中额外三个显式 `error_category` 比较样本验证新 fallible API 与真实 Java 异常的类别一致，不冒充异常类型/消息相同；最终双运行时结果 24 个 JSON 值严格一致、3 个错误类别一致、3 个旧 API 行为 DIFFERENT，整体退出码 1 正常保留。
- 比较器/Runner 自测从 18 增加到 21 项，拒绝缺失的错误类别以及 exception 对 value 的伪等价；运行结果 21/21 passed。
- 全量回归第一次发现小程序投诉查询 Mock 未接管 Token 刷新 URL，导致本应保留的 40001 被 HTTP 失败掩盖。修复测试 Mock 而非放松生产代码：本地 Token endpoint、2 次 Token + 2 次业务请求断言；定向测试 PASS。
- 全 Workspace 第二次验证 PASS：`cargo test --workspace --offline --locked` 5018 passed / 0 failed / 1 ignored，214 summary groups；`cargo fmt`、all-targets `cargo check`、all-features 严格 Clippy 均 exit 0；源集合哈希与最终双运行时结果一致。
- `cargo llvm-cov --package wx-rust-common --summary-only --offline --locked` PASS。仅 Common 包 Regions 91.08%、Functions 72.30%、Lines 93.20%，不可推广到 Workspace。
- 机器审计 `docs/verification/wp03-common-error-2026-11-12.json`，原始双方输出 `docs/verification/wp03-evidence-2026-11-12/`，完整日志 `artifacts/phase1-diff/common-validation-20261112-02/`，技术总结 `docs/verification/wp03-common-error-2026-11-12.md`。
- 不更新 `METHOD_COMPATIBILITY_MATRIX.csv` 的 `VERIFIED=0`；下一个任务仍是 Common HTTP/Token 错误传播、重试行为和副作用的真实双运行时差分，以及 Pay V3 / CP Native 专项。
- No commit / push / publish performed.
## 2026-11-13 第三轮：Common HTTP POST / Token Replay 真实差分

- Oracle：WxJava 4.8.4.B `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`；Rust 仍在 `feature/wxjava-1to1-phase1`，HEAD `bc5dbb7`，保护所有先前 dirty changes。
- 新增独立 Runner `tools/compat/run_http_diff.py`，不修改既有 Common Error / Pay Runner。Java Adapter 调用真实 `WxMaServiceImpl.post()`；Rust Adapter 调用真实 `WxMaService::post()`，两端都只访问 loopback server。
- 源码核实：Java `WxMaServiceImpl` extends `WxMaServiceHttpComponentsImpl`；`HttpComponentsSimplePostRequestExecutor` 以 `ContentType.APPLICATION_JSON.withCharset(StandardCharsets.UTF_8)` 创建 StringEntity。Java 实际 wire header 为 `application/json; charset=UTF-8`。
- RED：两端均真实执行 4 次请求（GET T1、POST T1→40001、GET T2、POST T2→成功），Method、Raw Target、Body、结果、Token/API 次数都一致；唯独 Rust API POST 缺少 Content-Type，真实差分 1/1 DIFFERENT。
- 先在 `execute_pipeline_test.rs` 增加 header assertion，实际 RED（headers=[]）；随后最小修改 Common `execute_pipeline_inner()`：Text body 增加 JSON UTF-8 Content-Type；None 继续 GET；Bytes 不推断文本 Content-Type。
- GREEN：定向 Pipeline 测试 8/8 passed；真实双运行时 HTTP 请求轨迹严格相等 1/1 EQUIVALENT，Token GET 2 次、API POST 2 次，返回体相同。
- Runner/fixture 测试 26/26 passed。Java class provenance 检查实际 Common `WxError` 与 MiniApp `WxMaServiceImpl` class 目录；固定 Oracle class SHA、Rust binary SHA、source/fixture SHA 均记录。
- 完整验证：fmt/check/strict all-feature Clippy 均 exit 0；`cargo test --workspace --offline --locked` 5018 passed、0 failed、1 ignored（214 summary groups）；`cargo llvm-cov --package wx-rust-common --summary-only --offline --locked` 行 93.11%、函数 72.30%、Region 91.01%。
- 最终 source SHA256：`682a43bb309cec21e25c51f2f70eca5b034b82e13c16d5403386dcdd9d5ab5c0`，HTTP GREEN 差分报告与完整验证前后 hash 相同。
- 机器报告 `docs/verification/wp03-common-http-2026-11-13.json`；技术报告 `docs/verification/wp03-common-http-2026-11-13.md`；可携带原始证据 `docs/verification/wp03-http-evidence-2026-11-13/`；本地完整日志 `artifacts/phase1-diff/http-validation-20261113-01/`。
- 范围限制：只证明 MiniApp POST + Token 40001 单次刷新/重放这一条路径；普通 API GET、任意自定义 Headers、非 2xx、空响应/XML、multipart/binary、并发/取消/超时/跨实例一致性、CP Native、Pay V3 仍未验证。方法矩阵继续 `VERIFIED=0`。
- No commit / push / merge / publish performed.
## 2026-11-13 第四轮：Common HTTP GET / 非 2xx / 空响应真实差分

- 保持既有 dirty checkout：`feature/wxjava-1to1-phase1` / `bc5dbb7`；固定 WxJava Oracle `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`；所有流量只到 127.0.0.1。
- Runner 扩展为 4 个共享真实场景，trace 记录 request method/raw target/headers/body + response status/body + 顺序和调用次数；支持字符串响应与 `{status, body}` 响应 fixture。`http_error` 比较类别+完整轨迹/次数，原始异常类型和消息保留。
- GET Query RED：Java `BaseWxMaServiceImpl.executeInternal` 先注入 access_token，`SimpleGetRequestExecutor` 后追加 queryParam；旧 Rust 顺序相反。保留 RED 真实请求轨迹，新增 Common `execute_pipeline_with_refresh_and_query`，MiniApp GET 通过该入口构造 URL；T1 40001 刷新 T2 后重放仍保持同一顺序，GREEN。
- HTTP 503 RED：Java `HttpResponseException` 被 WxRuntimeException 包装；旧 Rust 忽略 `TransportResponse.status`，将 Body 中 `errcode=0` 作为成功。Common Pipeline 在 JSON parse 前拒绝 status >=300，GREEN 错误类别 http_status + 轨迹一致。
- 空 Body RED：Java 实际 NullPointerException（Gson 返回 null 后访问 WxError），旧 Rust 返回 WxError(-99)。Common Pipeline 在 JSON parse 前拒绝 empty body，返回 Runtime response failure，GREEN 类别 empty_response_runtime + 轨迹一致。
- 最终 4 场景 4/4 EQUIVALENT：POST + GET 两个完整 JSON/trace；503 + empty 两个显式错误类别/完整 trace。跨语言异常 class/message 不宣称相同。
- RED 证据：`http-get-query-replay-red-20261113-01`（1 EQUIVALENT / 1 DIFFERENT）；`http-status-empty-red-20261113-01`（2 EQUIVALENT / 2 DIFFERENT）。GREEN：`http-status-empty-green-20261113-01`（4/4 EQUIVALENT）。
- Harness 32 passed；Common Pipeline 11 passed；fmt/check/strict all-feature Clippy exit 0；Workspace 5021 passed / 0 failed / 1 ignored（214 summary groups）。
- `cargo llvm-cov --package wx-rust-common --summary-only --offline --locked`：仅 Common 单包 lines 93.03%、functions 72.81%、regions 91.02%。
- 源集合 SHA256（双运行时 GREEN 与全量验证前后相同）：`446c15e11cfb4b23295d8b51855c2700bdae9dfe33c247c0bc212871285e791b`。
- 机器摘要 `docs/verification/wp03-common-http-get-status-2026-11-13.json`；技术报告 `docs/verification/wp03-common-http-get-status-2026-11-13.md`；可携带 33 份原始证据 `docs/verification/wp03-http-get-status-evidence-2026-11-13/`；完整本地日志 `artifacts/phase1-diff/http-get-status-validation-20261113-01/`。
- 未覆盖：任意自定义 Header、3xx/1xx、Token endpoint 非 2xx、XML、multipart/binary/streaming、并发/取消/超时/跨实例 single-flight、CP Native、Pay V3、完整模块/方法一比一。方法矩阵仍 `VERIFIED=0`。
## 2026-11-13 第五轮：Client default Headers / SimplePost XML 真实差分

- Oracle 仍固定 WxJava 4.8.4.B `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`；分支 `feature/wxjava-1to1-phase1`，HEAD `bc5dbb7`；保留所有既有 dirty changes，所有流量仅限 loopback。
- 源码核对：WxJava 默认 HttpComponents Client 的共享 requestInterceptors 同时作用于 Token/API 请求；WxMaService.get/post 没有 per-call Header map。本轮只覆盖静态 Client default headers，不声称动态 interceptor 或 per-call override 兼容。
- Java SimplePost 源码核实：空 Body 返回 `WxErrorException(-99, 无响应内容)`；精确 `<xml>` 前缀原文返回；其他非空 Body 继续 JSON WxError 解析；GET 不走 XML 快速路径。
- RED 新增 3/3 DIFFERENT：静态 Client Headers 未从 Rust 默认 Client 发出；XML POST 在 Rust 侧被当 JSON 解析；POST 空响应错误路径与 Java 不同。
- Rust 新增可注入预配置 Reqwest Client 的 `new_arc_with_http_client`，保留原 `new_arc`；MiniApp SimplePost Text Body 按 Java 分支处理空 Body -99 与 `<xml>` 原文；GET 空 Body Runtime 语义保持不变。
- GREEN 合并七场景 7/7 EQUIVALENT：四个成功场景严格 JSON+wire trace；三个错误场景比较类别、可用 error_code、完整请求/响应轨迹与调用次数，保留原始异常类名/消息。
- Harness 36 passed；Common Pipeline 13 passed；Workspace 5023 passed / 0 failed / 1 ignored（214 groups）；fmt/check/strict Clippy/git diff --check 全通过。
- Common 单包 llvm-cov：lines 92.94%、functions 72.81%、regions 90.93%，不代表 Workspace 覆盖率。
- 全量验证前后源集合 SHA256：`eddd877c5b6428ddd84db84c830035f8169dd6b093292f15d9fac394c709f2de`。
- 机器摘要 `docs/verification/wp03-common-http-client-xml-2026-11-13.json`；技术报告 `docs/verification/wp03-common-http-client-xml-2026-11-13.md`；26 份原始证据 `docs/verification/wp03-http-client-xml-evidence-2026-11-13/`；完整日志 `artifacts/phase1-diff/http-client-xml-validation-20261113-01/`。
- 未覆盖：动态 interceptor/per-call headers、GET XML、XML 声明/前导空白/schema、重复 Header 顺序/签名刷新、3xx/1xx、Token endpoint 非 2xx、multipart/binary/streaming、并发/取消/超时/跨实例 single-flight、CP Native、Pay V3、完整方法/模块兼容。方法矩阵仍 `VERIFIED=0`。
- No commit / push / merge / publish performed.
- No commit / push / merge / publish performed.

## 2026-11-15 第六轮：动态 Interceptor / Multipart / Binary 差分与 D 盘全量回归

- 在 `feature/wxjava-1to1-phase1` 原工作区继续；固定 WxJava 4.8.4.B Oracle，保留已有 dirty changes，不 reset/clean/switch/commit/push。
- 动态 HTTP Request Interceptor：真实 Token 请求和 API 请求分别添加 `token-hook-v1` / `api-hook-v1`；RED 1/1 DIFFERENT，GREEN 1/1 EQUIVALENT。
- 两类 Multipart：真实请求 Method/URL、返回值相同；按 MIME 独立解析，媒体二进制 14 bytes 和可选文本字段 22 bytes 的内容 SHA256 均一致；**原始 Wire 仍是 DIFFERENT**（Apache/Reqwest 随机 boundary、字段 Header 大小写）。不将语义一致当作字节级一比一。
- Binary 下载：已修复自定义 API Host；双方下载文件字节一致；Java 返回临时 File 和随机文件名，Rust 返回 Vec<u8>，返回模型仍 DIFFERENT。
- `WxMediaUploadResult` 补充可空值、snake_case/camelCase 的 serde 语义；对应修复 CP TP 返回 Option 以保留 Java null、MiniApp/CP 受影响测试断言。其 public Rust API 签名变化需要在后续兼容性评估中明确。
- Python Harness 40/40；受影响定向 Rust 测试 18/18；Cargo Check/fmt、Git diff --check、严格 Clippy 均通过；Cargo.lock 无变化。
- E 盘编译 Rust Workspace 首次失败于 no space on device（非业务断言）；未删除旧缓存。D 盘新 target 完成两次全量测试，最终 `5025 passed / 0 failed / 1 ignored`（215 组），退出码 0。最终日志 `D:\WxRust-Phase1-validation-20261115\workspace-tests-final-02.log`，SHA256 `CD60919E6801F4E067A02EEDEED24E7F7320D6D6DF942C2A56A7908152C9B425`。
- 严格 Clippy 日志 `D:\WxRust-Phase1-validation-20261115\clippy-all-targets-03.log`，SHA256 `7D6CB2B487DFA5AEFB562E48F61D27A3BCF486C78A5C2B6423374C99B2ED497B`；机器摘要及详细边界见 `docs/verification/wp03-dynamic-interceptor-multipart-binary-2026-11-15.{md,json}`。
- 方法矩阵保持 `VERIFIED=0`；不能宣布全部迁移完成。未执行 commit/push/merge/release。

## 2026-11-15 第七轮：MiniApp getMedia 文件返回语义差分

- 固定 WxJava 4.8.4.B `a49d6e1` 与 Rust 原分支 `feature/wxjava-1to1-phase1` / `bc5dbb7`，保留既有 dirty changes；只访问 127.0.0.1。
- 对照 Java `WxMaMediaServiceImpl.getMedia`、`HttpComponentsMediaDownloadRequestExecutor`、`HttpResponseProxy` 和 `FileUtils`，新增 Rust `get_media_file()->PathBuf`，原 `get_media()->Vec<u8>` 保留（ADAPTED）。
- TDD：Content-Disposition 中 quoted ASCII、UTF-8 filename* 优先、缺少响应头 3 项先 RED 再 GREEN。
- 真实成功下载 Strict JSON RED（随机文件名/目录名差异）；显式 `file_semantics` 对比文件存在、命名规则、10 字节内容及严格 HTTP 轨迹，GREEN（保留 `raw_value_equal=false`）。
- 缺少 Content-Disposition 双方真实 -99 与错误类别一致；HTTP 503 Body errcode=0 时 Rust 原错误生成文件 RED，改为发送/解析后在创建文件前拒绝，GREEN。最终同一源码三条 Loopback 全部 EQUIVALENT。
- 源集合 SHA256 `ffa7e9f108812c52f737936e395504022f9f25c98f5730bb679f1575c17cf5ea`；Python Harness 44/44，MiniApp 3/3；全 Workspace `5028 passed / 0 failed / 1 ignored`（215 groups）；Fmt/Check/严格 Clippy exit0。
- 技术报告 `docs/verification/wp03-media-file-compat-2026-11-15.md`，机器摘要 `docs/verification/wp03-media-file-compat-2026-11-15.json`，34 份可携带原始输入/日志/报告 `docs/verification/wp03-media-file-evidence-2026-11-15/`。
- 未消除 Java deleteOnExit vs Rust 显式清理和随机文件名不同；外部 trait implementor 新增方法的源码兼容待评审；Multipart Raw Wire 依旧 DIFFERENT，方法矩阵 VERIFIED=0。不 commit/push/merge。
