//! 小程序插件管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaPluginService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaPluginListResult;

/// 小程序插件管理服务。
///
/// 详情请见：
/// <https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/plugin-management/pluginManager.applyPlugin.html>
#[async_trait]
pub trait WxMaPluginService: Send + Sync {
    /// 向插件开发者发起使用插件的申请（对应 Java
    /// `WxMaPluginService.applyPlugin(String, String)`）。
    ///
    /// POST `/wxa/plugin`，请求体 `action=apply`/`plugin_appid`/`reason`。
    async fn apply_plugin(&self, plugin_app_id: &str, reason: &str)
    -> Result<(), WxErrorException>;

    /// 查询已添加的插件（对应 Java `WxMaPluginService.getPluginList()`）。
    ///
    /// POST `/wxa/plugin`，请求体 `action=list`。
    async fn get_plugin_list(&self) -> Result<WxMaPluginListResult, WxErrorException>;

    /// 删除已添加的插件（对应 Java
    /// `WxMaPluginService.unbindPlugin(String)`）。
    ///
    /// POST `/wxa/plugin`，请求体 `action=unbind`/`plugin_appid`。
    async fn unbind_plugin(&self, plugin_app_id: &str) -> Result<(), WxErrorException>;

    /// 快速更新插件版本号（第三方平台代小程序管理插件，对应 Java
    /// `WxMaPluginService.updatePlugin(String, String)`）。
    ///
    /// POST `/wxa/plugin`，请求体 `action=update`/`plugin_appid`/`user_version`。
    async fn update_plugin(
        &self,
        plugin_app_id: &str,
        user_version: &str,
    ) -> Result<(), WxErrorException>;
}
