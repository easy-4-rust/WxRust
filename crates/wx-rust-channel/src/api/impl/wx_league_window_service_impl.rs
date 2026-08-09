//! 优选联盟团长合作达人管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxLeagueWindowServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_league_window_service::WxLeagueWindowService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::window::{
    AuthInfoResponse, AuthStatusResponse, ProductSearchParam, WindowProductListResponse,
    WindowProductParam, WindowProductResponse,
};
use crate::enums::url_league::{
    ADD_SUPPLIER_GOODS_URL, GET_SUPPLIER_AUTH_STATUS_URL, GET_SUPPLIER_AUTH_URL,
    GET_SUPPLIER_GOODS_URL, LIST_SUPPLIER_GOODS_URL, REMOVE_SUPPLIER_GOODS_URL,
};

/// 优选联盟团长合作达人管理服务实现（对应 Java `WxLeagueWindowServiceImpl`）。
pub struct WxLeagueWindowServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxLeagueWindowServiceImpl {
    /// 构建服务（对应 Java `new WxLeagueWindowServiceImpl(shopService)`）。
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
impl WxLeagueWindowService for WxLeagueWindowServiceImpl {
    /// 添加团长商品到橱窗（对应 Java `addProduct`，内部构造 `WindowProductParam`，
    /// 请求体 `{"appid":"..","openfinderid":"..","product_id":".."}`）。
    async fn add_league_window_product(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = WindowProductParam {
            appid,
            openfinderid,
            product_id,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), ADD_SUPPLIER_GOODS_URL, &req_json).await
    }

    /// 查询橱窗上团长商品列表（对应 Java `listProduct(ProductSearchParam)`）。
    async fn list_league_window_product(
        &self,
        param: ProductSearchParam,
    ) -> Result<WindowProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), LIST_SUPPLIER_GOODS_URL, &req_json).await
    }

    /// 从橱窗移除团长商品（对应 Java `removeProduct`）。
    async fn remove_league_window_product(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = WindowProductParam {
            appid,
            openfinderid,
            product_id,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), REMOVE_SUPPLIER_GOODS_URL, &req_json).await
    }

    /// 查询橱窗上团长商品详情（对应 Java `getProductDetail`）。
    async fn get_league_window_product_detail(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WindowProductResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = WindowProductParam {
            appid,
            openfinderid,
            product_id,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_GOODS_URL, &req_json).await
    }

    /// 获取达人橱窗授权链接（对应 Java `getWindowAuthInfo`，请求体
    /// `{"finder_id":"..."}`）。
    async fn get_window_auth_info(
        &self,
        finder_id: String,
    ) -> Result<AuthInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\"}}");
        Self::post_as(svc.as_ref(), GET_SUPPLIER_AUTH_URL, &req_json).await
    }

    /// 获取达人橱窗授权状态（对应 Java `getWindowAuthStatus`，请求体
    /// `{"finder_id":"..."}`）。
    async fn get_window_auth_status(
        &self,
        finder_id: String,
    ) -> Result<AuthStatusResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\"}}");
        Self::post_as(svc.as_ref(), GET_SUPPLIER_AUTH_STATUS_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 添加团长商品到橱窗：`WindowProductParam` 序列化请求体与响应解析
    /// （对应 Java `addProduct`）。
    #[tokio::test]
    async fn test_add_league_window_product() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxLeagueWindowServiceImpl::new(weak);
        let resp = sub
            .add_league_window_product(
                "wx_appid".to_string(),
                "sph_openfinderid".to_string(),
                "product_1".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, ADD_SUPPLIER_GOODS_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["appid"], "wx_appid");
        assert_eq!(json["openfinderid"], "sph_openfinderid");
        assert_eq!(json["product_id"], "product_1");
    }

    /// 获取达人橱窗授权状态：字面量请求体 `{"finder_id":"..."}` 逐字对齐 Java。
    #[tokio::test]
    async fn test_get_window_auth_status() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","window_auth_status":1,"auth_url":"https://auth.example.com"}"#,
        );
        let sub = WxLeagueWindowServiceImpl::new(weak);
        let resp = sub
            .get_window_auth_status("sph_finder".to_string())
            .await
            .unwrap();
        assert_eq!(resp.window_auth_status, 1);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SUPPLIER_AUTH_STATUS_URL);
        assert_eq!(body, r#"{"finder_id":"sph_finder"}"#);
    }
}
