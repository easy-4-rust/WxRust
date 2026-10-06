//! 对应 Java `com.binarywang.wxjava.store.bean.fund.qrcode.QrCheckResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 QrCheckResponse；对应 Java com.binarywang.wxjava.store.bean.fund.qrcode.QrCheckResponse.java。
pub struct QrCheckResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "self_check_err_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_check_err_code: Option<i32>,
    #[serde(rename = "self_check_err_msg", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_check_err_msg: Option<String>,
    #[serde(rename = "scan_user_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_user_type: Option<i32>,
}
