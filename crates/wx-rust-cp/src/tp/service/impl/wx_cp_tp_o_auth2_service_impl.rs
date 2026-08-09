//! 企业微信第三方应用 OAuth2 服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpOAuth2ServiceImpl`：
//! 构造第三方应用 oauth2 链接（appid 取 `config.getSuiteId()`，redirect_uri
//! 按 `URIUtil.encodeURIComponent` 编码）。

use std::sync::{Arc, Weak};

use wx_rust_common::error::WxErrorException;

use crate::enums::url_oauth2;
use crate::tp::service::{WxCpTpOAuth2Service, WxCpTpService};

/// 企业微信第三方应用 OAuth2 服务实现。
pub struct WxCpTpOAuth2ServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpOAuth2ServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }
}

impl WxCpTpOAuth2Service for WxCpTpOAuth2ServiceImpl {
    fn build_authorize_url_with_scope(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &str,
    ) -> String {
        let suite_id = match self.service() {
            Ok(svc) => svc.wx_cp_tp_config_storage().suite_id(),
            Err(_) => String::new(),
        };
        let mut url = String::from(url_oauth2::URL_OAUTH2_AUTHORIZE);
        url.push_str(&format!("?appid={suite_id}"));
        url.push_str(&format!(
            "&redirect_uri={}",
            encode_uri_component(redirect_uri)
        ));
        url.push_str("&response_type=code");
        url.push_str(&format!("&scope={scope}"));
        if !state.is_empty() {
            url.push_str(&format!("&state={state}"));
        }
        url.push_str("#wechat_redirect");
        url
    }
}

/// URL 组件编码（对应 Java
/// `me.chanjar.weixin.common.util.http.URIUtil.encodeURIComponent`，与
/// JS `encodeURIComponent` 保留集一致；与 `WxCpService::build_qr_connect_url`
/// 的私有辅助同一语义）。
fn encode_uri_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for b in input.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric()
            || matches!(c, '-' | '_' | '.' | '!' | '~' | '*' | '\'' | '(' | ')')
        {
            out.push(c);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
