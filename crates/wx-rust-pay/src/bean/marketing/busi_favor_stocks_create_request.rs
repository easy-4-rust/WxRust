//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorStocksCreateRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::CustomEntrance;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::DisplayPatternInfo;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::NotifyConfig;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::StockSendRule;
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorStocksCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_name"
    )]
    pub stock_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "belong_merchant"
    )]
    pub belong_merchant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "comment")]
    pub comment: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "goods_name"
    )]
    pub goods_name: Option<String>,
    #[serde(default, rename = "stock_type")]
    pub stock_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_use_rule"
    )]
    pub coupon_use_rule: Option<CouponUseRule>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_send_rule"
    )]
    pub stock_send_rule: Option<StockSendRule>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "custom_entrance"
    )]
    pub custom_entrance: Option<CustomEntrance>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "display_pattern_info"
    )]
    pub display_pattern_info: Option<DisplayPatternInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_code_mode"
    )]
    pub coupon_code_mode: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_config"
    )]
    pub notify_config: Option<NotifyConfig>,
}
