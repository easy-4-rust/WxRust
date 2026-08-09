//! 企业微信第三方应用应用版本付费版本服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpEditionServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `mainService`）。延长试用期 POST `/cgi-bin/service/prolong_try`。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpTpProlongTryResult;
use crate::enums::url_tp;
use crate::tp::service::{WxCpTpEditionService, WxCpTpService};

/// 企业微信第三方应用应用版本付费版本服务实现。
pub struct WxCpTpEditionServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpEditionServiceImpl {
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
impl WxCpTpEditionService for WxCpTpEditionServiceImpl {
    async fn prolong_try(
        &self,
        buyer_corp_id: &str,
        prolong_days: Option<i32>,
        app_id: &str,
    ) -> Result<WxCpTpProlongTryResult, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tp::PROLONG_TRY);
        let body = serde_json::json!({
            "buyer_corpid": buyer_corp_id,
            "prolong_days": prolong_days,
            "appid": app_id,
        })
        .to_string();
        let result = service.post(&url, &body).await?;
        WxCpTpProlongTryResult::from_json(&result).map_err(WxErrorException::Serde)
    }
}
