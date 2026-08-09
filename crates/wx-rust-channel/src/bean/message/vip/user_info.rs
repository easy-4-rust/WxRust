//! 用户信息（会员）。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.vip.UserInfo.java`。

use serde::{Deserialize, Serialize};

/// 用户信息（对应 Java `UserInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    /// 入会时间（对应 Java `joinTime`）。
    #[serde(
        rename = "join_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub join_time: Option<i64>,
    /// 注销时间（对应 Java `closeTime`）。
    #[serde(
        rename = "close_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub close_time: Option<i64>,
    /// 手机号（对应 Java `phoneNumber`）。
    #[serde(rename = "phone_number", default)]
    pub phone_number: Option<String>,
    /// 等级（对应 Java `grade`）。
    #[serde(
        rename = "grade",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub grade: Option<i32>,
    /// 当前等级经验值（对应 Java `experienceValue`）。
    #[serde(
        rename = "experience_value",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub experience_value: Option<i64>,
    /// 当前积分（对应 Java `score`）。
    #[serde(
        rename = "score",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub score: Option<i64>,
    /// 本次改动积分,负数减少，正数新增（对应 Java `deltaScore`）。
    #[serde(
        rename = "delta_score",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub delta_score: Option<i64>,
}
