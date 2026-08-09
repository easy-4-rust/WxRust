//! WxMpWifi服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpWifiService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::wifi::{WxMpWifiShopDataResult, WxMpWifiShopListResult};

/// WxMpWifi服务。
#[async_trait]
pub trait WxMpWifiService: Send + Sync {
    async fn list_shop(
        &self,
        page_index: i32,
        page_size: i32,
    ) -> Result<WxMpWifiShopListResult, WxErrorException>;

    async fn get_shop_wifi_info(
        &self,
        shop_id: i32,
    ) -> Result<WxMpWifiShopDataResult, WxErrorException>;

    async fn update_shop_wifi_info(
        &self,
        shop_id: i32,
        old_ssid: &str,
        ssid: &str,
        password: Option<&str>,
    ) -> Result<bool, WxErrorException>;
}
