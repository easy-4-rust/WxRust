//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.response.WxMaPromotionGetShareMaterialResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetShareMaterialResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "share_path", default)]
    pub share_path: String,
    #[serde(rename = "qrcode", default)]
    pub qrcode: String,
    #[serde(rename = "tag", default)]
    pub tag: String,
}
