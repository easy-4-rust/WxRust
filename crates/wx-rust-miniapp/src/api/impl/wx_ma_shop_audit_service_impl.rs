//! 小程序交易组件-审核相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopAuditServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopAuditService;
use crate::bean::shop::request::{WxMaShopAuditBrandRequest, WxMaShopAuditCategoryRequest};
use crate::bean::shop::response::{
    WxMaShopAuditBrandResponse, WxMaShopAuditCategoryResponse, WxMaShopAuditResultResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_audit as audit_url;

/// 小程序交易组件-审核相关服务实现。
pub struct WxMaShopAuditServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopAuditServiceImpl {
    /// 构建审核服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopAuditService for WxMaShopAuditServiceImpl {
    /// 对应 Java `WxMaShopAuditServiceImpl.auditBrand`：
    /// POST `AUDIT_BRAND` 后校验 errcode 并解析响应。
    async fn audit_brand(
        &self,
        request: &WxMaShopAuditBrandRequest,
    ) -> Result<WxMaShopAuditBrandResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&audit_url::audit_brand_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAuditServiceImpl.auditCategory`：
    /// POST `AUDIT_CATEGORY` 后校验 errcode 并解析响应。
    async fn audit_category(
        &self,
        request: &WxMaShopAuditCategoryRequest,
    ) -> Result<WxMaShopAuditCategoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&audit_url::audit_category_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAuditServiceImpl.getAuditResult`：
    /// 构造 `{"audit_id": auditId}` 后 POST `AUDIT_RESULT` 并解析响应。
    async fn get_audit_result(
        &self,
        audit_id: &str,
    ) -> Result<WxMaShopAuditResultResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "audit_id": audit_id }).to_string();
        let response = svc
            .post(&audit_url::audit_result_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAuditServiceImpl.getMiniappCertificate`：
    /// 构造 `{"req_type": reqType}` 后 POST `GET_MINIAPP_CERTIFICATE`，
    /// 返回完整响应 JSON（Java 返回 `JsonObject`）。
    async fn get_miniapp_certificate(
        &self,
        req_type: i32,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "req_type": req_type }).to_string();
        let response = svc
            .post(
                &audit_url::get_miniapp_certificate_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
