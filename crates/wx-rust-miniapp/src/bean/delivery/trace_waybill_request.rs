//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.TraceWaybillRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TraceWaybillRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "sender_phone", default)]
    pub sender_phone: String,
    #[serde(rename = "receiver_phone", default)]
    pub receiver_phone: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "trans_id", default)]
    pub trans_id: String,
    #[serde(rename = "order_detail_path", default)]
    pub order_detail_path: String,
    #[serde(rename = "goods_info", default)]
    pub goods_info: WaybillGoodsInfo,
}

impl TraceWaybillRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("TraceWaybillRequest 序列化失败: {e}"))
    }
}
