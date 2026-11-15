# WxJava / WxRust 双运行时差分（WP-03 首批）

本工具实际执行 WxJava 固定源码的编译产物和 WxRust 当前源码，不用 Python 或另写的 Java 算法模拟 Oracle。它是场景级差分工具，不是全模块一比一认证。

## 固定范围

Oracle：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`（WxJava 4.8.4.B）。

首批 27 个共享 JSONL 样本：16 个 `common.error`（`WxError.fromJson`）、11 个 `pay.sign`（`SignUtils.createSign(Map,...)`）。Java Adapter 的 classpath 将固定 worktree 的 `target/classes` 放在依赖 JAR 前，并核验实际类加载路径。每次运行重新执行 Maven clean/package；`-DskipTests` 只表示上游 Surefire 套件未运行，不影响随后真实 SDK 方法调用。

Rust Adapter 位于 `crates/wx-rust-pay/examples/phase1_oracle.rs`，复用现有 common/pay 依赖，不增加 workspace crate 或三方依赖。2026-11-11 首批修复 Pay Map key 的 Java UTF-16 单元排序；2026-11-12 新增 WxError 的 Gson 基本类型转换和可失败解析接口。

## 运行

PowerShell 示例（使用 JDK 17，不依赖 PATH 中可能存在的 Java 8）：

```powershell
python -m unittest discover -s tools/compat -p 'test_*.py' -v
python tools/compat/run_diff.py --java-root D:\wxjava-phase1-oracle-20261111 --jdk-home D:\Java\jdk-17.0.12 --out artifacts/phase1-diff/my-new-run
```

输出目录必须不存在或为空；不得覆盖旧证据。Maven 首次构建可能下载依赖；Rust 使用 `--offline --locked`，依赖缺失属于环境失败。正常差分运行不连接微信、不发送支付请求，fixture 的 key 均为合成测试值。

退出码：`0` 所有指定样本一致；`1` 存在行为差异或未支持场景；`2` 环境、构建、协议、来源或稳定性校验失败。已知差异没有白名单，不能被自动变成通过。
## P0 Common HTTP POST / Token Replay 差分（2026-11-13）

与首批 Common Error / Pay Runner 分离，避免改变其 Oracle classpath 和历史证据。运行真实固定版本 WxJava MiniApp 默认 HttpComponents Service 与 WxRust MiniApp Service，双方各自向 `127.0.0.1` 的 loopback server 发送请求；服务器记录真实收到的 Method、Raw Target、Content-Type、Body、顺序和调用次数。

```powershell
python -m unittest discover -s tools/compat -p 'test_*.py' -v
python tools/compat/run_http_diff.py --java-root D:\wxjava-phase1-oracle-20261111 --jdk-home D:\Java\jdk-17.0.12 --out artifacts/phase1-diff/my-new-empty-http-run
```

共享场景 `tools/compat/http/fixtures.jsonl`：本地 Token endpoint 依次返回 T1/T2；API POST 第一次返回 40001，第二次返回成功。初始真实差分为 `DIFFERENT 1/1`，唯一差异是 Java 的 `application/json; charset=UTF-8` 与 Rust 原来缺失 Content-Type。Common Pipeline Text body 经测试先 RED 后修复；最终真实请求轨迹 `EQUIVALENT 1/1`，2 次 Token GET、2 次 API POST、同样的 URL/Body/响应和单次重放。

Java provenance 核验 Common `WxError` 与 MiniApp `WxMaServiceImpl` 的实际 class 加载目录。Runner 还记录固定 Java SHA、Rust HEAD/dirty、class/binary/source/fixture SHA，并拒绝覆盖非空证据目录。此场景不连接微信外网。

报告：`docs/verification/wp03-common-http-2026-11-13.md`；机器摘要：`docs/verification/wp03-common-http-2026-11-13.json`；可携带原始输出：`docs/verification/wp03-http-evidence-2026-11-13/`。

## P0 Common HTTP 第二轮：GET / 非 2xx / 空响应（2026-11-13）

沿用同一真实 MiniApp Service loopback Runner，新增共享 GET Query、HTTP 503、HTTP 200 空 Body 场景。GET Query fixture 包含 base query、caller queryParam、T1 返回 40001、刷新 T2 后单次重放；实际请求目标严格为 `base=1&access_token=Tn&trace=wp03&scene=get`。旧 Rust 顺序相反，真实 RED 后通过新增 `execute_pipeline_with_refresh_and_query` 修复。

HTTP 503 fixture 的响应 Body 特意返回 `errcode=0`：Java Apache ResponseHandler 仍因 status >= 300 抛出 `HttpResponseException` 并由 WxMa 包装成 `WxRuntimeException`；旧 Rust 忽略 status 并把 Body 当作成功。Common Pipeline 现在在 JSON 解析前拒绝 status >= 300。HTTP 200 空 Body 场景中，Java 实际抛 `NullPointerException`，旧 Rust 返回 -99；Rust 现在返回 Runtime response failure。

最终四个场景 4/4 等价：POST 与 GET 使用完整 JSON+wire trace 比较；503 和空响应使用显式 `http_error` 比较，只比较错误类别、完整 server trace 与调用次数，同时保留原始 Java/Rust exception type/message。不能据此声称异常类名或消息完全相同。

Python Harness 32 passed；Common Pipeline 11 passed；全 Workspace 5021 passed、0 failed、1 ignored；fmt/check/严格 Clippy 通过。仅 `wx-rust-common` 单包覆盖率：行 93.03%、函数 72.81%、Region 91.02%。

报告：`docs/verification/wp03-common-http-get-status-2026-11-13.md`；机器摘要：`docs/verification/wp03-common-http-get-status-2026-11-13.json`；可携带证据：`docs/verification/wp03-http-get-status-evidence-2026-11-13/`。

## P0 Common HTTP 第三轮：Client 默认 Headers / SimplePost XML 响应（2026-11-13）

本轮继续使用固定 WxJava 4.8.4.B 的真实 MiniApp 默认 HttpComponents Service 与 WxRust MiniApp Service；双方仍只访问各自绑定在 127.0.0.1 的 loopback server。新增三个真实场景，合并前两轮四个场景后共 7 个场景。

### P0-07：HTTP Client 级自定义 Headers

源码核验 WxJava DefaultHttpComponentsClientBuilder 支持配置 requestInterceptors，由共享 HttpClient 对 Token 与业务 API 请求统一追加 Header；WxMaService.get/post 本身没有每次调用独立传入 Headers 的参数。因此本次不虚构 per-call API，只验证静态 Client 默认 Header 的等价能力。

- Java：在真实默认 HttpComponents Builder 上配置 request interceptor。
- Rust：新增 WxMaServiceImpl::new_arc_with_http_client(config, client)，允许调用方注入预配置 reqwest::Client；原 new_arc 保持不变并继续创建默认 Client。
- Fixture 配置 X-WxRust-Probe: phase1 和 X-WxRust-Trace: wp03；双方 server 分别在 Token GET 和业务 GET 的实际接收端记录这两个 Header 的全部值。
- 初始 RED：Java 两次请求均收到两个 Header，旧 Rust 两次请求均未收到；真实差分 DIFFERENT。GREEN：Token 与业务请求的 Header 名和值均一致。

### P0-08：SimplePost XML 与空响应

固定 Java SimplePostRequestExecutor.handleResponse 源码确认：

1. Body 为空时抛出 WxErrorException(-99, "无响应内容")。
2. Body 以精确的 <xml> 前缀开头时，原样返回字符串，不调用 JSON WxError 解析。
3. 其他 Body 才进入 JSON errcode 解析。

真实共享 fixture 使用 HTTP 200 + Content-Type: text/xml; charset=UTF-8：

- XML 场景：Java 返回原始 XML；旧 Rust 尝试 JSON 解析并返回 -99。GREEN 后两侧返回的 XML 字符串和完整请求/响应轨迹严格相同。
- POST 空响应：Java 返回 WxErrorException，错误码 -99、消息「无响应内容」；旧 Rust 返回 Runtime response failure。GREEN 后错误类别、错误码、HTTP 轨迹和调用次数一致；异常类名仍按各语言原样保留。
- 该 XML 快速路径只适用于 SimplePost；本轮没有将它扩展到 GET，也没有实现通用 XML parser。

### RED / GREEN 与验证范围

- 三项新场景的真实双运行时 RED：artifacts/phase1-diff/http-custom-headers-xml-red-20261113-02，3/3 DIFFERENT。
- 合并之前四个 HTTP 场景后的 GREEN：artifacts/phase1-diff/http-custom-headers-xml-green-20261113-03，7/7 EQUIVALENT。
- 其中 4 个严格 JSON+完整 trace 场景：POST Token Replay、GET Query 顺序、Client 默认 Headers、POST XML 原文。
- 其中 3 个显式 http_error 场景：HTTP 503、GET 空响应、POST 空响应；比较错误类别、可用的 error_code、完整 trace 和 Token/API 调用次数，保留原始异常类型/消息。缺失的 error_code 是行为 DIFFERENT，不被当作 Harness 基础设施失败。
- 当前 Harness 自测 36 项，Common Pipeline 定向测试 13 项；最终全 Workspace 与严格 Clippy/check 结果以本轮完整验证日志为准。

边界：这证明的是两个静态 Client 默认 Header 的透传，不证明 Java 动态 interceptor 的任意逻辑、单次请求 Header 覆盖、重复 Header 排序、签名 Header 刷新或所有平台的自定义 Header 行为。XML 仅证明 SimplePost 对精确 <xml> 前缀的原文返回；XML 声明、前导空白、GET XML、业务 XML schema 均未验证。

报告：docs/verification/wp03-common-http-client-xml-2026-11-13.md；机器摘要：docs/verification/wp03-common-http-client-xml-2026-11-13.json；可携带 RED/GREEN 原始证据：docs/verification/wp03-http-client-xml-evidence-2026-11-13/。



## P0-11：Java File 下载语义对照（2026-11-15）

Rust 保留 `get_media(&str)->Vec<u8>` 便捷接口，额外提供 `get_media_file(&str)->PathBuf`：执行真正的 MiniApp SDK Token/HTTP 链，解析 `Content-Disposition`，分块写入独立 `wxma` 临时目录的文件。Java `getMedia(String)->File` 的 `deleteOnExit()` 与 Rust 调用者显式清理 **仍不完全等价**。

三份真实 Loopback Fixture：

- `http/media-download-file-fixture.jsonl`：Strict JSON RED（随机文件名与目录名不同）；显式 `file_semantics` GREEN，仍记录 `raw_value_equal=false`。
- `http/media-download-file-no-disposition-fixture.jsonl`：缺少响应头时，两侧真实 `-99` 和错误类别/完整请求轨迹一致。
- `http/media-download-file-http-503-fixture.jsonl`：修复 Rust 误将 503 保存成文件，Java/Rust 都在文件创建前返回 Runtime 错误，错误类别和轨迹一致。

比较模式 `file_semantics` 必须由 Fixture 显式声明，严格要求预期文件名前缀/扩展名、真实文件存在、预期字节内容、`wxma` 临时目录、Token/API 请求和完整 Wire Trace；**仅**不强制两次运行生成相同随机临时名。负向测试覆盖字节、存在性、扩展名、目录和请求轨迹差异，任何关键证据缺失 fail closed。

最新实际验证：Python Harness 44/44、Rust MiniApp 文件名解析 3/3、Workspace 5028 passed / 0 failed / 1 ignored，Fmt/Check/严格 Clippy exit0。详见 `docs/verification/wp03-media-file-compat-2026-11-15.md`、`docs/verification/wp03-media-file-compat-2026-11-15.json` 和 `docs/verification/wp03-media-file-evidence-2026-11-15/`。

这不覆盖 Multipart 原始 Wire 随机 boundary、所有文件名编码/文件生命周期或方法矩阵完整兼容；`VERIFIED=0` 不变。

## JSONL 协议 v1

输入行：

```json
{"id":"unique-case","operation":"common.error","comparison":"json","input":{"body":"{\"errcode\":0}"}}
```

输出行：

```json
{"id":"unique-case","operation":"common.error","result":{"kind":"value","value":{"error_code":0,"error_msg":null,"error_msg_en":null}}}
```

`result.kind` 支持 `value`、`exception`、`unsupported`、`adapter_error`。异常保留两侧实际类型和消息，不强制转成 `-99`。`unsupported` 永不计为一致，输出缺失/重复/多余及缺少操作名均阻断验收。每个 fixture 恰好需要两侧各一条输出。HTTP response fixture 可使用普通字符串（默认 status 200），或 `{"status":503,"body":"...","content_type":"text/xml; charset=UTF-8"}`；服务端轨迹记录实际请求/自定义 Header 与响应 status/content-type/body。

JSON 比较忽略对象 key 顺序，但保留数组顺序、缺失/null/空字符串、布尔/数字和整数精度差别。`error_category` 仅比较显式声明的异常类别；`http_error` 要求两侧真实返回 exception，并比较错误类别、双方均提供时的错误码、完整 HTTP 请求/响应轨迹和调用次数，同时保留原始类型/消息。XML 比较器忽略属性顺序但保留文本、子元素顺序；拒绝 DTD/实体声明。真实 HTTP SimplePost XML 原文返回场景已覆盖，但这不等于通用 XML parser 或 XML schema 兼容认证。

## 证据

每次运行保存 `fixtures.jsonl`、`java.stdout.log`、`rust.stdout.log`、编译日志与 `report.json`。报告记录固定 Java SHA、Rust HEAD/dirty 状态、输入与源文件集合 SHA256、Java class 与 Rust executable SHA256、真实类加载位置、步骤退出码及逐样本结果。运行前后的源文件摘要必须一致，避免把修改期间的旧结果当作当前结果。

原始日志/编译产物在 `artifacts/phase1-diff/`（不进入 Git）；可携带证据分别位于 `docs/verification/wp03-evidence-2026-11-11/`、`docs/verification/wp03-evidence-2026-11-12/`、`docs/verification/wp03-http-evidence-2026-11-13/`、`docs/verification/wp03-http-get-status-evidence-2026-11-13/` 和 `docs/verification/wp03-http-client-xml-evidence-2026-11-13/`，对应技术报告见各日期的 `wp03-*.md/json`。

## 已知差异与未覆盖边界

2026-11-11：基线 19/27 一致、8 项不同；修复 Pay Unicode key 顺序后 21/27 一致、6 项不同。

Common 未修复的六个样本：字符串 errcode、null errcode、数字 errmsg、非法 JSON、JSON null、JSON array。详见验证报告，不删除这些失败样本。

### 2026-11-12 第二轮补充（最新）

保留上述 2026-11-11 历史差分。第二轮在原有 27 个 Fixture 上修复字符串 `errcode`、null `errcode`、数字 `errmsg` 三项，27 个原样本变为 24 一致、3 差异；新增 3 个 `common.error.try` 场景，总数 30 个，最终 **24 个完整值一致 + 3 个仅错误类别一致 + 3 个旧接口行为差异**。整体差分仍返回 1，不等于 100% 兼容。

旧 `WxError::from_json` 和 `from_json_with_type` 返回 `Self`，非法 JSON、顶层 `null`、数组根节点仍返回 -99，和 WxJava 抛异常的行为不等价；新 `try_from_json`/`try_from_json_with_type` 返回 `Result`，通过 `WxErrorParseError` 显式区分错误。两端 Adapter 的 `comparison: "error_category"` 仅为三个明确声明的新增场景比较异常类别，并且要求 Java/Rust 两侧确实都返回 `kind: "exception"`。原始 Java/Rust 异常类名、消息均在证据里保留，不做强制归一化，也不会覆盖原有失败结果。

本轮附加验证：Python 自测 21 通过，Rust 全工作区 5018 通过、0 失败、1 忽略；**仅 wx-rust-common 单包**覆盖率行 93.20%、函数 72.30%、Regions 91.08%。请勿误认为整个 Workspace 的覆盖率。机器证据和原始 JSONL：`docs/verification/wp03-common-error-2026-11-12.json`、`docs/verification/wp03-evidence-2026-11-12/`，技术报告 `docs/verification/wp03-common-error-2026-11-12.md`。

本轮 Common 仅比较 `error_code/error_msg/error_msg_en`；不证明原始 JSON 字符串、Display/toString 和所有 Gson coercion 等价。Pay 只覆盖字符串值 Map 版本；不覆盖 Java null map value、Bean 反射签名、V3 RSA/证书/回调。

2026-11-13 已验证 MiniApp POST/GET、GET Query 顺序、HTTP 503、GET/POST 空响应、静态 Client 默认 Headers 和 SimplePost XML 原文返回七个真实 HTTP 场景，修复后 7/7 等价（4 严格值+轨迹，3 错误类别/错误码+轨迹）。仍未覆盖动态 request interceptor、per-call Header 覆盖、3xx/1xx、Token endpoint 非 2xx、GET XML、XML 声明/前导空白/业务 schema、multipart/binary、并发刷新/取消/超时、CP Finance Native 会话拉取/解密及真实账号联调。现有方法矩阵保持 `VERIFIED=0`；任何样本数都不能直接推导为完整方法或模块兼容认证。
