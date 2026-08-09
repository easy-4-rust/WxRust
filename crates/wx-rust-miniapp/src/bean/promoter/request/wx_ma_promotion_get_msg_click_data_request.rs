//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionGetMsgClickDataRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetMsgClickDataRequest {
    #[serde(rename = "send_date", default)]
    pub send_date: String,
    #[serde(rename = "dimonsion", default)]
    pub dimonsion: i64,
    #[serde(rename = "msg_type", default)]
    pub msg_type: i32,
    #[serde(rename = "msg_id", default)]
    pub msg_id: String,
    #[serde(rename = "begin_send_time", default)]
    pub begin_send_time: i64,
    #[serde(rename = "end_send_time", default)]
    pub end_send_time: i64,
}
