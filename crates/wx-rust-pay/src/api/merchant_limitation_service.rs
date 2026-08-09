//! 对应 Java `com.github.binarywang.wxpay.service.MerchantLimitationService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// MerchantLimitationService（对应 Java `MerchantLimitationService`）。
#[async_trait]
pub trait MerchantLimitationService: Send + Sync {
    /// 商户被管控能力及原因查询 接口 产品介绍
    async fn fetch_limitations(
        &self,
        sub_mch_id: &str,
    ) -> Result<MerchantLimitationResult, WxErrorException>;
}
