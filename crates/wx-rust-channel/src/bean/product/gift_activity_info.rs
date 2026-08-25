//! 对应 Java `me.chanjar.weixin.channel.bean.product.GiftActivityInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfo {
    /// 活动名称
    #[serde(rename = "activity_name", default)]
    pub activity_name: String,
    /// 活动开始时间
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    /// 活动结束时间
    #[serde(rename = "end_time", default)]
    pub end_time: String,
}
