//! 对应 Java `cn.binarywang.wx.miniapp.bean.shortlink.GenerateShortLinkRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenerateShortLinkRequest {
    #[serde(rename = "page_url", default)]
    pub page_url: String,
    #[serde(rename = "page_title", default)]
    pub page_title: String,
    #[serde(rename = "is_permanent", default)]
    pub is_permanent: bool,
}
