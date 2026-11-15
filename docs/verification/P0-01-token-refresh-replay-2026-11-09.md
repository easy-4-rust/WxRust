# P0-01：Token 失效后必须刷新再重放

日期：2026-11-09

## Java Oracle 语义

WxJava 4.8.4.B 的 MP/MiniApp/CP/Channel 执行链在收到 40001/40014/42001 时：

1. 对 Token 锁加锁。
2. 仅当配置中的当前 Token 仍等于本次请求使用的 Token 时才置过期。
3. 若 `autoRefreshToken && !doNotAutoRefresh`，递归再次执行。
4. 递归执行再次调用 `getAccessToken(false)`，因此会从 ConfigStorage 获取并发刷新后的 Token。
5. 使用新 Token 构造 URL，最多自动重放一次。

## WxRust 原问题

原 `wx-rust-common::pipeline::execute_pipeline` 在 token invalid callback 后沿用第一次请求的 `ctx.access_token` 和 URL。服务层虽然已使缓存 Token 过期，但管线重放未再次调用服务层 Token 获取方法，导致自动重放并未真正刷新 Token。

该路径被 MiniApp、MP、CP、Channel、Store、Qidian 的统一 GET/POST Service 执行器使用。

## 修复

- 新增 `execute_pipeline_with_refresh`，保留旧 `execute_pipeline` 兼容入口供没有刷新回调的测试/特殊调用。
- 新入口在收到 Token 失效后先执行原有加锁比对/置过期回调，再调用服务层 `get_access_token()`，成功后重建 URL 并只重放一次。
- 六个业务 crate 的管线 Service 调用均切换至新入口：MiniApp、MP、CP、Channel、Store、Qidian。
- 同时修复上述六个 crate 的通用 `execute_internal` RequestExecutor 路径：将 Token URL 构造移入重试循环，失效后释放锁、重新获取 Token，再构造新 URL；覆盖 multipart、二进制/自定义 RequestExecutor 等未走统一管线的请求。
- CP CorpGroup（42009 corp_access_token）与 CP Third-Party（42009 suite_access_token）原实现已在每次循环中重新获取 Token 并重建 URL；WxOpen component token 路径也已正确刷新并重建 URL，本次不重复改写。
- 若 Token 刷新本身失败，返回刷新错误，不以旧 Token 重试。
- 当 `auto_refresh_token=false` 时仍执行失效回调，但不刷新、不重放，保留 Java 配置语义。

## 回归测试

- Common：`token_invalid_refreshes_token_before_single_replay`，断言首次 URL 使用 T1、刷新回调仅调用一次、第二次 URL 使用 T2、总 HTTP 请求恰为 2。通过。
- Channel：`errcode_nonzero_throws` fixture 增加专用 Token endpoint，模拟 40001 → 成功刷新 Token → 第二次 API 仍返回 40001；断言最终保留 API 错误码且请求数为 3。通过。
- Channel generic executor：`generic_executor_refreshes_token_before_single_replay`，通过相同的 40001 → 刷新 → 重放路径，断言请求数为 3。通过。
- Store pipeline：`errcode_nonzero_throws`，同样断言请求数为 3。通过。
- 最终 `cargo check --all-targets`：通过，0 warnings / 0 errors。
- 最终 `cargo clippy --workspace --all-targets --all-features -- -D warnings`：通过，0 warnings / 0 errors。
- 统计更正（2026-11-11）：此前 1298 来自工具截断日志摘要，不是全工作区测试总数，不应作为全量验收数量引用。WP-03 在当前工作区用完整日志重新统计：`cargo test --workspace --offline --locked` 为 5007 passed、0 failed、1 ignored，详见 `wp03-differential-2026-11-11.json`。

## 尚未完成

- 尚未完成六个业务模块所有 token refresh 并发/取消/跨实例行为差分。
- 尚未与 Java Maven 测试使用相同 Mock HTTP Fixture 执行完整双运行时差分。
- 尚未证明 Redis 多实例 Token SingleFlight。
