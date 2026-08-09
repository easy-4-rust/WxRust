//! 小程序交易组件-商家入驻服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopAccountServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopAccountService;
use crate::bean::shop::request::WxMaShopAccountUpdateInfoRequest;
use crate::bean::shop::response::{
    WxMaShopAccountGetBrandListResponse, WxMaShopAccountGetCategoryListResponse,
    WxMaShopAccountGetInfoResponse, WxMaShopBaseResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_account as shop_account_url;

/// 小程序交易组件-商家入驻服务实现。
pub struct WxMaShopAccountServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopAccountServiceImpl {
    /// 构建商家入驻服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopAccountService for WxMaShopAccountServiceImpl {
    /// 对应 Java `WxMaShopAccountServiceImpl.getCategoryList`：
    /// POST `GET_CATEGORY_LIST`（空对象）后校验 errcode 并解析响应。
    async fn get_category_list(
        &self,
    ) -> Result<WxMaShopAccountGetCategoryListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &shop_account_url::get_category_list_url(config.as_ref()),
                "{}",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAccountServiceImpl.getBrandList`：
    /// POST `GET_BRAND_LIST`（空对象）后校验 errcode 并解析响应。
    async fn get_brand_list(
        &self,
    ) -> Result<WxMaShopAccountGetBrandListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&shop_account_url::get_brand_list_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAccountServiceImpl.updateInfo`：
    /// POST `UPDATE_INFO` 后校验 errcode 并解析响应。
    async fn update_info(
        &self,
        request: &WxMaShopAccountUpdateInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&shop_account_url::update_info_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAccountServiceImpl.getInfo`：
    /// POST `GET_INFO`（空对象）后校验 errcode 并解析响应。
    async fn get_info(&self) -> Result<WxMaShopAccountGetInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&shop_account_url::get_info_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
