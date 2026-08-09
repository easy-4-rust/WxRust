//! 对应 Java `service.impl.BusinessOperationTransferServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{BusinessOperationTransferService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// BusinessOperationTransferService 实现（对应 Java `BusinessOperationTransferServiceImpl`）。
pub struct BusinessOperationTransferServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl BusinessOperationTransferServiceImpl {
    /// 构建实现（对应 Java 构造器 `BusinessOperationTransferServiceImpl(WxPayService)`）。
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
impl BusinessOperationTransferService for BusinessOperationTransferServiceImpl {
    async fn create_operation_transfer(
        &self,
        request: &BusinessOperationTransferRequest,
    ) -> Result<BusinessOperationTransferResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：appid 为空时从配置补齐
        if request
            .appid
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            request.appid = config.app_id().map(str::to_string);
        }
        let mut json =
            serde_json::to_value(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java：user_name 非空时才加密（RsaCryptoUtil.encryptFields）
        if request
            .user_name
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            // 不加密
        } else {
            let public_key = impl_utils::platform_public_key(config.as_ref())?;
            impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["user_name"])?;
        }
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_operation_transfer(
        &self,
        request: &BusinessOperationTransferQueryRequest,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException> {
        // 对应 Java：out_bill_no / transfer_bill_no 二选一，均空则报错
        if request
            .out_bill_no
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
            && request
                .transfer_bill_no
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            return Err(impl_utils::runtime(
                "商户单号(out_bill_no)和微信转账单号(transfer_bill_no)必须提供其中一个",
            ));
        }
        if let Some(out_bill_no) = request.out_bill_no.as_deref() {
            if !out_bill_no.trim().is_empty() {
                return self
                    .query_operation_transfer_by_out_bill_no(out_bill_no)
                    .await;
            }
        }
        let transfer_bill_no = request.transfer_bill_no.as_deref().unwrap_or_default();
        self.query_operation_transfer_by_transfer_bill_no(transfer_bill_no)
            .await
    }

    async fn query_operation_transfer_by_out_bill_no(
        &self,
        out_bill_no: &str,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills/out-bill-no/{out_bill_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_operation_transfer_by_transfer_bill_no(
        &self,
        transfer_bill_no: &str,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/fund-app/mch-transfer/transfer-bills/transfer-bill-no/{transfer_bill_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
