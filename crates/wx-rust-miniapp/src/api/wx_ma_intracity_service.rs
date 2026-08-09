//! 同城配送服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaIntracityService`
//! （`impl.WxMaIntracityServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::intractiy::{
    PayMode, WxMaAddOrderRequest, WxMaAddOrderResponse, WxMaCancelOrderResponse,
    WxMaGetPayModeResponse, WxMaOrder, WxMaPreAddOrderRequest, WxMaPreAddOrderResponse,
    WxMaQueryFlowRequest, WxMaStore, WxMaStoreBalance, WxMaStoreChargeRequest,
    WxMaStoreFlowResponse, WxMaStoreRefundRequest, WxMaTransCity,
};

/// 同城配送服务。
///
/// 对应 Java `WxMaIntracityService`：门店权限申请/门店管理/运费充值退款与
/// 流水/余额/扣费主体/运费试算/配送单创建查询取消/支持城市查询。
#[async_trait]
pub trait WxMaIntracityService: Send + Sync {
    /// 申请开通门店权限（对应 Java `apply`）。
    async fn apply(&self) -> Result<(), WxErrorException>;

    /// 创建门店（对应 Java `createStore`，返回 `wx_store_id`）。
    async fn create_store(&self, store: &WxMaStore) -> Result<String, WxErrorException>;

    /// 更新门店（对应 Java `updateStore`；只更新 store 中不为 null 的部分）。
    async fn update_store(&self, store: &WxMaStore) -> Result<(), WxErrorException>;

    /// 查询门店（列出所有门店，对应 Java `listAllStores`）。
    async fn list_all_stores(&self) -> Result<Vec<WxMaStore>, WxErrorException>;

    /// 根据 `wx_store_id` 查询门店（对应 Java `queryStoreByWxStoreId`）。
    async fn query_store_by_wx_store_id(
        &self,
        wx_store_id: &str,
    ) -> Result<Option<WxMaStore>, WxErrorException>;

    /// 根据 `out_store_id` 查询门店（对应 Java `queryStoreByOutStoreId`）。
    async fn query_store_by_out_store_id(
        &self,
        out_store_id: &str,
    ) -> Result<Vec<WxMaStore>, WxErrorException>;

    /// 门店运费充值（对应 Java `storeCharge`，返回充值 URL）。
    async fn store_charge(
        &self,
        request: &WxMaStoreChargeRequest,
    ) -> Result<String, WxErrorException>;

    /// 门店运费退款（对应 Java `storeRefund`，返回退款金额）。
    async fn store_refund(&self, request: &WxMaStoreRefundRequest)
    -> Result<i32, WxErrorException>;

    /// 门店运费流水查询（对应 Java `queryFlow`）。
    async fn query_flow(
        &self,
        request: &WxMaQueryFlowRequest,
    ) -> Result<WxMaStoreFlowResponse, WxErrorException>;

    /// 查询门店余额（对应 Java `balanceQuery`）。
    async fn balance_query(
        &self,
        wx_store_id: Option<&str>,
        service_trans_id: Option<&str>,
        pay_mode: Option<PayMode>,
    ) -> Result<WxMaStoreBalance, WxErrorException>;

    /// 设置扣费主体（对应 Java `setPayMode`）。
    async fn set_pay_mode(&self, pay_mode: PayMode) -> Result<(), WxErrorException>;

    /// 查询扣费主体（对应 Java `getPayMode`）。
    async fn get_pay_mode(&self) -> Result<WxMaGetPayModeResponse, WxErrorException>;

    /// 查询运费（对应 Java `preAddOrder`）。
    async fn pre_add_order(
        &self,
        request: &WxMaPreAddOrderRequest,
    ) -> Result<WxMaPreAddOrderResponse, WxErrorException>;

    /// 创建配送单（对应 Java `addOrder`）。
    async fn add_order(
        &self,
        request: &WxMaAddOrderRequest,
    ) -> Result<WxMaAddOrderResponse, WxErrorException>;

    /// 根据微信订单号查询配送单（对应 Java `queryOrderByWxOrderId`）。
    async fn query_order_by_wx_order_id(
        &self,
        wx_order_id: &str,
    ) -> Result<WxMaOrder, WxErrorException>;

    /// 依据商户订单号查询配送单（对应 Java `queryOrderByStoreOrderId`）。
    async fn query_order_by_store_order_id(
        &self,
        wx_store_id: &str,
        store_order_id: &str,
    ) -> Result<WxMaOrder, WxErrorException>;

    /// 依据微信订单号取消配送单（对应 Java `cancelOrderByWxOrderId`）。
    async fn cancel_order_by_wx_order_id(
        &self,
        wx_order_id: &str,
        cancel_reason_id: i32,
        cancel_reason: Option<&str>,
    ) -> Result<WxMaCancelOrderResponse, WxErrorException>;

    /// 依据商户订单号取消配送单（对应 Java `cancelOrderByStoreOrderId`）。
    async fn cancel_order_by_store_order_id(
        &self,
        wx_store_id: &str,
        store_order_id: &str,
        cancel_reason_id: i32,
        cancel_reason: Option<&str>,
    ) -> Result<WxMaCancelOrderResponse, WxErrorException>;

    /// 查询支持同城配送的城市（对应 Java `getCity`，`service_trans_id` 传空则
    /// 返回所有）。
    async fn get_city(
        &self,
        service_trans_id: Option<&str>,
    ) -> Result<Vec<WxMaTransCity>, WxErrorException>;
}
