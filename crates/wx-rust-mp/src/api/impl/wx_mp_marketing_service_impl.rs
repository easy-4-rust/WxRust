//! WxMpMarketingService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMarketingServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMarketingService, WxMpService};
use crate::bean::marketing::{
    WxMpAdLeadFilter, WxMpAdLeadResult, WxMpUserAction, WxMpUserActionSet,
};
use crate::enums::wx_mp_api_url::marketing;

/// 公众号MarketingService实现。
pub struct WxMpMarketingServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMarketingServiceImpl {
    /// 构建 公众号MarketingService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpMarketingService for WxMpMarketingServiceImpl {
    async fn add_user_action_sets(
        &self,
        r#type: &str,
        name: &str,
        description: &str,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "name": name, "description": description});
        let response = svc
            .post(
                &marketing::add_user_action_sets(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("user_action_set_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "user_action_set_id 缺失"))
    }

    async fn get_user_action_sets(
        &self,
        user_action_set_id: i64,
    ) -> Result<Vec<WxMpUserActionSet>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"user_action_set_id": user_action_set_id});
        let response = svc
            .post(
                &marketing::get_user_action_sets(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("user_action_set")
            .ok_or_else(|| WxErrorException::from_code(-99, "user_action_set 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_user_action(&self, actions: &[WxMpUserAction]) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"user_actions": actions});
        svc.post(
            &marketing::add_user_action(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_ad_leads(
        &self,
        begin_date: &str,
        end_date: &str,
        filtering: &[WxMpAdLeadFilter],
        page: i32,
        page_size: i32,
    ) -> Result<WxMpAdLeadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("start_date".into(), serde_json::json!(begin_date));
        body.insert("end_date".into(), serde_json::json!(end_date));
        body.insert("filtering".into(), serde_json::json!(filtering));
        body.insert("page".into(), serde_json::json!(page));
        body.insert("page_size".into(), serde_json::json!(page_size));
        let response = svc
            .post(
                &marketing::get_ad_leads(config.as_ref()),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        WxMpAdLeadResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
