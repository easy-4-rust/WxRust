//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopSaleProfileData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::ShopField;

/// 微信小店 ShopSaleProfileData 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopSaleProfileData.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopSaleProfileData {
    #[serde(rename = "field_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_list: Option<Vec<ShopField>>,
}
