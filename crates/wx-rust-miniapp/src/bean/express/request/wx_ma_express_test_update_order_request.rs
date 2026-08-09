//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressTestUpdateOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressTestUpdateOrderRequest {
    #[serde(rename = "biz_id", default)]
    pub biz_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "action_time", default)]
    pub action_time: i64,
    #[serde(rename = "action_type", default)]
    pub action_type: i32,
    #[serde(rename = "action_msg", default)]
    pub action_msg: String,
}

impl WxMaExpressTestUpdateOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaExpressTestUpdateOrderRequest 序列化失败: {e}"))
    }
}
