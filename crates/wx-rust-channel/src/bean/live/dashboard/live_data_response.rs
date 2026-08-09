//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDataResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDataResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "trace_id", default)]
    pub trace_id: String,
    #[serde(rename = "live_dashboard_data", default)]
    pub live_dashboard_data: LiveDashboardData,
    #[serde(rename = "live_comparison_index", default)]
    pub live_comparison_index: LiveComparisonIndex,
    #[serde(rename = "live_ec_data_summary", default)]
    pub live_ec_data_summary: LiveEcDataSummary,
    #[serde(rename = "live_ec_conversion_metric", default)]
    pub live_ec_conversion_metric: LiveEcConversionMetric,
    #[serde(rename = "live_ec_profile", default)]
    pub live_ec_profile: LiveEcProfile,
    #[serde(rename = "live_distribution_channel", default)]
    pub live_distribution_channel: LiveDistributionChannel,
    #[serde(rename = "single_live_ec_spu_data_page_v2", default)]
    pub single_live_ec_spu_data_page_v2: SingleLiveEcSpuDataPageV2,
}
