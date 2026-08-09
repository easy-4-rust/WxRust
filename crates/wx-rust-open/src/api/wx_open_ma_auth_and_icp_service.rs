//! 小程序认证及备案服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaAuthAndIcpService`。
//! 微信第三方平台 小程序认证及备案。
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]
//! （`ma_auth_and_icp_query_url`/`ma_auth_and_icp_submit_url`，api_host
//! 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxOpenQueryAuthAndIcpResult;
use crate::bean::WxOpenSubmitAuthAndIcpParam;
use crate::bean::WxOpenSubmitAuthAndIcpResult;

/// 微信第三方平台 小程序认证及备案服务（对应 Java
/// `WxOpenMaAuthAndIcpService`）。
#[async_trait]
pub trait WxOpenMaAuthAndIcpService: Send + Sync {
    /// 查询小程序认证及备案进度（对应 Java
    /// `queryAuthAndIcp(String procedureId)`）。
    ///
    /// `procedure_id`：小程序认证及备案任务流程 id。
    async fn query_auth_and_icp(
        &self,
        procedure_id: &str,
    ) -> Result<WxOpenQueryAuthAndIcpResult, WxErrorException>;

    /// 提交小程序认证及备案信息（对应 Java
    /// `submitAuthAndIcp(WxOpenSubmitAuthAndIcpParam param)`）。
    async fn submit_auth_and_icp(
        &self,
        param: &WxOpenSubmitAuthAndIcpParam,
    ) -> Result<WxOpenSubmitAuthAndIcpResult, WxErrorException>;
}
