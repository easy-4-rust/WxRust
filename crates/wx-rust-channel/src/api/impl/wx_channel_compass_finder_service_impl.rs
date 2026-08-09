//! 罗盘达人版服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxChannelCompassFinderServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_compass_finder_service::WxChannelCompassFinderService;
use crate::bean::compass::CompassFinderBaseParam;
use crate::bean::compass::finder::{
    OverallResponse, ProductDataParam, ProductDataResponse, ProductListResponse,
    SaleProfileDataParam, SaleProfileDataResponse,
};
use crate::enums::url_compass_finder::{
    GET_OVERALL_URL, GET_PRODUCT_DATA_URL, GET_PRODUCT_LIST_URL, GET_SALE_PROFILE_DATA_URL,
};

/// 罗盘达人版服务实现（对应 Java `WxChannelCompassFinderServiceImpl`）。
pub struct WxChannelCompassFinderServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxChannelCompassFinderServiceImpl {
    /// 构建服务（对应 Java `new WxChannelCompassFinderServiceImpl(shopService)`）。
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
impl WxChannelCompassFinderService for WxChannelCompassFinderServiceImpl {
    /// 获取电商概览数据（对应 Java `getOverall`，请求体 `{"ds":"..."}`）。
    async fn get_overall(&self, ds: String) -> Result<OverallResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_OVERALL_URL, &req_json).await
    }

    /// 获取带货商品数据（对应 Java `getProductData`，内部构造 `ProductDataParam`，
    /// 请求体 `{"ds":"..","product_id":".."}`）。
    async fn get_product_data(
        &self,
        ds: String,
        product_id: String,
    ) -> Result<ProductDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = ProductDataParam { ds, product_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_PRODUCT_DATA_URL, &req_json).await
    }

    /// 获取带货商品列表（对应 Java `getProductList`）。
    async fn get_product_list(&self, ds: String) -> Result<ProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CompassFinderBaseParam { ds };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_PRODUCT_LIST_URL, &req_json).await
    }

    /// 获取带货人群数据（对应 Java `getSaleProfileData`，内部构造
    /// `SaleProfileDataParam`，请求体 `{"ds":"..","type":N}`）。
    async fn get_sale_profile_data(
        &self,
        ds: String,
        r#type: Option<i32>,
    ) -> Result<SaleProfileDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = SaleProfileDataParam {
            ds,
            r#type: r#type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SALE_PROFILE_DATA_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取电商概览数据：请求体 `{"ds":"..."}` 与响应解析（对应 Java
    /// `getOverall` + `CompassFinderBaseParam`）。
    #[tokio::test]
    async fn test_get_overall() {
        let (svc, weak) =
            test_support::build_service(r#"{"errcode":0,"errmsg":"ok","data":{"pay_gmv":"50"}}"#);
        let sub = WxChannelCompassFinderServiceImpl::new(weak);
        let resp = sub.get_overall("20240102".to_string()).await.unwrap();
        assert_eq!(resp.data.pay_gmv, "50");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_OVERALL_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["ds"], "20240102");
    }

    /// 获取带货人群数据：请求体 `{"ds":"..","type":N}`（对应 Java
    /// `getSaleProfileData` + `SaleProfileDataParam`）。
    #[tokio::test]
    async fn test_get_sale_profile_data() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","data":{"field_list":[{"field_name":"年龄"}]}}"#,
        );
        let sub = WxChannelCompassFinderServiceImpl::new(weak);
        let resp = sub
            .get_sale_profile_data("20240102".to_string(), Some(3))
            .await
            .unwrap();
        assert_eq!(resp.data.field_list.len(), 1);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SALE_PROFILE_DATA_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["ds"], "20240102");
        assert_eq!(json["type"], 3);
    }
}
