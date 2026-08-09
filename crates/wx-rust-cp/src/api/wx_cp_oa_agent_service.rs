//! 企业微信自建应用服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaAgentService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpOpenApprovalData;

/// 企业微信自建应用服务。
#[async_trait]
pub trait WxCpOaAgentService: Send + Sync {
    /// 查询第三方应用审批申请当前状态（对应 Java
    /// `WxCpOaAgentService.getOpenApprovalData(String)`）。
    async fn get_open_approval_data(
        &self,
        third_no: &str,
    ) -> Result<WxCpOpenApprovalData, WxErrorException>;
}
