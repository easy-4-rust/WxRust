//! 用户服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpUserServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpUserService};
use crate::bean::result::WxMpUser;
use crate::enums::wx_mp_api_url::user as user_url;

/// 用户服务实现。
pub struct WxMpUserServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpUserServiceImpl {
    /// 构建用户服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpUserService for WxMpUserServiceImpl {
    async fn user_info(&self, openid: &str) -> Result<WxMpUser, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = user_url::user_info(config.as_ref());
        let query = format!("openid={openid}&lang=zh_CN");
        let response = svc.get(&url, &query).await?;
        WxMpUser::from_json(&response).map_err(WxErrorException::Serde)
    }
}
