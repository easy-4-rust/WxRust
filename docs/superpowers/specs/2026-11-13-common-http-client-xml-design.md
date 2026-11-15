# P0 Common HTTP Client Headers / SimplePost XML Differential Design

## Goal

Extend the real WxJava 4.8.4.B / WxRust MiniApp loopback differential with:
1. Static HTTP Client default headers applied to Token and API requests.
2. SimplePost XML response pass-through.
3. SimplePost empty response error semantics.

## Authority and evidence

- Oracle commit: `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`.
- Java adapter invokes the real pinned MiniApp Service and default HttpComponents backend.
- Rust adapter invokes the real MiniApp Service and configured Reqwest client.
- Both sides send real requests to loopback servers bound only to `127.0.0.1`.
- Server traces record request method, raw target, Content-Type, explicitly configured custom header values, body, response status/content type/body, sequence and call counts.

## Source-derived behavior

### Client headers

WxJava's `DefaultHttpComponentsClientBuilder` supports shared `requestInterceptors`; the default MiniApp service uses the configured shared HttpClient for Token and API requests. The `WxMaService.get/post` call signatures do not accept per-call header maps.

This phase therefore verifies static client-level default headers only. It does not claim parity for arbitrary dynamic Java interceptors or per-call header overrides.

### SimplePost response handling

The pinned Java `SimplePostRequestExecutor.handleResponse` contract:
- Empty response body throws `WxErrorException(-99, "无响应内容")`.
- A response string beginning with the exact prefix `<xml>` is returned unchanged before JSON WxError parsing.
- Other non-empty responses continue through JSON WxError parsing.

The corresponding SimpleGet path does not use the XML pass-through branch. GET empty-body behavior remains a runtime response failure.

## Rust implementation contract

- Preserve `WxMaServiceImpl::new_arc(config)` and its default-client behavior.
- Add `new_arc_with_http_client(config, client)` so callers can inject a preconfigured Reqwest client, including `default_headers`.
- For Text POST bodies, map empty response to `WxErrorException(-99, "无响应内容")`.
- For Text POST responses beginning with exact `<xml>`, return the response body unchanged without JSON WxError parsing.
- Keep GET empty-body runtime behavior and all prior non-2xx / GET Query semantics unchanged.

## Comparison

- Four success/value scenarios use strict JSON result and complete server trace comparison.
- Three HTTP error scenarios compare explicit error category, optional error code (must match when present), full server trace and Token/API call counts.
- Original Java/Rust exception types and messages are retained; cross-language exception class names are not normalized.

## Non-goals

No certification of dynamic request-interceptor logic, per-call header override, duplicate-header ordering, signing-header refresh, 3xx/1xx, Token endpoint non-2xx, GET XML, XML declaration/leading whitespace/schema, XML parser semantics, multipart, binary/streaming, concurrency, cancellation, timeout, Redis/multi-process single-flight, CP Native, Pay V3, or whole-module parity.
