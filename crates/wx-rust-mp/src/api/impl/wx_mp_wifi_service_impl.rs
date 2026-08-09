//! WxMpWifiService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpWifiServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpWifiService};

use crate::bean::wifi::{WxMpWifiShopDataResult, WxMpWifiShopListResult};
use crate::enums::wx_mp_api_url::wifi;

/// WxMpWifi服务实现。
pub struct WxMpWifiServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpWifiServiceImpl {
    /// 构建 WxMpWifi服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpWifiService for WxMpWifiServiceImpl {
    async fn list_shop(
        &self,
        page_index: i32,
        page_size: i32,
    ) -> Result<WxMpWifiShopListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"pageindex": page_index, "pagesize": page_size});
        let response = svc
            .post(&wifi::shop_list(config.as_ref()), &body.to_string())
            .await?;
        WxMpWifiShopListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_shop_wifi_info(
        &self,
        shop_id: i32,
    ) -> Result<WxMpWifiShopDataResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"shop_id": shop_id});
        let response = svc
            .post(&wifi::shop_get(config.as_ref()), &body.to_string())
            .await?;
        WxMpWifiShopDataResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_shop_wifi_info(
        &self,
        shop_id: i32,
        old_ssid: &str,
        ssid: &str,
        password: Option<&str>,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("shop_id".into(), serde_json::json!(shop_id));
        body.insert("old_ssid".into(), serde_json::json!(old_ssid));
        body.insert("ssid".into(), serde_json::json!(ssid));
        if let Some(p) = password {
            body.insert("password".into(), serde_json::json!(p));
        }
        svc.post(
            &wifi::shop_update(config.as_ref()),
            &serde_json::Value::Object(body).to_string(),
        )
        .await?;
        Ok(true)
    }
}
