//! WxChannelAfterSaleService（对应 Java `me.chanjar.weixin.channel.api.WxChannelAfterSaleService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::after::{
    AfterSaleInfoResponse, AfterSaleListParam, AfterSaleListResponse, AfterSaleMerchantUpdateParam,
    AfterSaleReasonResponse, AfterSaleRejectReasonResponse,
};
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::complaint::ComplaintOrderResponse;

/// 售后服务（对应 Java `WxChannelAfterSaleService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_after_sale_service_impl` 的
/// `WxChannelAfterSaleServiceImpl`（Java `WxChannelAfterSaleServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelAfterSaleService: Send + Sync {
    /// 获取售后单列表（对应 Java
    /// `WxChannelAfterSaleService#listIds(Long, Long, String)`，`@Deprecated`，
    /// 请使用 `list_ids_by_param`）。
    ///
    /// # 参数
    /// - `begin_create_time`：订单创建启始时间 unix 时间戳
    /// - `end_create_time`：订单创建结束时间，`end - begin` 不得大于 24 小时
    /// - `next_key`：翻页参数，从第二页开始传，来源于上一页的返回值
    async fn list_ids(
        &self,
        begin_create_time: Option<i64>,
        end_create_time: Option<i64>,
        next_key: String,
    ) -> Result<AfterSaleListResponse, WxErrorException>;

    /// 获取售后单列表（对应 Java
    /// `WxChannelAfterSaleService#listIds(AfterSaleListParam)`）。
    async fn list_ids_by_param(
        &self,
        param: AfterSaleListParam,
    ) -> Result<AfterSaleListResponse, WxErrorException>;

    /// 获取售后单详情（对应 Java `WxChannelAfterSaleService#get(String)`）。
    async fn get_after_sale(
        &self,
        after_sale_order_id: String,
    ) -> Result<AfterSaleInfoResponse, WxErrorException>;

    /// 同意售后（对应 Java
    /// `WxChannelAfterSaleService#accept(String, String, Integer)`）。
    ///
    /// # 参数
    /// - `address_id`：同意退货时传入地址 id
    /// - `accept_type`：1 同意退货退款并通知用户退货；2 确认收到货并退款给用户。
    ///   不填则根据当前售后单状态自动选择相应操作
    async fn accept(
        &self,
        after_sale_order_id: String,
        address_id: String,
        accept_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 拒绝售后（对应 Java
    /// `WxChannelAfterSaleService#reject(String, String, Integer)`）。
    async fn reject(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 拒绝售后（支持拒绝凭证；对应 Java
    /// `WxChannelAfterSaleService#reject(String, String, Integer, List<String>)`）。
    async fn reject_with_certificates(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 上传退款凭证（对应 Java
    /// `WxChannelAfterSaleService#uploadRefundEvidence(String, String, List<String>)`）。
    async fn upload_refund_evidence(
        &self,
        after_sale_order_id: String,
        desc: String,
        certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 商家补充纠纷单留言（对应 Java
    /// `WxChannelAfterSaleService#addComplaintMaterial(String, String, List<String>)`）。
    async fn add_complaint_material(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 商家举证（对应 Java
    /// `WxChannelAfterSaleService#addComplaintEvidence(String, String, List<String>)`）。
    async fn add_complaint_evidence(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取纠纷单（对应 Java `WxChannelAfterSaleService#getComplaint(String)`）。
    async fn get_complaint(
        &self,
        complaint_id: String,
    ) -> Result<ComplaintOrderResponse, WxErrorException>;

    /// 获取全量售后原因（对应 Java `WxChannelAfterSaleService#getAllReason()`）。
    async fn get_all_reason(&self) -> Result<AfterSaleReasonResponse, WxErrorException>;

    /// 获取拒绝售后原因（对应 Java `WxChannelAfterSaleService#getRejectReason()`）。
    async fn get_reject_reason(&self) -> Result<AfterSaleRejectReasonResponse, WxErrorException>;

    /// 换货发货（对应 Java
    /// `WxChannelAfterSaleService#acceptExchangeReship(String, String, String)`）。
    ///
    /// # 参数
    /// - `waybill_id`：快递单号
    /// - `delivery_id`：快递公司 id
    async fn accept_exchange_reship(
        &self,
        after_sale_order_id: String,
        waybill_id: String,
        delivery_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 换货拒绝发货（对应 Java
    /// `WxChannelAfterSaleService#rejectExchangeReship(String, String, Integer, List<String>)`）。
    async fn reject_exchange_reship(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 商家协商（对应 Java
    /// `WxChannelAfterSaleService#merchantUpdateAfterSale(AfterSaleMerchantUpdateParam)`）。
    async fn merchant_update_after_sale(
        &self,
        param: AfterSaleMerchantUpdateParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
