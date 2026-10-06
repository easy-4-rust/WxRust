//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.FinderGmvItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FinderGmvData;

/// 微信小店 FinderGmvItem 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderGmvItem.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderGmvItem {
    #[serde(rename = "finder_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
    #[serde(rename = "finder_nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_nickname: Option<String>,
    #[serde(rename = "data", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FinderGmvData>,
}
