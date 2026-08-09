//! 小程序插件管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaPluginServiceImpl`：
//! POST `/wxa/plugin`，动作由请求体 `action` 字段区分
//! （apply/list/unbind/update）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaPluginService;
use crate::bean::WxMaPluginListResult;
use crate::enums::g2_urls::url_g2_content::plugin as plugin_url;

/// 小程序插件管理服务实现。
pub struct WxMaPluginServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaPluginServiceImpl {
    /// 构建插件管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaPluginService for WxMaPluginServiceImpl {
    /// 对应 Java `WxMaPluginServiceImpl.applyPlugin`。
    ///
    /// 请求体 `{"action":"apply","plugin_appid":...,"reason":...}`。
    async fn apply_plugin(
        &self,
        plugin_app_id: &str,
        reason: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut body = serde_json::json!({
            "action": "apply",
            "plugin_appid": plugin_app_id,
        });
        // Java `ImmutableMap.of(..., "reason", null)` 经 Gson 省略 `reason` 字段；
        // Rust 以空串表达 null，同样省略
        if !reason.is_empty() {
            body["reason"] = serde_json::json!(reason);
        }
        let config = svc.wx_ma_config();
        svc.post(&plugin_url::plugin_url(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    /// 对应 Java `WxMaPluginServiceImpl.getPluginList`。
    ///
    /// 请求体 `{"action":"list"}`，响应解析为 `WxMaPluginListResult`
    /// （Java `WxMaPluginListResult.fromJson`）。
    async fn get_plugin_list(&self) -> Result<WxMaPluginListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body = serde_json::json!({ "action": "list" });
        let config = svc.wx_ma_config();
        let response = svc
            .post(&plugin_url::plugin_url(config.as_ref()), &body.to_string())
            .await?;
        WxMaPluginListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 对应 Java `WxMaPluginServiceImpl.unbindPlugin`。
    ///
    /// 请求体 `{"action":"unbind","plugin_appid":...}`。
    async fn unbind_plugin(&self, plugin_app_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body = serde_json::json!({
            "action": "unbind",
            "plugin_appid": plugin_app_id,
        });
        let config = svc.wx_ma_config();
        svc.post(&plugin_url::plugin_url(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    /// 对应 Java `WxMaPluginServiceImpl.updatePlugin`。
    ///
    /// 请求体 `{"action":"update","plugin_appid":...,"user_version":...}`。
    async fn update_plugin(
        &self,
        plugin_app_id: &str,
        user_version: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body = serde_json::json!({
            "action": "update",
            "plugin_appid": plugin_app_id,
            "user_version": user_version,
        });
        let config = svc.wx_ma_config();
        svc.post(&plugin_url::plugin_url(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }
}
