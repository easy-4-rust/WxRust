//! WxStoreAfterSaleServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreAfterSaleServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_after_sale_service::WxStoreAfterSaleService;
use crate::bean::after::{
    AfterSaleAcceptExchangeReshipParam, AfterSaleIdParam, AfterSaleInfoResponse,
    AfterSaleListParam, AfterSaleListResponse, AfterSaleMerchantUpdateParam,
    AfterSaleReasonResponse, AfterSaleRejectReasonResponse, GuaranteeModifyRequest,
    GuaranteeOrderIdParam, GuaranteeOrderInfoResponse, GuaranteeOrderListParam,
    GuaranteeOrderListResponse, GuaranteeProofRequest, GuaranteeRefuseRequest, RefundEvidenceParam,
};
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::complaint::{ComplaintOrderResponse, ComplaintParam};
use crate::enums::url_after_sale as after_url;
use crate::enums::url_complaint as complaint_url;

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

/// 售后服务实现。
pub struct WxStoreAfterSaleServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreAfterSaleServiceImpl {
    /// 构建售后服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreAfterSaleService for WxStoreAfterSaleServiceImpl {
    /// 对应 Java `WxStoreAfterSaleServiceImpl.listIds(Long, Long, String)`：
    /// `AfterSaleListParam(begin, end, null, null, nextKey)`（空值跳过）后
    /// POST `AFTER_SALE_LIST_URL`。
    async fn list_ids(
        &self,
        begin_create_time: Option<i64>,
        end_create_time: Option<i64>,
        next_key: String,
    ) -> Result<AfterSaleListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "begin_create_time",
                begin_create_time
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "end_create_time",
                end_create_time
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "next_key",
                if next_key.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(next_key)
                },
            ),
        ]);
        let response = svc.post(after_url::AFTER_SALE_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.listIds(AfterSaleListParam)`：
    /// 序列化参数后 POST `AFTER_SALE_LIST_URL`。
    async fn list_ids_by_param(
        &self,
        param: AfterSaleListParam,
    ) -> Result<AfterSaleListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.get`：
    /// 序列化 `AfterSaleIdParam` 后 POST `AFTER_SALE_GET_URL`。
    async fn get_after_sale(
        &self,
        after_sale_order_id: String,
    ) -> Result<AfterSaleInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = AfterSaleIdParam {
            after_sale_order_id: Some(after_sale_order_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.accept`：
    /// `AfterSaleAcceptParam`（空值跳过）后 POST `AFTER_SALE_ACCEPT_URL`。
    async fn accept(
        &self,
        after_sale_order_id: String,
        address_id: String,
        accept_type: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "after_sale_order_id",
                serde_json::Value::String(after_sale_order_id),
            ),
            (
                "address_id",
                if address_id.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(address_id)
                },
            ),
            (
                "accept_type",
                accept_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(after_url::AFTER_SALE_ACCEPT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.reject(String, String, Integer)`：
    /// 委托给带凭证版本（凭证为空跳过）。
    async fn reject(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        self.reject_with_certificates(
            after_sale_order_id,
            reject_reason,
            reject_reason_type,
            Vec::new(),
        )
        .await
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.reject(String, String, Integer, List)`：
    /// `AfterSaleRejectParam`（空值跳过）后 POST `AFTER_SALE_REJECT_URL`。
    async fn reject_with_certificates(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "after_sale_order_id",
                serde_json::Value::String(after_sale_order_id),
            ),
            ("reject_reason", serde_json::Value::String(reject_reason)),
            (
                "reject_reason_type",
                reject_reason_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "reject_certificates",
                if reject_certificates.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::to_value(&reject_certificates).unwrap_or(serde_json::Value::Null)
                },
            ),
        ]);
        let response = svc.post(after_url::AFTER_SALE_REJECT_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.uploadRefundEvidence`：
    /// 序列化 `RefundEvidenceParam` 后 POST `AFTER_SALE_UPLOAD_URL`。
    async fn upload_refund_evidence(
        &self,
        after_sale_order_id: String,
        desc: String,
        certificates: Vec<String>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = RefundEvidenceParam {
            after_sale_order_id: Some(after_sale_order_id),
            desc: Some(desc),
            certificates: Some(certificates),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_UPLOAD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.addComplaintMaterial`：
    /// 序列化 `ComplaintParam` 后 POST `ADD_COMPLAINT_MATERIAL_URL`。
    async fn add_complaint_material(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = ComplaintParam {
            complaint_id: Some(complaint_id),
            content: Some(content),
            media_ids: Some(media_ids),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(complaint_url::ADD_COMPLAINT_MATERIAL_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.addComplaintEvidence`：
    /// 序列化 `ComplaintParam` 后 POST `ADD_COMPLAINT_PROOF_URL`。
    async fn add_complaint_evidence(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = ComplaintParam {
            complaint_id: Some(complaint_id),
            content: Some(content),
            media_ids: Some(media_ids),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(complaint_url::ADD_COMPLAINT_PROOF_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.getComplaint`：
    /// `{"complaint_id":".."}` 后 POST `GET_COMPLAINT_ORDER_URL`。
    async fn get_complaint(
        &self,
        complaint_id: String,
    ) -> Result<ComplaintOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("complaint_id", serde_json::Value::String(complaint_id))]);
        let response = svc
            .post(complaint_url::GET_COMPLAINT_ORDER_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.getAllReason`：
    /// POST `"{}"` 到 `AFTER_SALE_REASON_GET_URL`。
    async fn get_all_reason(&self) -> Result<AfterSaleReasonResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(after_url::AFTER_SALE_REASON_GET_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.getRejectReason`：
    /// POST `"{}"` 到 `AFTER_SALE_REJECT_REASON_GET_URL`。
    async fn get_reject_reason(&self) -> Result<AfterSaleRejectReasonResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc
            .post(after_url::AFTER_SALE_REJECT_REASON_GET_URL, "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.acceptExchangeReship`：
    /// 序列化 `AfterSaleAcceptExchangeReshipParam` 后 POST
    /// `AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL`。
    async fn accept_exchange_reship(
        &self,
        after_sale_order_id: String,
        waybill_id: String,
        delivery_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = AfterSaleAcceptExchangeReshipParam {
            after_sale_order_id: Some(after_sale_order_id),
            waybill_id: Some(waybill_id),
            delivery_id: Some(delivery_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.rejectExchangeReship`：
    /// `AfterSaleRejectExchangeReshipParam`（空值跳过）后 POST
    /// `AFTER_SALE_REJECT_EXCHANGE_RESHIP_URL`。
    async fn reject_exchange_reship(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "after_sale_order_id",
                serde_json::Value::String(after_sale_order_id),
            ),
            ("reject_reason", serde_json::Value::String(reject_reason)),
            (
                "reject_reason_type",
                reject_reason_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "reject_certificates",
                if reject_certificates.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::to_value(&reject_certificates).unwrap_or(serde_json::Value::Null)
                },
            ),
        ]);
        let response = svc
            .post(after_url::AFTER_SALE_REJECT_EXCHANGE_RESHIP_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.merchantUpdateAfterSale`：
    /// 序列化 `AfterSaleMerchantUpdateParam` 后 POST
    /// `AFTER_SALE_MERCHANT_UPDATE_URL`。
    async fn merchant_update_after_sale(
        &self,
        param: AfterSaleMerchantUpdateParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::AFTER_SALE_MERCHANT_UPDATE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.listGuaranteeOrder`：
    /// 序列化 `GuaranteeOrderListParam` 后 POST `GUARANTEE_ORDER_LIST_URL`。
    async fn list_guarantee_order(
        &self,
        param: GuaranteeOrderListParam,
    ) -> Result<GuaranteeOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::GUARANTEE_ORDER_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.getGuaranteeOrder`：
    /// 序列化 `GuaranteeOrderIdParam` 后 POST `GUARANTEE_ORDER_GET_URL`。
    async fn get_guarantee_order(
        &self,
        guarantee_order_id: String,
    ) -> Result<GuaranteeOrderInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = GuaranteeOrderIdParam {
            guarantee_order_id: Some(guarantee_order_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::GUARANTEE_ORDER_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.acceptGuarantee`：
    /// 序列化 `GuaranteeOrderIdParam` 后 POST `GUARANTEE_ORDER_ACCEPT_URL`。
    async fn accept_guarantee(
        &self,
        guarantee_order_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = GuaranteeOrderIdParam {
            guarantee_order_id: Some(guarantee_order_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::GUARANTEE_ORDER_ACCEPT_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.modifyGuarantee`：
    /// 序列化 `GuaranteeModifyRequest` 后 POST `GUARANTEE_ORDER_MODIFY_URL`。
    async fn modify_guarantee(
        &self,
        request: GuaranteeModifyRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::GUARANTEE_ORDER_MODIFY_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.proofGuarantee`：
    /// 序列化 `GuaranteeProofRequest` 后 POST `GUARANTEE_ORDER_PROOF_URL`。
    async fn proof_guarantee(
        &self,
        request: GuaranteeProofRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::GUARANTEE_ORDER_PROOF_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreAfterSaleServiceImpl.refuseGuarantee`：
    /// 序列化 `GuaranteeRefuseRequest` 后 POST `GUARANTEE_ORDER_REFUSE_URL`。
    async fn refuse_guarantee(
        &self,
        request: GuaranteeRefuseRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::GUARANTEE_ORDER_REFUSE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
