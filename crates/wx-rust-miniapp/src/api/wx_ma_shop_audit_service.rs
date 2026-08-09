//! 小程序交易组件-接入商品前必需接口（审核相关接口）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopAuditService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::{WxMaShopAuditBrandRequest, WxMaShopAuditCategoryRequest};
use crate::bean::shop::response::{
    WxMaShopAuditBrandResponse, WxMaShopAuditCategoryResponse, WxMaShopAuditResultResponse,
};

/// 小程序交易组件-审核相关服务。
#[async_trait]
pub trait WxMaShopAuditService: Send + Sync {
    /// 上传品牌信息（品牌审核，对应 Java `auditBrand(WxMaShopAuditBrandRequest)`）。
    async fn audit_brand(
        &self,
        request: &WxMaShopAuditBrandRequest,
    ) -> Result<WxMaShopAuditBrandResponse, WxErrorException>;

    /// 上传类目资质（类目审核，对应 Java `auditCategory(WxMaShopAuditCategoryRequest)`）。
    async fn audit_category(
        &self,
        request: &WxMaShopAuditCategoryRequest,
    ) -> Result<WxMaShopAuditCategoryResponse, WxErrorException>;

    /// 获取审核结果（对应 Java `getAuditResult(String)`）。
    async fn get_audit_result(
        &self,
        audit_id: &str,
    ) -> Result<WxMaShopAuditResultResponse, WxErrorException>;

    /// 获取小程序提交过的入驻资质信息（对应 Java `getMiniappCertificate(int)`，
    /// 返回完整响应 JSON，Java 为 JsonObject）。
    async fn get_miniapp_certificate(
        &self,
        req_type: i32,
    ) -> Result<serde_json::Value, WxErrorException>;
}
