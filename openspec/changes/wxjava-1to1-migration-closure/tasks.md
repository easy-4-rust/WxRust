# Tasks

## WP-01 完整迁移分母

- [x] T001 固定 WxJava 4.8.4.B Oracle SHA，并记录源码 worktree。
- [x] T002 枚举九个核心模块全部 `src/main/java/**/*.java` 文件。
- [x] T003 纳入包外 `com.tencent.wework.Finance.java`。
- [x] T004 将 package-info、module-info、无法解析类型单独分类，不静默丢弃。
- [x] T005 生成完整对象 CSV，包含 FQN、源路径、类型、crate、预期 Rust 路径和 Oracle SHA。
- [x] T006 修正对象审计状态和模块汇总，状态总数机器校验闭合。
- [x] T007 生成 WP-01 验证报告和固定基线摘要。

## WP-02 方法级兼容矩阵

- [x] T008 在 Oracle worktree 编译九个核心 Maven 模块。
- [x] T009 改进 javap 清单：返回类型、修饰符、参数、throws、descriptor、泛型签名、声明类和重载 ID。
- [x] T010 纳入包外 Finance Native 方法，单独记录构造器和继承关系。
- [x] T011 生成 METHOD_COMPATIBILITY_MATRIX.csv。
- [x] T012 自动映射仅标记 CANDIDATE_MAPPED、CANDIDATE_FILE_ONLY、NEEDS_REVIEW；不自动标记 VERIFIED。
- [x] T013 对 Finance Native 方法建立 UNSUPPORTED/ADAPTED 显式处置。
- [x] T014 校验方法分母、唯一 ID、矩阵行数和状态总数。
- [x] T015 生成 WP-02 验证报告。

## WP-03 P0 差分框架

- [x] T016 定义 Java/Rust Adapter 输入输出协议和 fixture 格式。首批协议与可运行入口见 `tools/compat/README.md`。
- [ ] T017 先选 common HTTP/错误处理、pay 签名/序列化、cp 会话存档三个可控场景。
  - [x] 2026-11-11 首批 27 个真实双运行时样本：21 一致、6 差异（历史快照）。
  - [x] 2026-11-12 扩展为 30 个样本：24 个 JSON 值一致、3 个仅异常类别一致、3 个旧非 fallible API 差异。整体命令仍以 1 退出；详见 `docs/verification/wp03-common-error-2026-11-12.md`。
  - [x] 2026-11-13 增加独立真实 HTTP Runner：WxJava/WxRust MiniApp POST + 40001 Token 刷新/单次重放；修复 Content-Type 后请求轨迹严格一致 1/1。详见 `docs/verification/wp03-common-http-2026-11-13.md`。
  - [x] 2026-11-13 第二轮真实 HTTP 差分：GET Query 顺序（含 40001 刷新重放）、HTTP 503、HTTP 200 空 Body；修复后四场景 4/4 等价（2 个严格 JSON+轨迹，2 个错误类别+轨迹）。详见 `docs/verification/wp03-common-http-get-status-2026-11-13.md`。
  - [x] 2026-11-13 第三轮真实 HTTP 差分：Client 级 default Headers（Java requestInterceptors / Rust 注入 reqwest::Client）、SimplePost XML 原文返回、SimplePost 空 Body -99；三项新增场景 RED 3/3 → GREEN 后总计 7/7 EQUIVALENT。详见 `docs/verification/wp03-common-http-client-xml-2026-11-13.md`。
  - [x] 2026-11-14~15 执行动态 Request Interceptor 真实 Loopback：Java/Rust 分别向 Token GET 注入 `token-hook-v1`，API GET 注入 `api-hook-v1`；RED 1 DIFFERENT → GREEN 1 EQUIVALENT。
  - [ ] Multipart 两类原始请求严格比较仍 DIFFERENT（Apache/Reqwest 随机 boundary，Common Upload 还有字段 Header 大小写差异）；独立 MIME 解析确认字段序列、文件名、Content-Type、文件字节及额外文本字段 SHA256 一致。Binary 下载字节一致，但 Java File vs Rust Vec<u8> 的方法契约尚不一致。详见 `docs/verification/wp03-dynamic-interceptor-multipart-binary-2026-11-15.md`。
  - [ ] Pay 序列化与 CP Native 会话存档尚未覆盖。
- [ ] T018 实现语义比较器，覆盖 JSON/XML、缺省值、错误和请求副作用。
  - [x] JSON/XML、缺失/null/空串、异常类别与基础设施边界的比较器/Runner 自测 21 项通过。
  - [x] 真实 HTTP 轨迹比较覆盖 Method、Raw Target、Content-Type、Client default Headers、Body、响应 Status/Content-Type/Body、顺序及 Token/API 次数；POST/GET/503/GET 空/Headers/XML POST/POST 空共 7 场景，RED→GREEN。错误场景按显式错误类别、双方均提供时的 error_code、完整轨迹比较，原始异常类名/消息保留。
  - [ ] 动态 Interceptor 的更广泛语义（签名、冲突、重放及多业务模块）、单次请求 Header 覆盖、3xx/1xx、Token endpoint 非 2xx、GET XML、XML 声明/前导空白/业务 schema、Multipart 原始字节/随机 boundary、Binary File 生命周期、压缩/流式响应与其他请求副作用仍未完成。
- [x] T019 首批至第五轮失败/通过 fixture、源/执行结果 SHA 与复现命令已固化。Common Error 仍保留 3 项旧 API 差异（差分退出码 1）；独立 HTTP Runner 的 7 个 POST/GET/503/空响应/Headers/XML 场景修复后 7/7 等价。见 `docs/verification/wp03-common-error-2026-11-12.md`、`docs/verification/wp03-common-http-2026-11-13.md`、`docs/verification/wp03-common-http-get-status-2026-11-13.md`、`docs/verification/wp03-common-http-client-xml-2026-11-13.md`。

## WP-04 P0 模块行为收口

- [ ] T020 Common：HTTP、Token 并发刷新、错误传播、Session。
  - [x] P0-01 修复六个业务 crate 的统一管线与通用 RequestExecutor 路径：Token 失效后重新获取 Token、重建 URL、单次重放。Common/Channel/Store 定向回归通过。详见 `docs/verification/P0-01-token-refresh-replay-2026-11-09.md`。
  - [x] P0-02：WxError 三项 Gson 字段类型转换由真实差分 RED→GREEN；新增不破坏旧 API 的 `try_from_json`/`try_from_json_with_type` 和结构化错误类别。原 `Self` API 对非法 JSON/null/array 的三项差异保留为 DIFFERENT，方法矩阵 VERIFIED=0。
  - [x] 小程序投诉查询 Mock Token endpoint 回归固定为本地，避免真实 Token 刷新时错误访问外网；保留业务错误码断言。
  - [x] P0-03：Common Pipeline Text POST 按 Java HttpComponents 设置 `Content-Type: application/json; charset=UTF-8`；Bytes 不推断文本 Content-Type。真实 MiniApp loopback 双运行时重放轨迹 1/1 等价。全 Workspace 5018 passed、0 failed、1 ignored；Common 单包行覆盖率 93.11%。
  - [x] P0-04：MiniApp GET Query 顺序对齐 Java（先 access_token，后 queryParam）；包含 40001 刷新与 T2 单次重放，真实请求轨迹严格一致。
  - [x] P0-05：Common Pipeline 在 JSON 解析前拒绝 HTTP status >= 300，避免 503 + errcode=0 被误判成功；与 Java HttpResponseException 包装后的 Runtime 错误类别及请求轨迹一致。
  - [x] P0-06：HTTP 200 空 Body 不再转换成 WxError(-99)，改为 Runtime response failure；与 Java NullPointerException 的错误类别和请求轨迹一致。
  - [x] P0-04~06 完整验证：32 Python Harness、11 Common Pipeline、5021 Workspace passed / 0 failed / 1 ignored；Common 单包行覆盖率 93.03%、函数 72.81%、Region 91.02%。详见 `docs/verification/wp03-common-http-get-status-2026-11-13.md`。
  - [x] P0-07：对齐 Java HttpClient requestInterceptors 的静态默认 Header 能力；Rust 新增可注入 reqwest::Client 的 new_arc_with_http_client，原 new_arc 保持兼容。真实 Token GET 与 API GET 均验证两个自定义 Header 的值。
  - [x] P0-08：对齐 SimplePostRequestExecutor 的精确 XML 快速路径与空 Body 行为；Text POST Body 以 `<xml>` 开头时原文返回，空响应返回 -99 /「无响应内容」。GET 空 Body 继续为 Runtime failure。
  - [x] P0-07~08 三个新增真实 HTTP 场景 RED 3/3、GREEN 后七场景 7/7 等价；最终完整验证与覆盖率见 `docs/verification/wp03-common-http-client-xml-2026-11-13.md`。
  - [x] P0-09 单条动态 HTTP Request Interceptor Token/API 双路径 RED→GREEN，1/1 严格轨迹一致；不扩展宣称为所有拦截器语义完成。
  - [ ] P0-10 Multipart Media/CommonUpload：语义字段与文件字节已独立复核一致，Raw Wire 仍 DIFFERENT，保留原始 boundary 与 Header；不将场景提前标为 VERIFIED。
  - [ ] P0-11 Binary 下载完整兼容仍未收口：自定义 API Host 和文件字节对齐；新增 `get_media_file()->PathBuf` 文件语义适配，并保留 `get_media()->Vec<u8>`。真实双运行时成功文件、缺少 Content-Disposition、HTTP 503 三场景分别按显式文件语义/错误类别比较 3/3 EQUIVALENT；随机文件名严格结果不同、Java deleteOnExit 与 Rust 手动清理不同，外部 trait 实现者的源码兼容仍待审。见 `docs/verification/wp03-media-file-compat-2026-11-15.md`。
  - [x] 2026-11-15 修复 `WxMediaUploadResult` nullable/蛇形或驼峰字段映射引起的 MiniApp/CP 调用和测试编译错误：`cargo check --workspace --all-targets --offline --locked` 通过；40 项 Python Harness 与 18 项受影响定向 Rust 测试通过。完整 Workspace 首轮因 E 盘空间不足失败，但 D 盘独立 Target 的全量回归已实际执行并通过 `5025 passed / 0 failed / 1 ignored`（215 组，日志位于 `D:\\WxRust-Phase1-validation-20261115\\workspace-tests.log`），严格 Clippy `--all-targets --all-features -- -D warnings` 也已实际通过；最后整理示例类型别名后再次全量回归仍为 `5025 passed / 0 failed / 1 ignored`，具体 SHA256/日志见 `docs/verification/wp03-dynamic-interceptor-multipart-binary-2026-11-15.md`。
  - [ ] 继续验证并发刷新、取消、超时、跨实例缓存一致性和 Java/Rust 差分。
  - [x] 2026-11-15 文件式下载 TDD 与真实差分 RED→GREEN：Python Harness 44/44、文件名解析 3/3、Workspace 5028 passed / 0 failed / 1 ignored（215 groups），fmt/check/严格 Clippy 通过；机器来源与原始 SHA 文件见 `docs/verification/wp03-media-file-compat-2026-11-15.json`。  - [ ] P0-12 MiniApp 文件式下载取消安全：临时文件在成功返回前由下载操作独占所有；下载错误或 Future 取消必须关闭句柄并删除部分文件及专属目录；成功返回 PathBuf 后由调用方负责清理。先建立真实 RED，再实现 RAII 清理并验证取消、正常成功及错误路径。  - [ ] P0-13 将 `get_media_file` 设为兼容性可选 Trait 方法：提供默认明确的 unsupported 错误，确保已有下游 `WxMaMediaService` 实现不因新增文件接口而必须修改；内置实现仍覆盖该方法。
- [ ] T021 Pay：V2/V3 签名、证书、AES-GCM 回调验签、幂等。
  - [x] PAY-SIGN-UTF16：Map key 的 UTF-16 顺序修复；两项 golden 回归 RED→GREEN，11 个签名场景双运行时一致。
  - [ ] 上述结果不代表 V3/证书/通知/幂等完成。
- [ ] T022 CP：授权隔离、消息路由、会话存档；Finance 未支持能力保持明确标记。

## WP-05 P1/P2 与上游同步

- [ ] T023 P1：MP、MiniApp、Open、Channel、Store。
- [ ] T024 P2：AISpeech、Qidian。
- [ ] T025 对照 WxJava 4.8.6.B 建立独立增量账本。
- [ ] T026 对所有新增/修改/删除 API 与行为修复建立同步状态。
- [ ] T027 更新总体验收报告；未验证项不得标记完成。

## 执行约束

- 每个 WP 独立验证、独立记录。
- 不覆盖既有工作区改动。
- 不推送、不发布，除非用户另行明确要求。
