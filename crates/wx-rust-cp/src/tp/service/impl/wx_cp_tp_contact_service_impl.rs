//! 企业微信第三方应用通讯录服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpContactServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `mainService`）。通讯录搜索使用服务商 provider_access_token。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpContactSearch, WxCpTpContactSearchResp};
use crate::enums::url_tp;
use crate::tp::service::{WxCpTpContactService, WxCpTpService};

/// 企业微信第三方应用通讯录服务实现。
pub struct WxCpTpContactServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpContactServiceImpl {
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

#[async_trait]
impl WxCpTpContactService for WxCpTpContactServiceImpl {
    async fn contact_search(
        &self,
        search: &WxCpTpContactSearch,
    ) -> Result<WxCpTpContactSearchResp, WxErrorException> {
        let service = self.service()?;
        let provider_access_token = service.get_wx_cp_provider_token().await?;
        let config = service.wx_cp_tp_config_storage();
        let url = format!(
            "{}?provider_access_token={provider_access_token}",
            config.api_url(url_tp::CONTACT_SEARCH)
        );
        let json = search.to_json().map_err(WxErrorException::Serde)?;
        let response_text = service.post(&url, &json).await?;
        WxCpTpContactSearchResp::from_json(&response_text).map_err(WxErrorException::Serde)
    }
}
