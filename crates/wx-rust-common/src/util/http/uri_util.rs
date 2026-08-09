//! URI 工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.URIUtil`。

/// URI 工具（encodeURIComponent 语义与 JS 一致）。
pub struct UriUtil;

impl UriUtil {
    /// 允许不转义的字符（与 Java `ALLOWED_CHARS` 一致）
    const ALLOWED_CHARS: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'()";

    /// 对输入做 URL 组件编码（encodeURIComponent 语义）。
    ///
    /// # 参数
    /// - `input`：需要编码的字符串
    ///
    /// # 返回
    /// 编码后的字符串；空输入返回原值。
    pub fn encode_uri_component(input: &str) -> String {
        if input.is_empty() {
            return input.to_string();
        }
        let mut out = String::with_capacity(input.len() * 3);
        for ch in input.chars() {
            if Self::ALLOWED_CHARS.contains(ch) {
                out.push(ch);
            } else {
                // 非允许字符：UTF-8 逐字节转 %XX（大写）
                let mut buf = [0u8; 4];
                for b in ch.encode_utf8(&mut buf).as_bytes() {
                    out.push('%');
                    out.push_str(&format!("{b:02X}"));
                }
            }
        }
        out
    }
}
