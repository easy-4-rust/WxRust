//! 罗盘商家版服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxChannelCompassShopServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_compass_shop_service::WxChannelCompassShopService;
use crate::bean::compass::CompassFinderBaseParam;
use crate::bean::compass::shop::{
    CompassFinderIdParam, FinderAuthListResponse, FinderListResponse, FinderOverallResponse,
    FinderProductListResponse, FinderProductOverallResponse, ShopLiveListResponse,
    ShopOverallResponse, ShopProductDataParam, ShopProductDataResponse, ShopProductListResponse,
    ShopSaleProfileDataParam, ShopSaleProfileDataResponse,
};
use crate::enums::url_compass_shop::{
    FINDER_AUTH_LIST_URL, FINDER_LIST_URL, GET_FINDER_OVERALL_URL, GET_FINDER_PRODUCT_LIST_URL,
    GET_FINDER_PRODUCT_OVERALL_URL, GET_LIVE_LIST_URL, GET_SHOP_OVERALL_URL,
    GET_SHOP_PRODUCT_DATA_URL, GET_SHOP_PRODUCT_LIST_URL, GET_SHOP_SALE_PROFILE_DATA_URL,
};

/// 罗盘商家版服务实现（对应 Java `WxChannelCompassShopServiceImpl`）。
pub struct WxChannelCompassShopServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxChannelCompassShopServiceImpl {
    /// 构建罗盘商家版服务（对应 Java `new WxChannelCompassShopServiceImpl(shopService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `shopService.post` +
    /// `ResponseUtils.decode`；errcode 校验由执行引擎完成，同 Java 语义）。
    async fn post_as<T>(
        svc: &dyn WxChannelService,
        url: &str,
        post_data: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_data).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxChannelCompassShopService for WxChannelCompassShopServiceImpl {
    /// 获取电商概览数据（对应 Java `getShopOverall`，请求体 `{"ds":"..."}`）。
    async fn get_shop_overall(&self, ds: String) -> Result<ShopOverallResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SHOP_OVERALL_URL, &req_json).await
    }

    /// 获取授权视频号列表（对应 Java `getFinderAuthorizationList`，POST 空对象 `{}`）。
    async fn get_finder_authorization_list(
        &self,
    ) -> Result<FinderAuthListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), FINDER_AUTH_LIST_URL, "{}").await
    }

    /// 获取带货达人列表（对应 Java `getFinderList`）。
    async fn get_finder_list(&self, ds: String) -> Result<FinderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), FINDER_LIST_URL, &req_json).await
    }

    /// 获取带货数据概览（对应 Java `getFinderOverall`）。
    async fn get_finder_overall(
        &self,
        ds: String,
    ) -> Result<FinderOverallResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_FINDER_OVERALL_URL, &req_json).await
    }

    /// 获取带货达人商品列表（对应 Java `getFinderProductList`，内部构造
    /// `CompassFinderIdParam`，请求体 `{"ds":"..","finder_id":".."}`）。
    async fn get_finder_product_list(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<FinderProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderIdParam { ds, finder_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_FINDER_PRODUCT_LIST_URL, &req_json).await
    }

    /// 获取带货达人详情（对应 Java `getFinderProductOverall`）。
    async fn get_finder_product_overall(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<FinderProductOverallResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderIdParam { ds, finder_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_FINDER_PRODUCT_OVERALL_URL, &req_json).await
    }

    /// 获取店铺开播列表（对应 Java `getShopLiveList`）。
    async fn get_shop_live_list(
        &self,
        ds: String,
        finder_id: String,
    ) -> Result<ShopLiveListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderIdParam { ds, finder_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LIVE_LIST_URL, &req_json).await
    }

    /// 获取商品详细信息（对应 Java `getShopProductData`，内部构造
    /// `ShopProductDataParam`，请求体 `{"ds":"..","product_id":".."}`）。
    async fn get_shop_product_data(
        &self,
        ds: String,
        product_id: String,
    ) -> Result<ShopProductDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = ShopProductDataParam { ds, product_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SHOP_PRODUCT_DATA_URL, &req_json).await
    }

    /// 获取商品列表（对应 Java `getShopProductList`）。
    async fn get_shop_product_list(
        &self,
        ds: String,
    ) -> Result<ShopProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SHOP_PRODUCT_LIST_URL, &req_json).await
    }

    /// 获取店铺人群数据（对应 Java `getShopSaleProfileData`，内部构造
    /// `ShopSaleProfileDataParam`，请求体 `{"ds":"..","type":N}`）。
    async fn get_shop_sale_profile_data(
        &self,
        ds: String,
        r#type: Option<i32>,
    ) -> Result<ShopSaleProfileDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = ShopSaleProfileDataParam {
            ds,
            r#type: r#type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SHOP_SALE_PROFILE_DATA_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取电商概览数据：请求体 `{"ds":"..."}` 与响应解析（对应 Java
    /// `getShopOverall` + `CompassFinderBaseParam`）。
    #[tokio::test]
    async fn test_get_shop_overall() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","data":{"pay_gmv":"100","pay_uv":"10"}}"#,
        );
        let sub = WxChannelCompassShopServiceImpl::new(weak);
        let resp = sub.get_shop_overall("20240101".to_string()).await.unwrap();
        assert_eq!(resp.data.pay_gmv, "100");
        assert_eq!(resp.data.pay_uv, "10");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SHOP_OVERALL_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["ds"], "20240101");
    }

    /// 获取授权视频号列表：POST 空对象 `{}`（对应 Java `getFinderAuthorizationList`）。
    #[tokio::test]
    async fn test_get_finder_authorization_list() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","main_finder_id":"f0","authorized_finder_id_list":["f1","f2"]}"#,
        );
        let sub = WxChannelCompassShopServiceImpl::new(weak);
        let resp = sub.get_finder_authorization_list().await.unwrap();
        assert_eq!(resp.main_finder_id, "f0");
        assert_eq!(resp.authorized_finder_id_list.len(), 2);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, FINDER_AUTH_LIST_URL);
        assert_eq!(body, "{}");
    }
}
