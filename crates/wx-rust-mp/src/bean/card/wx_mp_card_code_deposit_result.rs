//! 对应 Java `bean.card.WxMpCardCodeDepositResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCardCodeDepositResult {
    #[serde(rename = "succ_code", default)]
    pub success_codes: Vec<String>,
    #[serde(rename = "duplicate_code", default)]
    pub duplicate_codes: Vec<String>,
    #[serde(rename = "fail_code", default)]
    pub fail_codes: Vec<String>,
}
