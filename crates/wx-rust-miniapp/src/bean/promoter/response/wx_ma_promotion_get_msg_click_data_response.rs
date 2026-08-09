//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.response.WxMaPromotionGetMsgClickDataResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetMsgClickDataResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data_list", default)]
    pub data_list: Vec<Dimonsion>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dimonsion {
    #[serde(rename = "click_uv", default)]
    pub click_uv: i64,
    #[serde(rename = "click_pv", default)]
    pub click_pv: i64,
    #[serde(rename = "msg_type", default)]
    pub msg_type: i64,
    #[serde(rename = "msg_id", default)]
    pub msg_id: String,
    #[serde(rename = "send_time", default)]
    pub send_time: i64,
}
