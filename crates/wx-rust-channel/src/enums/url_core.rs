//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 视频号小店接口地址常量 @author <a href="https://github.com/lixize">Zeyes</a> / @UtilityClass public class WxChannelApiUrlConstants { / 获取access_token.（对应 Java `WxChannelApiUrlConstants` 常量 `GET_ACCESS_TOKEN_URL`）。
pub const GET_ACCESS_TOKEN_URL: &str =
    "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=%s&secret=%s";

/// 获取Stable access_token.（对应 Java `WxChannelApiUrlConstants` 常量 `GET_STABLE_ACCESS_TOKEN_URL`）。
pub const GET_STABLE_ACCESS_TOKEN_URL: &str = "https://api.weixin.qq.com/cgi-bin/stable_token";
