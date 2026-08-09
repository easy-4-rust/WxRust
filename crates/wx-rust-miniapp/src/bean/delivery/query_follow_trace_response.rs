//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.QueryFollowTraceResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryFollowTraceResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "waybill_info", default)]
    pub waybill_info: WaybillInfo,
    #[serde(rename = "shop_info", default)]
    pub shop_info: ShopInfo,
    #[serde(rename = "delivery_info", default)]
    pub delivery_info: DeliveryInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillInfo {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopInfo {
    #[serde(rename = "goods_info", default)]
    pub goods_info: WaybillGoodsInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryInfo {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}

impl QueryFollowTraceResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("QueryFollowTraceResponse 解析失败: {e}"))
    }
}
