//! 微信小商店开店服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMinishopService`。
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`minishop_*_url`，
//! api_host 前缀模式）。
//!
//! 注意：Java 实现（`WxOpenMinishopServiceImpl`）多数方法为上游桩
//! （执行请求后恒 `return null` 或不执行直接 `return null`），Rust
//! 严格镜像（`Ok(None)`），见 impl 注释。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::MinishopAuditStatus;
use crate::bean::MinishopBrandList;
use crate::bean::MinishopBusiLicense;
use crate::bean::MinishopCategories;
use crate::bean::MinishopIdcardInfo;
use crate::bean::MinishopNameInfo;
use crate::bean::MinishopOrganizationCodeInfo;
use crate::bean::MinishopReturnInfo;
use crate::bean::MinishopShopCatList;
use crate::bean::MinishopSuperAdministratorInfo;
use crate::bean::WxOpenResult;

/// 微信小商店开店服务（对应 Java `WxOpenMinishopService`）。
#[async_trait]
pub trait WxOpenMinishopService: Send + Sync {
    /// 提交小商店商户信息（对应 Java `submitMerchantInfo(String appId,
    /// String subjectType, MinishopBusiLicense busiLicense,
    /// MinishopOrganizationCodeInfo organizationCodeInfo, MinishopIdcardInfo
    /// idcardInfo, MinishopSuperAdministratorInfo superAdministratorInfo,
    /// String merchantShoprtName)`）。
    ///
    /// Java 实现为上游桩（执行请求后恒 `return null`）→ `Ok(None)` 镜像。
    #[allow(clippy::too_many_arguments)]
    async fn submit_merchant_info(
        &self,
        app_id: &str,
        subject_type: &str,
        busi_license: &MinishopBusiLicense,
        organization_code_info: &MinishopOrganizationCodeInfo,
        idcard_info: &MinishopIdcardInfo,
        super_administrator_info: &MinishopSuperAdministratorInfo,
        merchant_shortname: &str,
    ) -> Result<Option<WxOpenResult>, WxErrorException>;

    /// 提交小商店基础信息（对应 Java `submitBasicInfo(String appId,
    /// MinishopNameInfo nameInfo, MinishopReturnInfo returnInfo)`）。
    ///
    /// Java 实现为上游桩（不执行请求直接 `return null`）→ `Ok(None)`
    /// 镜像。
    async fn submit_basic_info(
        &self,
        app_id: &str,
        name_info: &MinishopNameInfo,
        return_info: &MinishopReturnInfo,
    ) -> Result<Option<WxOpenResult>, WxErrorException>;

    /// 异步状态查询（对应 Java `checkAuditStatus(String wxName)`）。
    ///
    /// Java 实现为上游桩（不执行请求直接 `return null`）→ `Ok(None)`
    /// 镜像。
    async fn check_audit_status(
        &self,
        wx_name: &str,
    ) -> Result<Option<MinishopAuditStatus>, WxErrorException>;

    /// 上传小商店图片（对应 Java `uploadImagePicFile(Integer height,
    /// Integer width, File file)`）。
    ///
    /// Java：URL 拼接 `?access_token={getAccessToken(true)}&height=
    /// {height}&width={width}` 后以 multipart（字段 `media`）上传，返回
    /// 响应字符串（`post(url, file)` 语义）。ADAPTED：Java `File` 入参 →
    /// Rust 文件路径 `&str`。
    async fn upload_image_pic_file(
        &self,
        height: i32,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 获取小商店类目（对应 Java `getCategory(Integer fCatId)`）。
    ///
    /// Java 实现为上游桩（不执行请求直接 `return null`）→ `Ok(None)`
    /// 镜像。
    async fn get_category(
        &self,
        f_cat_id: i32,
    ) -> Result<Option<MinishopCategories>, WxErrorException>;

    /// 获取小商店品牌（对应 Java `getBrands()`）。
    ///
    /// Java 实现为上游桩（不执行请求直接 `return null`）→ `Ok(None)`
    /// 镜像。
    async fn get_brands(&self) -> Result<Option<MinishopBrandList>, WxErrorException>;

    /// 获取店铺的商品分类（对应 Java `getShopCat()`）。
    ///
    /// Java 实现为上游桩（不执行请求直接 `return null`）→ `Ok(None)`
    /// 镜像。
    async fn get_shop_cat(&self) -> Result<Option<MinishopShopCatList>, WxErrorException>;
}
