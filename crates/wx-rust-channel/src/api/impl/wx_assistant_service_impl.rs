//! 视频号助手橱窗管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxAssistantServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_assistant_service::WxAssistantService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::window::request::{
    AddWindowProductRequest, GetWindowProductListRequest, WindowProductRequest,
};
use crate::bean::window::response::{GetWindowProductListResponse, GetWindowProductResponse};
use crate::enums::url_assistant::{
    ADD_WINDOW_PRODUCT_URL, GET_WINDOW_PRODUCT_URL, LIST_WINDOW_PRODUCT_URL, OFF_WINDOW_PRODUCT_URL,
};

/// 视频号助手橱窗管理服务实现（对应 Java `WxAssistantServiceImpl`）。
pub struct WxAssistantServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxAssistantServiceImpl {
    /// 构建服务（对应 Java `new WxAssistantServiceImpl(shopService)`）。
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
impl WxAssistantService for WxAssistantServiceImpl {
    /// 上架商品到橱窗（对应 Java `addWindowProduct(AddWindowProductRequest)`）。
    async fn add_window_product(
        &self,
        req: AddWindowProductRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), ADD_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 获取橱窗商品详情（对应 Java `getWindowProduct(WindowProductRequest)`）。
    async fn get_window_product(
        &self,
        req: WindowProductRequest,
    ) -> Result<GetWindowProductResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 获取已添加到橱窗的商品列表（对应 Java `getWindowProductList`）。
    async fn get_window_product_list(
        &self,
        req: GetWindowProductListRequest,
    ) -> Result<GetWindowProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), LIST_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 下架橱窗商品（对应 Java `offWindowProduct(WindowProductRequest)`）。
    async fn off_window_product(
        &self,
        req: WindowProductRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), OFF_WINDOW_PRODUCT_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 上架商品到橱窗：请求体字段与响应解析（对应 Java `addWindowProduct` +
    /// `AddWindowProductRequest`）。
    #[tokio::test]
    async fn test_add_window_product() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxAssistantServiceImpl::new(weak);
        let req = AddWindowProductRequest {
            product_id: "pid_1".to_string(),
            appid: String::new(),
            is_hide_for_window: false,
        };
        let resp = sub.add_window_product(req).await.unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, ADD_WINDOW_PRODUCT_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["product_id"], "pid_1");
        assert_eq!(json["is_hide_for_window"], false);
    }

    /// 下架橱窗商品：请求体 `{"product_id":..,"appid":..}`（对应 Java
    /// `offWindowProduct` + `WindowProductRequest`）。
    #[tokio::test]
    async fn test_off_window_product() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxAssistantServiceImpl::new(weak);
        let req = WindowProductRequest {
            product_id: "pid_2".to_string(),
            appid: "wx_app".to_string(),
        };
        let resp = sub.off_window_product(req).await.unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, OFF_WINDOW_PRODUCT_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["product_id"], "pid_2");
        assert_eq!(json["appid"], "wx_app");
    }
}
