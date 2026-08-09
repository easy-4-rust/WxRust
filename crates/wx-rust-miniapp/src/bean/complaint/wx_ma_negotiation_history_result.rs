//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint.WxMaNegotiationHistoryResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaNegotiationHistoryResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data", default)]
    pub data: Vec<NegotiationHistory>,
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NegotiationHistory {
    #[serde(rename = "operate_time", default)]
    pub operate_time: String,
    #[serde(rename = "operate_type", default)]
    pub operate_type: String,
    #[serde(rename = "operate_details", default)]
    pub operate_details: String,
    #[serde(rename = "image_list", default)]
    pub image_list: Vec<String>,
}
