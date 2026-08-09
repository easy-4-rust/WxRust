//! 企业微信第三方应用 OA 服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpOAService`：
//! 代应用可见范围内员工提交审批申请/获取审批模板详情/复制模板/获取
//! 审批申请详情（均使用授权企业的 access_token）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpApprovalDetailResult, WxCpOaApplyEventRequest, WxCpOaApprovalTemplateResult,
};

/// 企业微信第三方应用 OA 服务。
#[async_trait]
pub trait WxCpTpOAService: Send + Sync {
    /// 提交审批申请（对应 Java `apply(WxCpOaApplyEventRequest, String)`，
    /// 返回表单编号 `sp_no`）。
    async fn apply(
        &self,
        request: &WxCpOaApplyEventRequest,
        corp_id: &str,
    ) -> Result<String, WxErrorException>;

    /// 获取审批模板详情（对应 Java `getTemplateDetail(String, String)`）。
    async fn get_template_detail(
        &self,
        template_id: &str,
        corp_id: &str,
    ) -> Result<WxCpOaApprovalTemplateResult, WxErrorException>;

    /// 复制/更新模板到企业（对应 Java `copyTemplate(String, String)`，
    /// 返回模板 id）。
    async fn copy_template(
        &self,
        open_template_id: &str,
        corp_id: &str,
    ) -> Result<String, WxErrorException>;

    /// 获取审批申请详情（对应 Java `getApprovalDetail(String, String)`）。
    async fn get_approval_detail(
        &self,
        sp_no: &str,
        corp_id: &str,
    ) -> Result<WxCpApprovalDetailResult, WxErrorException>;
}
