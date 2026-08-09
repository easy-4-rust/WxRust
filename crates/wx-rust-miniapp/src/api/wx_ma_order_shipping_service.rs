//! 发货信息管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaOrderShippingService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::shipping::{
    WxMaOrderCombinedShippingInfoUploadRequest, WxMaOrderShippingInfoGetListRequest,
    WxMaOrderShippingInfoGetRequest, WxMaOrderShippingInfoNotifyConfirmRequest,
    WxMaOrderShippingInfoUploadRequest,
};
use crate::bean::shop::response::{
    WxMaOrderShippingITMCCompletedResult, WxMaOrderShippingInfoBaseResponse,
    WxMaOrderShippingInfoGetListResponse, WxMaOrderShippingInfoGetResponse,
    WxMaOrderShippingIsTradeManagedResponse,
};

/// 发货信息管理服务。
#[async_trait]
pub trait WxMaOrderShippingService: Send + Sync {
    /// 查询小程序是否已开通发货信息管理服务（对应 Java `isTradeManaged(String)`）。
    async fn is_trade_managed(
        &self,
        app_id: &str,
    ) -> Result<WxMaOrderShippingIsTradeManagedResponse, WxErrorException>;

    /// 发货信息录入接口（对应 Java `upload(WxMaOrderShippingInfoUploadRequest)`）。
    async fn upload(
        &self,
        request: &WxMaOrderShippingInfoUploadRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException>;

    /// 发货信息合单录入接口（对应 Java `upload(WxMaOrderCombinedShippingInfoUploadRequest)`）。
    async fn upload_combined(
        &self,
        request: &WxMaOrderCombinedShippingInfoUploadRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException>;

    /// 查询订单发货状态（对应 Java `get(WxMaOrderShippingInfoGetRequest)`）。
    async fn get(
        &self,
        request: &WxMaOrderShippingInfoGetRequest,
    ) -> Result<WxMaOrderShippingInfoGetResponse, WxErrorException>;

    /// 查询订单列表（对应 Java `getList(WxMaOrderShippingInfoGetListRequest)`）。
    async fn get_list(
        &self,
        request: &WxMaOrderShippingInfoGetListRequest,
    ) -> Result<WxMaOrderShippingInfoGetListResponse, WxErrorException>;

    /// 确认收货提醒接口（对应 Java `notifyConfirmReceive(WxMaOrderShippingInfoNotifyConfirmRequest)`）。
    async fn notify_confirm_receive(
        &self,
        request: &WxMaOrderShippingInfoNotifyConfirmRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException>;

    /// 消息跳转路径设置接口（对应 Java `setMsgJumpPath(String)`）。
    async fn set_msg_jump_path(
        &self,
        path: &str,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException>;

    /// 查询小程序是否已完成交易结算管理确认（对应 Java
    /// `isTradeManagementConfirmationCompleted(String)`）。
    async fn is_trade_management_confirmation_completed(
        &self,
        app_id: &str,
    ) -> Result<WxMaOrderShippingITMCCompletedResult, WxErrorException>;

    /// 特殊发货报备（对应 Java `opSpecialOrder(String, Integer, Long)`）。
    ///
    /// `type` 1 为预售商品订单，2 为测试订单；`delay_to` type 为 1 时必填。
    async fn op_special_order(
        &self,
        order_id: &str,
        r#type: i32,
        delay_to: Option<i64>,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException>;
}
