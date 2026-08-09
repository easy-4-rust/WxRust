//! 企业微信第三方应用 OA 服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpOAServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面，OA 接口均使用授权企业的
//! access_token（`config.getAccessToken(corpId)`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpApprovalDetailResult, WxCpOaApplyEventRequest, WxCpOaApprovalTemplateResult,
};
use crate::enums::url_oa;
use crate::tp::service::{WxCpTpOAService, WxCpTpService};

/// 企业微信第三方应用 OA 服务实现。
pub struct WxCpTpOAServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpOAServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 拼接带授权企业 access_token 的 URL（对应 Java 各方法内
    /// `getApiUrl(path) + "?access_token=" + getAccessToken(corpId)`）。
    fn url_with_corp_token(
        &self,
        service: &dyn WxCpTpService,
        corp_id: &str,
        path: &str,
    ) -> String {
        let config = service.wx_cp_tp_config_storage();
        format!(
            "{}?access_token={}",
            config.api_url(path),
            config.access_token(corp_id).unwrap_or_default()
        )
    }
}

#[async_trait]
impl WxCpTpOAService for WxCpTpOAServiceImpl {
    async fn apply(
        &self,
        request: &WxCpOaApplyEventRequest,
        corp_id: &str,
    ) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_oa::APPLY_EVENT);
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        let response_content = service.post(&url, &json).await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        tmp.get("sp_no")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "sp_no 字段缺失"))
    }

    async fn get_template_detail(
        &self,
        template_id: &str,
        corp_id: &str,
    ) -> Result<WxCpOaApprovalTemplateResult, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "template_id": template_id }).to_string();
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_oa::GET_TEMPLATE_DETAIL);
        let response_content = service.post(&url, &body).await?;
        WxCpOaApprovalTemplateResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn copy_template(
        &self,
        open_template_id: &str,
        corp_id: &str,
    ) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "open_template_id": open_template_id }).to_string();
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_oa::COPY_TEMPLATE);
        let response_content = service.post(&url, &body).await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        tmp.get("template_id")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "template_id 字段缺失"))
    }

    async fn get_approval_detail(
        &self,
        sp_no: &str,
        corp_id: &str,
    ) -> Result<WxCpApprovalDetailResult, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "sp_no": sp_no }).to_string();
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_oa::GET_APPROVAL_DETAIL);
        let response_content = service.post(&url, &body).await?;
        // Java `WxCpGsonBuilder.fromJson(response, WxCpApprovalDetailResult.class)`
        serde_json::from_str(&response_content).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
