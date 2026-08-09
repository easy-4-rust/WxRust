//! 对应 Java `com.github.binarywang.wxpay.service.PayrollService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// PayrollService（对应 Java `PayrollService`）。
#[async_trait]
pub trait PayrollService: Send + Sync {
    /// 微工卡-对接微信api created on 2021/12/7 14:26
    async fn payroll_card_tokens(
        &self,
        request: &TokensRequest,
    ) -> Result<TokensResult, WxErrorException>;

    /// 查询微工卡授权关系API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/relations/{openid} 请求方式：GE
    async fn payroll_card_relations(
        &self,
        request: &RelationsRequest,
    ) -> Result<RelationsResult, WxErrorException>;

    /// 微工卡核身预下单API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/authentications/pre-order 请
    async fn payroll_card_pre_order(
        &self,
        request: &PreOrderRequest,
    ) -> Result<PreOrderResult, WxErrorException>;

    /// 获取核身结果API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/authentications/{authenticate
    async fn payroll_card_authentications_number(
        &self,
        sub_mchid: &str,
        authenticate_number: &str,
    ) -> Result<AuthenticationsResult, WxErrorException>;

    /// 查询核身记录API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/authentications 请求方式：GET
    async fn payroll_card_authentications(
        &self,
        request: &AuthRecordRequest,
    ) -> Result<AuthRecordResult, WxErrorException>;

    /// 微工卡核身预下单（流程中完成授权） 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/authentications/pre-o
    async fn payroll_card_pre_order_with_auth(
        &self,
        request: &PreOrderWithAuthRequest,
    ) -> Result<PreOrderWithAuthResult, WxErrorException>;

    /// 按日下载提现异常文件API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/merchant/fund/withdraw/bill-type/{bill
    async fn merchant_fund_withdraw_bill_type(
        &self,
        bill_type: &str,
        bill_date: &str,
        tar_type: &str,
    ) -> Result<WxPayApplyBillV3Result, WxErrorException>;

    /// 微工卡批量转账API 适用对象：服务商 请求URL：https://api.mch.weixin.qq.com/v3/payroll-card/transfer-batches 请求方式：POST
    async fn payroll_card_transfer_batches(
        &self,
        request: &PayrollTransferBatchesRequest,
    ) -> Result<PayrollTransferBatchesResult, WxErrorException>;
}
