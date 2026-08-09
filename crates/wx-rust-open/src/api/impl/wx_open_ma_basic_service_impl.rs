//! 小程序基础信息服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaBasicServiceImpl`
//! （持有 `WxMaService` 与 `WxOpenComponentService`，构造入参
//! `new WxOpenMaBasicServiceImpl(this, wxOpenComponentService)`）。
//!
//! 依赖表达与 [`crate::api::r#impl::WxOpenMaAuthServiceImpl`] 相同
//! （`Weak<dyn WxOpenService>` + 按 appid 取回代 ma 桥接服务，ADAPTED）；
//! `getComponentRebindAdminUrl` 需组件服务取 `component_app_id`（Java
//! `wxOpenComponentService.getWxOpenConfigStorage().getComponentAppId()`）。
//!
//! `getComponentRebindAdminUrl` 的 redirect_uri 编码对应 Java
//! `URLEncoder.encode(redirectUri, "UTF-8")`（application/x-www-form-
//! urlencoded 语义：空格 → `+`，与 JS encodeURIComponent 不同，原样
//! 镜像）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaBasicService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::WxFastMaAccountBasicInfoResult;
use crate::bean::WxFastMaBeenSetCategoryResult;
use crate::bean::WxFastMaCategory;
use crate::bean::WxFastMaCheckNickameResult;
use crate::bean::WxFastMaQueryNicknameStatusResult;
use crate::bean::WxFastMaSetNickameResult;
use crate::bean::WxOpenGetAllCategoriesByTypeResult;
use crate::bean::WxOpenMaCategoryNameListResult;
use crate::bean::WxOpenMaGetOrderPathResult;
use crate::bean::WxOpenResult;
use crate::enums::url_ma_domain::{
    component_rebind_admin_url, ma_add_category_url, ma_check_wx_verify_nickname_url,
    ma_component_rebind_admin_url, ma_delete_category_url, ma_get_account_basic_info_url,
    ma_get_all_categories_by_type_url, ma_get_all_categories_url, ma_get_all_category_name_url,
    ma_get_category_url, ma_get_order_path_info_url, ma_modify_category_url,
    ma_modify_head_image_url, ma_modify_signature_url, ma_query_nickname_url, ma_set_nickname_url,
};

/// 小程序基础信息服务实现（对应 Java `WxOpenMaBasicServiceImpl`）。
pub struct WxOpenMaBasicServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaBasicServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaBasicServiceImpl(WxMaService, WxOpenComponentService)`；
    /// Rust 以门面弱引用统一表达两个依赖，ADAPTED）。
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    pub fn app_id(&self) -> &str {
        &self.app_id
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

    /// 组件配置存储的 component_appid（对应 Java
    /// `wxOpenComponentService.getWxOpenConfigStorage().getComponentAppId()`）。
    fn component_app_id(&self) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        Ok(svc
            .wx_open_config_storage()
            .component_app_id()
            .unwrap_or_default())
    }
}

#[async_trait]
impl WxOpenMaBasicService for WxOpenMaBasicServiceImpl {
    /// 获取小程序的信息（对应 Java `getAccountBasicInfo()`：GET 空参数 →
    /// `WxFastMaAccountBasicInfoResult`）。
    async fn get_account_basic_info(
        &self,
    ) -> Result<WxFastMaAccountBasicInfoResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_get_account_basic_info_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 小程序名称设置及改名（对应 Java `setNickname(...)`：POST
    /// `{"nick_name","id_card","license","naming_other_stuff_1",
    /// "naming_other_stuff_2"}` → `WxFastMaSetNickameResult`）。
    async fn set_nickname(
        &self,
        nickname: &str,
        id_card: &str,
        license: &str,
        naming_other_stuff1: &str,
        naming_other_stuff2: &str,
    ) -> Result<WxFastMaSetNickameResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({
            "nick_name": nickname,
            "id_card": id_card,
            "license": license,
            "naming_other_stuff_1": naming_other_stuff1,
            "naming_other_stuff_2": naming_other_stuff2,
        });
        let response = ma
            .post(&ma_set_nickname_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 小程序改名审核状态查询（对应 Java
    /// `querySetNicknameStatus(String auditId)`：POST `{"audit_id": ...}`）。
    async fn query_set_nickname_status(
        &self,
        audit_id: &str,
    ) -> Result<WxFastMaQueryNicknameStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "audit_id": audit_id });
        let response = ma
            .post(&ma_query_nickname_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 微信认证名称检测（对应 Java
    /// `checkWxVerifyNickname(String nickname)`：POST
    /// `{"nick_name": ...}`）。
    async fn check_wx_verify_nickname(
        &self,
        nickname: &str,
    ) -> Result<WxFastMaCheckNickameResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "nick_name": nickname });
        let response = ma
            .post(
                &ma_check_wx_verify_nickname_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 修改头像（对应 Java `modifyHeadImage(...)`：POST
    /// `{"head_img_media_id","x1","y1","x2","y2"}`，坐标以 f32 镜像
    /// Java float）。
    async fn modify_head_image(
        &self,
        head_img_media_id: &str,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({
            "head_img_media_id": head_img_media_id,
            "x1": x1,
            "y1": y1,
            "x2": x2,
            "y2": y2,
        });
        let response = ma
            .post(
                &ma_modify_head_image_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 修改功能介绍（对应 Java `modifySignature(String signature)`：
    /// POST `{"signature": ...}`）。
    async fn modify_signature(&self, signature: &str) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "signature": signature });
        let response = ma
            .post(&ma_modify_signature_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取换绑管理员 URL（对应 Java
    /// `getComponentRebindAdminUrl(String redirectUri, String appId)`）。
    ///
    /// redirect_uri 经 `URLEncoder.encode(..., "UTF-8")` 语义编码（空格 →
    /// `+`）后按 `URL_COMPONENT_REBIND_ADMIN` 格式化；纯字符串构建
    /// 不抛错。
    fn get_component_rebind_admin_url(&self, redirect_uri: &str, app_id: &str) -> String {
        let component_app_id = self.component_app_id().unwrap_or_default();
        let encoded = url_encoder_encode(redirect_uri);
        component_rebind_admin_url(app_id, &component_app_id, &encoded)
    }

    /// 管理员换绑（对应 Java `componentRebindAdmin(String taskId)`：
    /// POST `{"taskid": ...}`）。
    async fn component_rebind_admin(
        &self,
        task_id: &str,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "taskid": task_id });
        let response = ma
            .post(
                &ma_component_rebind_admin_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取账号可以设置的所有类目（对应 Java `getAllCategories()`：
    /// GET 空参数，直接返回原始字符串）。
    async fn get_all_categories(&self) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        ma.get(&ma_get_all_categories_url(config.as_ref()), "")
            .await
    }

    /// 获取不同类型主体可设置的类目（对应 Java
    /// `getAllCategoriesByType(String verifyType)`：POST
    /// `{"verify_type": ...}`）。
    async fn get_all_categories_by_type(
        &self,
        verify_type: &str,
    ) -> Result<WxOpenGetAllCategoriesByTypeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "verify_type": verify_type });
        let response = ma
            .post(
                &ma_get_all_categories_by_type_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 添加类目（对应 Java `addCategory(List<WxFastMaCategory>
    /// categoryList)`：POST `{"categories": [...]}`）。
    async fn add_category(
        &self,
        category_list: &[WxFastMaCategory],
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "categories": category_list });
        let response = ma
            .post(&ma_add_category_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 删除类目（对应 Java `deleteCategory(int first, int second)`：
    /// POST `{"first": ..., "second": ...}`）。
    async fn delete_category(
        &self,
        first: i32,
        second: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "first": first, "second": second });
        let response = ma
            .post(&ma_delete_category_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取账号已经设置的所有类目（对应 Java `getCategory()`：GET 空
    /// 参数 → `WxFastMaBeenSetCategoryResult`）。
    async fn get_category(&self) -> Result<WxFastMaBeenSetCategoryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma.get(&ma_get_category_url(config.as_ref()), "").await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 修改类目（对应 Java `modifyCategory(WxFastMaCategory category)`：
    /// POST 实体序列化）。
    async fn modify_category(
        &self,
        category: &WxFastMaCategory,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(category).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_modify_category_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取类目名称信息（对应 Java `getAllCategoryName()`：GET 空参数）。
    async fn get_all_category_name(
        &self,
    ) -> Result<WxOpenMaCategoryNameListResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_get_all_category_name_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取订单页 path 信息（对应 Java `getOrderPathInfo(int infoType)`：
    /// POST `{"info_type": ...}`；0 线上版，1 审核版）。
    async fn get_order_path_info(
        &self,
        info_type: i32,
    ) -> Result<WxOpenMaGetOrderPathResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "info_type": info_type });
        let response = ma
            .post(
                &ma_get_order_path_info_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

/// `URLEncoder.encode(input, "UTF-8")` 语义编码。
///
/// 字母数字与 `_.-*` 保留，空格 → `+`，其余字节按 UTF-8 百分号编码
/// （与 JS `encodeURIComponent` 不同，严格镜像 Java
/// `java.net.URLEncoder`）。
fn url_encoder_encode(input: &str) -> String {
    let mut out = String::new();
    for &b in input.as_bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b'*' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
