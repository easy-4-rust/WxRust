//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.FinderGmvData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FinderGmvData；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderGmvData.java。
pub struct FinderGmvData {
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
    #[serde(rename = "pay_product_id_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_product_id_cnt: Option<String>,
    #[serde(rename = "pay_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_uv: Option<String>,
    #[serde(rename = "refund_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_gmv: Option<String>,
    #[serde(rename = "pay_refund_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_gmv: Option<String>,
}
