//! WxFinderLiveService（对应 Java `me.chanjar.weixin.channel.api.WxFinderLiveService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::lead::component::request::{
    GetFinderLiveDataListRequest, GetFinderLiveLeadsDataRequest,
};
use crate::bean::lead::component::response::{
    FinderAttrResponse, GetFinderLiveDataListResponse, GetFinderLiveLeadsDataResponse,
};

/// 视频号助手 留资服务的直播数据服务（对应 Java `WxFinderLiveService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_finder_live_service_impl` 的
/// `WxFinderLiveServiceImpl`（Java `WxFinderLiveServiceImpl`）。
#[async_trait]
pub trait WxFinderLiveService: Send + Sync {
    /// 获取视频号账号信息（对应 Java `WxFinderLiveService#getFinderAttrByAppid`）。
    async fn get_finder_attr_by_appid(&self) -> Result<FinderAttrResponse, WxErrorException>;

    /// 获取留资直播间数据详情（对应 Java `WxFinderLiveService#getFinderLiveDataList`）。
    async fn get_finder_live_data_list(
        &self,
        req: GetFinderLiveDataListRequest,
    ) -> Result<GetFinderLiveDataListResponse, WxErrorException>;

    /// 获取账号收集的留资数量（对应 Java `WxFinderLiveService#getFinderLiveLeadsData`；
    /// 该接口只统计 2023.9.13 起的数据，start_time 应大于等于 1694534400）。
    async fn get_finder_live_leads_data(
        &self,
        req: GetFinderLiveLeadsDataRequest,
    ) -> Result<GetFinderLiveLeadsDataResponse, WxErrorException>;
}
