//! 小程序修改服务器地址、成员管理 API（大部分只能是第三方平台调用）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaSettingService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaDomainAction;

/// 小程序修改服务器地址、成员管理 API。
#[async_trait]
pub trait WxMaSettingService: Send + Sync {
    /// 操作服务器域名（对应 Java `modifyDomain(WxMaDomainAction)`）。
    ///
    /// 除了 webViewDomain，都是有效的；以下字段仅在 get 时返回完整字段。
    async fn modify_domain(
        &self,
        domain_action: &WxMaDomainAction,
    ) -> Result<WxMaDomainAction, WxErrorException>;

    /// 设置小程序业务域名（对应 Java `setWebViewDomain(WxMaDomainAction)`，
    /// 仅供第三方代小程序调用）。
    ///
    /// 只有 action 和 webViewDomain 是有效的。
    async fn set_web_view_domain(
        &self,
        domain_action: &WxMaDomainAction,
    ) -> Result<WxMaDomainAction, WxErrorException>;

    /// 绑定微信用户为小程序体验者（对应 Java `bindTester(String)`）。
    async fn bind_tester(&self, wechat_id: &str) -> Result<(), WxErrorException>;

    /// 解除绑定小程序的体验者（对应 Java `unbindTester(String)`）。
    async fn unbind_tester(&self, wechat_id: &str) -> Result<(), WxErrorException>;
}
