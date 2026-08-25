//! WxChannelQicService（对应 Java `me.chanjar.weixin.channel.api.WxChannelQicService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::qic::{
    InspectCodeResponse, InspectConfigResponse, RegisterLogisticsRequest, SubmitConfigResponse,
    SubmitInspectRequest,
};

/// 质检管理服务（对应 Java `WxChannelQicService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_qic_service_impl` 的
/// `WxChannelQicServiceImpl`（Java `WxChannelQicServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelQicService: Send + Sync {
    /// 查询质检仓配置（对应 Java `WxChannelQicService#getInspectConfig()`）。
    async fn get_inspect_config(&self) -> Result<InspectConfigResponse, WxErrorException>;

    /// 查询送检配置模板信息（对应 Java `WxChannelQicService#getSubmitConfig(String)`）。
    async fn get_submit_config_with_order(
        &self,
        order_id: String,
    ) -> Result<SubmitConfigResponse, WxErrorException>;

    /// 查询送检配置模板信息（对应 Java `WxChannelQicService#getSubmitConfig()`）。
    async fn get_submit_config(&self) -> Result<SubmitConfigResponse, WxErrorException>;

    /// 打印质检码（对应 Java `WxChannelQicService#printInspectCode(String)`）。
    async fn print_inspect_code(
        &self,
        order_id: String,
    ) -> Result<InspectCodeResponse, WxErrorException>;

    /// 绑定送检信息（对应 Java `WxChannelQicService#submitInspectInfo(SubmitInspectRequest)`）。
    async fn submit_inspect_info(
        &self,
        request: SubmitInspectRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 自寄快递送检（对应 Java `WxChannelQicService#registerLogistics(RegisterLogisticsRequest)`）。
    async fn register_logistics(
        &self,
        request: RegisterLogisticsRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
