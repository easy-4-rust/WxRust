//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryUserBalanceResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryUserBalanceResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "balance", default)]
    pub balance: i64,
    #[serde(rename = "present_balance", default)]
    pub present_balance: i64,
    #[serde(rename = "sum_save", default)]
    pub sum_save: i64,
    #[serde(rename = "sum_present", default)]
    pub sum_present: i64,
    #[serde(rename = "sum_balance", default)]
    pub sum_balance: i64,
    #[serde(rename = "sum_cost", default)]
    pub sum_cost: i64,
    #[serde(rename = "first_save_flag", default)]
    pub first_save_flag: bool,
}

impl WxMaXPayQueryUserBalanceResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryUserBalanceResponse 序列化失败: {e}"))
    }
}
