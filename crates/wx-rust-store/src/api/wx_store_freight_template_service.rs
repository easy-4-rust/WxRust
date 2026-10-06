//! WxStoreFreightTemplateService（对应 Java
//! `com.binarywang.wxjava.store.api.WxStoreFreightTemplateService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::freight::{
    FreightTemplate, TemplateIdResponse, TemplateInfoResponse, TemplateListResponse,
};

/// 运费模板服务（对应 Java `WxStoreFreightTemplateService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_freight_template_service_impl` 的
/// `WxStoreFreightTemplateServiceImpl`（Java `WxStoreFreightTemplateServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreFreightTemplateService: Send + Sync {
    /// 获取运费模板列表（对应 Java
    /// `WxStoreFreightTemplateService#listTemplate(Integer, Integer)`）。
    ///
    /// # 参数
    /// - `offset`：起始位置
    /// - `limit`：拉取个数
    async fn list_template(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<TemplateListResponse, WxErrorException>;

    /// 获取运费模板（对应 Java
    /// `WxStoreFreightTemplateService#getTemplate(String)`）。
    async fn get_template(
        &self,
        template_id: String,
    ) -> Result<TemplateInfoResponse, WxErrorException>;

    /// 添加运费模板（对应 Java
    /// `WxStoreFreightTemplateService#addTemplate(FreightTemplate)`）。
    async fn add_template(
        &self,
        template: FreightTemplate,
    ) -> Result<TemplateIdResponse, WxErrorException>;

    /// 更新运费模板（对应 Java
    /// `WxStoreFreightTemplateService#updateTemplate(FreightTemplate)`）。
    async fn update_template(
        &self,
        template: FreightTemplate,
    ) -> Result<TemplateIdResponse, WxErrorException>;
}
