# P0 Common HTTP POST + Token Replay Differential Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove and close one real WxJava/WxRust HTTP POST execution-chain difference, including token refresh and exactly-once replay, against a loopback server.

**Architecture:** Add a dedicated differential runner so the existing Common Error/Pay runner and its evidence remain unchanged. Java calls the pinned WxMaServiceImpl with its default HttpComponents backend; Rust calls the current WxMaServiceImpl and Common pipeline/Reqwest transport. Both consume one shared JSONL scenario and report actual server-observed requests.

**Tech Stack:** Java 17, Maven, JDK HttpServer, Rust/Tokio/Reqwest, Python stdlib JSONL runner, existing WxJava 4.8.4.B Oracle.

**Spec:** `docs/superpowers/specs/2026-11-13-common-http-post-replay-design.md`

## Global Constraints

- Java Oracle SHA is `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`.
- Keep current branch `feature/wxjava-1to1-phase1` and all existing uncommitted files; do not reset, clean, switch branches or create a new worktree.
- No external network: bind only `127.0.0.1`; all tokens/secrets are synthetic.
- No new Rust crates or third-party dependencies.
- Do not alter the existing `tools/compat/run_diff.py` or prior WP-03 fixtures/reports.
- No commit, push, merge or publish.
- No method matrix VERIFIED promotion from scenario-level evidence.

## Review Focus

- Java's actual default HttpComponents backend may emit Content-Type differently from Rust Reqwest; capture the received header rather than assuming it.
- Query parameter order and encoding must preserve `trace=wp03&access_token=Tn`.
- The first 40001 must trigger exactly one token refresh and exactly one API replay; a second token error must not loop.
- The local token endpoint itself must be used by both SDKs; tests must never fall through to the real WeChat host.
- Request bodies must remain byte-for-byte UTF-8 equal, and trace ordering must be deterministic.

---

### Task 1: Add an isolated real-SDK HTTP differential harness

**Files:**
- Create: `tools/compat/http/fixtures.jsonl`
- Create: `tools/compat/http/java/Phase1HttpOracle.java`
- Create: `crates/wx-rust-miniapp/examples/phase1_http_oracle.rs`
- Create: `tools/compat/run_http_diff.py`
- Create: `tools/compat/test_http_diff.py`

**Interfaces:**
- Fixture operation: `miniapp.http.post-replay`.
- Both adapters accept one fixture JSONL path and emit one JSONL row with `id`, `operation`, `result.kind`, and `result.value`.
- Result value contains the returned response string, ordered request trace, token call count and API call count.
- Each trace entry contains `sequence`, `kind`, `method`, `target`, `content_type`, and `body`.

- [x] **Step 1: Write the shared fixture** with synthetic app ID/secret, fixed API URL/body, two token responses (T1/T2), and API responses (40001 then 0).
- [x] **Step 2: Write runner protocol tests** for fixture validation, unique IDs, and fail-closed comparison (5 Python tests); separately prove a pre-populated output directory exits 2 without overwriting its marker or creating a report.
- [x] **Step 3: Implement the Java adapter** using the pinned `WxMaServiceImpl.post()`, `WxMaDefaultConfigImpl`, and a JDK loopback HTTP server; record actual received requests and actual Java result.
- [x] **Step 4: Implement the Rust adapter** using `WxMaServiceImpl::new_arc`, `WxMaService::post()`, the existing Common pipeline and a Tokio loopback TCP server; record actual received requests and actual Rust result.
- [x] **Step 5: Implement the isolated runner**: verify Oracle SHA/clean tracked source, build only Maven `weixin-java-miniapp -am`, verify loaded `WxError` and `WxMaServiceImpl` class origins, build the Rust MiniApp example, execute both, save logs and compare strict JSON traces.
- [x] **Step 6: Run harness tests and real dual-runtime scenario**. Four actual requests observed; RED was retained as DIFFERENT, then GREEN became EQUIVALENT without a whitelist.

### Task 2: Pin the Common pipeline request-header contract

**Files:**
- Modify: `crates/wx-rust-common/tests/execute_pipeline_test.rs`

**Interfaces:**
- `TransportBody::Text(String)` remains a text POST body.
- Java default SimplePost HttpComponents executor sets `Content-Type: application/json; charset=UTF-8`.

- [x] **Step 1: Add a MockTransport regression test** asserting that a Text body produces POST, exact URL/body and `content-type=application/json; charset=UTF-8`.
- [x] **Step 2: Run only the new test and verify it fails because the current pipeline sends an empty header list.**
- [x] **Step 3: Compare the test's expected header with the actual Java loopback trace before changing production code.**

### Task 3: Apply the minimal Common pipeline fix

**Files:**
- Modify: `crates/wx-rust-common/src/pipeline/mod.rs`

**Interfaces:**
- `execute_pipeline_inner()` keeps its current retry, parse, token invalidation, refresh and breaker behavior.
- For `TransportBody::Text`, add the Java-compatible JSON UTF-8 Content-Type header.
- For `TransportBody::None` and `TransportBody::Bytes`, do not add this text-JSON header.

- [x] **Step 1: Add the header only to the Text-body request construction.**
- [x] **Step 2: Run the focused pipeline regression test and verify GREEN.**
- [x] **Step 3: Re-run the real Java/Rust HTTP differential; require all trace fields and call counts to match exactly.**
- [x] **Step 4: If any other field differs, stop and investigate the source/transport boundary; do not normalize or whitelist it.**

### Task 4: Verify, preserve evidence and update the migration ledger

**Files:**
- Create: `docs/verification/wp03-common-http-2026-11-13.md`
- Create: `docs/verification/wp03-common-http-2026-11-13.json`
- Create: `docs/verification/wp03-http-evidence-2026-11-13/`
- Modify: `tools/compat/README.md`
- Modify: `openspec/changes/wxjava-1to1-migration-closure/tasks.md`
- Modify: `docs/migration/wp03-execution-ledger.md`
- Modify: `README.md`
- Modify: `README.zh-CN.md`

- [x] **Step 1: Run** Python harness tests, focused pipeline tests, `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --offline --locked`, strict all-feature Clippy, and `cargo test --workspace --offline --locked` (26 Python tests, 8 pipeline tests, 5018 workspace tests).
- [x] **Step 2: Run the HTTP differential again into a fresh evidence directory**; preserve the first RED run and final GREEN run separately.
- [x] **Step 3: Verify source/fixture hashes are stable before/after execution and every evidence file hash matches the machine report.**
- [x] **Step 4: Document the POST-phase boundary. At the time of this POST-only phase, GET, non-2xx and empty-body scenarios were still unverified; the subsequent GET/status phase now covers one real scenario for each. Broader GET edge cases, arbitrary caller headers, XML, multipart/binary, concurrency, cancellation, timeout and Redis/multi-process single-flight remain unverified. See `docs/superpowers/plans/2026-11-13-common-http-get-status.md`.**
- [x] **Step 5: Run `git diff --check` and inspect Git status; keep all prior dirty work untouched. Do not commit or push.
