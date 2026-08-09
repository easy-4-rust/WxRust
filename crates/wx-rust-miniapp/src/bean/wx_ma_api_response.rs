//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaApiResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaApiResponse {
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "headers", default)]
    pub headers: std::collections::HashMap<String, String>,
}
