//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopProductDataParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::compass::CompassFinderBaseParam;

/// 微信小店 ShopProductDataParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopProductDataParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopProductDataParam {
    #[serde(rename = "ds", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds: Option<String>,
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
