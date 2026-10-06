//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleDetail；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleDetail.java。
pub struct AfterSaleDetail {
    #[serde(rename = "desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "receive_product", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_product: Option<bool>,
    #[serde(rename = "cancel_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_time: Option<i64>,
    #[serde(rename = "prove_imgs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prove_imgs: Option<Vec<String>>,
    #[serde(rename = "tel_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_number: Option<String>,
    #[serde(rename = "media_id_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id_list: Option<Vec<String>>,
}
