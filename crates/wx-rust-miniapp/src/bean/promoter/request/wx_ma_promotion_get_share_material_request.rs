//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionGetShareMaterialRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetShareMaterialRequest {
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "extra_info", default)]
    pub extra_info: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "share_type", default)]
    pub share_type: i64,
    #[serde(rename = "env_version", default)]
    pub env_version: String,
}
