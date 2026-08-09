//! 小程序交易组件-订单服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopOrderService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::WxMaShopOrderInfo;
use crate::bean::shop::request::WxMaShopOrderPayRequest;
use crate::bean::shop::response::{
    WxMaShopAddOrderResponse, WxMaShopBaseResponse, WxMaShopGetOrderListResponse,
    WxMaShopGetOrderResponse, WxMaShopGetPaymentParamsResponse,
};

/// 小程序交易组件-订单服务。
#[async_trait]
pub trait WxMaShopOrderService: Send + Sync {
    /// 场景检查（对应 Java `checkScene(Integer)`，返回响应 `is_matched` 布尔值）。
    async fn check_scene(&self, scene: i32) -> Result<bool, WxErrorException>;

    /// 添加订单（对应 Java `addOrder(WxMaShopOrderInfo)`）。
    async fn add_order(
        &self,
        order_info: &WxMaShopOrderInfo,
    ) -> Result<WxMaShopAddOrderResponse, WxErrorException>;

    /// 订单支付（对应 Java `orderPay(WxMaShopOrderPayRequest)`）。
    async fn order_pay(
        &self,
        request: &WxMaShopOrderPayRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取订单（对应 Java `getOrder(Long, String, String)`）。
    async fn get_order(
        &self,
        order_id: Option<i64>,
        out_order_id: Option<&str>,
        openid: Option<&str>,
    ) -> Result<WxMaShopGetOrderResponse, WxErrorException>;

    /// 获取订单列表（对应 Java `getOrderList(Integer, Integer, Boolean, Date, Date)`）。
    ///
    /// `page` 默认 1；`page_size` 默认 10；`desc` 为 true 时传 1、否则传 2；
    /// 时间戳（毫秒）按 Java `FastDateFormat("yyyy-MM-dd HH:mm:ss")` 格式化。
    async fn get_order_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        desc: bool,
        start_create_time: Option<i64>,
        end_create_time: Option<i64>,
    ) -> Result<WxMaShopGetOrderListResponse, WxErrorException>;

    /// 生成支付参数（对应 Java `getPaymentParams(String, String, String)`）。
    async fn get_payment_params(
        &self,
        order_id: Option<&str>,
        out_order_id: Option<&str>,
        openid: Option<&str>,
    ) -> Result<WxMaShopGetPaymentParamsResponse, WxErrorException>;
}
