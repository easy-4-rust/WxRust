//! 对应 Java `com.github.binarywang.wxpay.service.MerchantTransferService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// MerchantTransferService（对应 Java `MerchantTransferService`）。
#[async_trait]
pub trait MerchantTransferService: Send + Sync {
    /// 商家转账到零钱（直联商户） created on 2022-6-11
    async fn create_transfer(
        &self,
        request: &TransferCreateRequest,
    ) -> Result<TransferCreateResult, WxErrorException>;

    /// 微信批次单号查询批次单API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_2.shtml 请求UR
    async fn query_wx_batches(
        &self,
        request: &WxBatchesQueryRequest,
    ) -> Result<BatchesQueryResult, WxErrorException>;

    /// 微信明细单号查询明细单API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_3.shtml 请求UR
    async fn query_wx_details(
        &self,
        request: &WxDetailsQueryRequest,
    ) -> Result<DetailsQueryResult, WxErrorException>;

    /// 商家批次单号查询批次单API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_5.shtml 请求UR
    async fn query_merchant_batches(
        &self,
        request: &MerchantBatchesQueryRequest,
    ) -> Result<BatchesQueryResult, WxErrorException>;

    /// 商家明细单号查询明细单API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_6.shtml 请求UR
    async fn query_merchant_details(
        &self,
        request: &MerchantDetailsQueryRequest,
    ) -> Result<DetailsQueryResult, WxErrorException>;

    /// 转账电子回单申请受理API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012716452 请求URL：https://api
    async fn apply_electronic_bill(
        &self,
        request: &ElectronicBillApplyRequest,
    ) -> Result<ElectronicBillResult, WxErrorException>;

    /// 查询转账电子回单API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4012716436 请求URL：https://api.m
    async fn query_electronic_bill(
        &self,
        out_bill_no: &str,
    ) -> Result<ElectronicBillResult, WxErrorException>;

    /// 转账明细电子回单受理API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_9.shtml 请求URL
    async fn apply_detail_electronic_bill(
        &self,
        request: &DetailElectronicBillRequest,
    ) -> Result<DetailElectronicBillResult, WxErrorException>;

    /// 查询转账明细电子回单受理结果API 适用对象：直连商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter4_3_10.shtml
    async fn query_detail_electronic_bill(
        &self,
        request: &DetailElectronicBillRequest,
    ) -> Result<DetailElectronicBillResult, WxErrorException>;

    /// 商户查询用户授权信息接口. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293 请求URL：https://api.mch.weixi
    async fn get_user_authorization_status(
        &self,
        openid: &str,
        transfer_scene_id: &str,
    ) -> Result<UserAuthorizationStatusResult, WxErrorException>;

    /// 批量预约商家转账接口. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293 请求URL：https://api.mch.weixin.
    async fn reservation_transfer_batch(
        &self,
        request: &ReservationTransferBatchRequest,
    ) -> Result<ReservationTransferBatchResult, WxErrorException>;

    /// 商户预约批次单号查询批次单接口. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293 请求URL：https://api.mch.we
    async fn get_reservation_transfer_batch_by_out_batch_no(
        &self,
        out_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException>;

    /// 微信预约批次单号查询批次单接口. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293 请求URL：https://api.mch.we
    async fn get_reservation_transfer_batch_by_reservation_batch_no(
        &self,
        reservation_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException>;

    /// 解析预约商家转账通知回调结果. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293
    async fn parse_reservation_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ReservationTransferNotifyResult, WxErrorException>;

    /// 关闭预约商家转账批次接口. 文档详见: https://pay.weixin.qq.com/doc/v3/merchant/4014399293 请求URL：https://api.mch.weixi
    async fn close_reservation_transfer_batch(
        &self,
        out_batch_no: &str,
    ) -> Result<(), WxErrorException>;
}
