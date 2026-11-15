# P0 Common HTTP Client Headers / SimplePost XML Differential Plan

> Execution record for 2026-11-13. Existing dirty checkout is preserved; no commit/push.

**Goal:** Verify and close static Client default Headers plus SimplePost XML/empty response behavior against the pinned real WxJava SDK.

**Spec:** `docs/superpowers/specs/2026-11-13-common-http-client-xml-design.md`

## Constraints

- Fixed WxJava Oracle `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`.
- Existing branch `feature/wxjava-1to1-phase1`, HEAD `bc5dbb7`; preserve all prior dirty changes.
- Loopback only; synthetic app credentials and response data.
- Keep RED and GREEN runs in separate output directories; never overwrite evidence.
- Scenario evidence must not promote the method matrix from VERIFIED=0.

## Task 1: Client default Headers

- [x] Confirm WxJava shared HttpComponents requestInterceptors are applied to Token and API requests.
- [x] Confirm WxMaService.get/post have no per-call header argument; scope the test to static client-level default headers.
- [x] Add Rust `new_arc_with_http_client`, preserving `new_arc`.
- [x] Configure Java request interceptor and Rust Reqwest default_headers with two synthetic headers.
- [x] Record actual received header values on both Token and API requests.
- [x] RED 1/1 (Java headers present; old Rust default client has none), then GREEN 1/1.

## Task 2: SimplePost XML / empty response

- [x] Verify Java SimplePost exact `<xml>` prefix returns raw response body.
- [x] Verify Java SimplePost empty body maps to WxErrorException(-99, "无响应内容").
- [x] Add Rust RED tests for XML raw body and empty POST response.
- [x] Implement Text POST XML pass-through and -99 empty response mapping without changing GET empty-body behavior.
- [x] RED 2/2, then GREEN 2/2 in Rust tests and real Java/Rust loopback differential.

## Task 3: Harness and comparator

- [x] Extend response fixtures with content_type and record response Content-Type.
- [x] Record configured custom header values as observed by the loopback servers.
- [x] Add fail-closed custom-header and SimplePost XML/empty fixture validation.
- [x] Extend http_error comparison to compare optional error_code; missing-vs-present is DIFFERENT, not INFRA_ERROR.
- [x] Python Harness 36 passed.
- [x] Real RED for three new scenarios: 3/3 DIFFERENT.
- [x] Combined seven-scenario GREEN: 7/7 EQUIVALENT.

## Task 4: Final verification and closeout

- [x] Full Workspace fmt/check/strict Clippy/tests/llvm-cov.
- [x] Verify all raw evidence hashes and source digest.
- [x] Update technical report, machine summary, OpenSpec, README, tool README and execution ledger.
- [x] Keep method matrix VERIFIED=0; list unverified dynamic interceptors, per-call headers, GET XML, XML edge cases and other HTTP paths.

## Final scope

Seven MiniApp scenarios: four strict JSON+trace values (POST replay, GET query order, custom Client headers, XML POST) and three explicit error-category+trace scenarios (HTTP 503, GET empty body, POST empty body). No claim of complete HTTP or module parity.
