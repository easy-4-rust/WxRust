//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaAddOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaAddOrderResponse {
    #[serde(rename = "wxOrderId", default)]
    pub wx_order_id: String,
    #[serde(rename = "storeOrderId", default)]
    pub store_order_id: String,
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "distance", default)]
    pub distance: i32,
    #[serde(rename = "transOrderId", default)]
    pub trans_order_id: String,
    #[serde(rename = "waybillId", default)]
    pub waybill_id: String,
    #[serde(rename = "fee", default)]
    pub fee: i32,
    #[serde(rename = "fetchCode", default)]
    pub fetch_code: String,
    #[serde(rename = "orderSeq", default)]
    pub order_seq: String,
}
