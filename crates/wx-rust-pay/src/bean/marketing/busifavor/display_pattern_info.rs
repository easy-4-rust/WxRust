//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.DisplayPatternInfo.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisplayPatternInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_logo_url"
    )]
    pub merchant_logo_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_name"
    )]
    pub merchant_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "background_color"
    )]
    pub background_color: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_image_url"
    )]
    pub coupon_image_url: Option<String>,
}
