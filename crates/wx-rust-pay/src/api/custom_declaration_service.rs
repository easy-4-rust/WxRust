//! 对应 Java `com.github.binarywang.wxpay.service.CustomDeclarationService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// CustomDeclarationService（对应 Java `CustomDeclarationService`）。
#[async_trait]
pub trait CustomDeclarationService: Send + Sync {
    /// 微信支付 支付报关 API. Created by xifengzhu on 2022/05/05.
    async fn declare(
        &self,
        request: &DeclarationRequest,
    ) -> Result<DeclarationResult, WxErrorException>;

    /// 报关查询API 文档地址: ...
    async fn query(
        &self,
        request: &DeclarationQueryRequest,
    ) -> Result<DeclarationQueryResult, WxErrorException>;

    /// 身份信息校验API 文档地址: ...
    async fn verify_certificate(
        &self,
        request: &VerifyCertificateRequest,
    ) -> Result<VerifyCertificateResult, WxErrorException>;

    /// 报关信息修改API 文档地址: ...
    async fn modify(
        &self,
        request: &DeclarationRequest,
    ) -> Result<DeclarationResult, WxErrorException>;

    /// 报关重推API 文档地址: ...
    async fn redeclare(
        &self,
        request: &RedeclareRequest,
    ) -> Result<RedeclareResult, WxErrorException>;
}
