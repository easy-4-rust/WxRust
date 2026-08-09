//! WxChannelLiveDashboardService（对应 Java `me.chanjar.weixin.channel.api.WxChannelLiveDashboardService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::live::dashboard::{LiveDataResponse, LiveListResponse};

/// 视频号助手 直播大屏数据服务（对应 Java `WxChannelLiveDashboardService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_channel_live_dashboard_service_impl` 的
/// `WxChannelLiveDashboardServiceImpl`（Java `WxChannelLiveDashboardServiceImpl`）。
#[async_trait]
pub trait WxChannelLiveDashboardService: Send + Sync {
    /// 获取直播大屏直播列表（对应 Java `WxChannelLiveDashboardService#getLiveList`；
    /// `ds`：日期，格式 yyyyMMdd）。
    async fn get_live_list(&self, ds: Option<i64>) -> Result<LiveListResponse, WxErrorException>;

    /// 获取直播大屏数据（对应 Java `WxChannelLiveDashboardService#getLiveData`；
    /// `export_id`：直播唯一 ID）。
    async fn get_live_data(&self, export_id: String) -> Result<LiveDataResponse, WxErrorException>;
}
