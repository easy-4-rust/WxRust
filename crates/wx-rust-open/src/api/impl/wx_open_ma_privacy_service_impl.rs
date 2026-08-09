//! 小程序用户隐私保护指引服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaPrivacyServiceImpl`
//! （`@AllArgsConstructor` 持有 `WxMaService`——代 ma 桥接服务）。
//!
//! NOT_MIRRORED：`uploadPrivacyFile` 在 Java 上游本身未实现
//! （TODO 注释 + 恒抛 `WxError(5003, "暂未实现用户隐私指引内容上传")`），
//! Rust 在 trait 默认实现中严格镜像该错误（本文件无需覆写）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaPrivacyService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::ApplyPrivacyInterface;
use crate::bean::ApplyPrivacyInterfaceResult;
use crate::bean::GetPrivacyInterfaceResult;
use crate::bean::GetPrivacySettingResult;
use crate::bean::SetPrivacySetting;
use crate::enums::url_ma_domain::{
    ma_privacy_apply_interface_url, ma_privacy_get_interface_url, ma_privacy_get_url,
    ma_privacy_set_url,
};

/// 小程序用户隐私保护指引服务实现（对应 Java
/// `WxOpenMaPrivacyServiceImpl`）。
pub struct WxOpenMaPrivacyServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaPrivacyServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaPrivacyServiceImpl(WxMaService)`）。
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
impl WxOpenMaPrivacyService for WxOpenMaPrivacyServiceImpl {
    /// 查询小程序用户隐私保护指引（对应 Java
    /// `getPrivacySetting(Integer privacyVer)`：POST
    /// `{"privacy_ver": ...}`，`privacy_ver` 为 null 时不携带该字段）。
    async fn get_privacy_setting(
        &self,
        privacy_ver: Option<i32>,
    ) -> Result<GetPrivacySettingResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let mut body = serde_json::Map::new();
        if let Some(ver) = privacy_ver {
            body.insert("privacy_ver".to_string(), serde_json::json!(ver));
        }
        let body = serde_json::Value::Object(body);
        let response = ma
            .post(&ma_privacy_get_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 设置小程序用户隐私保护指引（对应 Java
    /// `setPrivacySetting(SetPrivacySetting dto)`：POST 实体序列化，
    /// 忽略响应）。
    async fn set_privacy_setting(&self, dto: &SetPrivacySetting) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(dto).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        ma.post(&ma_privacy_set_url(config.as_ref()), &body).await?;
        Ok(())
    }

    /// 隐私接口-获取接口列表（对应 Java `getPrivacyInterface()`：GET
    /// 空参数）。
    async fn get_privacy_interface(&self) -> Result<GetPrivacyInterfaceResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_privacy_get_interface_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 隐私接口-申请接口（对应 Java
    /// `applyPrivacyInterface(ApplyPrivacyInterface dto)`：POST 实体
    /// 序列化）。
    async fn apply_privacy_interface(
        &self,
        dto: &ApplyPrivacyInterface,
    ) -> Result<ApplyPrivacyInterfaceResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(dto).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_privacy_apply_interface_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
