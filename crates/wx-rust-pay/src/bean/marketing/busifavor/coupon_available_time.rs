//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.CouponAvailableTime.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponAvailableTime {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_begin_time"
    )]
    pub available_begin_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_end_time"
    )]
    pub available_end_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_day_after_receive"
    )]
    pub available_day_after_receive: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_week"
    )]
    pub available_week: Option<AvailableWeek>,
    #[serde(default, rename = "irregulary_avaliable_time")]
    pub irregulary_avaliable_time: Vec<IrregularyAvaliableTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wait_days_after_receive"
    )]
    pub wait_days_after_receive: Option<i32>,
}
