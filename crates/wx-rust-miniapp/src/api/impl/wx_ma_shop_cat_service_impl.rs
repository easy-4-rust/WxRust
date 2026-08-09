//! 小程序交易组件-商品类目服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopCatServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopCatService;
use crate::bean::shop::response::WxMaShopCatGetResponse;
use crate::enums::g3_urls::url_g3_shop::shop_cat as cat_url;

/// 小程序交易组件-商品类目服务实现。
pub struct WxMaShopCatServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopCatServiceImpl {
    /// 构建商品类目服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopCatService for WxMaShopCatServiceImpl {
    /// 对应 Java `WxMaShopCatServiceImpl.getCat`：
    /// POST `GET_CAT`（空对象）后校验 errcode 并解析响应。
    async fn get_cat(&self) -> Result<WxMaShopCatGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&cat_url::get_cat_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
