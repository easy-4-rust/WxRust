//! 对应 Java `com.github.binarywang.wxpay.service.ComplaintService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// ComplaintService（对应 Java `ComplaintService`）。
#[async_trait]
pub trait ComplaintService: Send + Sync {
    /// 微信支付 消费者投诉2.0 API. Created by jmdhappy on 2022/3/19.
    async fn query_complaints(
        &self,
        request: &ComplaintRequest,
    ) -> Result<ComplaintResult, WxErrorException>;

    /// 查询投诉单详情API 商户可通过调用此接口，查询指定投诉单的用户投诉详情，包含投诉内容、投诉关联订单、投诉人联系方式等信息，方便商户处理投诉。 文档详见: ...
    async fn get_complaint(
        &self,
        request: &ComplaintDetailRequest,
    ) -> Result<ComplaintDetailResult, WxErrorException>;

    /// 查询投诉协商历史API 商户可通过调用此接口，查询指定投诉的用户商户协商历史，以分页输出查询结果，方便商户根据处理历史来制定后续处理方案。 文档详见: ...
    async fn query_negotiation_historys(
        &self,
        request: &NegotiationHistoryRequest,
    ) -> Result<NegotiationHistoryResult, WxErrorException>;

    /// 创建投诉通知回调地址API 商户通过调用此接口创建投诉通知回调URL，当用户产生新投诉且投诉状态已变更时，微信支付会通过回 调URL通知商户。对于服务商、渠道商，会收到所有子商户的投诉信息推送。 文档
    async fn add_complaint_notify_url(
        &self,
        request: &ComplaintNotifyUrlRequest,
    ) -> Result<ComplaintNotifyUrlResult, WxErrorException>;

    /// 查询投诉通知回调地址API 商户通过调用此接口查询投诉通知的回调URL。 文档详见: ...
    async fn get_complaint_notify_url(&self) -> Result<ComplaintNotifyUrlResult, WxErrorException>;

    /// 更新投诉通知回调地址API 商户通过调用此接口更新投诉通知的回调URL。 文档详见: ...
    async fn update_complaint_notify_url(
        &self,
        request: &ComplaintNotifyUrlRequest,
    ) -> Result<ComplaintNotifyUrlResult, WxErrorException>;

    /// 删除投诉通知回调地址API 当商户不再需要推送通知时，可通过调用此接口删除投诉通知的回调URL，取消通知回调。 文档详见: ...
    async fn delete_complaint_notify_url(&self) -> Result<(), WxErrorException>;

    /// 提交回复API 商户可通过调用此接口，提交回复内容。其中上传图片凭证需首先调用商户上传反馈图片接口，得到图片id，再将id填入请求。 回复可配置文字链，传入跳转链接文案和跳转链接字段，用户点击即可跳转
    async fn submit_response(&self, request: &ResponseRequest) -> Result<(), WxErrorException>;

    /// 反馈处理完成API 商户可通过调用此接口，反馈投诉单已处理完成。 文档详见: ...
    async fn complete(&self, request: &CompleteRequest) -> Result<(), WxErrorException>;

    /// 更新退款审批结果API 针对“申请退款单”，需要商户明确返回是否可退款的审批结果。 若根据用户描述，核实可以退款，审批动作传入“APPROVE”，同意退款，并给出一个预计退款时间。传入“同意退款”后，
    async fn update_refund_progress(
        &self,
        request: &UpdateRefundProgressRequest,
    ) -> Result<(), WxErrorException>;

    /// 商户上传反馈图片API（对应 Java `uploadResponseImage(File)` /
    /// `uploadResponseImage(InputStream, String)`，接口地址
    /// `/v3/merchant-service/images/upload`，multipart）。
    ///
    /// `ADAPTED`：Java `File`/`InputStream` 重载合并为 `(文件名, 文件字节)`。
    async fn upload_response_image(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<ImageUploadResult, WxErrorException>;
}
