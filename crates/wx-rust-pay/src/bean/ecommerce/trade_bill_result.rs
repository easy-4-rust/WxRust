//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.TradeBillResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TradeBillResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hash_type")]
    pub hash_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hash_value"
    )]
    pub hash_value: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "download_url"
    )]
    pub download_url: Option<String>,
}
