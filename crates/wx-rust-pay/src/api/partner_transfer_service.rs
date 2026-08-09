//! 对应 Java `com.github.binarywang.wxpay.service.PartnerTransferService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// PartnerTransferService（对应 Java `PartnerTransferService`）。
#[async_trait]
pub trait PartnerTransferService: Send + Sync {
    /// 微信批量转账到零钱【V3接口】服务商API created on 2021-12-06
    async fn batch_transfer(
        &self,
        request: &PartnerTransferRequest,
    ) -> Result<PartnerTransferResult, WxErrorException>;

    /// 微信支付批次单号查询批次单API 接口说明 适用对象：服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/cha
    async fn query_batch_by_batch_id(
        &self,
        request: &BatchNumberRequest,
    ) -> Result<BatchNumberResult, WxErrorException>;

    /// 微信支付明细单号查询明细单API 接口说明 适用对象：服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/cha
    async fn query_batch_detail_by_we_chat(
        &self,
        batch_id: &str,
        detail_id: &str,
    ) -> Result<BatchDetailsResult, WxErrorException>;

    /// 商家批次单号查询批次单API 接口说明 适用对象：服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/chapt
    async fn query_batch_by_out_batch_no(
        &self,
        request: &MerchantBatchRequest,
    ) -> Result<BatchNumberResult, WxErrorException>;

    /// 商家明细单号查询明细单API 接口说明 适用对象：服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/chapt
    async fn query_batch_detail_by_mch(
        &self,
        out_batch_no: &str,
        out_detail_no: &str,
    ) -> Result<BatchDetailsResult, WxErrorException>;

    /// 转账电子回单申请受理API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012716452 请求URL：ht
    async fn receipt_bill(
        &self,
        request: &ReceiptBillRequest,
    ) -> Result<BillReceiptResult, WxErrorException>;

    /// 查询转账电子回单API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012716436 请求URL：http
    async fn query_bill_receipt(
        &self,
        out_bill_no: &str,
    ) -> Result<BillReceiptResult, WxErrorException>;

    /// 转账明细电子回单受理API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/c
    async fn transfer_electronic(
        &self,
        request: &ElectronicReceiptsRequest,
    ) -> Result<ElectronicReceiptsResult, WxErrorException>;

    /// 查询转账明细电子回单受理结果API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transf
    async fn query_transfer_electronic_result(
        &self,
        request: &ElectronicReceiptsRequest,
    ) -> Result<ElectronicReceiptsResult, WxErrorException>;

    /// 下载电子回单API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/chapt
    async fn transfer_download(&self, url: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 查询账户实时余额API 接口说明 适用对象：直连商户 服务商 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/cha
    async fn fund_balance(
        &self,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException>;

    /// 服务商账户日终余额 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/pay/transfer/chapter5_2.shtml 文档地址: h
    async fn sp_day_end_balance(
        &self,
        account_type: SpAccountTypeEnum,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException>;
}
