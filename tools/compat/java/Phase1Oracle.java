import com.google.gson.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.util.*;
import me.chanjar.weixin.common.error.WxError;
import me.chanjar.weixin.common.enums.WxType;
import com.github.binarywang.wxpay.util.SignUtils;

/** Thin adapter: executes the pinned SDK classes, never duplicates SDK algorithms. */
public final class Phase1Oracle {
  private static final Gson GSON = new GsonBuilder().serializeNulls().disableHtmlEscaping().create();

  private static JsonObject value(JsonElement value) {
    JsonObject result = new JsonObject();
    result.addProperty("kind", "value");
    result.add("value", value);
    return result;
  }

  private static JsonObject execute(JsonObject fixture) {
    String operation = fixture.get("operation").getAsString();
    JsonObject input = fixture.getAsJsonObject("input");
    try {
      if (operation.equals("common.error") || operation.equals("common.error.try")) {
        String body = input.get("body").getAsString();
        WxType type = input.has("wx_type") && !input.get("wx_type").isJsonNull()
            ? WxType.valueOf(input.get("wx_type").getAsString()) : null;
        WxError error = WxError.fromJson(body, type);
        JsonObject result = new JsonObject();
        result.addProperty("error_code", error.getErrorCode());
        result.addProperty("error_msg", error.getErrorMsg());
        result.addProperty("error_msg_en", error.getErrorMsgEn());
        return value(result);
      }
      if (operation.equals("pay.sign")) {
        Map<String, String> params = new HashMap<>();
        for (Map.Entry<String, JsonElement> entry : input.getAsJsonObject("params").entrySet()) {
          if (!entry.getValue().isJsonPrimitive() || !entry.getValue().getAsJsonPrimitive().isString()) {
            throw new IllegalArgumentException("fixture params must be strings");
          }
          params.put(entry.getKey(), entry.getValue().getAsString());
        }
        String signType = input.get("sign_type").isJsonNull() ? null : input.get("sign_type").getAsString();
        String[] ignored = GSON.fromJson(input.get("ignored"), String[].class);
        return value(new JsonPrimitive(SignUtils.createSign(params, signType, input.get("sign_key").getAsString(), ignored)));
      }
      JsonObject result = new JsonObject();
      result.addProperty("kind", "unsupported");
      result.addProperty("reason", "operation not implemented by this adapter");
      return result;
    } catch (Exception error) {
      JsonObject result = new JsonObject();
      result.addProperty("kind", "exception");
      result.addProperty("type", error.getClass().getName());
      result.addProperty("message", error.getMessage());      // A separate, explicitly opted-in semantic category for Java exceptions.
      // Preserve the actual Java type and message in the evidence unchanged.
      // Unknown exceptions remain uncategorized and fail closed in the comparator.
      if (operation.equals("common.error.try")) {
        if (error instanceof JsonSyntaxException) {
          result.addProperty("category", "syntax_error");
        } else if (error instanceof NullPointerException) {
          result.addProperty("category", "null_root");
        }
      }
      return result;
    }
  }

  public static void main(String[] args) throws Exception {
    if (args.length != 1) throw new IllegalArgumentException("fixture JSONL path required");
    // Provenance on stderr; stdout is exclusively the result protocol.
    System.err.println("ORACLE_CLASS_SOURCE=" + WxError.class.getProtectionDomain().getCodeSource().getLocation());
    System.err.println("PAY_CLASS_SOURCE=" + SignUtils.class.getProtectionDomain().getCodeSource().getLocation());
    for (String line : Files.readAllLines(Paths.get(args[0]), StandardCharsets.UTF_8)) {
      if (line.trim().isEmpty()) continue;
      JsonObject fixture = JsonParser.parseString(line).getAsJsonObject();
      JsonObject row = new JsonObject();
      row.add("id", fixture.get("id"));
      row.add("operation", fixture.get("operation"));
      row.add("result", execute(fixture));
      System.out.println(GSON.toJson(row));
    }
  }
}
