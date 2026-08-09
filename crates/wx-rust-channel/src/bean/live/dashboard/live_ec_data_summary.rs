//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveEcDataSummary.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveEcDataSummary {
    #[serde(rename = "total_gmv", default)]
    pub total_gmv: i64,
    #[serde(rename = "total_pay_pv", default)]
    pub total_pay_pv: i64,
    #[serde(rename = "total_pay_uv", default)]
    pub total_pay_uv: i64,
    #[serde(rename = "total_create_pv", default)]
    pub total_create_pv: i64,
    #[serde(rename = "total_create_uv", default)]
    pub total_create_uv: i64,
    #[serde(rename = "total_clk_pv", default)]
    pub total_clk_pv: i64,
    #[serde(rename = "total_clk_uv", default)]
    pub total_clk_uv: i64,
    #[serde(rename = "total_exp_pv", default)]
    pub total_exp_pv: i64,
    #[serde(rename = "total_exp_uv", default)]
    pub total_exp_uv: i64,
    #[serde(rename = "online_audience_count", default)]
    pub online_audience_count: i64,
    #[serde(rename = "cumulative_audience_count", default)]
    pub cumulative_audience_count: i64,
    #[serde(rename = "new_audience_count", default)]
    pub new_audience_count: i64,
    #[serde(rename = "leaved_audience_count", default)]
    pub leaved_audience_count: i64,
    #[serde(rename = "average_watch_seconds_per_audience", default)]
    pub average_watch_seconds_per_audience: i64,
    #[serde(rename = "new_follow_count", default)]
    pub new_follow_count: i64,
    #[serde(rename = "new_comment_count", default)]
    pub new_comment_count: i64,
    #[serde(rename = "share_live_audience_count", default)]
    pub share_live_audience_count: i64,
    #[serde(rename = "new_fans_club_count", default)]
    pub new_fans_club_count: i64,
    #[serde(rename = "refund_pv", default)]
    pub refund_pv: i64,
    #[serde(rename = "refund_uv", default)]
    pub refund_uv: i64,
    #[serde(rename = "refund_rate", default)]
    pub refund_rate: f64,
    #[serde(rename = "refund_amount", default)]
    pub refund_amount: i64,
    #[serde(rename = "refund_product_cnt", default)]
    pub refund_product_cnt: i64,
    #[serde(rename = "ads_cumulative_audience_count", default)]
    pub ads_cumulative_audience_count: i64,
    #[serde(rename = "ads_cumulative_watch_count", default)]
    pub ads_cumulative_watch_count: i64,
    #[serde(rename = "promotion_cumulative_watch_count", default)]
    pub promotion_cumulative_watch_count: i64,
    #[serde(rename = "gmv_per_thousand_cumulative_watch_pv", default)]
    pub gmv_per_thousand_cumulative_watch_pv: f64,
    #[serde(rename = "audience_pay_ratio", default)]
    pub audience_pay_ratio: f64,
    #[serde(rename = "clk_pay_ratio", default)]
    pub clk_pay_ratio: f64,
    #[serde(rename = "new_buyer_uv", default)]
    pub new_buyer_uv: i64,
    #[serde(rename = "old_buyer_uv", default)]
    pub old_buyer_uv: i64,
    #[serde(rename = "customer_price", default)]
    pub customer_price: i64,
}
