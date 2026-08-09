//! 对应 Java `com.github.binarywang.wxpay.service.RealNameService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// RealNameService（对应 Java `RealNameService`）。
#[async_trait]
pub trait RealNameService: Send + Sync {
    /// 微信支付实名验证相关服务类. 详见文档：https://pay.wechatpay.cn/doc/v2/merchant/4011987607
    async fn query_real_name(
        &self,
        request: &RealNameRequest,
    ) -> Result<RealNameResult, WxErrorException>;

    /// 微信支付实名验证相关服务类. 详见文档：https://pay.wechatpay.cn/doc/v2/merchant/4011987607
    async fn query_real_name_with_openid(
        &self,
        openid: &str,
    ) -> Result<RealNameResult, WxErrorException>;
}
