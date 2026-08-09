//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint.WxMaComplaintDetailResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaComplaintDetailResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
    #[serde(rename = "complaint_time", default)]
    pub complaint_time: String,
    #[serde(rename = "complaint_detail", default)]
    pub complaint_detail: String,
    #[serde(rename = "complaint_state", default)]
    pub complaint_state: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "phone_number", default)]
    pub phone_number: String,
    #[serde(rename = "complaint_order_info", default)]
    pub complaint_order_info: ComplaintOrderInfo,
    #[serde(rename = "complaint_media_list", default)]
    pub complaint_media_list: Vec<ComplaintMedia>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintOrderInfo {
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "out_trade_no", default)]
    pub out_trade_no: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintMedia {
    #[serde(rename = "media_type", default)]
    pub media_type: String,
    #[serde(rename = "media_url", default)]
    pub media_url: String,
}
