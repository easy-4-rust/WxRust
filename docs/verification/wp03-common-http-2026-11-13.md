# WP-03 / P0 Common HTTP POST + Token Replay 真实差分报告

日期：2026-11-13

## 结论

固定 WxJava 4.8.4.B Oracle 与 WxRust 当前工作树，通过本地 loopback HTTP 服务对两个 SDK 的真实 MiniApp POST 执行链进行差分。**初始差分发现一项实际请求头差异；修复后 1/1 场景严格等价。**

- Oracle：WxJava commit `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`。
- Rust：分支 `feature/wxjava-1to1-phase1`，基线 HEAD `bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`，工作树仍为 dirty（保留此前未提交改动）。
- 场景：`http-post-token-replay` / `miniapp.http.post-replay`。
- 初始状态：`DIFFERENT 1/1`。
- 修复后：`EQUIVALENT 1/1`，请求轨迹严格 JSON 相等。
- 全程仅访问 `127.0.0.1`，没有真实微信 API、账号或支付请求。

这是一条真实 SDK 执行链的**场景级**认证，不代表 Common 模块所有 HTTP 方法、所有平台或全部 57,440 个 Java 声明方法完成一比一迁移。方法矩阵继续保持 `VERIFIED=0`。

## 1. 真实执行路径

### WxJava 4.8.4.B

- `WxMaServiceImpl` 继承 `WxMaServiceHttpComponentsImpl`。
- 默认 HTTP backend 为 Apache HttpComponents。
- POST 使用 `HttpComponentsSimplePostRequestExecutor.execute`。
- 固定源码将字符串请求体包装为：

```java
new StringEntity(
    postEntity,
    ContentType.APPLICATION_JSON.withCharset(StandardCharsets.UTF_8)
)
```

即 HTTP 请求头 `Content-Type: application/json; charset=UTF-8`。

- `BaseWxMaServiceImpl.executeInternal` 负责 Token 注入、40001 失效处理、刷新 Token 和单次重放。

### WxRust

- `WxMaService::post`
- `execute_post_via_pipeline`
- `execute_pipeline_with_refresh`
- Common `execute_pipeline_inner`
- `ReqwestTransport`

差异根因位于 Common Pipeline 构造 `TransportRequest` 时，原先对所有请求都固定使用 `headers: vec![]`。Reqwest 本身不会根据任意原始文本 Body 自动推断出 Java 的 JSON Content-Type。

## 2. 共享场景

Fixture：`tools/compat/http/fixtures.jsonl`。

所有参数均为合成值：

- app ID：`fixture-app`
- secret：`fixture-secret`
- Token endpoint：本地 `/cgi-bin/token`
- API path：`/cgi-bin/compat/http-probe?trace=wp03`
- POST Body：`{"probe":"http-post-replay","sequence":1}`
- Token 响应：依次返回 T1、T2。
- API 响应：第一次返回 `errcode=40001`，第二次返回 `errcode=0`。

Java 和 Rust Adapter 都调用各自真实 SDK Service API，只有 HTTP 服务端是测试桩；服务端实际记录接收到的 Method、Raw Target、Content-Type、Body 和请求顺序。

### 两侧实际请求轨迹

| 顺序 | 请求 | Token / 结果 |
|---:|---|---|
| 1 | GET `/cgi-bin/token?...appid=fixture-app&secret=fixture-secret` | 返回 T1 |
| 2 | POST `/cgi-bin/compat/http-probe?trace=wp03&access_token=T1` | 40001 |
| 3 | GET `/cgi-bin/token?...appid=fixture-app&secret=fixture-secret` | 刷新为 T2 |
| 4 | POST `/cgi-bin/compat/http-probe?trace=wp03&access_token=T2` | 0，成功 |

两次 API POST 的 Body 完全相同：

```json
{"probe":"http-post-replay","sequence":1}
```

两次 API POST 的 Content-Type 完全相同：

```text
application/json; charset=UTF-8
```

最终两侧均返回：

```json
{"errcode":0,"errmsg":"ok","data":{"accepted":true}}
```

Token endpoint 各调用 2 次，业务 API 各调用 2 次；没有额外重试。

## 3. RED → GREEN

### RED：真实差分

初始 Java/Rust 双运行时执行都成功，URL、Method、Body、返回值和 Token 重放次数相同，唯一差异是：

| 字段 | Java | Rust 原实现 |
|---|---|---|
| API POST Content-Type | `application/json; charset=UTF-8` | 缺失 |

初始差分报告保留在 `artifacts/phase1-diff/http-post-replay-red-20261113-02/report.json`，结果为 `DIFFERENT 1/1`。该失败没有加入白名单，也没有在比较器中忽略 Header。

### Rust 定向测试 RED

在 `crates/wx-rust-common/tests/execute_pipeline_test.rs` 的 Text POST 测试中增加请求头断言，先运行后实际失败：

```text
left: []
right: [("Content-Type", "application/json; charset=UTF-8")]
```

### 最小修复

只修改 `crates/wx-rust-common/src/pipeline/mod.rs`：

- `TransportBody::Text`：添加 `Content-Type: application/json; charset=UTF-8`。
- `TransportBody::None`：继续 GET，不添加该请求头。
- `TransportBody::Bytes`：继续 POST，不推断文本/JSON Content-Type。
- Token 刷新、失效回调、重试次数、URL 构造、响应解析均未改变。

定向回归 RED → GREEN：1 passed。整个 `execute_pipeline_test`：8 passed，0 failed。

### GREEN：真实双运行时

重新 clean 构建固定 Java MiniApp 模块，重新构建 Rust MiniApp Adapter，再次执行同一 Fixture。结果：

- `EQUIVALENT 1/1`。
- 两侧完整请求轨迹严格相等。
- 两次 Token 请求、两次 API 请求。
- 两侧最终响应体完全相同。

最终差分证据：`artifacts/phase1-diff/http-post-replay-green-20261113-02/report.json`。

## 4. 工程验证

完整验证运行：`artifacts/phase1-diff/http-validation-20261113-01/`。

| 验证 | 结果 |
|---|---|
| Python 比较器 / Runner 自测 | 26 passed |
| Common Pipeline 定向测试 | 8 passed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo check --workspace --all-targets --offline --locked` | exit 0 |
| 全 Features 严格 Clippy | exit 0 |
| `cargo test --workspace --offline --locked` | **5018 passed / 0 failed / 1 ignored** |
| Workspace 测试 summary groups | 214 |
| `cargo llvm-cov --package wx-rust-common --summary-only --offline --locked` | exit 0 |

本轮 Common 单包覆盖率：

- Lines：93.11%
- Functions：72.30%
- Regions：91.01%

覆盖率仅属于 `wx-rust-common`，不代表整个 Workspace。

最终 HTTP 差分源集合 SHA256 与完整验证前后 SHA256 一致：

`682a43bb309cec21e25c51f2f70eca5b034b82e13c16d5403386dcdd9d5ab5c0`

机器证据：`docs/verification/wp03-common-http-2026-11-13.json`。可携带的原始 JSONL、RED/GREEN 报告和验证日志：`docs/verification/wp03-http-evidence-2026-11-13/`。

Runner 防覆盖验证：对预先包含 marker 文件的输出目录运行差分命令，按预期返回退出码 2；marker 原样保留，未生成/覆盖 report。该结果记录于机器摘要 `runner_guard`。

## 5. 本轮未认证的范围

以下不能由本次 POST 场景推导为已完成：

- 普通 API GET 请求的完整双运行时差分（本场景仅真实覆盖 Token GET endpoint）。
- 调用方自定义任意 Headers；当前 `WxMaService::post` 场景验证的是 SDK 默认 JSON Content-Type。
- 非 2xx HTTP Status、空 Body、XML 响应及错误消息/异常传播全矩阵。
- Multipart、二进制上传/下载、流式传输。
- 并发 Token 刷新、任务取消、超时、跨实例缓存一致性、Redis / 多进程 single-flight。
- CP Finance Native 会话存档、Pay V3 证书/回调和其他模块全部行为差异。

这些项目继续保留未验证状态；不更新方法矩阵 `VERIFIED`。

## 6. 下一步

1. Common：对 GET、非 2xx、空响应、XML、Header/重试副作用继续建立共享真实 Fixture。
2. Common Token：独立验证并发刷新、取消、超时和跨实例一致性。
3. Pay：V3 RSA、证书、AES-GCM 回调验签和幂等。
4. CP：授权隔离、消息路由和 Finance Native 会话存档。

本轮没有 commit、push、merge 或 publish；所有既有未提交改动均保留。
