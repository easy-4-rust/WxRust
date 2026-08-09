//! 对应 Java `com.github.binarywang.wxpay.service.BusinessOperationTransferService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// BusinessOperationTransferService（对应 Java `BusinessOperationTransferService`）。
#[async_trait]
pub trait BusinessOperationTransferService: Send + Sync {
    /// 运营工具-商家转账API 微信支付为商户提供的运营工具转账能力，用于商户的日常运营活动中进行转账操作
    async fn create_operation_transfer(
        &self,
        request: &BusinessOperationTransferRequest,
    ) -> Result<BusinessOperationTransferResult, WxErrorException>;

    /// 查询运营工具转账结果 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com/v3/fund-app/mch-transfer/transfer-bill
    async fn query_operation_transfer(
        &self,
        request: &BusinessOperationTransferQueryRequest,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException>;

    /// 通过商户单号查询运营工具转账结果 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com/v3/fund-app/mch-transfer/transfe
    async fn query_operation_transfer_by_out_bill_no(
        &self,
        out_bill_no: &str,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException>;

    /// 通过微信转账单号查询运营工具转账结果 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com/v3/fund-app/mch-transfer/trans
    async fn query_operation_transfer_by_transfer_bill_no(
        &self,
        transfer_bill_no: &str,
    ) -> Result<BusinessOperationTransferQueryResult, WxErrorException>;
}
