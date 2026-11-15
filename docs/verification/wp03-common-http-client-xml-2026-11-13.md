# WP-03 / P0 Common HTTP Client Headers 与 SimplePost XML 差分

日期：2026-11-13
Oracle：WxJava 4.8.4.B（固定 commit `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`）
Rust：`feature/wxjava-1to1-phase1` / `bc5dbb7`

## 结论

在既有真实 MiniApp HTTP loopback Runner 上新增三项场景：静态 HTTP Client 默认 Header、SimplePost XML 原文响应、SimplePost 空响应。新增场景 RED 3/3，修复后与此前四个场景合并 GREEN 7/7。

本轮验证的是静态 Client 级 default headers，不是任意动态 Java interceptor 或单次调用 Header override。XML 只验证 Java SimplePost 的精确 `<xml>` 前缀分支，不代表完整 XML 解析器/Schema 兼容。

## 1. 来源核对与范围

### Client Headers

固定 WxJava 的默认 HttpComponents Client Builder 支持共享 `requestInterceptors`；MiniApp Service 使用配置的共享 HttpClient 发出 Token 与 API 请求。公开的 `WxMaService.get/post` 方法没有 per-call Header map 参数。

因此本轮比较：
- Java：为真实 HttpComponents Client 配置 request interceptor，向 Token GET 和 API GET 加入两个合成 Header。
- Rust：给真实 Reqwest Client 配置 `default_headers`，通过注入 Client 的 Service 构造入口执行请求。
- Loopback Server 实际记录收到的 Header 值；不只比较客户端配置对象。

### SimplePost XML / 空响应

固定 Java `SimplePostRequestExecutor.handleResponse` 行为：
- 空响应 Body：抛出 `WxErrorException(-99, "无响应内容")`。
- Body 以精确字符串 `<xml>` 开头：直接返回原始响应字符串，跳过 JSON WxError 解析。
- 其他非空响应：继续 JSON WxError 解析。
- GET 不走此 XML 快速路径；GET 空响应仍按此前 Runtime response failure 处理。

## 2. RED → GREEN

| 新增场景 | RED 观察 | 修复后 |
|---|---|---|
| Client default Headers | Java Token/API 请求收到两个 Header，Rust 默认 Client 未发送 | Java/Rust Token GET 和 API GET 均收到相同 Header 值 |
| SimplePost XML | Java 返回 XML 原文；Rust 进入 JSON 解析并报 -99 | 两端原文返回一致 |
| SimplePost 空 Body | Java 返回 -99 /「无响应内容」；Rust 原来走 GET 通用空响应 Runtime 路径 | Rust Text POST 映射为 -99 /「无响应内容」 |

RED 报告：`artifacts/phase1-diff/http-custom-headers-xml-red-20261113-02/report.json`，3/3 DIFFERENT。

GREEN 报告：`artifacts/phase1-diff/http-custom-headers-xml-green-20261113-04/report.json`，总计 7/7 EQUIVALENT。

七个场景：
1. POST + Token 40001 刷新/单次重放；
2. GET Query 顺序 + Token 刷新/单次重放；
3. GET HTTP 503（Body 故意返回 errcode=0）；
4. GET HTTP 200 空 Body；
5. Client default Headers；
6. SimplePost XML 原文；
7. SimplePost 空 Body。

其中四个成功场景严格比较返回值与完整 wire trace；三个异常场景比较显式错误类别、若双方提供则比较 error_code、完整请求/响应轨迹及调用次数。两侧原始异常类名和消息保留，不伪装成同一异常类型。

## 3. Rust 实现

- `WxMaServiceImpl` 新增 `new_arc_with_http_client(config, client)`，用于注入预配置 Reqwest Client；原 `new_arc(config)` 继续保留。
- Common Pipeline 继续保留统一 HTTP 传输路径；Service 级 Client 注入不复制/模拟 SDK 请求逻辑。
- MiniApp SimplePost 的 Text Body 响应解析先处理空 Body：返回 `WxErrorException(-99, "无响应内容")`。
- 对非空响应，仅当 Body 以精确 `<xml>` 开头时原文返回；不做 trim、不把 XML 声明或前导空白视作等价前缀。
- GET 空 Body 行为保持 Runtime response failure，避免将 SimplePost 特例泄漏到所有 HTTP 请求。

## 4. 全量验证

完整日志目录：`artifacts/phase1-diff/http-client-xml-validation-20261113-01/`。

| 检查 | 结果 |
|---|---:|
| Python Harness | 36 passed |
| Common Pipeline 定向测试 | 13 passed |
| Workspace Tests | **5023 passed / 0 failed / 1 ignored** |
| Workspace summary groups | 214 |
| Cargo fmt --check | exit 0 |
| Cargo check workspace/all-targets/offline/locked | exit 0 |
| 全 Features 严格 Clippy | exit 0 |
| git diff --check | exit 0 |
| Common 单包 llvm-cov | exit 0 |

仅 `wx-rust-common` 单包覆盖率：
- Lines：92.94%（4122 total / 291 missed）
- Functions：72.81%（217 total / 59 missed）
- Regions：90.93%（4542 total / 412 missed）

这不是整个 Workspace 的覆盖率。

全量验证前后源集合 SHA256 一致：

`eddd877c5b6428ddd84db84c830035f8169dd6b093292f15d9fac394c709f2de`

## 5. 证据

- 机器摘要：`docs/verification/wp03-common-http-client-xml-2026-11-13.json`
- 本报告：`docs/verification/wp03-common-http-client-xml-2026-11-13.md`
- 可携带原始证据：`docs/verification/wp03-http-client-xml-evidence-2026-11-13/`
- 完整验证日志：`artifacts/phase1-diff/http-client-xml-validation-20261113-01/`

可复现：

```powershell
python -m unittest discover -s tools/compat -p 'test_*.py' -v
python tools/compat/run_http_diff.py --java-root D:\wxjava-phase1-oracle-20261111 --jdk-home D:\Java\jdk-17.0.12 --out artifacts/phase1-diff/a-new-empty-output-directory
cargo test --offline --locked -p wx-rust-common --test execute_pipeline_test
cargo test --workspace --offline --locked
```

## 6. 未验证范围

- 动态 Java request interceptor 逻辑、per-call Header override。
- 重复 Header 顺序、签名 Header 的动态刷新。
- 3xx/1xx、Token endpoint 非 2xx。
- GET XML。
- XML 声明、前导空白、业务 Schema、完整 XML parser 语义。
- multipart、binary、压缩/流式响应。
- Token 并发刷新、取消、超时、Redis/多进程 single-flight。
- CP Finance Native、Pay V3、整个 Common 或全模块方法级兼容。

方法矩阵仍为 `VERIFIED=0`。本报告仅证明以上七个明确场景。

没有 commit、push、merge 或 publish；保留既有所有未提交工作。
