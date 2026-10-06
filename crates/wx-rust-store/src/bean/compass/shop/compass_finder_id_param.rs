//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.CompassFinderIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::compass::CompassFinderBaseParam;

/// 微信小店 CompassFinderIdParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.CompassFinderIdParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompassFinderIdParam {
    #[serde(rename = "ds", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds: Option<String>,
    #[serde(rename = "finder_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
}
