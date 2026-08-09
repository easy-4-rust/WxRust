//! 小程序设置服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaSettingServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaService, WxMaSettingService};
use crate::bean::WxMaDomainAction;
use crate::enums::url_g1_core::setting as setting_url;

/// 小程序设置服务实现。
pub struct WxMaSettingServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaSettingServiceImpl {
    /// 构建设置服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaSettingService for WxMaSettingServiceImpl {
    async fn modify_domain(
        &self,
        domain_action: &WxMaDomainAction,
    ) -> Result<WxMaDomainAction, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `modifyDomain`：POST `MODIFY_DOMAIN_URL`（
        // `https://api.weixin.qq.com/wxa/modify_domain`）后
        // `WxMaDomainAction.fromJson`
        let config = svc.wx_ma_config();
        let body = domain_action.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&setting_url::modify_domain_url(config.as_ref()), &body)
            .await?;
        WxMaDomainAction::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_web_view_domain(
        &self,
        domain_action: &WxMaDomainAction,
    ) -> Result<WxMaDomainAction, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `setWebViewDomain`：POST `SET_WEB_VIEW_DOMAIN_URL`（
        // `https://api.weixin.qq.com/wxa/setwebviewdomain`）后
        // `WxMaDomainAction.fromJson`
        let config = svc.wx_ma_config();
        let body = domain_action.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(
                &setting_url::set_web_view_domain_url(config.as_ref()),
                &body,
            )
            .await?;
        WxMaDomainAction::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn bind_tester(&self, wechat_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `bindTester`：POST `BIND_TESTER_URL`，请求体 `{"wechatid": ...}`
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "wechatid": wechat_id }).to_string();
        svc.post(&setting_url::bind_tester_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn unbind_tester(&self, wechat_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `unbindTester`：POST `UNBIND_TESTER_URL`，请求体 `{"wechatid": ...}`
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "wechatid": wechat_id }).to_string();
        svc.post(&setting_url::unbind_tester_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }
}
