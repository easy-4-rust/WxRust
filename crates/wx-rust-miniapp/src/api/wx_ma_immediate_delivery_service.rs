//! 微信小程序即时配送服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaImmediateDeliveryService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaBaseResponse;
use crate::bean::delivery::{
    AbnormalConfirmRequest, AbnormalConfirmResponse, AddOrderRequest, AddOrderResponse,
    BindAccountResponse, CancelOrderRequest, CancelOrderResponse, FollowWaybillRequest,
    FollowWaybillResponse, GetDeliveryListResponse, GetOrderRequest, GetOrderResponse,
    MockUpdateOrderRequest, MockUpdateOrderResponse, QueryFollowTraceRequest,
    QueryFollowTraceResponse, QueryWaybillTraceRequest, QueryWaybillTraceResponse,
    TraceWaybillRequest, TraceWaybillResponse, UpdateWaybillGoodsRequest,
};

/// 微信小程序即时配送服务。
#[async_trait]
pub trait WxMaImmediateDeliveryService: Send + Sync {
    /// 拉取已绑定账号（对应 Java `getBindAccount()`）。
    async fn get_bind_account(&self) -> Result<BindAccountResponse, WxErrorException>;

    /// 下配送单接口（对应 Java `addOrder(AddOrderRequest)`，
    /// 自动计算并携带运力侧签名 delivery_sign）。
    async fn add_order(
        &self,
        request: &AddOrderRequest,
    ) -> Result<AddOrderResponse, WxErrorException>;

    /// 拉取配送单信息（对应 Java `getOrder(GetOrderRequest)`）。
    async fn get_order(
        &self,
        request: &GetOrderRequest,
    ) -> Result<GetOrderResponse, WxErrorException>;

    /// 取消配送单接口（对应 Java `cancelOrder(CancelOrderRequest)`）。
    async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<CancelOrderResponse, WxErrorException>;

    /// 异常件退回商家确认收货接口（对应 Java `abnormalConfirm(AbnormalConfirmRequest)`）。
    async fn abnormal_confirm(
        &self,
        request: &AbnormalConfirmRequest,
    ) -> Result<AbnormalConfirmResponse, WxErrorException>;

    /// 模拟配送公司更新配送单状态（对应 Java `mockUpdateOrder(MockUpdateOrderRequest)`，
    /// 仅用于沙盒环境）。
    async fn mock_update_order(
        &self,
        request: &MockUpdateOrderRequest,
    ) -> Result<MockUpdateOrderResponse, WxErrorException>;

    /// 传运单（对应 Java `traceWaybill(TraceWaybillRequest)`，
    /// 向微信提供交易单号对应的运单号）。
    async fn trace_waybill(
        &self,
        request: &TraceWaybillRequest,
    ) -> Result<TraceWaybillResponse, WxErrorException>;

    /// 查询运单详情（对应 Java `queryWaybillTrace(QueryWaybillTraceRequest)`）。
    async fn query_waybill_trace(
        &self,
        request: &QueryWaybillTraceRequest,
    ) -> Result<QueryWaybillTraceResponse, WxErrorException>;

    /// 传运单（订阅消息版，对应 Java `followWaybill(FollowWaybillRequest)`）。
    async fn follow_waybill(
        &self,
        request: &FollowWaybillRequest,
    ) -> Result<FollowWaybillResponse, WxErrorException>;

    /// 查运单（订阅消息版，对应 Java `queryFollowTrace(QueryFollowTraceRequest)`）。
    async fn query_follow_trace(
        &self,
        request: &QueryFollowTraceRequest,
    ) -> Result<QueryFollowTraceResponse, WxErrorException>;

    /// 获取运力 id 列表（对应 Java `getDeliveryList()`）。
    async fn get_delivery_list(&self) -> Result<GetDeliveryListResponse, WxErrorException>;

    /// 更新物流物品信息接口（对应 Java `updateWaybillGoods(UpdateWaybillGoodsRequest)`）。
    async fn update_waybill_goods(
        &self,
        request: &UpdateWaybillGoodsRequest,
    ) -> Result<WxMaBaseResponse, WxErrorException>;
}
