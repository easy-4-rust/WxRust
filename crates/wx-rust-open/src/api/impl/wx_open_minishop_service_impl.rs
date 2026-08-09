//! 微信小商店开店服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMinishopServiceImpl`
//! （`extends WxMaServiceImpl implements WxOpenMinishopService`，构造
//! `new WxOpenMinishopServiceImpl(wxOpenComponentService, appId,
//! wxMaConfig)`，Java 自持 ma 配置 + `initHttp()`）。
//!
//! Java 多数方法为上游桩代码，Rust 严格镜像：
//! - `submitMerchantInfo`：按 Java 手工拼装 snake_case JSON（`toJsonObject`
//!   线格式，复用组件实现 [`crate::api::r#impl::wx_open_component_service_impl`
//!   的 minishop_json 拼装]）POST 后恒 `return null` → `Ok(None)`；
//! - `submitBasicInfo`/`checkAuditStatus`/`getCategory`/`getBrands`/
//!   `getShopCat`：Java 不执行请求直接 `return null` → `Ok(None)`
//!   （NOT_MIRRORED：Java 上游未实现，Rust 无法镜像语义，如实标注）；
//! - `uploadImagePicFile`：唯一完整实现——URL 拼接
//!   `?access_token={getAccessToken(true)}&height={height}&width={width}`
//!   后 multipart（字段 `media`）上传，返回响应字符串（Java
//!   `WxMaServiceImpl.post(String, File)` 语义；URL 已含 token，Rust
//!   直连执行器上传，绕开执行引擎的 token 注入守卫，ADAPTED）。
//!
//! 依赖表达：`Weak<dyn WxOpenService>` + appid（Java 自持 ma 服务语义，
//! ADAPTED）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::MediaUploadRequestExecutor;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMinishopService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
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
use crate::enums::url_ma_domain::{minishop_submit_merchant_info_url, minishop_upload_img_url};

/// 微信小商店开店服务实现（对应 Java `WxOpenMinishopServiceImpl`）。
pub struct WxOpenMinishopServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMinishopServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMinishopServiceImpl(WxOpenComponentService, appId,
    /// WxMaConfig)`；Rust 以门面弱引用 + appid 表达，ADAPTED）。
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 取代 ma 桥接服务（同
    /// [`WxOpenMaAuthServiceImpl::ma_service`]）。
    fn ma_service(&self) -> Result<Arc<dyn WxMaService>, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let any = component
            .get_wx_ma_service_by_appid(&self.app_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "getWxMaServiceByAppid 返回 None"))?;
        let ma = any.downcast::<WxOpenMaService>().map_err(|_| {
            WxErrorException::from_code(-99, "代 ma 服务 downcast 失败（缓存类型不匹配）")
        })?;
        Ok(ma as Arc<dyn WxMaService>)
    }
}

#[async_trait]
impl WxOpenMinishopService for WxOpenMinishopServiceImpl {
    /// 提交小商店商户信息（对应 Java `submitMerchantInfo(...)`：按
    /// `toJsonObject()` snake_case 线格式拼装 POST 后恒 `return null`）。
    async fn submit_merchant_info(
        &self,
        app_id: &str,
        subject_type: &str,
        busi_license: &MinishopBusiLicense,
        organization_code_info: &MinishopOrganizationCodeInfo,
        idcard_info: &MinishopIdcardInfo,
        super_administrator_info: &MinishopSuperAdministratorInfo,
        merchant_shortname: &str,
    ) -> Result<Option<WxOpenResult>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({
            "app_id": app_id,
            "subject_type": subject_type,
            "busi_license": minishop_json::busi_license(busi_license),
            "organization_code_info": minishop_json::organization_code_info(organization_code_info),
            "id_card_info": minishop_json::idcard_info(idcard_info),
            "super_administrator_info": minishop_json::super_administrator_info(super_administrator_info),
        });
        let response = ma
            .post(
                &minishop_submit_merchant_info_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java 仅 POST 后 return null（merchant_shortname 未参与拼装，Java
        // 上游遗漏，镜像）
        let _ = (merchant_shortname, response);
        Ok(None)
    }

    /// 提交小商店基础信息（对应 Java `submitBasicInfo(...)`：Java 不执行
    /// 请求直接 `return null`）。
    async fn submit_basic_info(
        &self,
        _app_id: &str,
        _name_info: &MinishopNameInfo,
        _return_info: &MinishopReturnInfo,
    ) -> Result<Option<WxOpenResult>, WxErrorException> {
        // Java 桩：无任何 HTTP 调用
        Ok(None)
    }

    /// 异步状态查询（对应 Java `checkAuditStatus(String wxName)`：Java
    /// 不执行请求直接 `return null`）。
    async fn check_audit_status(
        &self,
        _wx_name: &str,
    ) -> Result<Option<MinishopAuditStatus>, WxErrorException> {
        // Java 桩：无任何 HTTP 调用
        Ok(None)
    }

    /// 上传小商店图片（对应 Java `uploadImagePicFile(Integer height,
    /// Integer width, File file)`：URL 拼接 access_token + 尺寸后
    /// multipart `media` 上传，返回响应字符串）。
    async fn upload_image_pic_file(
        &self,
        height: i32,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        // Java `getAccessToken(true)`（强制刷新语义经代 ma 桥接委托
        // getAuthorizerAccessToken(appId, true)）
        let access_token = ma.get_access_token_with_force(true).await?;
        let url = format!(
            "{}?access_token={access_token}&height={height}&width={width}",
            minishop_upload_img_url(config.as_ref())
        );
        let content = std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取上传文件失败: {e}")))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let executor = MediaUploadRequestExecutor::new(ma.http_client().clone());
        let param = CommonUploadParam::new("media", CommonUploadData::new(file_name, content));
        // URL 已含 access_token，直连执行器（绕开执行引擎 token 注入守卫）
        executor.upload(&url, param, WxType::MiniApp).await
    }

    /// 获取小商店类目（对应 Java `getCategory(Integer fCatId)`：Java
    /// 不执行请求直接 `return null`）。
    async fn get_category(
        &self,
        _f_cat_id: i32,
    ) -> Result<Option<MinishopCategories>, WxErrorException> {
        // Java 桩：无任何 HTTP 调用（getCategoryUrl 等在 Java 接口声明
        // 但实现未使用，镜像）
        Ok(None)
    }

    /// 获取小商店品牌（对应 Java `getBrands()`：Java 不执行请求直接
    /// `return null`）。
    async fn get_brands(&self) -> Result<Option<MinishopBrandList>, WxErrorException> {
        // Java 桩：无任何 HTTP 调用
        Ok(None)
    }

    /// 获取店铺的商品分类（对应 Java `getShopCat()`：Java 不执行请求
    /// 直接 `return null`）。
    async fn get_shop_cat(&self) -> Result<Option<MinishopShopCatList>, WxErrorException> {
        // Java 桩：无任何 HTTP 调用
        Ok(None)
    }
}

/// 商户信息入参拼装（逐键镜像 Java bean `toJsonObject()` snake_case
/// 线格式；Rust bean serde 为 camelCase，不可直接序列化；与组件实现
/// [`crate::api::r#impl::wx_open_component_service_impl`] 的
/// `minishop_json` 同一线格式，此处按需复制所需子集）。
mod minishop_json {
    use serde_json::{Value, json};

    use crate::bean::{
        MinishopBusiLicense, MinishopIdcardInfo, MinishopOrganizationCodeInfo, MinishopPicFile,
        MinishopSuperAdministratorInfo,
    };

    /// 对应 Java `MinishopPicFile.toJsonObject()`。
    fn pic_file(p: &MinishopPicFile) -> Value {
        json!({ "media_id": p.media_id, "pay_media_id": p.pay_media_id })
    }

    /// 对应 Java `MinishopBusiLicense.toJsonObject()`。
    pub(super) fn busi_license(b: &MinishopBusiLicense) -> Value {
        let mut v = json!({
            "license_type": b.license_type,
            "pic_file": pic_file(&b.pic_file),
            "registration_num": b.registration_num,
            "merchant_name": b.merchant_name,
            "legal_representative": b.legal_representative,
            "start_date": b.start_date,
            "end_date": b.end_date,
        });
        // Java `if (registeredAddrs != null)`；Rust String 非空判断镜像
        if !b.registered_addrs.is_empty() {
            v["registered_addrs"] = json!(b.registered_addrs);
        }
        v
    }

    /// 对应 Java `MinishopOrganizationCodeInfo.toJsonObject()`。
    pub(super) fn organization_code_info(o: &MinishopOrganizationCodeInfo) -> Value {
        json!({
            "pic_file": pic_file(&o.pic_file),
            "organization_code": o.organization_code,
            "start_date": o.start_date,
            "end_date": o.end_date,
        })
    }

    /// 对应 Java `MinishopIdcardInfo.toJsonObject()`。
    pub(super) fn idcard_info(i: &MinishopIdcardInfo) -> Value {
        json!({
            "portrait_pic_file": pic_file(&i.portrait_pic_file),
            "nation_pic_file": pic_file(&i.nation_pic_file),
            "id_card_name": i.id_card_name,
            "id_card_number": i.id_card_number,
            "start_date": i.start_date,
            "end_date": i.end_date,
        })
    }

    /// 对应 Java `MinishopSuperAdministratorInfo.toJsonObject()`。
    pub(super) fn super_administrator_info(s: &MinishopSuperAdministratorInfo) -> Value {
        json!({
            "type": s.r#type,
            "name": s.name,
            "id_card_number": s.id_card_number,
            "phone": s.phone,
            "mail": s.mail,
        })
    }
}
