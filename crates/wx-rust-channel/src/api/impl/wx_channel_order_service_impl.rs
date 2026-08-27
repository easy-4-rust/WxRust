//! WxChannelOrderServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelOrderServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_order_service::WxChannelOrderService;
use crate::bean::base::{AddressInfo, WxChannelBaseResponse};
use crate::bean::delivery::{
    DeliveryCompanyResponse, DeliveryInfo, DeliverySendParam, FreshInspectParam, PackageAuditInfo,
};
use crate::bean::order::{
    ChangeOrderInfo, DecodeSensitiveInfoResponse, DeliveryUpdateParam, OrderAddressParam,
    OrderCompensationDeliveryParam, OrderIdParam, OrderInfoResponse, OrderListParam,
    OrderListResponse, OrderRemarkParam, OrderSearchParam, PreShipmentChangeSkuRejectParam,
    PreShipmentChangeSkuResponse, PresentNoteAddParam, PresentSubOrderResponse,
    PrivateNumberAddPhoneParam, PrivateNumberGetPhoneResponse, PrivateNumberSendVerifyCodeParam,
    RealNumberViewAuditResponse, VirtualTelNumberResponse,
};
use crate::enums::url_delivery as delivery_url;
use crate::enums::url_order as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 订单服务实现。
pub struct WxChannelOrderServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelOrderServiceImpl {
    /// 构建订单服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelOrderService for WxChannelOrderServiceImpl {
    /// 对应 Java `WxChannelOrderServiceImpl.getOrder(String)`：
    /// `OrderInfoParam(orderId, null)`（空值跳过）后 POST `ORDER_GET_URL`。
    async fn get_order(&self, order_id: String) -> Result<OrderInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("order_id", serde_json::Value::String(order_id))]);
        let response = svc.post(url::ORDER_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getOrder(String, Boolean)`：
    /// `OrderInfoParam(orderId, encodeSensitiveInfo)`（空值跳过，Java Jackson
    /// `NON_NULL`）后 POST `ORDER_GET_URL`。
    async fn get_order_with_encode(
        &self,
        order_id: String,
        encode_sensitive_info: Option<bool>,
    ) -> Result<OrderInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            ("order_id", serde_json::Value::String(order_id)),
            (
                "encode_sensitive_info",
                encode_sensitive_info
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::ORDER_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getOrders`：
    /// 序列化 `OrderListParam` 后 POST `ORDER_LIST_URL`。
    async fn get_orders(
        &self,
        param: OrderListParam,
    ) -> Result<OrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ORDER_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.searchOrder`：
    /// 序列化 `OrderSearchParam` 后 POST `ORDER_SEARCH_URL`。
    async fn search_order(
        &self,
        param: OrderSearchParam,
    ) -> Result<OrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ORDER_SEARCH_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.updatePrice`：
    /// `OrderPriceParam(orderId, expressFee, changeOrderInfos)`（Java 构造器将
    /// `changeExpress` 置为 `expressFee != null`，`expressFee` 为空时按
    /// Jackson `NON_NULL` 跳过）后 POST `UPDATE_PRICE_URL`。
    async fn update_price(
        &self,
        order_id: String,
        express_fee: Option<i32>,
        change_order_infos: Vec<ChangeOrderInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            ("order_id", serde_json::Value::String(order_id)),
            (
                "change_express",
                serde_json::Value::from(express_fee.is_some()),
            ),
            (
                "express_fee",
                express_fee
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "change_order_infos",
                serde_json::to_value(&change_order_infos).unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::UPDATE_PRICE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.updateRemark`：
    /// 序列化 `OrderRemarkParam` 后 POST `UPDATE_REMARK_URL`。
    async fn update_remark(
        &self,
        order_id: String,
        merchant_notes: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderRemarkParam {
            order_id,
            merchant_notes,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_REMARK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.updateAddress`：
    /// 序列化 `OrderAddressParam` 后 POST `UPDATE_ADDRESS_URL`。
    async fn update_order_address(
        &self,
        order_id: String,
        user_address: AddressInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderAddressParam {
            order_id,
            user_address,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.updateDelivery`：
    /// 序列化 `DeliveryUpdateParam` 后 POST `UPDATE_EXPRESS_URL`。
    async fn update_delivery(
        &self,
        param: DeliveryUpdateParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_EXPRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.acceptAddressModify`：
    /// 序列化 `OrderIdParam` 后 POST `ACCEPT_ADDRESS_MODIFY_URL`。
    async fn accept_address_modify(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ACCEPT_ADDRESS_MODIFY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.rejectAddressModify`：
    /// 序列化 `OrderIdParam` 后 POST `REJECT_ADDRESS_MODIFY_URL`。
    async fn reject_address_modify(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::REJECT_ADDRESS_MODIFY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.closeOrder`：Java 暂不支持，
    /// 直接返回内部错误（err_code=-99，err_msg="内部错误"）。
    async fn close_order(
        &self,
        _order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        Ok(WxChannelBaseResponse {
            err_code: -99,
            err_msg: "内部错误".to_string(),
        })
    }

    /// 对应 Java `WxChannelOrderServiceImpl.listDeliveryCompany()`：
    /// POST `"{}"` 到 `GET_DELIVERY_COMPANY_URL`。
    async fn list_delivery_company(&self) -> Result<DeliveryCompanyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc
            .post(delivery_url::GET_DELIVERY_COMPANY_URL, "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.listDeliveryCompany(Boolean)`：
    /// POST `"{}"` 或 `{"ewaybill_only":..}` 到 `GET_DELIVERY_COMPANY_NEW_URL`。
    async fn list_delivery_company_ewaybill_only(
        &self,
        ewaybill_only: Option<bool>,
    ) -> Result<DeliveryCompanyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[(
            "ewaybill_only",
            ewaybill_only
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        )]);
        let response = svc
            .post(delivery_url::GET_DELIVERY_COMPANY_NEW_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.deliveryOrder`：
    /// 序列化 `DeliverySendParam` 后 POST `DELIVERY_SEND_URL`。
    async fn delivery_order(
        &self,
        order_id: String,
        delivery_list: Vec<DeliveryInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = DeliverySendParam {
            order_id,
            delivery_list,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(delivery_url::DELIVERY_SEND_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.uploadFreshInspect`：
    /// 序列化 `FreshInspectParam` 后 POST `UPLOAD_FRESH_INSPECT_URL`。
    async fn upload_fresh_inspect(
        &self,
        order_id: String,
        items: Vec<PackageAuditInfo>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = FreshInspectParam {
            order_id,
            audit_items: items,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPLOAD_FRESH_INSPECT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getVirtualTelNumber`：
    /// `{"order_id":".."}` 后 POST `VIRTUAL_TEL_NUMBER_URL`。
    async fn get_virtual_tel_number(
        &self,
        order_id: String,
    ) -> Result<VirtualTelNumberResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("order_id", serde_json::Value::String(order_id))]);
        let response = svc.post(url::VIRTUAL_TEL_NUMBER_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.decodeSensitiveInfo`：
    /// `{"order_id":".."}` 后 POST `DECODE_SENSITIVE_INFO_URL`。
    async fn decode_sensitive_info(
        &self,
        order_id: String,
    ) -> Result<DecodeSensitiveInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("order_id", serde_json::Value::String(order_id))]);
        let response = svc.post(url::DECODE_SENSITIVE_INFO_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.addPresentNote`：
    /// 序列化 `PresentNoteAddParam` 后 POST `PRESENT_NOTE_ADD_URL`。
    async fn add_present_note(
        &self,
        order_id: String,
        note: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = PresentNoteAddParam { order_id, note };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::PRESENT_NOTE_ADD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getPresentSubOrders`：
    /// 序列化 `OrderIdParam` 后 POST `PRESENT_SUB_ORDER_GET_URL`。
    async fn get_present_sub_orders(
        &self,
        order_id: String,
    ) -> Result<PresentSubOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::PRESENT_SUB_ORDER_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getPreShipmentChangeSku`：
    /// 序列化 `OrderIdParam` 后 POST `PRE_SHIPMENT_CHANGE_SKU_GET_URL`。
    async fn get_pre_shipment_change_sku(
        &self,
        order_id: String,
    ) -> Result<PreShipmentChangeSkuResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::PRE_SHIPMENT_CHANGE_SKU_GET_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.approvePreShipmentChangeSku`：
    /// 序列化 `OrderIdParam` 后 POST `PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL`。
    async fn approve_pre_shipment_change_sku(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.rejectPreShipmentChangeSku`：
    /// 序列化 `PreShipmentChangeSkuRejectParam` 后 POST
    /// `PRE_SHIPMENT_CHANGE_SKU_REJECT_URL`。
    async fn reject_pre_shipment_change_sku(
        &self,
        order_id: String,
        reject_reason: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = PreShipmentChangeSkuRejectParam {
            order_id,
            reject_reason,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::PRE_SHIPMENT_CHANGE_SKU_REJECT_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.applyRealNumber`：
    /// 序列化 `OrderIdParam` 后 POST `REAL_NUMBER_APPLY_URL`。
    async fn apply_real_number(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::REAL_NUMBER_APPLY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getRealNumberViewAudit`：
    /// 序列化 `OrderIdParam` 后 POST `REAL_NUMBER_VIEW_AUDIT_GET_URL`。
    async fn get_real_number_view_audit(
        &self,
        order_id: String,
    ) -> Result<RealNumberViewAuditResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::REAL_NUMBER_VIEW_AUDIT_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.applyVirtualNumberAgain`：
    /// 序列化 `OrderIdParam` 后 POST `VIRTUAL_NUMBER_APPLY_AGAIN_URL`。
    async fn apply_virtual_number_again(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::VIRTUAL_NUMBER_APPLY_AGAIN_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.delayVirtualNumber`：
    /// 序列化 `OrderIdParam` 后 POST `VIRTUAL_NUMBER_DELAY_URL`。
    async fn delay_virtual_number(
        &self,
        order_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = OrderIdParam { order_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::VIRTUAL_NUMBER_DELAY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.addPrivatePhone`：
    /// 序列化 `PrivateNumberAddPhoneParam` 后 POST `ADD_PHONE_URL`。
    async fn add_private_phone(
        &self,
        phone: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = PrivateNumberAddPhoneParam { phone };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_PHONE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.sendPrivatePhoneVerifyCode`：
    /// 序列化 `PrivateNumberSendVerifyCodeParam` 后 POST `SEND_VERIFY_CODE_URL`。
    async fn send_private_phone_verify_code(
        &self,
        phone: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = PrivateNumberSendVerifyCodeParam { phone };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SEND_VERIFY_CODE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.getPrivatePhone`：
    /// POST `"{}"` 到 `GET_PHONE_URL`。
    async fn get_private_phone(&self) -> Result<PrivateNumberGetPhoneResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::GET_PHONE_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelOrderServiceImpl.compensationDelivery`：
    /// 序列化 `OrderCompensationDeliveryParam` 后 POST `DELIVERY_COMPENSATION_URL`。
    async fn compensation_delivery(
        &self,
        param: OrderCompensationDeliveryParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::DELIVERY_COMPENSATION_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
