//! WxChannelOrderService（对应 Java `me.chanjar.weixin.channel.api.WxChannelOrderService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::{AddressInfo, WxChannelBaseResponse};
use crate::bean::delivery::{DeliveryCompanyResponse, DeliveryInfo, PackageAuditInfo};
use crate::bean::order::{
    ChangeOrderInfo, DecodeSensitiveInfoResponse, DeliveryUpdateParam,
    OrderCompensationDeliveryParam, OrderInfoResponse, OrderListParam, OrderListResponse,
    OrderSearchParam, PreShipmentChangeSkuResponse, PresentSubOrderResponse,
    PrivateNumberGetPhoneResponse, RealNumberViewAuditResponse, VirtualTelNumberResponse,
};

/// 订单服务（对应 Java `WxChannelOrderService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_order_service_impl` 的
/// `WxChannelOrderServiceImpl`（Java `WxChannelOrderServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelOrderService: Send + Sync {
    /// 获取订单（对应 Java `WxChannelOrderService#getOrder(String)`）。
    async fn get_order(&self, order_id: String) -> Result<OrderInfoResponse, WxErrorException>;

    /// 获取订单详情（对应 Java
    /// `WxChannelOrderService#getOrder(String, Boolean)`）。
    ///
    /// # 参数
    /// - `encode_sensitive_info`：是否编码敏感信息
    async fn get_order_with_encode(
        &self,
        order_id: String,
        encode_sensitive_info: Option<bool>,
    ) -> Result<OrderInfoResponse, WxErrorException>;

    /// 获取订单列表（对应 Java `WxChannelOrderService#getOrders(OrderListParam)`）。
    async fn get_orders(
        &self,
        param: OrderListParam,
    ) -> Result<OrderListResponse, WxErrorException>;

    /// 订单搜索（对应 Java `WxChannelOrderService#searchOrder(OrderSearchParam)`）。
    async fn search_order(
        &self,
        param: OrderSearchParam,
    ) -> Result<OrderListResponse, WxErrorException>;

    /// 更改订单价格（对应 Java
    /// `WxChannelOrderService#updatePrice(String, Integer, List<ChangeOrderInfo>)`）。
    ///
    /// # 参数
    /// - `express_fee`：运费价格（以分为单位，不填不改）
    /// - `change_order_infos`：改价列表
    async fn update_price(
        &self,
        order_id: String,
        express_fee: Option<i32>,
        change_order_infos: Vec<ChangeOrderInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 更改订单备注（对应 Java `WxChannelOrderService#updateRemark(String, String)`）。
    async fn update_remark(
        &self,
        order_id: String,
        merchant_notes: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 更新订单地址（对应 Java `WxChannelOrderService#updateAddress(String, AddressInfo)`）。
    async fn update_order_address(
        &self,
        order_id: String,
        user_address: AddressInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 修改物流信息（发货完成的订单可以修改，最多修改 1 次；拆包发货的订单暂
    /// 不允许修改物流；虚拟商品订单暂不允许修改物流；对应 Java
    /// `WxChannelOrderService#updateDelivery(DeliveryUpdateParam)`）。
    async fn update_delivery(
        &self,
        param: DeliveryUpdateParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 同意用户修改收货地址请求（对应 Java
    /// `WxChannelOrderService#acceptAddressModify(String)`）。
    async fn accept_address_modify(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 拒接用户修改收货地址请求（对应 Java
    /// `WxChannelOrderService#rejectAddressModify(String)`）。
    async fn reject_address_modify(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 关闭订单（需要订单状态为未付款状态；对应 Java
    /// `WxChannelOrderService#closeOrder(String)`；Java 当前返回内部错误占位）。
    async fn close_order(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取快递公司列表-旧（对应 Java `WxChannelOrderService#listDeliveryCompany()`）。
    async fn list_delivery_company(&self) -> Result<DeliveryCompanyResponse, WxErrorException>;

    /// 获取快递公司列表（对应 Java
    /// `WxChannelOrderService#listDeliveryCompany(Boolean)`）。
    ///
    /// # 参数
    /// - `ewaybill_only`：是否仅返回支持电子面单功能的快递公司
    async fn list_delivery_company_ewaybill_only(
        &self,
        ewaybill_only: Option<bool>,
    ) -> Result<DeliveryCompanyResponse, WxErrorException>;

    /// 订单发货（对应 Java
    /// `WxChannelOrderService#deliveryOrder(String, List<DeliveryInfo>)`）。
    async fn delivery_order(
        &self,
        order_id: String,
        delivery_list: Vec<DeliveryInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 上传生鲜质检信息（对应 Java
    /// `WxChannelOrderService#uploadFreshInspect(String, List<PackageAuditInfo>)`）。
    async fn upload_fresh_inspect(
        &self,
        order_id: String,
        items: Vec<PackageAuditInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 兑换虚拟号（对应 Java `WxChannelOrderService#getVirtualTelNumber(String)`）。
    async fn get_virtual_tel_number(
        &self,
        order_id: String,
    ) -> Result<VirtualTelNumberResponse, WxErrorException>;

    /// 解码订单包含的敏感数据（对应 Java
    /// `WxChannelOrderService#decodeSensitiveInfo(String)`）。
    async fn decode_sensitive_info(
        &self,
        order_id: String,
    ) -> Result<DecodeSensitiveInfoResponse, WxErrorException>;

    /// 礼物订单新增备注（对应 Java `WxChannelOrderService#addPresentNote(String, String)`）。
    async fn add_present_note(
        &self,
        order_id: String,
        note: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取礼物子单列表（对应 Java `WxChannelOrderService#getPresentSubOrders(String)`）。
    async fn get_present_sub_orders(
        &self,
        order_id: String,
    ) -> Result<PresentSubOrderResponse, WxErrorException>;

    /// 获取待发货前更换 SKU 请求（对应 Java
    /// `WxChannelOrderService#getPreShipmentChangeSku(String)`）。
    async fn get_pre_shipment_change_sku(
        &self,
        order_id: String,
    ) -> Result<PreShipmentChangeSkuResponse, WxErrorException>;

    /// 同意待发货前更换 SKU（对应 Java
    /// `WxChannelOrderService#approvePreShipmentChangeSku(String)`）。
    async fn approve_pre_shipment_change_sku(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 拒绝待发货前更换 SKU（对应 Java
    /// `WxChannelOrderService#rejectPreShipmentChangeSku(String, String)`）。
    async fn reject_pre_shipment_change_sku(
        &self,
        order_id: String,
        reject_reason: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 申请真实号（对应 Java `WxChannelOrderService#applyRealNumber(String)`）。
    async fn apply_real_number(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查看真实号审核状态（对应 Java
    /// `WxChannelOrderService#getRealNumberViewAudit(String)`）。
    async fn get_real_number_view_audit(
        &self,
        order_id: String,
    ) -> Result<RealNumberViewAuditResponse, WxErrorException>;

    /// 再次申请虚拟号（对应 Java
    /// `WxChannelOrderService#applyVirtualNumberAgain(String)`）。
    async fn apply_virtual_number_again(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 延长虚拟号有效期（对应 Java `WxChannelOrderService#delayVirtualNumber(String)`）。
    async fn delay_virtual_number(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 添加待认证手机号（对应 Java `WxChannelOrderService#addPrivatePhone(String)`）。
    async fn add_private_phone(
        &self,
        phone: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取短信验证码（对应 Java
    /// `WxChannelOrderService#sendPrivatePhoneVerifyCode(String)`）。
    async fn send_private_phone_verify_code(
        &self,
        phone: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取小店手机号认证状态（对应 Java `WxChannelOrderService#getPrivatePhone`）。
    async fn get_private_phone(&self) -> Result<PrivateNumberGetPhoneResponse, WxErrorException>;

    /// 订单补发货（对应 Java
    /// `WxChannelOrderService#compensationDelivery(OrderCompensationDeliveryParam)`）。
    async fn compensation_delivery(
        &self,
        param: OrderCompensationDeliveryParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
