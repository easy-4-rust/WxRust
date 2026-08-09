//! WxChannelCategoryService（对应 Java `me.chanjar.weixin.channel.api.WxChannelCategoryService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::audit::{AuditApplyResponse, AuditResponse, CategoryAuditInfo};
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::category::{
    CategoryDetailResult, CategoryQualificationResponse, PassCategoryResponse,
    RelationCategoryResponse, ShopCategory, ShopCategoryResponse,
};

/// 商品类目服务（对应 Java `WxChannelCategoryService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_category_service_impl` 的
/// `WxChannelCategoryServiceImpl`（Java `WxChannelCategoryServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelCategoryService: Send + Sync {
    /// 获取所有的类目（对应 Java `WxChannelCategoryService#listAllCategory`）。
    async fn list_all_category(&self) -> Result<CategoryQualificationResponse, WxErrorException>;

    /// 获取商品类目列表（全量），有频率限制（对应 Java
    /// `WxChannelCategoryService#listAvailableCategory(String)`，
    /// `@Deprecated`，请使用 `list_available_categories`）。
    async fn list_available_category(
        &self,
        f_cat_id: String,
    ) -> Result<Vec<ShopCategory>, WxErrorException>;

    /// 获取可用的子类目详情（对应 Java
    /// `WxChannelCategoryService#listAvailableCategories(String)`）。
    ///
    /// # 参数
    /// - `f_cat_id`：父类目 ID，可先填 0 获取根部类目
    async fn list_available_categories(
        &self,
        f_cat_id: String,
    ) -> Result<ShopCategoryResponse, WxErrorException>;

    /// 获取类目信息（对应 Java `WxChannelCategoryService#getCategoryDetail(String)`）。
    ///
    /// # 参数
    /// - `id`：三级类目 id
    async fn get_category_detail(
        &self,
        id: String,
    ) -> Result<CategoryDetailResult, WxErrorException>;

    /// 上传类目资质（对应 Java
    /// `WxChannelCategoryService#addCategory(String, String, String, List<String>)`，
    /// `@Deprecated`，请使用 `add_category_by_info`）。
    async fn add_category(
        &self,
        level1: String,
        level2: String,
        level3: String,
        certificate: Vec<String>,
    ) -> Result<AuditApplyResponse, WxErrorException>;

    /// 上传类目资质（对应 Java `WxChannelCategoryService#addCategory(CategoryAuditInfo)`）。
    async fn add_category_by_info(
        &self,
        info: CategoryAuditInfo,
    ) -> Result<AuditApplyResponse, WxErrorException>;

    /// 取消类目提审（对应 Java `WxChannelCategoryService#cancelCategoryAudit(String)`）。
    async fn cancel_category_audit(
        &self,
        audit_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询类目审核结果（对应 Java `WxChannelCategoryService#getAudit(String)`）。
    async fn get_audit(&self, audit_id: String) -> Result<AuditResponse, WxErrorException>;

    /// 获取账号申请通过的类目和资质信息（对应 Java
    /// `WxChannelCategoryService#listPassCategory`）。
    async fn list_pass_category(&self) -> Result<PassCategoryResponse, WxErrorException>;

    /// 获取店铺的类目权限列表（对应 Java
    /// `WxChannelCategoryService#listRelationCategory(Boolean, Integer)`）。
    ///
    /// # 参数
    /// - `is_filter_status`：是否按状态筛选
    /// - `status`：类目状态（当 `is_filter_status` 为 true 时有效）
    async fn list_relation_category(
        &self,
        is_filter_status: Option<bool>,
        status: Option<i32>,
    ) -> Result<RelationCategoryResponse, WxErrorException>;
}
