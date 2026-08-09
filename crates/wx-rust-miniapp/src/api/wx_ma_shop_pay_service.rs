//! 小程序支付管理订单相关接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopPayService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::{WxMaShopPayCreateOrderRequest, WxMaShopPayOrderRefundRequest};
use crate::bean::shop::response::{
    WxMaShopBaseResponse, WxMaShopPayCreateOrderResponse, WxMaShopPayGetOrderResponse,
};

/// 小程序支付管理订单服务。
#[async_trait]
pub trait WxMaShopPayService: Send + Sync {
    /// 创建订单（对应 Java `createOrder(WxMaShopPayCreateOrderRequest)`）。
    async fn create_order(
        &self,
        request: &WxMaShopPayCreateOrderRequest,
    ) -> Result<WxMaShopPayCreateOrderResponse, WxErrorException>;

    /// 查询订单详情（对应 Java `getOrder(String)`，trade_no 为商户单号）。
    async fn get_order(
        &self,
        trade_no: &str,
    ) -> Result<WxMaShopPayGetOrderResponse, WxErrorException>;

    /// 订单退款（对应 Java `refundOrder(WxMaShopPayOrderRefundRequest)`）。
    async fn refund_order(
        &self,
        request: &WxMaShopPayOrderRefundRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
