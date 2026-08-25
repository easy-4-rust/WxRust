//! WxChannelEwaybillService（对应 Java `me.chanjar.weixin.channel.api.WxChannelEwaybillService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::ewaybill::{
    AccountInfoResponse, AddSubOrderRequest, BatchPrintOrderRequest, CreateOrderRequest,
    CreateOrderResponse, DeliveryListResponse, OrderDetailResponse, PreCreateRequest,
    PreCreateResponse, PrintContentResponse, PrintOrderRequest, TemplateConfigResponse,
    TemplateCreateRequest, TemplateIdResponse, TemplateInfoResponse, TemplateUpdateRequest,
};

/// 电子面单服务（对应 Java `WxChannelEwaybillService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_ewaybill_service_impl` 的
/// `WxChannelEwaybillServiceImpl`（Java `WxChannelEwaybillServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelEwaybillService: Send + Sync {
    /// 获取可用的标准面单模板（对应 Java `WxChannelEwaybillService#getTemplateConfig()`）。
    async fn get_template_config(&self) -> Result<TemplateConfigResponse, WxErrorException>;

    /// 创建商家面单模板（对应 Java `WxChannelEwaybillService#createTemplate(TemplateCreateRequest)`）。
    async fn create_template(
        &self,
        req: TemplateCreateRequest,
    ) -> Result<TemplateIdResponse, WxErrorException>;

    /// 删除商家面单模板（对应 Java `WxChannelEwaybillService#deleteTemplate(String)`）。
    async fn delete_template(
        &self,
        template_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 更新商家面单模板（对应 Java `WxChannelEwaybillService#updateTemplate(TemplateUpdateRequest)`）。
    async fn update_template(
        &self,
        req: TemplateUpdateRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询标准模板信息（对应 Java `WxChannelEwaybillService#getTemplate(String)`）。
    async fn get_template(
        &self,
        template_code: String,
    ) -> Result<TemplateInfoResponse, WxErrorException>;

    /// 按模板 ID 查询商家模板（对应 Java `WxChannelEwaybillService#getTemplateById(String)`）。
    async fn get_template_by_id(
        &self,
        template_id: String,
    ) -> Result<TemplateInfoResponse, WxErrorException>;

    /// 查询已开通电子面单的网点和账号（对应 Java `WxChannelEwaybillService#getAccount()`）。
    async fn get_account(&self) -> Result<AccountInfoResponse, WxErrorException>;

    /// 查询已开通电子面单的快递公司（对应 Java `WxChannelEwaybillService#getDeliveryList()`）。
    async fn get_delivery_list(&self) -> Result<DeliveryListResponse, WxErrorException>;

    /// 预取电子面单号（对应 Java `WxChannelEwaybillService#preCreateOrder(PreCreateRequest)`）。
    async fn pre_create_order(
        &self,
        req: PreCreateRequest,
    ) -> Result<PreCreateResponse, WxErrorException>;

    /// 获取电子面单号（对应 Java `WxChannelEwaybillService#createOrder(CreateOrderRequest)`）。
    async fn create_order(
        &self,
        req: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, WxErrorException>;

    /// 追加电子面单子件（对应 Java `WxChannelEwaybillService#addSubOrder(AddSubOrderRequest)`）。
    async fn add_sub_order(
        &self,
        req: AddSubOrderRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 取消电子面单下单（对应 Java `WxChannelEwaybillService#cancelOrder(PrintOrderRequest)`）。
    async fn cancel_order(
        &self,
        req: PrintOrderRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询电子面单详情（对应 Java `WxChannelEwaybillService#getOrder(String)`）。
    async fn get_order(
        &self,
        ewaybill_order_id: String,
    ) -> Result<OrderDetailResponse, WxErrorException>;

    /// 获取打印报文（对应 Java `WxChannelEwaybillService#getPrintContent(String, String)`）。
    async fn get_print_content(
        &self,
        ewaybill_order_id: String,
        template_id: String,
    ) -> Result<PrintContentResponse, WxErrorException>;

    /// 通知单个运单打印成功（对应 Java `WxChannelEwaybillService#printOrder(PrintOrderRequest)`）。
    async fn print_order(
        &self,
        req: PrintOrderRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 批量通知运单打印成功（对应 Java `WxChannelEwaybillService#batchPrintOrder(BatchPrintOrderRequest)`）。
    async fn batch_print_order(
        &self,
        req: BatchPrintOrderRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
