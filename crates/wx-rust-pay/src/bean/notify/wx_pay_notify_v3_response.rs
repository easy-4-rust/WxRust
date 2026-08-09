//! 对应 Java `com.github.binarywang.wxpay.bean.notify.WxPayNotifyV3Response.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayNotifyV3Response {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "message")]
    pub message: Option<String>,
}

/// v3 通知响应构造（对应 Java `WxPayNotifyV3Response` 静态方法，JSON）。
impl WxPayNotifyV3Response {
    /// 成功响应 JSON（对应 Java `success(String msg)`）。
    pub fn success(msg: &str) -> String {
        serde_json::json!({ "code": "SUCCESS", "message": msg }).to_string()
    }

    /// 失败响应 JSON（对应 Java `fail(String msg)`）。
    pub fn fail(msg: &str) -> String {
        serde_json::json!({ "code": "FAIL", "message": msg }).to_string()
    }
}
