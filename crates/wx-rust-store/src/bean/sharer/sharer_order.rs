//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.SharerOrder.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FinderSceneInfo;

/// 微信小店 SharerOrder 数据类型；对应 Java com.binarywang.wxjava.store.bean.sharer.SharerOrder.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerOrder {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "share_scene", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_scene: Option<i32>,
    #[serde(rename = "sharer_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_openid: Option<String>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "from_wecom", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_wx_work: Option<bool>,
    #[serde(rename = "finder_scene_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<FinderSceneInfo>,
}
