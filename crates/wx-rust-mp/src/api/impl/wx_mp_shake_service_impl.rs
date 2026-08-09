//! WxMpShakeService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpShakeServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpShakeService};
use crate::bean::shake::{
    WxMpShakeAroundDeviceBindPageQuery, WxMpShakeAroundPageAddQuery, WxMpShakeAroundPageAddResult,
    WxMpShakeAroundRelationSearchQuery, WxMpShakeAroundRelationSearchResult,
};
use crate::bean::{WxMpShakeInfoResult, WxMpShakeQuery};
use crate::enums::wx_mp_api_url::shake;

/// 公众号ShakeService实现。
pub struct WxMpShakeServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpShakeServiceImpl {
    /// 构建 公众号ShakeService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpShakeService for WxMpShakeServiceImpl {
    async fn get_shake_info(
        &self,
        query: &WxMpShakeQuery,
    ) -> Result<WxMpShakeInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&shake::get_shake_info(config.as_ref()), &body)
            .await?;
        WxMpShakeInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn page_add(
        &self,
        query: &WxMpShakeAroundPageAddQuery,
    ) -> Result<WxMpShakeAroundPageAddResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&shake::page_add(config.as_ref()), &body).await?;
        WxMpShakeAroundPageAddResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn device_bind_page_query(
        &self,
        query: &WxMpShakeAroundDeviceBindPageQuery,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&shake::device_bind_page(config.as_ref()), &body)
            .await?;
        Ok(true)
    }

    async fn relation_search(
        &self,
        query: &WxMpShakeAroundRelationSearchQuery,
    ) -> Result<WxMpShakeAroundRelationSearchResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&shake::relation_search(config.as_ref()), &body)
            .await?;
        WxMpShakeAroundRelationSearchResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
