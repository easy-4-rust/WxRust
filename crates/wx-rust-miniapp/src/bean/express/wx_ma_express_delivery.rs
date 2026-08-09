//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.WxMaExpressDelivery.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressDelivery {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
    #[serde(rename = "can_use_cash", default)]
    pub can_use_cash: i32,
    #[serde(rename = "can_get_quota", default)]
    pub can_get_quota: i32,
    #[serde(rename = "cash_biz_id", default)]
    pub cash_biz_id: String,
    #[serde(rename = "service_type", default)]
    pub service_type: Vec<ServiceType>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceType {
    #[serde(rename = "service_type", default)]
    pub service_type: i32,
    #[serde(rename = "service_name", default)]
    pub service_name: String,
}

impl WxMaExpressDelivery {
    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `data` 数组）。
    pub fn from_json(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("WxMaExpressDelivery 列表解析失败: {e}"))?;
        let list = value
            .get("data")
            .ok_or_else(|| "缺少 data 字段".to_string())?;
        serde_json::from_value(list.clone())
            .map_err(|e| format!("WxMaExpressDelivery 列表解析失败: {e}"))
    }
}
