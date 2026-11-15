# P0 Common HTTP POST + Token Replay Differential Design

## Intent

Create one reproducible, no-external-network, real-SDK differential scenario for the shared HTTP execution path used by WxMaService POST calls. Compare pinned WxJava 4.8.4.B with the current WxRust MiniApp service, including the Common pipeline and Reqwest transport. The scenario must observe the HTTP request actually received by a local loopback server, not an adapter-side reimplementation of either SDK.

## Goals

- Exercise Java `WxMaServiceImpl.post()` through `BaseWxMaServiceImpl.executeWithRetry/executeInternal` and the default HttpComponents `SimplePostRequestExecutor`.
- Exercise Rust `WxMaService::post()` through `execute_post_via_pipeline`, `execute_pipeline_with_refresh`, and `ReqwestTransport`.
- Verify initial token fetch, API POST, 40001 invalidation, second token fetch, and exactly one replay.
- Compare method, raw request target (path + query), Content-Type, exact body, result body, request order and call counts.
- Persist shared fixtures, both raw traces, pinned Java provenance, Rust binary/source hashes and a machine-readable report.

## Non-goals

- No real WeChat credentials, API endpoints, payment requests or outbound network calls.
- No broad refactor of HTTP abstractions or executor APIs.
- No certification of GET, XML, multipart, binary streaming, arbitrary caller-supplied headers, non-2xx semantics, concurrency, cancellation, timeout or Redis/multi-process single-flight.
- No method-matrix VERIFIED promotion from a single scenario.

## Chosen approach

Use a dedicated HTTP differential runner and adapters under `tools/compat/http/`, leaving the already-working Common Error/Pay runner untouched. The Java adapter starts a JDK loopback HTTP server and calls the pinned SDK service. The Rust example starts a Tokio loopback TCP server and calls the existing Rust SDK service. Both consume the same JSONL scenario and emit the existing fail-closed `id/operation/result.kind` envelope. The runner builds the pinned Java MiniApp module and the Rust MiniApp example, verifies loaded Java class origins, runs both, compares exact normalized traces and saves immutable evidence.

This is preferred over extending `run_diff.py` because that runner's provenance and build scope are deliberately limited to Common + Pay; broadening it risks regressing existing evidence and mixes independent module build lifecycles. A mock-only pipeline test is insufficient because it cannot prove the actual SDK HTTP backend's headers and URL behavior.

## Scenario contract

- Bind only to `127.0.0.1` on an ephemeral port.
- Configure Java `apiHostUrl` and Rust `api_host_url` to the local server; configure each SDK's custom access-token URL to the same local server.
- Token endpoint: GET `/cgi-bin/token?grant_type=client_credential&appid=fixture-app&secret=fixture-secret`; return token T1 on first call and T2 on second.
- API endpoint: POST `/cgi-bin/compat/http-probe?trace=wp03&access_token=Tn`.
- Body: one fixed UTF-8 JSON string from the shared fixture.
- First API response: HTTP 200 with `errcode=40001`; second API response: HTTP 200 with `errcode=0`.
- Expected trace: token T1 → API T1 → token T2 → API T2; exactly 2 token calls and 2 API calls.
- Capture `Content-Type` for API requests, method, raw target, exact body and sequence. Ignore volatile backend-specific headers (Host, User-Agent, connection framing) while retaining the semantically relevant content type.

## Compatibility decision

The pinned Java default HttpComponents SimplePost executor creates a `StringEntity` with `ContentType.APPLICATION_JSON.withCharset(UTF_8)`. The Rust pipeline currently creates `TransportRequest { method: Post, headers: [], body: Text(..) }`. Treat the actual differential as the authority: if it confirms the expected Content-Type mismatch, the minimal fix is to attach `application/json; charset=UTF-8` for `TransportBody::Text` requests in the common pipeline. Do not add content type to binary `Bytes` bodies. The regression test must assert the actual `TransportRequest.headers` value and the dual-runtime trace.

## Acceptance

1. The unmodified Rust pipeline produces a real, reproducible difference if its request headers differ from Java.
2. A focused Common test watches the missing Content-Type assertion fail before the production fix.
3. The fixed pipeline test passes and the real Java/Rust trace becomes equivalent for the declared scenario.
4. All original Common Error/Pay evidence remains intact and unchanged.
5. Run `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --offline --locked`, strict all-feature Clippy, targeted tests and `cargo test --workspace --offline --locked`.
6. Save original run logs and evidence hashes. Keep the overall migration and method matrix status open; one scenario is not module-wide parity.
7. No commit, push or publish in this task.
