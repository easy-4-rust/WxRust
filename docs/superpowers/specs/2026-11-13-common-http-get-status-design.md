# P0 Common HTTP GET / Status / Empty-Body Differential Design

## Goal

Extend the real WxJava 4.8.4.B / WxRust MiniApp loopback differential to cover:
1. GET query parameter ordering around access_token injection and token replay.
2. HTTP status >= 300 response handling.
3. HTTP 200 with an empty response body.

## Authority and evidence

- Oracle commit: `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`.
- Java adapter invokes the real pinned `WxMaServiceImpl.get()` / `post()`.
- Rust adapter invokes the real `WxMaService::get()` / `post()`.
- Both adapters talk only to independent loopback HTTP servers bound to `127.0.0.1`.
- Server traces include request method, raw target, Content-Type, request body, response status, response body and order.
- Error-category comparisons preserve original Java/Rust exception types/messages and compare only explicitly declared semantic category + full wire trace + call counts.

## Source-derived behavior

Java GET sequence:
1. `BaseWxMaServiceImpl.executeInternal()` appends access_token to the URI.
2. `SimpleGetRequestExecutor.execute()` appends caller queryParam afterwards.

Java HTTP handling:
- Apache `AbstractHttpClientResponseHandler` throws `HttpResponseException` for status >= 300; BaseWxMa wraps the IOException in `WxRuntimeException`.
- Empty successful response yields no Gson WxError object; subsequent error-code access raises `NullPointerException`.

## Rust implementation contract

- Add an additive Common pipeline entry point accepting an optional query suffix; preserve existing pipeline entry points and URL behavior.
- Build the GET URL by injecting access_token first, then appending queryParam; repeat the same order after token refresh.
- Reject status >= 300 before attempting WxError JSON parsing.
- Reject an empty body as a runtime response failure before JSON parsing.
- Do not infer Content-Type for GET/None or Bytes; retain the prior Text POST JSON UTF-8 Content-Type fix.

## Comparison modes

- `json`: strict result and full trace equality.
- `http_error`: both sides must return an exception; compare declared category, full server-observed trace, token/API call counts. Retain and report raw exception type/message; never normalize them into equality.

## Non-goals

No certification of 3xx redirects, 1xx, arbitrary custom headers, token-endpoint non-2xx, XML, multipart, binary/streaming, concurrency, cancellation, timeout, Redis/multi-process single-flight, CP Native, Pay V3, or whole-module parity.
