//! 企业微信第三方应用 OAuth2 服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpOAuth2Service`：
//! 构造第三方应用 oauth2 链接（appid 取第三方应用 suiteId）
//! （https://developer.work.weixin.qq.com/document/path/91120）。

/// 企业微信第三方应用 OAuth2 服务。
pub trait WxCpTpOAuth2Service: Send + Sync {
    /// 构造第三方应用 oauth2 链接（静默授权，对应 Java
    /// `buildAuthorizeUrl(String, String)`，scope 默认
    /// `snsapi_base`）。
    fn build_authorize_url(&self, redirect_uri: &str, state: &str) -> String {
        self.build_authorize_url_with_scope(redirect_uri, state, "snsapi_base")
    }

    /// 构造第三方应用 oauth2 链接（对应 Java `buildAuthorizeUrl(String,
    /// String, String)`：scope `snsapi_base` 静默授权 /
    /// `snsapi_privateinfo` 手动授权）。
    ///
    /// 链接格式：
    /// `https://open.weixin.qq.com/connect/oauth2/authorize?appid=
    /// <suiteId>&redirect_uri=<encodeURIComponent>&response_type=code&
    /// scope=<scope>[&state=<state>]#wechat_redirect`。
    fn build_authorize_url_with_scope(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &str,
    ) -> String;
}
