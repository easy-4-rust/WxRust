//! 对应 Java `service.impl.SubscriptionBillingServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{SubscriptionBillingService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// SubscriptionBillingService 实现（对应 Java `SubscriptionBillingServiceImpl`）。
pub struct SubscriptionBillingServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl SubscriptionBillingServiceImpl {
    /// 构建实现（对应 Java 构造器 `SubscriptionBillingServiceImpl(WxPayService)`）。
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
impl SubscriptionBillingService for SubscriptionBillingServiceImpl {
    async fn schedule_subscription(
        &self,
        request: &SubscriptionScheduleRequest,
    ) -> Result<SubscriptionScheduleResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/subscription-billing/schedule",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/subscription-billing/schedule/{subscription_id}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn cancel_subscription(
        &self,
        request: &SubscriptionCancelRequest,
    ) -> Result<SubscriptionCancelResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/subscription-billing/schedule/{}/cancel",
            svc.get_pay_base_url(),
            request.subscription_id.as_deref().unwrap_or_default()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn instant_billing(
        &self,
        request: &SubscriptionInstantBillingRequest,
    ) -> Result<SubscriptionInstantBillingResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/subscription-billing/instant-billing",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_transactions(
        &self,
        request: &SubscriptionTransactionQueryRequest,
    ) -> Result<SubscriptionTransactionQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/subscription-billing/transactions",
            svc.get_pay_base_url()
        );
        // 对应 Java：openid/begin_time/end_time/limit/offset 逐个拼接
        let mut parts: Vec<String> = Vec::new();
        if let Some(v) = request.openid.as_deref() {
            parts.push(format!("openid={v}"));
        }
        if let Some(v) = request.begin_time.as_deref() {
            parts.push(format!("begin_time={v}"));
        }
        if let Some(v) = request.end_time.as_deref() {
            parts.push(format!("end_time={v}"));
        }
        if let Some(v) = request.limit {
            parts.push(format!("limit={v}"));
        }
        if let Some(v) = request.offset {
            parts.push(format!("offset={v}"));
        }
        if !parts.is_empty() {
            url.push('?');
            url.push_str(&parts.join("&"));
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
