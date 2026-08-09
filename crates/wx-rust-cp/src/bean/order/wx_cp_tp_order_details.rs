//! 对应 Java `me.chanjar.weixin.cp.bean.order.WxCpTpOrderDetails.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpOrderDetails {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "orderid", default)]
    pub order_id: String,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
    #[serde(rename = "paid_corpid", default)]
    pub paid_corp_id: String,
    #[serde(rename = "operator_id", default)]
    pub operator_id: String,
    #[serde(rename = "suiteid", default)]
    pub suite_id: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "edition_id", default)]
    pub edition_id: String,
    #[serde(rename = "edition_name", default)]
    pub edition_name: String,
    #[serde(rename = "price", default)]
    pub price: i64,
    #[serde(rename = "user_count", default)]
    pub user_count: i32,
    #[serde(rename = "order_period", default)]
    pub order_period: i32,
    #[serde(rename = "order_time", default)]
    pub order_time: i64,
    #[serde(rename = "paid_time", default)]
    pub paid_time: i64,
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "order_from", default)]
    pub order_from: i32,
    #[serde(rename = "operator_corpid", default)]
    pub operator_corp_id: String,
    #[serde(rename = "service_share_amount", default)]
    pub service_share_amount: i64,
    #[serde(rename = "platform_share_amount", default)]
    pub platform_share_amount: i64,
    #[serde(rename = "dealer_share_amount", default)]
    pub dealer_share_amount: i64,
    #[serde(rename = "dealer_corp_info", default)]
    pub dealer_corp_info: DealerCorpInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DealerCorpInfo {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "corp_name", default)]
    pub corp_name: String,
}

impl WxCpTpOrderDetails {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpOrderDetails 解析失败: {e}"))
    }
}

impl WxCpTpOrderDetails {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpOrderDetails 序列化失败: {e}"))
    }
}
