//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaOrder.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrder {
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "userName", default)]
    pub user_name: String,
    #[serde(rename = "userPhone", default)]
    pub user_phone: String,
    #[serde(rename = "userLng", default)]
    pub user_lng: f64,
    #[serde(rename = "userLat", default)]
    pub user_lat: f64,
    #[serde(rename = "userAddress", default)]
    pub user_address: String,
    #[serde(rename = "useSandbox", default)]
    pub use_sandbox: i32,
    #[serde(rename = "storeOrderId", default)]
    pub store_order_id: String,
    #[serde(rename = "userOpenid", default)]
    pub user_openid: String,
    #[serde(rename = "orderSeq", default)]
    pub order_seq: String,
    #[serde(rename = "verifyCodeType", default)]
    pub verify_code_type: i32,
    #[serde(rename = "orderDetailPath", default)]
    pub order_detail_path: String,
    #[serde(rename = "callbackUrl", default)]
    pub callback_url: String,
    #[serde(rename = "cargo", default)]
    pub cargo: Cargo,
    #[serde(rename = "wxOrderId", default)]
    pub wx_order_id: String,
    #[serde(rename = "orderStatus", default)]
    pub order_status: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "deliveryNo", default)]
    pub delivery_no: String,
    #[serde(rename = "actualfee", default)]
    pub actualfee: i32,
    #[serde(rename = "deductfee", default)]
    pub deductfee: i32,
    #[serde(rename = "distance", default)]
    pub distance: i32,
    #[serde(rename = "createTime", default)]
    pub create_time: i64,
    #[serde(rename = "acceptTime", default)]
    pub accept_time: i64,
    #[serde(rename = "fetchTime", default)]
    pub fetch_time: i64,
    #[serde(rename = "finishTime", default)]
    pub finish_time: i64,
    #[serde(rename = "cancelTime", default)]
    pub cancel_time: i64,
    #[serde(rename = "expectedFinishTime", default)]
    pub expected_finish_time: i64,
    #[serde(rename = "fetchCode", default)]
    pub fetch_code: String,
    #[serde(rename = "recvCode", default)]
    pub recv_code: String,
    #[serde(rename = "transporterInfo", default)]
    pub transporter_info: TransporterInfo,
    #[serde(rename = "storeInfo", default)]
    pub store_info: StoreInfo,
    #[serde(rename = "receiverInfo", default)]
    pub receiver_info: ReceiverInfo,
    #[serde(rename = "cargoInfo", default)]
    pub cargo_info: Cargo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransporterInfo {
    #[serde(rename = "transporterName", default)]
    pub transporter_name: String,
    #[serde(rename = "transporterPhone", default)]
    pub transporter_phone: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoreInfo {
    #[serde(rename = "storeName", default)]
    pub store_name: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "lng", default)]
    pub lng: f64,
    #[serde(rename = "lat", default)]
    pub lat: f64,
    #[serde(rename = "phoneNum", default)]
    pub phone_num: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReceiverInfo {
    #[serde(rename = "receiverName", default)]
    pub receiver_name: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "phoneNum", default)]
    pub phone_num: String,
    #[serde(rename = "lng", default)]
    pub lng: f64,
    #[serde(rename = "lat", default)]
    pub lat: f64,
}
