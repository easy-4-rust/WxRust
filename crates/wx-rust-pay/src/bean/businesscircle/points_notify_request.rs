//! 对应 Java `com.github.binarywang.wxpay.bean.businesscircle.PointsNotifyRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsNotifyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "earn_points"
    )]
    pub earn_points: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "increased_points"
    )]
    pub increased_points: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "points_update_time"
    )]
    pub points_update_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "no_points_remarks"
    )]
    pub no_points_remarks: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_points"
    )]
    pub total_points: Option<i32>,
}
