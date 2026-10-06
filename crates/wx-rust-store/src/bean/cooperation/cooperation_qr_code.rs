//! 对应 Java `com.binarywang.wxjava.store.bean.cooperation.CooperationQrCode.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CooperationQrCode；对应 Java com.binarywang.wxjava.store.bean.cooperation.CooperationQrCode.java。
pub struct CooperationQrCode {
    #[serde(rename = "qrcode_base64", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_base64: Option<i32>,
}
