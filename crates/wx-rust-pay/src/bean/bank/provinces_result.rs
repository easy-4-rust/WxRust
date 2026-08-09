//! 对应 Java `com.github.binarywang.wxpay.bean.bank.ProvincesResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvincesResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, rename = "data")]
    pub data: Vec<ProvinceInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvinceInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "province_name"
    )]
    pub province_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "province_code"
    )]
    pub province_code: Option<i32>,
}
