//! 对应 Java `com.github.binarywang.wxpay.service.BusinessCircleService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// BusinessCircleService（对应 Java `BusinessCircleService`）。
#[async_trait]
pub trait BusinessCircleService: Send + Sync {
    /// 微信支付智慧商圈API
    async fn notify_points(&self, request: &PointsNotifyRequest) -> Result<(), WxErrorException>;

    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<BusinessCircleNotifyData, WxErrorException>;

    async fn decrypt_paid_notify_data_resource(
        &self,
        data: &BusinessCircleNotifyData,
    ) -> Result<PaidResult, WxErrorException>;

    async fn decrypt_refund_notify_data_resource(
        &self,
        data: &BusinessCircleNotifyData,
    ) -> Result<RefundResult, WxErrorException>;
}
