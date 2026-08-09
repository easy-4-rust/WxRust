//! 对应 Java `com.github.binarywang.wxpay.service.RedpackService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// RedpackService（对应 Java `RedpackService`）。
#[async_trait]
pub trait RedpackService: Send + Sync {
    /// 红包相关接口. created on 2019-12-26
    async fn send_mini_program_redpack(
        &self,
        request: &WxPaySendMiniProgramRedpackRequest,
    ) -> Result<WxPaySendMiniProgramRedpackResult, WxErrorException>;

    /// 发送微信红包给个人用户. 文档详见: 发送普通红包 https://pay.weixin.qq.com/wiki/doc/api/tools/cash_coupon.php?chapter=13_4&
    async fn send_redpack(
        &self,
        request: &WxPaySendRedpackRequest,
    ) -> Result<WxPaySendRedpackResult, WxErrorException>;

    /// 查询红包记录. 用于商户对已发放的红包进行查询红包的具体信息，可支持普通红包和裂变包。 请求Url：https://api.mch.weixin.qq.com/mmpaymkttransfers/ge
    async fn query_redpack(
        &self,
        mch_bill_no: &str,
    ) -> Result<WxPayRedpackQueryResult, WxErrorException>;

    /// 查询红包记录. 用于商户对已发放的红包进行查询红包的具体信息，可支持普通红包和裂变包。 请求Url：https://api.mch.weixin.qq.com/mmpaymkttransfers/ge
    async fn query_redpack_with_request(
        &self,
        request: &WxPayRedpackQueryRequest,
    ) -> Result<WxPayRedpackQueryResult, WxErrorException>;
}
