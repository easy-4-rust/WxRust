//! WxLeadComponentService（对应 Java `me.chanjar.weixin.channel.api.WxLeadComponentService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::lead::component::request::{
    GetLeadInfoByComponentRequest, GetLeadsComponentIdRequest,
    GetLeadsComponentPromoteRecordRequest, GetLeadsInfoByRequestIdRequest,
    GetLeadsRequestIdRequest,
};
use crate::bean::lead::component::response::{
    GetLeadsComponentIdResponse, GetLeadsComponentPromoteRecordResponse, GetLeadsRequestIdResponse,
    LeadInfoResponse,
};

/// 视频号助手 留资组件管理服务（对应 Java `WxLeadComponentService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_lead_component_service_impl` 的
/// `WxLeadComponentServiceImpl`（Java `WxLeadComponentServiceImpl`）。
#[async_trait]
pub trait WxLeadComponentService: Send + Sync {
    /// 按时间获取留资信息详情（对应 Java `WxLeadComponentService#getLeadsInfoByComponentId`）。
    async fn get_leads_info_by_component_id(
        &self,
        req: GetLeadInfoByComponentRequest,
    ) -> Result<LeadInfoResponse, WxErrorException>;

    /// 按直播场次获取留资信息详情（对应 Java `WxLeadComponentService#getLeadsInfoByRequestId`）。
    async fn get_leads_info_by_request_id(
        &self,
        req: GetLeadsInfoByRequestIdRequest,
    ) -> Result<LeadInfoResponse, WxErrorException>;

    /// 获取留资 request_id 列表详情（对应 Java `WxLeadComponentService#getLeadsRequestId`）。
    async fn get_leads_request_id(
        &self,
        req: GetLeadsRequestIdRequest,
    ) -> Result<GetLeadsRequestIdResponse, WxErrorException>;

    /// 获取留资组件直播推广记录信息详情（对应 Java
    /// `WxLeadComponentService#getLeadsComponentPromoteRecord`）。
    async fn get_leads_component_promote_record(
        &self,
        req: GetLeadsComponentPromoteRecordRequest,
    ) -> Result<GetLeadsComponentPromoteRecordResponse, WxErrorException>;

    /// 获取留资组件 Id 列表详情（对应 Java `WxLeadComponentService#getLeadsComponentId`）。
    async fn get_leads_component_id(
        &self,
        req: GetLeadsComponentIdRequest,
    ) -> Result<GetLeadsComponentIdResponse, WxErrorException>;
}
