import cn.binarywang.wx.miniapp.api.impl.WxMaServiceImpl;
import cn.binarywang.wx.miniapp.config.impl.WxMaDefaultConfigImpl;
import me.chanjar.weixin.common.bean.CommonUploadParam;
import me.chanjar.weixin.common.util.http.hc.DefaultHttpComponentsClientBuilder;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonNull;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

/** Executes the pinned WxJava MiniApp service against a local HTTP server only. */
public final class Phase1HttpOracle {
  private static final Gson GSON = new GsonBuilder().serializeNulls().disableHtmlEscaping().create();
  private static final class MockResponse {
    final int status;
    final String body;
    final String contentType;
    final JsonObject responseHeaders;
    MockResponse(int status, String body, String contentType, JsonObject responseHeaders) {
      this.status = status;
      this.body = body;
      this.contentType = contentType;
      this.responseHeaders = responseHeaders;
    }
  }

  private static byte[] decodeHex(String value) {
    if ((value.length() & 1) != 0) throw new IllegalArgumentException("hex payload must have even length");
    byte[] bytes = new byte[value.length() / 2];
    for (int i = 0; i < bytes.length; i++) {
      bytes[i] = (byte) Integer.parseInt(value.substring(i * 2, i * 2 + 2), 16);
    }
    return bytes;
  }

  private static String toHex(byte[] bytes) {
    StringBuilder value = new StringBuilder(bytes.length * 2);
    for (byte item : bytes) value.append(String.format("%02x", item & 0xff));
    return value.toString();
  }

  private static MockResponse mockResponse(JsonElement value) {
    if (value.isJsonPrimitive()) {
      return new MockResponse(200, value.getAsString(), "application/json; charset=UTF-8", new JsonObject());
    }
    JsonObject object = value.getAsJsonObject();
    int status = object.has("status") ? object.get("status").getAsInt() : 200;
    String body = object.has("body") ? object.get("body").getAsString() : "";
    String contentType = object.has("content_type")
        ? object.get("content_type").getAsString()
        : "application/json; charset=UTF-8";
    JsonObject responseHeaders = object.has("response_headers")
        ? object.getAsJsonObject("response_headers").deepCopy() : new JsonObject();
    return new MockResponse(status, body, contentType, responseHeaders);
  }

  private static void configureDefaultHeaders(JsonObject headers, JsonObject dynamicHeaders, String tokenPath) {
    List<org.apache.hc.core5.http.HttpRequestInterceptor> interceptors = new ArrayList<>();
    if (headers != null && !headers.entrySet().isEmpty()) {
      interceptors.add((request, entity, context) -> {
        for (java.util.Map.Entry<String, JsonElement> entry : headers.entrySet()) {
          request.addHeader(entry.getKey(), entry.getValue().getAsString());
        }
      });
    }
    if (dynamicHeaders != null && !dynamicHeaders.entrySet().isEmpty()) {
      String headerName = dynamicHeaders.get("name").getAsString();
      String tokenValue = dynamicHeaders.get("token_value").getAsString();
      String apiValue = dynamicHeaders.get("api_value").getAsString();
      interceptors.add((request, entity, context) -> {
        String path = request.getRequestUri().split("\\?", 2)[0];
        request.setHeader(headerName, path.equals(tokenPath) ? tokenValue : apiValue);
      });
    }
    DefaultHttpComponentsClientBuilder.get().setRequestInterceptors(interceptors);
  }



  private static String exceptionCategory(Throwable error) {
    for (Throwable current = error; current != null; current = current.getCause()) {
      if (current.getClass().getSimpleName().equals("HttpResponseException")) return "http_status";
      if (current instanceof NullPointerException) return "empty_response_runtime";
      if (current instanceof me.chanjar.weixin.common.error.WxErrorException wxError
          && wxError.getError() != null
          && "无法获取到文件名，Content-disposition为空".equals(wxError.getError().getErrorMsg())) {
        return "missing_content_disposition";
      }
      if (current instanceof me.chanjar.weixin.common.error.WxErrorException wxError
          && wxError.getError().getErrorCode() == -99
          && "无响应内容".equals(wxError.getError().getErrorMsg())) {
        return "empty_response_business";
      }
    }
    return null;
  }


  private static final class LoopbackServer implements AutoCloseable {
    private final HttpServer server;
    private final ExecutorService executor = Executors.newCachedThreadPool();
    private final AtomicInteger sequence = new AtomicInteger();
    private final AtomicInteger tokenCalls = new AtomicInteger();
    private final AtomicInteger apiCalls = new AtomicInteger();
    private final ConcurrentLinkedQueue<JsonObject> trace = new ConcurrentLinkedQueue<>();
    private final List<JsonElement> tokenResponses;
    private final List<JsonElement> apiResponses;
    private final String apiPath;
    private final JsonObject defaultHeaders;
    private final JsonObject dynamicHeaders;

    LoopbackServer(JsonObject input) throws IOException {
      this.tokenResponses = responses(input.getAsJsonArray("token_responses"));
      this.apiResponses = responses(input.getAsJsonArray("api_responses"));
      this.apiPath = input.get("api_path").getAsString();
      this.defaultHeaders = input.has("default_headers")
          ? input.getAsJsonObject("default_headers").deepCopy() : new JsonObject();
      this.dynamicHeaders = input.has("dynamic_headers")
          ? input.getAsJsonObject("dynamic_headers").deepCopy() : null;
      this.server = HttpServer.create(
          new InetSocketAddress(InetAddress.getByName("127.0.0.1"), 0), 0);
      this.server.createContext(input.get("token_path").getAsString(), exchange -> handle("token", exchange));
      this.server.createContext(apiPath, exchange -> handle("api", exchange));
      this.server.setExecutor(executor);
      this.server.start();
    }

    private static List<JsonElement> responses(JsonArray values) {
      List<JsonElement> result = new ArrayList<>();
      for (JsonElement value : values) result.add(value.deepCopy());
      return result;
    }

    String baseUrl() {
      return "http://127.0.0.1:" + server.getAddress().getPort();
    }

    private void handle(String kind, HttpExchange exchange) throws IOException {
      byte[] bodyBytes = exchange.getRequestBody().readAllBytes();
      String body = new String(bodyBytes, StandardCharsets.UTF_8);
      String rawQuery = exchange.getRequestURI().getRawQuery();
      String target = exchange.getRequestURI().getRawPath()
          + (rawQuery == null ? "" : "?" + rawQuery);
      String contentType = exchange.getRequestHeaders().getFirst("Content-Type");

      JsonObject row = new JsonObject();
      row.addProperty("sequence", sequence.incrementAndGet());
      row.addProperty("kind", kind);
      row.addProperty("method", exchange.getRequestMethod());
      row.addProperty("target", target);
      if (contentType == null) row.add("content_type", JsonNull.INSTANCE);
      else row.addProperty("content_type", contentType);
      row.addProperty("body", body);
      row.addProperty("body_hex", toHex(bodyBytes));
      JsonObject customHeaders = new JsonObject();
      for (java.util.Map.Entry<String, JsonElement> entry : defaultHeaders.entrySet()) {
        JsonArray values = new JsonArray();
        List<String> received = exchange.getRequestHeaders().get(entry.getKey());
        if (received != null) received.forEach(values::add);
        customHeaders.add(entry.getKey().toLowerCase(java.util.Locale.ROOT), values);
      }
      if (dynamicHeaders != null) {
        String dynamicName = dynamicHeaders.get("name").getAsString();
        List<String> dynamicValues = exchange.getRequestHeaders().get(dynamicName);
        JsonArray values = new JsonArray();
        if (dynamicValues != null) dynamicValues.forEach(values::add);
        customHeaders.add(dynamicName.toLowerCase(java.util.Locale.ROOT), values);
      }
      row.add("custom_headers", customHeaders);
      int index = kind.equals("token") ? tokenCalls.getAndIncrement() : apiCalls.getAndIncrement();
      List<JsonElement> responses = kind.equals("token") ? tokenResponses : apiResponses;
      MockResponse response = index < responses.size()
          ? mockResponse(responses.get(index))
          : new MockResponse(200, "{\"errcode\":-1,\"errmsg\":\"unexpected extra request\"}", "application/json; charset=UTF-8", new JsonObject());
      row.addProperty("response_status", response.status);
      row.addProperty("response_content_type", response.contentType);
      row.add("response_headers", response.responseHeaders.deepCopy());
      row.addProperty("response_body", response.body);
      byte[] bytes = response.body.getBytes(StandardCharsets.UTF_8);
      row.addProperty("response_body_hex", toHex(bytes));
      trace.add(row);

      exchange.getResponseHeaders().set("Content-Type", response.contentType);
      for (java.util.Map.Entry<String, JsonElement> entry : response.responseHeaders.entrySet()) {
        exchange.getResponseHeaders().set(entry.getKey(), entry.getValue().getAsString());
      }
      exchange.getResponseHeaders().set("Connection", "close");
      exchange.sendResponseHeaders(response.status, bytes.length);
      try (OutputStream output = exchange.getResponseBody()) {
        output.write(bytes);
      }
    }

    JsonArray requestTrace() {
      List<JsonObject> rows = new ArrayList<>(trace);
      rows.sort(Comparator.comparingInt(row -> row.get("sequence").getAsInt()));
      JsonArray result = new JsonArray();
      rows.forEach(result::add);
      return result;
    }

    int tokenCalls() { return tokenCalls.get(); }
    int apiCalls() { return apiCalls.get(); }

    @Override
    public void close() {
      server.stop(0);
      executor.shutdownNow();
    }
  }

  private static JsonObject execute(JsonObject fixture) {
    JsonObject input = fixture.getAsJsonObject("input");
    JsonObject result = new JsonObject();
    LoopbackServer server = null;
    try {
      server = new LoopbackServer(input);
      WxMaDefaultConfigImpl config = new WxMaDefaultConfigImpl();
      config.setAppid(input.get("app_id").getAsString());
      config.setSecret(input.get("app_secret").getAsString());
      config.setApiHostUrl(server.baseUrl());
      config.setAccessTokenUrl(server.baseUrl()
          + input.get("token_path").getAsString()
          + "?grant_type=client_credential&appid=%s&secret=%s");

      // This is the real default WxJava implementation:
      // WxMaServiceImpl -> WxMaServiceHttpComponentsImpl -> BaseWxMaServiceImpl.
      WxMaServiceImpl service = new WxMaServiceImpl();
      service.setWxMaConfig(config);

      String url = "https://api.weixin.qq.com"
          + input.get("api_path").getAsString()
          + "?" + input.get("api_query").getAsString();
      String response;
      String operation = fixture.get("operation").getAsString();
      if (operation.equals("miniapp.http.media-download") || operation.equals("miniapp.http.media-download-file")) {
        var downloadedFile = service.getMediaService().getMedia(input.get("media_id").getAsString());
        try {
          JsonObject downloadResult = new JsonObject();
          downloadResult.addProperty("file_name", downloadedFile.getName());
          downloadResult.addProperty("bytes_hex", toHex(Files.readAllBytes(downloadedFile.toPath())));
          if (operation.equals("miniapp.http.media-download-file")) {
            downloadResult.addProperty("file_exists", downloadedFile.isFile());
            downloadResult.addProperty("temp_dir_name", downloadedFile.getParentFile().getName());
          }
          response = GSON.toJson(downloadResult);
        } finally {
          Files.deleteIfExists(downloadedFile.toPath());
          if (operation.equals("miniapp.http.media-download-file")) Files.deleteIfExists(downloadedFile.toPath().getParent());
        }
      } else if (operation.equals("miniapp.http.common-upload")
          || operation.equals("miniapp.http.media-upload")) {
        Path tempDir = Files.createTempDirectory("wxrust-phase1-upload-");
        Path file = tempDir.resolve(input.get("file_name").getAsString());
        Files.write(file, decodeHex(input.get("file_bytes_hex").getAsString()));
        try {
          if (operation.equals("miniapp.http.common-upload")) {
            CommonUploadParam param = CommonUploadParam.fromFile("media", file.toFile());
            JsonObject formFields = input.getAsJsonObject("form_fields");
            if (formFields != null) {
              for (java.util.Map.Entry<String, JsonElement> entry : formFields.entrySet()) {
                param.addFormField(entry.getKey(), entry.getValue().getAsString());
              }
            }
            response = service.upload(url, param);
          } else {
            var uploadResult = service.getMediaService().uploadMedia(
                input.get("media_type").getAsString(), file.toFile());
            response = GSON.toJson(uploadResult);
          }
        } finally {
          Files.deleteIfExists(file);
          Files.deleteIfExists(tempDir);
        }
      } else if (operation.equals("miniapp.http.get-query-order")
          || operation.equals("miniapp.http.get-custom-client-headers")
          || operation.equals("miniapp.http.get-dynamic-interceptor")) {
        response = service.get(url, input.get("query_param").getAsString());
      } else {
        response = service.post(url, input.get("api_body").getAsString());
      }

      JsonObject value = new JsonObject();
      value.addProperty("response_body", response);
      value.add("request_trace", server.requestTrace());
      value.addProperty("token_calls", server.tokenCalls());
      value.addProperty("api_calls", server.apiCalls());
      result.addProperty("kind", "value");
      result.add("value", value);
    } catch (Exception error) {
      result.addProperty("kind", "exception");
      result.addProperty("type", error.getClass().getName());
      result.addProperty("message", error.getMessage());
      if (error instanceof me.chanjar.weixin.common.error.WxErrorException wxError) {
        result.addProperty("error_code", wxError.getError().getErrorCode());
      }
      if (error.getCause() != null) result.addProperty("cause_type", error.getCause().getClass().getName());
      String category = exceptionCategory(error);
      if (category != null) result.addProperty("category", category);
      if (server != null) {
        result.add("request_trace", server.requestTrace());
        result.addProperty("token_calls", server.tokenCalls());
        result.addProperty("api_calls", server.apiCalls());
      }
    } finally {
      if (server != null) server.close();
    }
    return result;
  }

  public static void main(String[] args) throws Exception {
    if (args.length != 1) throw new IllegalArgumentException("fixture JSONL path required");
    System.err.println("ORACLE_COMMON_CLASS_SOURCE="
        + me.chanjar.weixin.common.error.WxError.class.getProtectionDomain().getCodeSource().getLocation());
    System.err.println("ORACLE_MINIAPP_CLASS_SOURCE="
        + WxMaServiceImpl.class.getProtectionDomain().getCodeSource().getLocation());
    List<JsonObject> fixtures = new ArrayList<>();
    for (String line : Files.readAllLines(Path.of(args[0]), StandardCharsets.UTF_8)) {
      if (!line.isBlank()) fixtures.add(JsonParser.parseString(line).getAsJsonObject());
    }
    JsonObject defaultHeaders = fixtures.isEmpty()
        ? new JsonObject() : fixtures.get(0).getAsJsonObject("input").getAsJsonObject("default_headers");
    JsonObject dynamicHeaders = fixtures.isEmpty() ? null
        : fixtures.get(0).getAsJsonObject("input").getAsJsonObject("dynamic_headers");
    String tokenPath = fixtures.isEmpty() ? ""
        : fixtures.get(0).getAsJsonObject("input").get("token_path").getAsString();
    for (JsonObject fixture : fixtures) {
      JsonObject headers = fixture.getAsJsonObject("input").getAsJsonObject("default_headers");
      JsonObject rowDynamicHeaders = fixture.getAsJsonObject("input").getAsJsonObject("dynamic_headers");
      String rowTokenPath = fixture.getAsJsonObject("input").get("token_path").getAsString();
      if (!defaultHeaders.equals(headers)
          || !java.util.Objects.equals(dynamicHeaders, rowDynamicHeaders)
          || !tokenPath.equals(rowTokenPath)) {
        throw new IllegalArgumentException("all fixtures must use the same interceptor configuration for the singleton Java HttpClient");
      }
    }
    configureDefaultHeaders(defaultHeaders, dynamicHeaders, tokenPath);
    for (JsonObject fixture : fixtures) {
      JsonObject row = new JsonObject();
      row.add("id", fixture.get("id"));
      row.add("operation", fixture.get("operation"));
      row.add("result", execute(fixture));
      System.out.println(GSON.toJson(row));
    }
  }
}
