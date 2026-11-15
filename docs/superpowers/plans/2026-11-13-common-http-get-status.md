# P0 Common HTTP GET / Status / Empty-Body Differential Plan

> Execution record for the 2026-11-13 continuation. All work remains on the existing dirty checkout; no commit/push.

**Goal:** Extend the existing real-SDK loopback differential to GET query ordering, non-2xx status, and empty response bodies.

**Spec:** `docs/superpowers/specs/2026-11-13-common-http-get-status-design.md`

## Constraints

- Fixed WxJava Oracle `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`.
- Preserve branch `feature/wxjava-1to1-phase1`, HEAD `bc5dbb7`, and all previous uncommitted changes.
- Only loopback network; synthetic credentials and payloads.
- Preserve all RED and GREEN runs in separate non-overwriting evidence directories.
- Do not mark method matrix VERIFIED from scenario-level evidence.

## Task 1: Generalize HTTP fixture and adapter protocol

- [x] Support GET query-order fixture while retaining prior POST fixture.
- [x] Record response status and response body in server-observed traces.
- [x] Allow response fixtures as plain JSON strings or explicit `{status, body}` objects.
- [x] Add fail-closed `http_error` comparison: category + exact trace + call counts; preserve raw exception types/messages.
- [x] Prove query-order RED with a GET 40001 -> refresh -> replay fixture.

## Task 2: Fix GET query ordering

- [x] Verify Java BaseWxMaServiceImpl appends access_token before SimpleGetRequestExecutor appends queryParam.
- [x] Add a MockTransport regression for token-before-query ordering; observe RED.
- [x] Add `execute_pipeline_with_refresh_and_query` without changing existing entry points.
- [x] Route MiniApp GET queryParam through that entry point; rebuild URL in the same order on replay.
- [x] Run real Java/Rust GET + token refresh/replay; strict trace GREEN.

## Task 3: Fix non-2xx and empty-body behavior

- [x] Add 503 response fixture whose JSON body says errcode=0; observe Java runtime exception vs Rust success.
- [x] Add HTTP 200 empty-body fixture; observe Java NullPointerException vs Rust -99 business error.
- [x] Add Rust RED tests for non-2xx and empty body.
- [x] Reject status >= 300 before JSON parsing with Runtime error.
- [x] Reject empty body before JSON parsing with Runtime error.
- [x] Re-run actual Java/Rust fixtures; both error categories and full traces GREEN.

## Task 4: Full verification and closeout

- [x] Python harness: 32 passed.
- [x] Common pipeline tests: 11 passed.
- [x] cargo fmt/check/strict all-feature Clippy passed.
- [x] Workspace tests: 5021 passed, 0 failed, 1 ignored.
- [x] Common-only llvm-cov: lines 93.03%, functions 72.81%, regions 91.02%.
- [x] Save query RED, status/empty RED, four-scenario GREEN, raw traces, validation logs and hashes.
- [x] Update technical report, machine summary, OpenSpec, README and execution ledger.
- [x] Keep method matrix VERIFIED=0 and list all unverified boundaries.

## Final scope

Four real scenarios are EQUIVALENT: two strict JSON+trace comparisons and two explicit error-category+trace comparisons. This does not certify all Common HTTP paths or the WxRust migration as a whole.
