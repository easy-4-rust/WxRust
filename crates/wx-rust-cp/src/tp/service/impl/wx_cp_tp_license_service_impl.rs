//! 企业微信第三方应用接口许可服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpLicenseServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面；所有请求拼接
//! `?provider_access_token=<token>` 且服务商 token 获取自带，请求经
//! 门面 post 通道（Java 原样，未显式 withoutSuiteToken）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpBaseResp;
use crate::bean::license::account::{
    WxCpTpLicenseActiveInfoByUserResp, WxCpTpLicenseBatchActiveResultResp,
    WxCpTpLicenseBatchCodeInfoResp, WxCpTpLicenseBatchTransferResp, WxCpTpLicenseCodeInfoResp,
    WxCpTpLicenseCorpAccountListResp,
};
use crate::bean::license::order::{
    WxCpTpLicenseCreateOrderResp, WxCpTpLicenseNewOrderRequest, WxCpTpLicenseOrderAccountListResp,
    WxCpTpLicenseOrderInfoResp, WxCpTpLicenseOrderListResp, WxCpTpLicenseRenewOrderJobRequest,
    WxCpTpLicenseRenewOrderJobResp, WxCpTpLicenseRenewOrderRequest,
};
use crate::bean::license::{WxCpTpLicenseActiveAccount, WxCpTpLicenseTransfer};
use crate::enums::url_license;
use crate::tp::service::{WxCpTpLicenseService, WxCpTpService};

/// 企业微信第三方应用接口许可服务实现。
pub struct WxCpTpLicenseServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpLicenseServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 拼接带 provider_access_token 的 URL（对应 Java
    /// `getApiUrl(path) + getProviderAccessToken()`）。
    async fn url_with_provider_token(&self, path: &str) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let provider_access_token = service.get_wx_cp_provider_token().await?;
        Ok(format!(
            "{}?provider_access_token={provider_access_token}",
            config.api_url(path)
        ))
    }
}

#[async_trait]
impl WxCpTpLicenseService for WxCpTpLicenseServiceImpl {
    async fn create_new_order(
        &self,
        request: &WxCpTpLicenseNewOrderRequest,
    ) -> Result<WxCpTpLicenseCreateOrderResp, WxErrorException> {
        let service = self.service()?;
        let url = self
            .url_with_provider_token(url_license::CREATE_NEW_ORDER)
            .await?;
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        let result_text = service.post(&url, &json).await?;
        WxCpTpLicenseCreateOrderResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn create_renew_order_job(
        &self,
        request: &WxCpTpLicenseRenewOrderJobRequest,
    ) -> Result<WxCpTpLicenseRenewOrderJobResp, WxErrorException> {
        let service = self.service()?;
        let url = self
            .url_with_provider_token(url_license::CREATE_RENEW_ORDER_JOB)
            .await?;
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        let result_text = service.post(&url, &json).await?;
        WxCpTpLicenseRenewOrderJobResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn submit_renew_order(
        &self,
        request: &WxCpTpLicenseRenewOrderRequest,
    ) -> Result<WxCpTpLicenseCreateOrderResp, WxErrorException> {
        let service = self.service()?;
        let url = self
            .url_with_provider_token(url_license::SUBMIT_ORDER_JOB)
            .await?;
        let json = request.to_json().map_err(WxErrorException::Serde)?;
        let result_text = service.post(&url, &json).await?;
        WxCpTpLicenseCreateOrderResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_order_list(
        &self,
        corp_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        cursor: &str,
        limit: i32,
    ) -> Result<WxCpTpLicenseOrderListResp, WxErrorException> {
        let service = self.service()?;
        let mut body = serde_json::Map::new();
        body.insert(
            "corpid".to_string(),
            serde_json::Value::String(corp_id.to_string()),
        );
        body.insert(
            "cursor".to_string(),
            serde_json::Value::String(cursor.to_string()),
        );
        body.insert("limit".to_string(), serde_json::Value::Number(limit.into()));
        if let Some(start_time) = start_time {
            body.insert(
                "start_time".to_string(),
                serde_json::Value::Number(start_time.timestamp().into()),
            );
        }
        if let Some(end_time) = end_time {
            body.insert(
                "end_time".to_string(),
                serde_json::Value::Number(end_time.timestamp().into()),
            );
        }
        let url = self
            .url_with_provider_token(url_license::LIST_ORDER)
            .await?;
        let result_text = service
            .post(&url, &serde_json::Value::Object(body).to_string())
            .await?;
        WxCpTpLicenseOrderListResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_order_info(
        &self,
        order_id: &str,
    ) -> Result<WxCpTpLicenseOrderInfoResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "order_id": order_id }).to_string();
        let url = self.url_with_provider_token(url_license::GET_ORDER).await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseOrderInfoResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_order_account_list(
        &self,
        order_id: &str,
        limit: i32,
        cursor: &str,
    ) -> Result<WxCpTpLicenseOrderAccountListResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "order_id": order_id,
            "cursor": cursor,
            "limit": limit,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::LIST_ORDER_ACCOUNT)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseOrderAccountListResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn active_code(
        &self,
        code: &str,
        corp_id: &str,
        user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "active_code": code,
            "corpid": corp_id,
            "userid": user_id,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::ACTIVE_ACCOUNT)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpBaseResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn batch_active_code(
        &self,
        corp_id: &str,
        active_account_list: &[WxCpTpLicenseActiveAccount],
    ) -> Result<WxCpTpLicenseBatchActiveResultResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "corpid": corp_id,
            "active_list": active_account_list,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::BATCH_ACTIVE_ACCOUNT)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseBatchActiveResultResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_active_info_by_code(
        &self,
        code: &str,
        corp_id: &str,
    ) -> Result<WxCpTpLicenseCodeInfoResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "active_code": code,
            "corpid": corp_id,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::GET_ACTIVE_INFO_BY_CODE)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseCodeInfoResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn batch_get_active_info_by_code(
        &self,
        codes: &[String],
        corp_id: &str,
    ) -> Result<WxCpTpLicenseBatchCodeInfoResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "active_code_list": codes,
            "corpid": corp_id,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::BATCH_GET_ACTIVE_INFO_BY_CODE)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseBatchCodeInfoResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_corp_account_list(
        &self,
        corp_id: &str,
        limit: i32,
        cursor: &str,
    ) -> Result<WxCpTpLicenseCorpAccountListResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "corpid": corp_id,
            "cursor": cursor,
            "limit": limit,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::LIST_ACTIVED_ACCOUNT)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseCorpAccountListResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn get_active_info_by_user(
        &self,
        corp_id: &str,
        user_id: &str,
    ) -> Result<WxCpTpLicenseActiveInfoByUserResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "corpid": corp_id,
            "userid": user_id,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::GET_ACTIVE_INFO_BY_USER)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseActiveInfoByUserResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }

    async fn batch_transfer_license(
        &self,
        corp_id: &str,
        transfer_list: &[WxCpTpLicenseTransfer],
    ) -> Result<WxCpTpLicenseBatchTransferResp, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "corpid": corp_id,
            "transfer_list": transfer_list,
        })
        .to_string();
        let url = self
            .url_with_provider_token(url_license::BATCH_TRANSFER_LICENSE)
            .await?;
        let result_text = service.post(&url, &body).await?;
        WxCpTpLicenseBatchTransferResp::from_json(&result_text).map_err(WxErrorException::Serde)
    }
}
