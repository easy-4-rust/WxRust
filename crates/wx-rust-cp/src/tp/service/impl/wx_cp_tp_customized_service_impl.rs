//! 企业微信第三方应用代开发服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpCustomizedServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `mainService`）。请求均携带 `provider_access_token` 且不带
//! suite token。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpCustomizedAppDetail, WxCpTpTemplateList};
use crate::enums::url_tp;
use crate::tp::service::{WxCpTpCustomizedService, WxCpTpService};

/// 企业微信第三方应用代开发服务实现。
pub struct WxCpTpCustomizedServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpCustomizedServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 获取 provider_access_token 参数（对应 Java `getProviderAccessToken()`）。
    async fn provider_access_token(&self) -> Result<String, WxErrorException> {
        Ok(format!(
            "?provider_access_token={}",
            self.service()?.get_wx_cp_provider_token().await?
        ))
    }
}

#[async_trait]
impl WxCpTpCustomizedService for WxCpTpCustomizedServiceImpl {
    async fn get_template_list(&self) -> Result<WxCpTpTemplateList, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = format!(
            "{}{}",
            config.api_url(url_tp::GET_TEMPLATE_LIST),
            self.provider_access_token().await?
        );
        let response_text = service.get_without_suite_token(&url, "", true).await?;
        WxCpTpTemplateList::from_json(&response_text).map_err(WxErrorException::Serde)
    }

    async fn get_customized_app_detail(
        &self,
        auth_corp_id: &str,
        agent_id: Option<i32>,
    ) -> Result<WxCpTpCustomizedAppDetail, WxErrorException> {
        let service = self.service()?;
        let mut body = serde_json::Map::new();
        body.insert(
            "auth_corpid".to_string(),
            serde_json::Value::String(auth_corp_id.to_string()),
        );
        if let Some(agent_id) = agent_id {
            body.insert(
                "agentid".to_string(),
                serde_json::Value::Number(agent_id.into()),
            );
        }
        let config = service.wx_cp_tp_config_storage();
        let url = format!(
            "{}{}",
            config.api_url(url_tp::GET_CUSTOMIZED_APP_DETAIL),
            self.provider_access_token().await?
        );
        let response_text = service
            .post_without_suite_token(&url, &serde_json::Value::Object(body).to_string(), true)
            .await?;
        WxCpTpCustomizedAppDetail::from_json(&response_text).map_err(WxErrorException::Serde)
    }
}
