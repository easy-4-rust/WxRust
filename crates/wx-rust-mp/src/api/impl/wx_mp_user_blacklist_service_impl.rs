//! WxMpUserBlacklistService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpUserBlacklistServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpUserBlacklistService};

use crate::bean::result::WxMpUserBlacklistGetResult;
use crate::enums::wx_mp_api_url::blacklist;

/// WxMpUserBlacklist服务实现。
pub struct WxMpUserBlacklistServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpUserBlacklistServiceImpl {
    /// 构建 WxMpUserBlacklist服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpUserBlacklistService for WxMpUserBlacklistServiceImpl {
    async fn get_blacklist(
        &self,
        next_openid: &str,
    ) -> Result<WxMpUserBlacklistGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"begin_openid": next_openid});
        let response = svc
            .post(
                &blacklist::get_blacklist(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpUserBlacklistGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn push_to_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"openid_list": openid_list});
        svc.post(
            &blacklist::batch_blacklist(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn pull_from_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"openid_list": openid_list});
        svc.post(
            &blacklist::batch_unblacklist(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }
}
