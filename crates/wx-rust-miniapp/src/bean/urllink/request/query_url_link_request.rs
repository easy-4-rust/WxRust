//! 对应 Java `cn.binarywang.wx.miniapp.bean.urllink.request.QueryUrlLinkRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::urllink::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryUrlLinkRequest {
    #[serde(rename = "url_link", default)]
    pub url_link: String,
}
