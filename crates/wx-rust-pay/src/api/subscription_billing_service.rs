//! 对应 Java `com.github.binarywang.wxpay.service.SubscriptionBillingService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// SubscriptionBillingService（对应 Java `SubscriptionBillingService`）。
#[async_trait]
pub trait SubscriptionBillingService: Send + Sync {
    /// 微信支付-预约扣费服务 (连续包月功能) 微信支付预约扣费功能，支持商户在用户授权的情况下， 按照约定的时间和金额，自动从用户的支付账户中扣取费用。 主要用于连续包月、订阅服务等场景。 文档详见: h
    async fn schedule_subscription(
        &self,
        request: &SubscriptionScheduleRequest,
    ) -> Result<SubscriptionScheduleResult, WxErrorException>;

    /// 查询预约扣费 商户可以通过该接口查询已预约的扣费信息。 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012161105 请求URL: https:
    async fn query_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<SubscriptionQueryResult, WxErrorException>;

    /// 取消预约扣费 商户可以通过该接口取消已预约的扣费。 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012161105 请求URL: https://
    async fn cancel_subscription(
        &self,
        request: &SubscriptionCancelRequest,
    ) -> Result<SubscriptionCancelResult, WxErrorException>;

    /// 立即扣费 商户可以通过该接口立即执行扣费操作。 通常用于补扣失败的费用或者特殊情况下的即时扣费。 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/401
    async fn instant_billing(
        &self,
        request: &SubscriptionInstantBillingRequest,
    ) -> Result<SubscriptionInstantBillingResult, WxErrorException>;

    /// 查询扣费记录 商户可以通过该接口查询扣费记录。 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012161105 请求URL: https://ap
    async fn query_transactions(
        &self,
        request: &SubscriptionTransactionQueryRequest,
    ) -> Result<SubscriptionTransactionQueryResult, WxErrorException>;
}
