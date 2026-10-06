//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopSaleProfileDataParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::compass::CompassFinderBaseParam;

/// 微信小店 ShopSaleProfileDataParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopSaleProfileDataParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopSaleProfileDataParam {
    #[serde(rename = "ds", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}
