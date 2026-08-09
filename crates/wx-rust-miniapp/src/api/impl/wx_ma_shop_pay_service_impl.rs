//! 小程序支付管理订单相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopPayServiceImpl`。
//! 注意：Java Impl 仅解析响应不回查 errcode（执行引擎已统一校验）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopPayService;
use crate::bean::shop::request::{WxMaShopPayCreateOrderRequest, WxMaShopPayOrderRefundRequest};
use crate::bean::shop::response::{
    WxMaShopBaseResponse, WxMaShopPayCreateOrderResponse, WxMaShopPayGetOrderResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_pay as pay_url;

/// 小程序支付管理订单服务实现。
pub struct WxMaShopPayServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopPayServiceImpl {
    /// 构建支付管理订单服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopPayService for WxMaShopPayServiceImpl {
    /// 对应 Java `WxMaShopPayServiceImpl.createOrder`：
    /// POST `CREATE_ORDER`（序列化 `WxMaShopPayCreateOrderRequest`）后解析响应。
    async fn create_order(
        &self,
        request: &WxMaShopPayCreateOrderRequest,
    ) -> Result<WxMaShopPayCreateOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&pay_url::create_order_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopPayServiceImpl.getOrder`：
    /// 构造 `{"trade_no": tradeNo}` 后 POST `GET_ORDER` 并解析响应。
    async fn get_order(
        &self,
        trade_no: &str,
    ) -> Result<WxMaShopPayGetOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "trade_no": trade_no }).to_string();
        let response = svc
            .post(&pay_url::get_order_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopPayServiceImpl.refundOrder`：
    /// POST `REFUND_ORDER`（序列化 `WxMaShopPayOrderRefundRequest`）后解析响应。
    async fn refund_order(
        &self,
        request: &WxMaShopPayOrderRefundRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&pay_url::refund_order_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
