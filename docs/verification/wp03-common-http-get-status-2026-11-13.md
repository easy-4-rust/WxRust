# WP-03 / P0 Common HTTP 第二轮：GET、非 2xx 与空响应真实差分

日期：2026-11-13
Oracle：WxJava 4.8.4.B，commit `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`
Rust 分支：`feature/wxjava-1to1-phase1`，HEAD `bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`

## 结论

继续复用真实 MiniApp Service + 本地 loopback HTTP Server 的双运行时 Runner，新增 GET Query 顺序、HTTP 非 2xx、空响应三个场景，并保留此前 POST + Token Replay 场景。

最终 4 个场景全部通过：

- 2 个场景按完整 JSON 结果与完整服务器请求轨迹严格比较：POST + Token Replay、GET + Query 顺序。
- 2 个异常场景按明确的跨语言错误类别、完整服务器请求轨迹、响应状态/Body、请求顺序和调用次数比较：HTTP 503、HTTP 200 空响应。
- 对异常场景，Java 与 Rust 的异常类名和异常消息仍保留在原始报告中；类别一致不表示跨语言异常类型相同。
- 所有流量只访问 `127.0.0.1`，没有真实微信 API、账号或支付请求。
- 方法矩阵继续 `VERIFIED=0`，这只是四个明确场景的证据，不是 Common 模块整体认证。

## 1. 真实执行链

### WxJava

GET 真实执行顺序由固定源码确认：

1. `BaseWxMaServiceImpl.executeInternal()` 先构造带 `access_token` 的 URL。
2. `SimpleGetRequestExecutor.execute()` 再追加调用方 `queryParam`。
3. 默认 MiniApp 实现使用 `WxMaServiceHttpComponentsImpl` / Apache HttpComponents。

HTTP 状态行为：

- Apache `AbstractHttpClientResponseHandler` 对 HTTP status >= 300 抛出 `HttpResponseException`。
- `BaseWxMaServiceImpl` 将该 IOException 包装为 `WxRuntimeException`。
- HTTP 200 空响应时，Gson 没有返回有效的 `WxError` 对象，后续访问错误码触发 `NullPointerException`。

### WxRust

GET 执行链：

`WxMaService::get` → `execute_get_via_pipeline` → `execute_via_pipeline` → Common `execute_pipeline_with_refresh_and_query` → `execute_pipeline_inner` → Reqwest Transport。

此前 GET Helper 先追加 query，再由 Common Pipeline 注入 Token；与 Java 的追加顺序相反。Common Pipeline 原先也没有拒绝非 2xx status，空 Body 则被 `WxError::from_json` 转成 -99。

## 2. 四个共享真实场景

共享 Fixture：`tools/compat/http/fixtures.jsonl`。

| 场景 | 输入/触发 | Java 真实行为 | 修复后 Rust | 验收 |
|---|---|---|---|---|
| POST Token Replay | T1；API 首次 40001；刷新 T2 | GET Token、POST、GET Token、POST | 完全相同 | 严格 JSON + 轨迹一致 |
| GET Query Order | base=1，queryParam=trace=wp03&scene=get；T1 返回 40001 后刷新 T2 | Token 在前、queryParam 在后；单次重放 | 顺序及重放相同 | 严格 JSON + 轨迹一致 |
| GET HTTP 503 | API 返回 503，Body 的 errcode=0 | `WxRuntimeException`，cause 为 `HttpResponseException` | `WxErrorException::Runtime`，类别 http_status | 类别 + 轨迹一致 |
| GET Empty Body | API 返回 200，Body 为空 | `NullPointerException`，类别 empty_response_runtime | `WxErrorException::Runtime`，类别 empty_response_runtime | 类别 + 轨迹一致 |

### GET Query 顺序的实际差异

初始 RED 时，两侧真实收到的请求目标分别为：

Java：

```text
/cgi-bin/compat/http-probe?base=1&access_token=T1&trace=wp03&scene=get
```

旧 Rust：

```text
/cgi-bin/compat/http-probe?base=1&trace=wp03&scene=get&access_token=T1
```

这不是参数集合差异，而是 wire URL 的实际顺序差异。修复后 T1 和 T2 两次 GET 都与 Java 顺序一致。

### 非 2xx 与空响应的比较边界

HTTP 错误场景使用显式 `comparison: "http_error"`。比较器要求两端都真实返回 exception，并检查：

- 错误类别；
- 完整服务器请求轨迹；
- 实际 HTTP status 与 response body；
- Token/API 请求次数。

它不会把 Java/Rust 异常类型、消息强制改写成相同值。原始 Java/Rust 类型与消息均保留。HTTP 503 修复后，Java 仍是 `WxRuntimeException(HttpResponseException)`，Rust 是 `WxErrorException::Runtime`；两侧错误类别和实际 HTTP 轨迹相同。

## 3. RED → GREEN

### GET Query

- RED：保留旧 Rust query-before-token URL 构造，真实双运行时 GET + 40001 + Token 刷新/单次重放场景为 DIFFERENT。
- 修复：新增 Common `execute_pipeline_with_refresh_and_query`；Pipeline 每次请求（包括 Token 刷新后重放）均先注入 Token，再追加 queryParam。
- GREEN：GET 请求目标顺序、T1/T2 重放、响应及请求次数全部一致。

### HTTP 非 2xx

- RED：Java 收到 HTTP 503 后抛 `HttpResponseException` 并由 `WxRuntimeException` 包装；旧 Rust 忽略 `TransportResponse.status`，解析 Body 中 `errcode=0` 并作为成功返回。
- 修复：Common Pipeline 在业务 JSON 解析前检查 `resp.status >= 300`，返回 `WxErrorException::Runtime(WxRuntimeError)`。
- GREEN：HTTP 503 错误类别、请求轨迹、状态码、Body 和请求次数一致。

### 空响应

- RED：Java 空响应触发 NullPointerException；旧 Rust 返回 -99 的业务错误对象。
- 修复：Common Pipeline 在 JSON 解析前检测空 Body，返回 Runtime response error。
- GREEN：两端均为 runtime response failure 类别；服务器轨迹、HTTP 200 和空 Body 一致。

## 4. 代码修改

- `crates/wx-rust-common/src/pipeline/mod.rs`
  - 新增 `execute_pipeline_with_refresh_and_query`，保留既有 `execute_pipeline_with_refresh` 兼容入口。
  - 新增 token URL 后追加 GET queryParam 的统一构造，Token 刷新后重放复用相同规则。
  - HTTP status >= 300 返回 Runtime error。
  - 空响应 Body 返回 Runtime error，不再转换成 WxError(-99)。
  - 保留 Text POST 的 JSON UTF-8 Content-Type 修复；Bytes 不推断文本 Content-Type。
- `crates/wx-rust-miniapp/src/api/impl/base_wx_ma_service_impl.rs`
  - GET 不再预先拼接 queryParam，改由 Common Pipeline 在 Token 之后追加。
- `crates/wx-rust-common/tests/execute_pipeline_test.rs`
  - 增加 GET 参数顺序、非 2xx 和空 Body 定向回归；当前 Pipeline 测试 11/11 通过。
- `tools/compat/run_http_diff.py`、Java/Rust HTTP Adapters
  - 支持响应项字符串或 `{status, body}`；
  - 记录真实响应 status/body；
  - 错误结果保留两侧真实异常类型、类别和请求轨迹。
- `tools/compat/compare.py`
  - 新增显式 `http_error` 比较模式，拒绝 value/exception 混比、缺少类别、缺少 trace/call count 或轨迹差异。

## 5. 完整工程验证

完整日志：`artifacts/phase1-diff/http-get-status-validation-20261113-01/`。

| 验证 | 结果 |
|---|---|
| Python Harness | 32 passed |
| Common Pipeline 定向测试 | 11 passed / 0 failed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo check --workspace --all-targets --offline --locked` | exit 0 |
| 全 Features 严格 Clippy | exit 0 |
| `cargo test --workspace --offline --locked` | **5021 passed / 0 failed / 1 ignored** |
| Workspace summary groups | 214 |
| Common 单包 llvm-cov | exit 0 |

`wx-rust-common` 单包覆盖率：

- Lines：93.03%（4115 total / 287 missed）
- Functions：72.81%（217 total / 59 missed）
- Regions：91.02%（4532 total / 407 missed）

覆盖率只针对 `wx-rust-common`，不是整个 Workspace。

最终源集合 SHA256（运行前后相同）：

`446c15e11cfb4b23295d8b51855c2700bdae9dfe33c247c0bc212871285e791b`

## 6. 可复现证据

- 机器摘要：`docs/verification/wp03-common-http-get-status-2026-11-13.json`。
- 可携带原始输入、RED/GREEN 输出、报告、验证日志：`docs/verification/wp03-http-get-status-evidence-2026-11-13/`。
- GET Query RED：`artifacts/phase1-diff/http-get-query-replay-red-20261113-01/`。
- 503/空响应 RED：`artifacts/phase1-diff/http-status-empty-red-20261113-01/`。
- 四场景 GREEN：`artifacts/phase1-diff/http-status-empty-green-20261113-01/`。

复现命令：

```powershell
python -m unittest discover -s tools/compat -p 'test_*.py' -v
python tools/compat/run_http_diff.py --java-root D:\wxjava-phase1-oracle-20261111 --jdk-home D:\Java\jdk-17.0.12 --out artifacts/phase1-diff/a-new-empty-directory
cargo test --offline --locked -p wx-rust-common --test execute_pipeline_test
cargo test --workspace --offline --locked
```

## 7. 未验证范围

本轮不覆盖：

- GET query 编码及空/缺失 queryParam 的全部边界。
- 任意调用方自定义 Header。
- 3xx 重定向、1xx 信息响应。
- Token endpoint 自身非 2xx。
- XML、压缩响应、multipart、二进制上传/下载、流式 Body。
- 并发 Token 刷新、取消、超时、Redis/多进程 single-flight。
- CP Finance Native 会话存档、Pay V3 证书/回调。
- Common 全模块和 WxJava/WxRust 全方法一比一兼容。

方法矩阵继续 `VERIFIED=0`。本报告仅证明上述四个场景。

没有 commit、push、merge 或 publish；所有先前未提交工作均保留。

验证完成后仅补充了 `crates/wx-rust-common/src/pipeline/mod.rs` 的模块级文档注释，澄清 HTTP status/空 Body 与 GET Query 顺序；未改动可执行代码。随后重新运行 `cargo fmt --all -- --check` 与 `git diff --check`，均通过。完整 Workspace 测试所对应的源集合 SHA256 为 `446c15e11cfb4b23295d8b51855c2700bdae9dfe33c247c0bc212871285e791b`；文档注释更新后的当前源集合 SHA256 为 `d0b6e56feeeab7f5859de10f8b6b2561a16b4047321122cd1d3182cf5f26867e`，两个值均记录在机器摘要中。
