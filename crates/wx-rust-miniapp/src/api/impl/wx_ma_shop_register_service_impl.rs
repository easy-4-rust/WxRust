//! 小程序交易组件-申请接入服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopRegisterServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopRegisterService;
use crate::bean::shop::request::{
    WxMaShopRegisterApplySceneRequest, WxMaShopRegisterFinishAccessInfoRequest,
};
use crate::bean::shop::response::{WxMaShopBaseResponse, WxMaShopRegisterCheckResponse};
use crate::enums::g3_urls::url_g3_shop::shop_register as register_url;

/// 小程序交易组件-申请接入服务实现。
pub struct WxMaShopRegisterServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopRegisterServiceImpl {
    /// 构建申请接入服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopRegisterService for WxMaShopRegisterServiceImpl {
    /// 对应 Java `WxMaShopRegisterServiceImpl.registerApply`：
    /// POST `REGISTER_APPLY`（空对象）后校验 errcode 并解析响应。
    async fn register_apply(&self) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&register_url::register_apply_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopRegisterServiceImpl.registerCheck`：
    /// POST `REGISTER_CHECK`（空对象）后校验 errcode 并解析响应。
    async fn register_check(&self) -> Result<WxMaShopRegisterCheckResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&register_url::register_check_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopRegisterServiceImpl.registerFinishAccessInfo`：
    /// POST `REGISTER_FINISH_ACCESS_INFO`（序列化请求）后校验 errcode 并解析响应。
    async fn register_finish_access_info(
        &self,
        request: &WxMaShopRegisterFinishAccessInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &register_url::register_finish_access_info_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopRegisterServiceImpl.registerApplyScene`：
    /// POST `REGISTER_APPLY_SCENE`（序列化请求）后校验 errcode 并解析响应。
    async fn register_apply_scene(
        &self,
        request: &WxMaShopRegisterApplySceneRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &register_url::register_apply_scene_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
