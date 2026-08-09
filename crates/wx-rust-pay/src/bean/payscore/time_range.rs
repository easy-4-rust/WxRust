//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.TimeRange.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "start_time"
    )]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "end_time")]
    pub end_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "start_time_remark"
    )]
    pub start_time_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "end_time_remark"
    )]
    pub end_time_remark: Option<String>,
}
