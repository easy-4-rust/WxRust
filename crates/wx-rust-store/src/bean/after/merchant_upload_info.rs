//! 对应 Java `com.binarywang.wxjava.store.bean.after.MerchantUploadInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 MerchantUploadInfo；对应 Java com.binarywang.wxjava.store.bean.after.MerchantUploadInfo.java。
pub struct MerchantUploadInfo {
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "refund_certificates", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_certificates: Option<Vec<String>>,
}
