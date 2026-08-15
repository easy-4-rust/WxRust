//! 对应 Java `service.impl.TransferServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{TransferService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// TransferService 实现（对应 Java `TransferServiceImpl`）。
pub struct TransferServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl TransferServiceImpl {
    /// 构建实现（对应 Java 构造器 `TransferServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }
}

#[async_trait]
impl TransferService for TransferServiceImpl {
    async fn transfer_batches(
        &self,
        request: &TransferBatchesRequest,
    ) -> Result<TransferBatchesResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：对每个 transfer_detail_list 元素执行 encryptFields（user_name）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &["transfer_detail_list.*.user_name"],
        )?;
        let url = format!("{}/v3/transfer/batches", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<TransferNotifyResult, WxErrorException> {
        // 对应 Java `baseParseOrderNotifyV3Result`：验签 + AES-GCM 解密
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn transfer_batches_batch_id(
        &self,
        request: &QueryTransferBatchesRequest,
    ) -> Result<QueryTransferBatchesResult, WxErrorException> {
        let svc = self.svc()?;
        let url = if request.need_query_detail.unwrap_or_default() {
            format!(
                "{}/v3/transfer/batches/batch-id/{}?need_query_detail=true&offset={}&limit={}&detail_status={}",
                svc.get_pay_base_url(),
                request.batch_id.as_deref().unwrap_or_default(),
                request.offset.unwrap_or_default(),
                request.limit.unwrap_or_default(),
                request.detail_status.as_deref().unwrap_or_default()
            )
        } else {
            format!(
                "{}/v3/transfer/batches/batch-id/{}?need_query_detail=false",
                svc.get_pay_base_url(),
                request.batch_id.as_deref().unwrap_or_default()
            )
        };
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_batches_batch_id_detail(
        &self,
        batch_id: &str,
        detail_id: &str,
    ) -> Result<TransferBatchDetailResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer/batches/batch-id/{batch_id}/details/detail-id/{detail_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_batches_out_batch_no(
        &self,
        request: &QueryTransferBatchesRequest,
    ) -> Result<QueryTransferBatchesResult, WxErrorException> {
        let svc = self.svc()?;
        let url = if request.need_query_detail.unwrap_or_default() {
            format!(
                "{}/v3/transfer/batches/out-batch-no/{}?need_query_detail=true&offset={}&limit={}&detail_status={}",
                svc.get_pay_base_url(),
                request.out_batch_no.as_deref().unwrap_or_default(),
                request.offset.unwrap_or_default(),
                request.limit.unwrap_or_default(),
                request.detail_status.as_deref().unwrap_or_default()
            )
        } else {
            format!(
                "{}/v3/transfer/batches/out-batch-no/{}?need_query_detail=false",
                svc.get_pay_base_url(),
                request.out_batch_no.as_deref().unwrap_or_default()
            )
        };
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_batches_out_batch_no_detail(
        &self,
        out_batch_no: &str,
        out_detail_no: &str,
    ) -> Result<TransferBatchDetailResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/transfer/batches/out-batch-no/{out_batch_no}/details/out-detail-no/{out_detail_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transfer_bills(
        &self,
        request: &TransferBillsRequest,
    ) -> Result<TransferBillsResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：user_name 非空时才加密
        let user_name = json
            .get("user_name")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if !user_name.trim().is_empty() {
            let public_key = impl_utils::platform_public_key(config.as_ref())?;
            impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["user_name"])?;
        }
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn transform_bills_cancel(
        &self,
        out_bill_no: &str,
    ) -> Result<TransferBillsCancelResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills/out-bill-no/{out_bill_no}/cancel",
            svc.get_pay_base_url()
        );
        let result = svc.post_v3(&url, "").await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_bills_by_out_bill_no(
        &self,
        out_bill_no: &str,
    ) -> Result<TransferBillsGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills/out-bill-no/{out_bill_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_bills_by_transfer_bill_no(
        &self,
        transfer_bill_no: &str,
    ) -> Result<TransferBillsGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills/transfer-bill-no/{transfer_bill_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_transfer_bills_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<TransferBillsNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn get_user_authorization_status(
        &self,
        openid: &str,
        transfer_scene_id: &str,
    ) -> Result<UserAuthorizationStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/authorization/openid/{openid}?transfer_scene_id={transfer_scene_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn reservation_transfer_batch(
        &self,
        request: &ReservationTransferBatchRequest,
    ) -> Result<ReservationTransferBatchResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：transfer_detail_list 元素 user_name 非空时加密
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        if let Some(list) = json
            .get_mut("transfer_detail_list")
            .and_then(|v| v.as_array_mut())
        {
            for item in list.iter_mut() {
                let user_name = item
                    .get("user_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                if !user_name.trim().is_empty() {
                    impl_utils::encrypt_spec_fields_json(item, &public_key, &["user_name"])?;
                }
            }
        }
        let url = format!(
            "{}/v3/fund-app/mch-transfer/reservation/transfer-batches",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_reservation_transfer_batch_by_out_batch_no(
        &self,
        out_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = build_reservation_batch_query_url(
            svc.get_pay_base_url(),
            "out-batch-no",
            out_batch_no,
            Some(need_query_detail),
            Some(offset),
            Some(limit),
            detail_state,
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_reservation_transfer_batch_by_reservation_batch_no(
        &self,
        reservation_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = build_reservation_batch_query_url(
            svc.get_pay_base_url(),
            "reservation-batch-no",
            reservation_batch_no,
            Some(need_query_detail),
            Some(offset),
            Some(limit),
            detail_state,
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_reservation_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ReservationTransferNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn close_reservation_transfer_batch(
        &self,
        out_batch_no: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/reservation/transfer-batches/out-batch-no/{out_batch_no}/close",
            svc.get_pay_base_url()
        );
        svc.post_v3(&url, "").await?;
        Ok(())
    }
}

/// 预约批次查询 URL 组装（对应 Java `buildReservationBatchQueryUrl`）。
fn build_reservation_batch_query_url(
    base_url: String,
    batch_no_type: &str,
    batch_no: &str,
    need_query_detail: Option<bool>,
    offset: Option<i32>,
    limit: Option<i32>,
    detail_state: &str,
) -> String {
    let mut url = format!(
        "{base_url}/v3/fund-app/mch-transfer/reservation/transfer-batches/{batch_no_type}/{batch_no}"
    );
    let mut has_params = false;
    if let Some(need) = need_query_detail {
        url.push_str(&format!("?need_query_detail={need}"));
        has_params = true;
    }
    if let Some(v) = offset {
        url.push_str(&format!("{}offset={v}", if has_params { "&" } else { "?" }));
        has_params = true;
    }
    if let Some(v) = limit {
        url.push_str(&format!("{}limit={v}", if has_params { "&" } else { "?" }));
        has_params = true;
    }
    if !detail_state.trim().is_empty() {
        url.push_str(&format!(
            "{}detail_state={detail_state}",
            if has_params { "&" } else { "?" }
        ));
    }
    url
}
