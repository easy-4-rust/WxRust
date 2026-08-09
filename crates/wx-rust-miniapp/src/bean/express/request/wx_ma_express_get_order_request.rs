//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressGetOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressGetOrderRequest {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
}

impl WxMaExpressGetOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaExpressGetOrderRequest 序列化失败: {e}"))
    }
}
