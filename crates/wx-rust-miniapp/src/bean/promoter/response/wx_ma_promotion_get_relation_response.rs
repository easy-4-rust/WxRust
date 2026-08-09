//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.response.WxMaPromotionGetRelationResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetRelationResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "relation_list", default)]
    pub relation_list: Vec<Relation>,
    #[serde(rename = "total_cnt", default)]
    pub total_cnt: i64,
    #[serde(rename = "start_id", default)]
    pub start_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Relation {
    #[serde(rename = "promoter_openid", default)]
    pub promoter_openid: String,
    #[serde(rename = "role_id", default)]
    pub role_id: i64,
    #[serde(rename = "retail_id", default)]
    pub retail_id: String,
    #[serde(rename = "extra_info", default)]
    pub extra_info: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "scene", default)]
    pub scene: i64,
    #[serde(rename = "share_extra_info", default)]
    pub share_extra_info: String,
}
