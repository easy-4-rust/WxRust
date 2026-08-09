//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.GetDeliveryListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetDeliveryListResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<DeliveryList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryList {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}

impl GetDeliveryListResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("GetDeliveryListResponse 解析失败: {e}"))
    }
}
