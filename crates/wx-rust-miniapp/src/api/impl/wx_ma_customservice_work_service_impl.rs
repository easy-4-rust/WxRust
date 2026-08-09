//! 小程序 - 微信客服相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaCustomserviceWorkServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaCustomserviceWorkService;
use crate::bean::customservice::WxMaCustomserviceResult;
use crate::enums::g3_urls::url_g3_shop::customservice_work as customservice_url;

/// 小程序微信客服服务实现。
pub struct WxMaCustomserviceWorkServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaCustomserviceWorkServiceImpl {
    /// 构建微信客服服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaCustomserviceWorkService for WxMaCustomserviceWorkServiceImpl {
    /// 对应 Java `WxMaCustomserviceWorkServiceImpl.getCustomservice`：
    /// GET `GET_CUSTOMSERVICE_URL` 后 `fromJson` 解析。
    async fn get_customservice(&self) -> Result<WxMaCustomserviceResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .get(
                &customservice_url::get_customservice_url(config.as_ref()),
                "",
            )
            .await?;
        WxMaCustomserviceResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaCustomserviceWorkServiceImpl.bindCustomservice`：
    /// 构造 `{"corpid": corpid}` 后 POST `BIND_CUSTOMSERVICE_URL` 并 `fromJson` 解析。
    async fn bind_customservice(
        &self,
        corpid: &str,
    ) -> Result<WxMaCustomserviceResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "corpid": corpid }).to_string();
        let response = svc
            .post(
                &customservice_url::bind_customservice_url(config.as_ref()),
                &body,
            )
            .await?;
        WxMaCustomserviceResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaCustomserviceWorkServiceImpl.unbindCustomservice`：
    /// 构造 `{"corpid": corpid}` 后 POST `UNBIND_CUSTOMSERVICE_URL` 并 `fromJson` 解析。
    async fn unbind_customservice(
        &self,
        corpid: &str,
    ) -> Result<WxMaCustomserviceResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "corpid": corpid }).to_string();
        let response = svc
            .post(
                &customservice_url::unbind_customservice_url(config.as_ref()),
                &body,
            )
            .await?;
        WxMaCustomserviceResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
