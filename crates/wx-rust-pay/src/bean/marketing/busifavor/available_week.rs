//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.AvailableWeek.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AvailableWeek {
    #[serde(default, rename = "week_day")]
    pub week_day: Vec<Option<i32>>,
    #[serde(default, rename = "available_day_time")]
    pub available_day_time: Vec<AvailableDayTimeItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AvailableDayTimeItem {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "begin_time"
    )]
    pub begin_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "end_time")]
    pub end_time: Option<i32>,
}
