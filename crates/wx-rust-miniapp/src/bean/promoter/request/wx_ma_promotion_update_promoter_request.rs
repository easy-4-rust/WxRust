//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionUpdatePromoterRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionUpdatePromoterRequest {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "role_id", default)]
    pub role_id: i64,
    #[serde(rename = "retail_id", default)]
    pub retail_id: String,
    #[serde(rename = "extra_info", default)]
    pub extra_info: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "decl_status", default)]
    pub decl_status: String,
}
