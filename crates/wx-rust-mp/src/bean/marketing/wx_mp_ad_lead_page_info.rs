//! 对应 Java `bean.marketing.WxMpAdLeadPageInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpAdLeadPageInfo {
    #[serde(rename = "page", default)]
    pub page: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "total_page", default)]
    pub total_page: i32,
    #[serde(rename = "total_number", default)]
    pub total_number: i32,
}
