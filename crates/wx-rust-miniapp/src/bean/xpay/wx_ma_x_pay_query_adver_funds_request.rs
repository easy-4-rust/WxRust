//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryAdverFundsRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryAdverFundsRequest {
    #[serde(rename = "page", default)]
    pub page: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "filter", default)]
    pub filter: Filter,
    #[serde(rename = "env", default)]
    pub env: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Filter {
    #[serde(rename = "settle_begin", default)]
    pub settle_begin: i64,
    #[serde(rename = "settle_end", default)]
    pub settle_end: i64,
    #[serde(rename = "fund_type", default)]
    pub fund_type: i32,
}

impl WxMaXPayQueryAdverFundsRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryAdverFundsRequest 序列化失败: {e}"))
    }
}
