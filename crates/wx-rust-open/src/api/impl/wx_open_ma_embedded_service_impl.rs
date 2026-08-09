//! 半屏小程序管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaEmbeddedServiceImpl`
//! （`@AllArgsConstructor` 持有 `WxMaService`——代 ma 桥接服务）。
//!
//! Java 每个方法在 post/get 后额外以 `WxError.fromJson(response)` 校验
//! `errcode != 0` 抛异常；由于代 ma 执行引擎（SimplePost/GetRequestExecutor）
//! 对 `errcode != 0` 已抛错，该二次校验在 Java 侧同为防御性死代码，Rust
//! 不再重复（ADAPTED，见各方法注释）。
//!
//! `getOwnList()` 无参重载对应 Java 直接 GET `?num=1000`（默认分页起始
//! 0、一次拉取最大 1000）；`getOwnList(start, num)` 对 null 取默认值
//! （0/10）、`num > 1000` 截断为 1000，Rust 以 `Option` 入参镜像。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaEmbeddedService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::WxOpenMaEmbeddedListResult;
use crate::enums::url_ma_domain::{
    ma_embedded_add_url, ma_embedded_del_authorize_url, ma_embedded_del_url,
    ma_embedded_get_list_url, ma_embedded_get_own_list_url, ma_embedded_set_authorize_url,
};

/// 半屏小程序管理服务实现（对应 Java `WxOpenMaEmbeddedServiceImpl`）。
pub struct WxOpenMaEmbeddedServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaEmbeddedServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaEmbeddedServiceImpl(WxMaService)`）。
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
}

#[async_trait]
impl WxOpenMaEmbeddedService for WxOpenMaEmbeddedServiceImpl {
    /// 添加半屏小程序（对应 Java `addEmbedded(...)`：POST
    /// `{"appid": ...}`，`apply_reason` 非空才携带
    /// `{"apply_reason": ...}`，Java `StringUtils.isNotBlank` 语义）。
    async fn add_embedded(
        &self,
        embedded_app_id: &str,
        apply_reason: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let mut body = serde_json::json!({ "appid": embedded_app_id });
        if !apply_reason.trim().is_empty() {
            body["apply_reason"] = serde_json::Value::String(apply_reason.to_string());
        }
        ma.post(&ma_embedded_add_url(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    /// 删除半屏小程序（对应 Java `deleteEmbedded(...)`：POST
    /// `{"appid": ...}`）。
    async fn delete_embedded(&self, embedded_app_id: &str) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "appid": embedded_app_id });
        ma.post(&ma_embedded_del_url(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    /// 获取半屏小程序调用列表（对应 Java `getEmbeddedList()`：GET
    /// `null` 参数 → `WxOpenMaEmbeddedListResult`）。
    async fn get_embedded_list(&self) -> Result<WxOpenMaEmbeddedListResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_embedded_get_list_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 取消授权小程序（对应 Java `deleteAuthorizedEmbedded(...)`：POST
    /// `{"appid": ...}`）。
    async fn delete_authorized_embedded(
        &self,
        embedded_app_id: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "appid": embedded_app_id });
        ma.post(
            &ma_embedded_del_authorize_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    /// 获取半屏小程序授权列表（对应 Java `getOwnList()`：GET
    /// `?num=1000`）。
    async fn get_own_list(&self) -> Result<WxOpenMaEmbeddedListResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let url = format!("{}?num=1000", ma_embedded_get_own_list_url(config.as_ref()));
        let response = ma.get(&url, "").await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取半屏小程序授权列表（对应 Java `getOwnList(Integer start,
    /// Integer num)`：null 取默认值 0/10、`num > 1000` 截断，GET
    /// `?start={start}&num={num}`）。
    async fn get_own_list_with(
        &self,
        start: Option<i32>,
        num: Option<i32>,
    ) -> Result<WxOpenMaEmbeddedListResult, WxErrorException> {
        let start = start.unwrap_or(0);
        let num = num.unwrap_or(10).min(1000);
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let url = format!(
            "{}?start={start}&num={num}",
            ma_embedded_get_own_list_url(config.as_ref())
        );
        let response = ma.get(&url, "").await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 设置授权方式（对应 Java `setAuthorizedEmbedded(Integer flag)`：
    /// POST `{"flag": ...}`；0 需要管理员验证，1 自动通过，2 自动拒绝）。
    async fn set_authorized_embedded(&self, flag: i32) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "flag": flag });
        ma.post(
            &ma_embedded_set_authorize_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }
}
