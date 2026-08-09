//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressAddOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressAddOrderRequest {
    #[serde(rename = "add_source", default)]
    pub add_source: i32,
    #[serde(rename = "wx_appid", default)]
    pub wx_appid: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "biz_id", default)]
    pub biz_id: String,
    #[serde(rename = "custom_remark", default)]
    pub custom_remark: String,
    #[serde(rename = "tagid", default)]
    pub tagid: i32,
    #[serde(rename = "expect_time", default)]
    pub expect_time: i64,
    #[serde(rename = "sender", default)]
    pub sender: WxMaExpressOrderPerson,
    #[serde(rename = "receiver", default)]
    pub receiver: WxMaExpressOrderPerson,
    #[serde(rename = "cargo", default)]
    pub cargo: WxMaExpressOrderCargo,
    #[serde(rename = "shop", default)]
    pub shop: WxMaExpressOrderShop,
    #[serde(rename = "insured", default)]
    pub insured: WxMaExpressOrderInsured,
    #[serde(rename = "service", default)]
    pub service: ServiceType,
}

impl WxMaExpressAddOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaExpressAddOrderRequest 序列化失败: {e}"))
    }
}
