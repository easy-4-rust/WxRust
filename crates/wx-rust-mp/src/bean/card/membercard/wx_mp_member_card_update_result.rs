//! 对应 Java `bean.card.membercard.WxMpMemberCardUpdateResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::card::enums::*;
use crate::bean::card::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMemberCardUpdateResult {
    #[serde(rename = "errorCode", default)]
    pub error_code: String,
    #[serde(rename = "errorMsg", default)]
    pub error_msg: String,
    #[serde(rename = "openId", default)]
    pub open_id: String,
    #[serde(rename = "resultBonus", default)]
    pub result_bonus: i32,
    #[serde(rename = "resultBalance", default)]
    pub result_balance: f64,
}

impl WxMpMemberCardUpdateResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpMemberCardUpdateResult 解析失败: {e}"))
    }
}
