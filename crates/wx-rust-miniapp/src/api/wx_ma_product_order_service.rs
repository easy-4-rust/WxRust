//! 小程序交易组件-标准版-商品订单服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaProductOrderService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::product::{
    WxMiniBatchGetAfterSaleOrderResponse, WxMiniGetAfterSaleOrderResponse,
    WxMiniOrderDeliveryRequest, WxMinishopOrderDetailResponse, WxMinishopOrderListResponse,
};
use crate::bean::shop::response::WxMaShopBaseResponse;

/// 小程序交易组件-标准版-商品订单服务。
#[async_trait]
pub trait WxMaProductOrderService: Send + Sync {
    /// 获取订单列表（对应 Java `getOrderList(String, String, String, String, Integer, Integer, Integer, Integer)`）。
    ///
    /// `start_create_time` 等时间参数为字符串；`page` 最小填 1；`page_size`
    /// 每页数量（不超过 10,000）；`source` 1:小商店, 2:CPS 带货。
    #[allow(clippy::too_many_arguments)]
    async fn get_order_list(
        &self,
        start_create_time: Option<&str>,
        end_create_time: Option<&str>,
        start_update_time: Option<&str>,
        end_update_time: Option<&str>,
        status: Option<i32>,
        page: Option<i32>,
        page_size: Option<i32>,
        source: Option<i32>,
    ) -> Result<WxMinishopOrderListResponse, WxErrorException>;

    /// 获取订单详情（对应 Java `getOrderDetail(Long)`）。
    async fn get_order_detail(
        &self,
        order_id: i64,
    ) -> Result<WxMinishopOrderDetailResponse, WxErrorException>;

    /// 修改订单备注（对应 Java `changeMerchantNotes(Long, String)`，Java 无返回值）。
    async fn change_merchant_notes(
        &self,
        order_id: i64,
        merchant_notes: &str,
    ) -> Result<(), WxErrorException>;

    /// 订单发货（对应 Java `deliverySend(WxMiniOrderDeliveryRequest)`）。
    async fn delivery_send(
        &self,
        request: &WxMiniOrderDeliveryRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取售后单（对应 Java `getAfterSaleOrder(Long)`）。
    async fn get_after_sale_order(
        &self,
        after_sale_order_id: i64,
    ) -> Result<WxMiniGetAfterSaleOrderResponse, WxErrorException>;

    /// 批量获取售后单（对应 Java `batchGetAfterSaleOrder(List<Long>)`）。
    async fn batch_get_after_sale_order(
        &self,
        after_sale_order_id_list: &[i64],
    ) -> Result<WxMiniBatchGetAfterSaleOrderResponse, WxErrorException>;

    /// 同意售后申请（对应 Java `afterSaleAccept(Long, Long)`）。
    async fn after_sale_accept(
        &self,
        order_id: i64,
        address_id: i64,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 拒绝售后申请（对应 Java `afterSaleReject(Long, String)`）。
    async fn after_sale_reject(
        &self,
        after_sale_order_id: i64,
        reject_reason: &str,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
