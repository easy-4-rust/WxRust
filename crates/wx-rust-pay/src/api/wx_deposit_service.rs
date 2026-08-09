//! 对应 Java `com.github.binarywang.wxpay.service.WxDepositService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// WxDepositService（对应 Java `WxDepositService`）。
#[async_trait]
pub trait WxDepositService: Send + Sync {
    /// 微信押金支付相关接口. https://pay.weixin.qq.com/wiki/doc/api/deposit_sl.php?chapter=27_7&index=1 created on 20
    async fn unified_order(
        &self,
        request: &WxDepositUnifiedOrderRequest,
    ) -> Result<WxDepositUnifiedOrderResult, WxErrorException>;

    /// 查询押金订单 详见：https://pay.weixin.qq.com/wiki/doc/api/deposit_sl.php?chapter=27_7&index=3 通过商户订单号或微信订单号查询
    async fn query_order(
        &self,
        request: &WxDepositOrderQueryRequest,
    ) -> Result<WxDepositOrderQueryResult, WxErrorException>;

    /// 押金消费 详见：https://pay.weixin.qq.com/wiki/doc/api/deposit_sl.php?chapter=27_7&index=4 用于对已支付的押金进行消费扣减
    async fn consume(
        &self,
        request: &WxDepositConsumeRequest,
    ) -> Result<WxDepositConsumeResult, WxErrorException>;

    /// 押金撤销 详见：https://pay.weixin.qq.com/wiki/doc/api/deposit_sl.php?chapter=27_7&index=5 用于对已支付的押金进行撤销退还
    async fn unfreeze(
        &self,
        request: &WxDepositUnfreezeRequest,
    ) -> Result<WxDepositUnfreezeResult, WxErrorException>;

    /// 押金退款 详见：https://pay.weixin.qq.com/wiki/doc/api/deposit_sl.php?chapter=27_7&index=6 用于对已消费的押金进行退款
    async fn refund(
        &self,
        request: &WxDepositRefundRequest,
    ) -> Result<WxDepositRefundResult, WxErrorException>;
}
