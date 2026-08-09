//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaStoreFlowResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaStoreFlowResponse {
    #[serde(rename = "totalPayAmt", default)]
    pub total_pay_amt: i64,
    #[serde(rename = "totalRefundAmt", default)]
    pub total_refund_amt: i64,
    #[serde(rename = "totalDeductAmt", default)]
    pub total_deduct_amt: i64,
    #[serde(rename = "flowList", default)]
    pub flow_list: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BasicFlowRecord {
    #[serde(rename = "flowType", default)]
    pub flow_type: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "payOrderId", default)]
    pub pay_order_id: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "payAmount", default)]
    pub pay_amount: i32,
    #[serde(rename = "payTime", default)]
    pub pay_time: i64,
    #[serde(rename = "createTime", default)]
    pub create_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChargeFlowRecord {
    #[serde(rename = "flowType", default)]
    pub flow_type: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "payOrderId", default)]
    pub pay_order_id: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "payAmount", default)]
    pub pay_amount: i32,
    #[serde(rename = "payTime", default)]
    pub pay_time: i64,
    #[serde(rename = "createTime", default)]
    pub create_time: i64,
    #[serde(rename = "payStatus", default)]
    pub pay_status: String,
    #[serde(rename = "consumeDeadline", default)]
    pub consume_deadline: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundFlowRecord {
    #[serde(rename = "flowType", default)]
    pub flow_type: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "payOrderId", default)]
    pub pay_order_id: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "payAmount", default)]
    pub pay_amount: i32,
    #[serde(rename = "payTime", default)]
    pub pay_time: i64,
    #[serde(rename = "createTime", default)]
    pub create_time: i64,
    #[serde(rename = "refundAmount", default)]
    pub refund_amount: i32,
    #[serde(rename = "refundTime", default)]
    pub refund_time: i64,
    #[serde(rename = "consumeDeadline", default)]
    pub consume_deadline: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsumeFlowRecord {
    #[serde(rename = "flowType", default)]
    pub flow_type: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "payOrderId", default)]
    pub pay_order_id: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "payAmount", default)]
    pub pay_amount: i32,
    #[serde(rename = "payTime", default)]
    pub pay_time: i64,
    #[serde(rename = "createTime", default)]
    pub create_time: i64,
    #[serde(rename = "wxOrderId", default)]
    pub wx_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "deliveryStatus", default)]
    pub delivery_status: String,
    #[serde(rename = "payStatus", default)]
    pub pay_status: String,
    #[serde(rename = "refundStatus", default)]
    pub refund_status: String,
    #[serde(rename = "refundAmount", default)]
    pub refund_amount: i32,
    #[serde(rename = "deductAmount", default)]
    pub deduct_amount: i32,
    #[serde(rename = "billId", default)]
    pub bill_id: String,
    #[serde(rename = "deliveryFinishedTime", default)]
    pub delivery_finished_time: i64,
}
