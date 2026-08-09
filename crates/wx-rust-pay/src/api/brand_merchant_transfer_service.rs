//! 对应 Java `com.github.binarywang.wxpay.service.BrandMerchantTransferService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// BrandMerchantTransferService（对应 Java `BrandMerchantTransferService`）。
#[async_trait]
pub trait BrandMerchantTransferService: Send + Sync {
    /// 品牌商户发放红包商家转账到零钱（直联商户）
    async fn create_brand_transfer(
        &self,
        request: &BrandTransferBatchesRequest,
    ) -> Result<BrandTransferBatchesResult, WxErrorException>;

    /// 品牌红包微信批次单号查询批次单API 适用对象：直连商户 文档详见: 请求URL：https://api.mch.weixin.qq.com/v3/fund-app/brand-redpacket/b
    async fn query_brand_wx_batches(
        &self,
        request: &BrandWxBatchesQueryRequest,
    ) -> Result<BrandBatchesQueryResult, WxErrorException>;

    /// 品牌红包微信支付明细单号查询明细单API 适用对象：直连商户 文档详见: 请求URL：https://api.mch.weixin.qq.com/v3/fund-app/brand-redpacket
    async fn query_brand_wx_details(
        &self,
        request: &BrandWxDetailsQueryRequest,
    ) -> Result<BrandDetailsQueryResult, WxErrorException>;

    /// 品牌红包商家批次单号查询批次单API 适用对象：直连商户 文档详见: 请求URL：https://api.mch.weixin.qq.com/v3/fund-app/brand-redpacket/b
    async fn query_brand_merchant_batches(
        &self,
        request: &BrandMerchantBatchesQueryRequest,
    ) -> Result<BrandBatchesQueryResult, WxErrorException>;

    /// 品牌红包商家明细单号查询明细单API 适用对象：直连商户 文档详见: 请求URL：https://api.mch.weixin.qq.com/v3/fund-app/brand-redpacket/b
    async fn query_brand_merchant_details(
        &self,
        request: &BrandMerchantDetailsQueryRequest,
    ) -> Result<BrandDetailsQueryResult, WxErrorException>;
}
