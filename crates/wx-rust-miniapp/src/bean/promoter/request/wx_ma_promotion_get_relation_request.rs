//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionGetRelationRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetRelationRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "scene", default)]
    pub scene: i64,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "start_id", default)]
    pub start_id: String,
    #[serde(rename = "need_unionid", default)]
    pub need_unionid: i64,
}
