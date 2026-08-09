//! WxChannelCategoryServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelCategoryServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_category_service::WxChannelCategoryService;
use crate::bean::audit::{
    AuditApplyResponse, AuditResponse, CategoryAuditInfo, CategoryAuditRequest,
};
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::category::{
    CategoryDetailResult, CategoryQualificationResponse, PassCategoryResponse,
    RelationCategoryRequest, RelationCategoryResponse, ShopCategory, ShopCategoryResponse,
};
use crate::enums::url_category as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 商品类目服务实现。
pub struct WxChannelCategoryServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelCategoryServiceImpl {
    /// 构建商品类目服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelCategoryService for WxChannelCategoryServiceImpl {
    /// 对应 Java `WxChannelCategoryServiceImpl.listAllCategory`：
    /// GET `LIST_ALL_CATEGORY_URL`（数据量太大不记录日志，Java
    /// `executeWithoutLog` 语义一致）。
    async fn list_all_category(&self) -> Result<CategoryQualificationResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.get(url::LIST_ALL_CATEGORY_URL, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.listAvailableCategory`：
    /// `f_cat_id` 必须为数字，否则返回空列表（Java `Collections.emptyList()`）。
    async fn list_available_category(
        &self,
        f_cat_id: String,
    ) -> Result<Vec<ShopCategory>, WxErrorException> {
        let Ok(pid) = f_cat_id.parse::<i64>() else {
            // Java：log.error("parentId必须为数字") 后返回空列表
            return Ok(Vec::new());
        };
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = format!(r#"{{"f_cat_id": {pid}}}"#);
        let response = svc.post(url::AVAILABLE_CATEGORY_URL, &body).await?;
        let shop_category_response: ShopCategoryResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(shop_category_response.categories)
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.listAvailableCategories`：
    /// `{"f_cat_id": <fCatId>}`（裸数字）后 POST `AVAILABLE_CATEGORY_URL`。
    async fn list_available_categories(
        &self,
        f_cat_id: String,
    ) -> Result<ShopCategoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[(
            "f_cat_id",
            f_cat_id
                .parse::<i64>()
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::String(f_cat_id)),
        )]);
        let response = svc.post(url::AVAILABLE_CATEGORY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.getCategoryDetail`：
    /// `id` 必须为数字，否则返回内部错误（err_code=-99）。
    async fn get_category_detail(
        &self,
        id: String,
    ) -> Result<CategoryDetailResult, WxErrorException> {
        let Ok(cat_id) = id.parse::<i64>() else {
            // Java：log.error("id必须为数字") 后返回 internalError
            return Ok(CategoryDetailResult {
                err_code: -99,
                err_msg: "内部错误".to_string(),
                ..Default::default()
            });
        };
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = format!(r#"{{"cat_id": {cat_id}}}"#);
        let response = svc.post(url::GET_CATEGORY_DETAIL_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.addCategory(String, String, String, List)`：
    /// 三级类目 ID 必须为数字；解析失败时返回内部错误响应。
    async fn add_category(
        &self,
        level1: String,
        level2: String,
        level3: String,
        certificate: Vec<String>,
    ) -> Result<AuditApplyResponse, WxErrorException> {
        let (Ok(l1), Ok(l2), Ok(l3)) = (
            level1.parse::<i64>(),
            level2.parse::<i64>(),
            level3.parse::<i64>(),
        ) else {
            // Java：log.error("微信请求异常") 后以 null 请求体发起请求；
            // Rust 以内部错误响应表达该退化路径
            return Ok(AuditApplyResponse {
                err_code: -99,
                err_msg: "内部错误".to_string(),
                ..Default::default()
            });
        };
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let info = CategoryAuditInfo {
            level1: l1,
            level2: l2,
            level3: l3,
            certificates: certificate,
            ..Default::default()
        };
        let request = CategoryAuditRequest {
            category_info: info,
        };
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_CATEGORY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.addCategory(CategoryAuditInfo)`：
    /// 序列化 `CategoryAuditRequest` 后 POST `ADD_CATEGORY_URL`。
    async fn add_category_by_info(
        &self,
        info: CategoryAuditInfo,
    ) -> Result<AuditApplyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let request = CategoryAuditRequest {
            category_info: info,
        };
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_CATEGORY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.cancelCategoryAudit`：
    /// `{"audit_id": ".."}` 后 POST `CANCEL_CATEGORY_AUDIT_URL`。
    async fn cancel_category_audit(
        &self,
        audit_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("audit_id", serde_json::Value::String(audit_id))]);
        let response = svc.post(url::CANCEL_CATEGORY_AUDIT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.getAudit`：
    /// `{"audit_id": ".."}` 后 POST `GET_CATEGORY_AUDIT_URL`。
    async fn get_audit(&self, audit_id: String) -> Result<AuditResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("audit_id", serde_json::Value::String(audit_id))]);
        let response = svc.post(url::GET_CATEGORY_AUDIT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.listPassCategory`：
    /// GET `LIST_PASS_CATEGORY_URL`。
    async fn list_pass_category(&self) -> Result<PassCategoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.get(url::LIST_PASS_CATEGORY_URL, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelCategoryServiceImpl.listRelationCategory`：
    /// `RelationCategoryRequest(isFilterStatus != null ? isFilterStatus : false,
    /// status != null ? status : 0)`（两字段恒非空）后 POST
    /// `LIST_RELATION_CATEGORY_URL`。
    async fn list_relation_category(
        &self,
        is_filter_status: Option<bool>,
        status: Option<i32>,
    ) -> Result<RelationCategoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let request = RelationCategoryRequest {
            is_filter_status: is_filter_status.unwrap_or(false),
            status: status.unwrap_or(0),
        };
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::LIST_RELATION_CATEGORY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
