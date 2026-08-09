//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayGetComplaintDetailResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayGetComplaintDetailResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "complaint", default)]
    pub complaint: Complaint,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Complaint {
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
    #[serde(rename = "complaint_time", default)]
    pub complaint_time: String,
    #[serde(rename = "complaint_detail", default)]
    pub complaint_detail: String,
    #[serde(rename = "complaint_state", default)]
    pub complaint_state: String,
    #[serde(rename = "payer_phone", default)]
    pub payer_phone: String,
    #[serde(rename = "payer_openid", default)]
    pub payer_openid: String,
    #[serde(rename = "complaint_order_info", default)]
    pub complaint_order_info: Vec<ComplaintOrderInfo>,
    #[serde(rename = "complaint_full_refunded", default)]
    pub complaint_full_refunded: bool,
    #[serde(rename = "incoming_user_response", default)]
    pub incoming_user_response: bool,
    #[serde(rename = "user_complaint_times", default)]
    pub user_complaint_times: i32,
    #[serde(rename = "complaint_media_list", default)]
    pub complaint_media_list: Vec<ComplaintMedia>,
    #[serde(rename = "problem_description", default)]
    pub problem_description: String,
    #[serde(rename = "problem_type", default)]
    pub problem_type: String,
    #[serde(rename = "apply_refund_amount", default)]
    pub apply_refund_amount: i32,
    #[serde(rename = "user_tag_list", default)]
    pub user_tag_list: Vec<String>,
    #[serde(rename = "service_order_info", default)]
    pub service_order_info: Vec<ServiceOrderInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintOrderInfo {
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "out_trade_no", default)]
    pub out_trade_no: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "wxa_out_trade_no", default)]
    pub wxa_out_trade_no: String,
    #[serde(rename = "wx_order_id", default)]
    pub wx_order_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintMedia {
    #[serde(rename = "media_type", default)]
    pub media_type: String,
    #[serde(rename = "media_url", default)]
    pub media_url: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceOrderInfo {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "out_order_no", default)]
    pub out_order_no: String,
    #[serde(rename = "state", default)]
    pub state: String,
}

impl WxMaXPayGetComplaintDetailResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayGetComplaintDetailResponse 序列化失败: {e}"))
    }
}
