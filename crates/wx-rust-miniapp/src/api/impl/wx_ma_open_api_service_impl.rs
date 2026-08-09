//! openApi 管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaOpenApiServiceImpl`：
//! 清空配额 / 查询配额 / 查询 rid / AppSecret 重置配额；
//! Java 的显式 errcode 校验（`parseErrorResponse`）已被执行引擎覆盖
//! （同一语义）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaOpenApiService;
use crate::bean::openapi::{WxMiniGetApiQuotaResult, WxMiniGetRidInfoResult};
use crate::enums::g2_urls::url_g2_content::openapi as openapi_url;

/// openApi 管理服务实现。
pub struct WxMaOpenApiServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaOpenApiServiceImpl {
    /// 构建 openApi 管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaOpenApiService for WxMaOpenApiServiceImpl {
    /// 对应 Java `WxMaOpenApiServiceImpl.clearQuota`。
    ///
    /// POST `/cgi-bin/clear_quota`，请求体 `{"appid":...}`；成功返回 true。
    async fn clear_quota(&self) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "appid": config.app_id() });
        svc.post(
            &openapi_url::clear_quota_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(true)
    }

    /// 对应 Java `WxMaOpenApiServiceImpl.getApiQuota`。
    ///
    /// POST `/cgi-bin/openapi/quota/get`，请求体 `{"cgi_path":...}`。
    async fn get_api_quota(
        &self,
        cgi_path: &str,
    ) -> Result<WxMiniGetApiQuotaResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body = serde_json::json!({ "cgi_path": cgi_path });
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &openapi_url::get_api_quota_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str::<WxMiniGetApiQuotaResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaOpenApiServiceImpl.getRidInfo`。
    ///
    /// POST `/cgi-bin/openapi/rid/get`，请求体 `{"rid":...}`；响应含
    /// `request` 字段时解析该对象返回，否则返回 `None`（Java 返回 null）。
    async fn get_rid_info(
        &self,
        rid: &str,
    ) -> Result<Option<WxMiniGetRidInfoResult>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body = serde_json::json!({ "rid": rid });
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &openapi_url::get_rid_info_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        match json.get("request") {
            Some(request) => serde_json::from_value(request.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            None => Ok(None),
        }
    }

    /// 对应 Java `WxMaOpenApiServiceImpl.clearQuotaByAppSecret`。
    ///
    /// POST `/cgi-bin/clear_quota/v2?appid=&appsecret=`（Java
    /// `String.format(CLEAR_QUOTA_BY_APP_SECRET, appid, secret)`），请求体
    /// 为空字符串；成功返回 true。
    async fn clear_quota_by_app_secret(&self) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let url = openapi_url::clear_quota_by_app_secret_url(
            config.as_ref(),
            config.app_id(),
            config.secret(),
        );
        svc.post(&url, "").await?;
        Ok(true)
    }
}
