//! 对应 Java `com.github.binarywang.wxpay.service.PartnerPayScoreSignPlanService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// PartnerPayScoreSignPlanService（对应 Java `PartnerPayScoreSignPlanService`）。
#[async_trait]
pub trait PartnerPayScoreSignPlanService: Send + Sync {
    /// 文档更新时间：2023.10.13 微信支付分签约计划是不同模式的支付分接口（随着国家大力推广教培行业先享后付政策,微信支付也紧跟政策于2023.07.25上线第一版签约计划接口以适用教培行业先享后付
    async fn create_plans(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException>;

    /// description： 查询支付分计划 author：UltramanNoa create Time： 2023/11/3 14:03 version： v.1.0
    async fn query_plans(
        &self,
        merchant_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException>;

    /// description： 停止支付分计划 author：UltramanNoa create Time： 2023/11/3 14:24 version： v.1.0
    async fn stop_plans(
        &self,
        merchant_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException>;

    /// description： 创建用户的签约计划详情对应的服务订单 author：UltramanNoa create Time： 2023/11/3 14:53 version： v.1.0
    async fn sign_plan_service_order(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreSignPlanResult, WxErrorException>;

    /// description： 创建用户的签约计划 author：UltramanNoa create Time： 2023/11/3 17:48 version： v.1.0
    async fn create_user_sign_plans(
        &self,
        request: &WxPartnerPayScoreSignPlanRequest,
    ) -> Result<WxPartnerPayScoreUserSignPlanResult, WxErrorException>;

    /// description： 查询用户的签约计划 author：UltramanNoa create Time： 2023/11/3 18:01 version： v.1.0
    async fn query_user_sign_plans(
        &self,
        merchant_sign_plan_no: &str,
        sub_mchid: &str,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException>;

    /// description： 取消用户的签约计划 author：UltramanNoa create Time： 2023/11/3 18:01 version： v.1.0
    async fn stop_user_sign_plans(
        &self,
        merchant_sign_plan_no: &str,
        sub_mchid: &str,
        stop_reason: &str,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException>;

    /// description： 回调通知校验解密 author：UltramanNoa create Time： 2023/11/6 10:27 version： v.1.0
    async fn parse_sign_plan_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<PartnerUserSignPlanEntity, WxErrorException>;
}
