//! 对应 Java `me.chanjar.weixin.channel.bean.sharer.SharerOrder.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerOrder {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "share_scene", default)]
    pub sharer_scene: i32,
    #[serde(rename = "sharer_openid", default)]
    pub sharer_openid: String,
    #[serde(rename = "sharer_type", default)]
    pub sharer_type: i32,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "from_wecom", default)]
    pub from_wx_work: bool,
    #[serde(rename = "finder_scene_info", default)]
    pub scene_info: FinderSceneInfo,
}
