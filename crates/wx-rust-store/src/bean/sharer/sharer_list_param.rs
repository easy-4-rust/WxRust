//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.SharerListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::PageParam;

/// 微信小店 SharerListParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.sharer.SharerListParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerListParam {
    #[serde(rename = "page", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
}
