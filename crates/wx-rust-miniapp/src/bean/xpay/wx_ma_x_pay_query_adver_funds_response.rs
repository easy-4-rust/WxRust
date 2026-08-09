//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryAdverFundsResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryAdverFundsResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "adver_funds_list", default)]
    pub adver_funds_list: Vec<AdverFunds>,
    #[serde(rename = "total_page", default)]
    pub total_page: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdverFunds {
    #[serde(rename = "settle_begin", default)]
    pub settle_begin: i64,
    #[serde(rename = "settle_end", default)]
    pub settle_end: i64,
    #[serde(rename = "total_amount", default)]
    pub total_amount: i32,
    #[serde(rename = "remain_amount", default)]
    pub remain_amount: i32,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "fund_type", default)]
    pub fund_type: i32,
    #[serde(rename = "fund_id", default)]
    pub fund_id: String,
}

impl WxMaXPayQueryAdverFundsResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryAdverFundsResponse 序列化失败: {e}"))
    }
}
