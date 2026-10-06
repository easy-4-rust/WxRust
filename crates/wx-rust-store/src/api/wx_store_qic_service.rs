//! WxStoreQicService（对应 Java `com.binarywang.wxjava.store.api.WxStoreQicService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::qic::{
    InspectCodeResponse, InspectConfigResponse, RegisterLogisticsRequest, SubmitConfigResponse,
    SubmitInspectRequest,
};

/// 质检管理服务（对应 Java `WxStoreQicService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_qic_service_impl` 的
/// `WxStoreQicServiceImpl`（Java `WxStoreQicServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreQicService: Send + Sync {
    /// 查询质检仓配置（对应 Java `WxStoreQicService#getInspectConfig()`）。
    async fn get_inspect_config(&self) -> Result<InspectConfigResponse, WxErrorException>;

    /// 查询送检配置模板信息（对应 Java `WxStoreQicService#getSubmitConfig(String)`）。
    async fn get_submit_config_with_order(
        &self,
        order_id: String,
    ) -> Result<SubmitConfigResponse, WxErrorException>;

    /// 查询送检配置模板信息（对应 Java `WxStoreQicService#getSubmitConfig()`）。
    async fn get_submit_config(&self) -> Result<SubmitConfigResponse, WxErrorException>;

    /// 打印质检码（对应 Java `WxStoreQicService#printInspectCode(String)`）。
    async fn print_inspect_code(
        &self,
        order_id: String,
    ) -> Result<InspectCodeResponse, WxErrorException>;

    /// 绑定送检信息（对应 Java `WxStoreQicService#submitInspectInfo(SubmitInspectRequest)`）。
    async fn submit_inspect_info(
        &self,
        request: SubmitInspectRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 自寄快递送检（对应 Java `WxStoreQicService#registerLogistics(RegisterLogisticsRequest)`）。
    async fn register_logistics(
        &self,
        request: RegisterLogisticsRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
