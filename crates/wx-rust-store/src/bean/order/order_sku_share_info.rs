//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSkuShareInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSkuShareInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderSkuShareInfo.java。
pub struct OrderSkuShareInfo {
    #[serde(rename = "sharer_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_openid: Option<String>,
    #[serde(rename = "sharer_unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_unionid: Option<String>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
    #[serde(rename = "share_scene", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_scene: Option<i32>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "from_wecom", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_wecom: Option<bool>,
}
