//! WxChannelAfterSaleServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelAfterSaleServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_after_sale_service::WxChannelAfterSaleService;
use crate::bean::after::{
    AfterSaleAcceptExchangeReshipParam, AfterSaleIdParam, AfterSaleInfoResponse,
    AfterSaleListParam, AfterSaleListResponse, AfterSaleMerchantUpdateParam,
    AfterSaleReasonResponse, AfterSaleRejectReasonResponse, RefundEvidenceParam,
};
use crate::bean::base::WxChannelBaseResponse;
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
pub struct WxChannelAfterSaleServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelAfterSaleServiceImpl {
    /// 构建售后服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelAfterSaleService for WxChannelAfterSaleServiceImpl {
    /// 对应 Java `WxChannelAfterSaleServiceImpl.listIds(Long, Long, String)`：
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
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
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

    /// 对应 Java `WxChannelAfterSaleServiceImpl.listIds(AfterSaleListParam)`：
    /// 序列化参数后 POST `AFTER_SALE_LIST_URL`。
    async fn list_ids_by_param(
        &self,
        param: AfterSaleListParam,
    ) -> Result<AfterSaleListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.get`：
    /// 序列化 `AfterSaleIdParam` 后 POST `AFTER_SALE_GET_URL`。
    async fn get_after_sale(
        &self,
        after_sale_order_id: String,
    ) -> Result<AfterSaleInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AfterSaleIdParam {
            after_sale_order_id,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.accept`：
    /// `AfterSaleAcceptParam`（空值跳过）后 POST `AFTER_SALE_ACCEPT_URL`。
    async fn accept(
        &self,
        after_sale_order_id: String,
        address_id: String,
        accept_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
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

    /// 对应 Java `WxChannelAfterSaleServiceImpl.reject(String, String, Integer)`：
    /// 委托给带凭证版本（凭证为空跳过）。
    async fn reject(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        self.reject_with_certificates(
            after_sale_order_id,
            reject_reason,
            reject_reason_type,
            Vec::new(),
        )
        .await
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.reject(String, String, Integer, List)`：
    /// `AfterSaleRejectParam`（空值跳过）后 POST `AFTER_SALE_REJECT_URL`。
    async fn reject_with_certificates(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
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

    /// 对应 Java `WxChannelAfterSaleServiceImpl.uploadRefundEvidence`：
    /// 序列化 `RefundEvidenceParam` 后 POST `AFTER_SALE_UPLOAD_URL`。
    async fn upload_refund_evidence(
        &self,
        after_sale_order_id: String,
        desc: String,
        certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = RefundEvidenceParam {
            after_sale_order_id,
            desc,
            certificates,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(after_url::AFTER_SALE_UPLOAD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.addComplaintMaterial`：
    /// 序列化 `ComplaintParam` 后 POST `ADD_COMPLAINT_MATERIAL_URL`。
    async fn add_complaint_material(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = ComplaintParam {
            complaint_id,
            content,
            media_ids,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(complaint_url::ADD_COMPLAINT_MATERIAL_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.addComplaintEvidence`：
    /// 序列化 `ComplaintParam` 后 POST `ADD_COMPLAINT_PROOF_URL`。
    async fn add_complaint_evidence(
        &self,
        complaint_id: String,
        content: String,
        media_ids: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = ComplaintParam {
            complaint_id,
            content,
            media_ids,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(complaint_url::ADD_COMPLAINT_PROOF_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.getComplaint`：
    /// `{"complaint_id":".."}` 后 POST `GET_COMPLAINT_ORDER_URL`。
    async fn get_complaint(
        &self,
        complaint_id: String,
    ) -> Result<ComplaintOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("complaint_id", serde_json::Value::String(complaint_id))]);
        let response = svc
            .post(complaint_url::GET_COMPLAINT_ORDER_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.getAllReason`：
    /// POST `"{}"` 到 `AFTER_SALE_REASON_GET_URL`。
    async fn get_all_reason(&self) -> Result<AfterSaleReasonResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(after_url::AFTER_SALE_REASON_GET_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.getRejectReason`：
    /// POST `"{}"` 到 `AFTER_SALE_REJECT_REASON_GET_URL`。
    async fn get_reject_reason(&self) -> Result<AfterSaleRejectReasonResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc
            .post(after_url::AFTER_SALE_REJECT_REASON_GET_URL, "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.acceptExchangeReship`：
    /// 序列化 `AfterSaleAcceptExchangeReshipParam` 后 POST
    /// `AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL`。
    async fn accept_exchange_reship(
        &self,
        after_sale_order_id: String,
        waybill_id: String,
        delivery_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AfterSaleAcceptExchangeReshipParam {
            after_sale_order_id,
            waybill_id,
            delivery_id,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::AFTER_SALE_ACCEPT_EXCHANGE_RESHIP_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAfterSaleServiceImpl.rejectExchangeReship`：
    /// `AfterSaleRejectExchangeReshipParam`（空值跳过）后 POST
    /// `AFTER_SALE_REJECT_EXCHANGE_RESHIP_URL`。
    async fn reject_exchange_reship(
        &self,
        after_sale_order_id: String,
        reject_reason: String,
        reject_reason_type: Option<i32>,
        reject_certificates: Vec<String>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
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

    /// 对应 Java `WxChannelAfterSaleServiceImpl.merchantUpdateAfterSale`：
    /// 序列化 `AfterSaleMerchantUpdateParam` 后 POST
    /// `AFTER_SALE_MERCHANT_UPDATE_URL`。
    async fn merchant_update_after_sale(
        &self,
        param: AfterSaleMerchantUpdateParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(after_url::AFTER_SALE_MERCHANT_UPDATE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
