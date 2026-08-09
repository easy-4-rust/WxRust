//! 对应 Java `com.github.binarywang.wxpay.service.TransferService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// TransferService（对应 Java `TransferService`）。
#[async_trait]
pub trait TransferService: Send + Sync {
    /// 商家转账到零钱 created on 2022/6/17
    async fn transfer_batches(
        &self,
        request: &TransferBatchesRequest,
    ) -> Result<TransferBatchesResult, WxErrorException>;

    /// 解析商家转账结果 详见
    async fn parse_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<TransferNotifyResult, WxErrorException>;

    /// 微信批次单号查询批次单API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：微信批次单号查询批次单API
    async fn transfer_batches_batch_id(
        &self,
        request: &QueryTransferBatchesRequest,
    ) -> Result<QueryTransferBatchesResult, WxErrorException>;

    /// 微信明细单号查询明细单API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：微信明细单号查询明细单API
    async fn transfer_batches_batch_id_detail(
        &self,
        batch_id: &str,
        detail_id: &str,
    ) -> Result<TransferBatchDetailResult, WxErrorException>;

    /// 商家批次单号查询批次单API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商家批次单号查询批次单API
    async fn transfer_batches_out_batch_no(
        &self,
        request: &QueryTransferBatchesRequest,
    ) -> Result<QueryTransferBatchesResult, WxErrorException>;

    /// 商家明细单号查询明细单API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商家明细单号查询明细单API
    async fn transfer_batches_out_batch_no_detail(
        &self,
        out_batch_no: &str,
        out_detail_no: &str,
    ) -> Result<TransferBatchDetailResult, WxErrorException>;

    /// 2025.1.15 开始新接口 发起商家转账API 请求方式：POST（HTTPS） 请求地址：请求地址 文档地址：发起商家转账API
    async fn transfer_bills(
        &self,
        request: &TransferBillsRequest,
    ) -> Result<TransferBillsResult, WxErrorException>;

    /// 2025.1.15 开始新接口 撤销转账API 请求方式：POST（HTTPS） 请求地址：请求地址 文档地址：商户撤销转账API
    async fn transform_bills_cancel(
        &self,
        out_bill_no: &str,
    ) -> Result<TransferBillsCancelResult, WxErrorException>;

    /// 2025.1.15 开始新接口 发起商家转账API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商户单号查询转账单API
    async fn get_bills_by_out_bill_no(
        &self,
        out_bill_no: &str,
    ) -> Result<TransferBillsGetResult, WxErrorException>;

    /// 2025.1.15 开始新接口 微信单号查询转账单API 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商户单号查询转账单API
    async fn get_bills_by_transfer_bill_no(
        &self,
        transfer_bill_no: &str,
    ) -> Result<TransferBillsGetResult, WxErrorException>;

    /// 2025.1.15 开始新接口 解析商家转账结果 详见
    async fn parse_transfer_bills_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<TransferBillsNotifyResult, WxErrorException>;

    /// 商户查询用户授权信息接口 商户通过此接口可查询用户是否对商户的商家转账场景进行了授权。 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商户查询用户授权信息
    async fn get_user_authorization_status(
        &self,
        openid: &str,
        transfer_scene_id: &str,
    ) -> Result<UserAuthorizationStatusResult, WxErrorException>;

    /// 批量预约商家转账接口 商户可以通过批量预约接口一次发起批量转账请求，最多可以同时向50个用户发起转账。 批量预约接口适用于用户已授权免确认的场景，在转账时无需用户确认即可完成转账。 请求方式：POST
    async fn reservation_transfer_batch(
        &self,
        request: &ReservationTransferBatchRequest,
    ) -> Result<ReservationTransferBatchResult, WxErrorException>;

    /// 商户预约批次单号查询批次单接口 通过商户预约批次单号查询批量预约商家转账批次单基本信息。 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：商户预约批次单号查询批次单
    async fn get_reservation_transfer_batch_by_out_batch_no(
        &self,
        out_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException>;

    /// 微信预约批次单号查询批次单接口 通过微信预约批次单号查询批量预约商家转账批次单基本信息。 请求方式：GET（HTTPS） 请求地址：请求地址 文档地址：微信预约批次单号查询批次单
    async fn get_reservation_transfer_batch_by_reservation_batch_no(
        &self,
        reservation_batch_no: &str,
        need_query_detail: bool,
        offset: i32,
        limit: i32,
        detail_state: &str,
    ) -> Result<ReservationTransferBatchGetResult, WxErrorException>;

    /// 解析预约商家转账通知回调结果 预约批次单中的明细单在转账成功或转账失败时，微信会把相关结果信息发送给商户。 文档地址：预约商家转账通知
    async fn parse_reservation_transfer_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ReservationTransferNotifyResult, WxErrorException>;

    /// 关闭预约商家转账批次接口 商户可以通过此接口关闭预约商家转账批次单。关闭后，该批次内所有未成功的转账将被取消。 请求方式：POST（HTTPS） 请求地址：请求地址 文档地址：关闭预约商家转账批次
    async fn close_reservation_transfer_batch(
        &self,
        out_batch_no: &str,
    ) -> Result<(), WxErrorException>;
}
