//! 小程序交易投诉服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaComplaintService`
//! （`impl.WxMaComplaintServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::complaint::{
    WxMaComplaintDetailRequest, WxMaComplaintDetailResult, WxMaComplaintNotifyUrlRequest,
    WxMaComplaintNotifyUrlResult, WxMaComplaintRequest, WxMaComplaintResult, WxMaCompleteRequest,
    WxMaNegotiationHistoryRequest, WxMaNegotiationHistoryResult, WxMaResponseRequest,
};

/// 小程序交易投诉服务。
///
/// 对应 Java `WxMaComplaintService`：投诉单列表/详情/协商历史、通知回调地址
/// 管理、提交回复、反馈处理完成、上传反馈图片。
#[async_trait]
pub trait WxMaComplaintService: Send + Sync {
    /// 查询投诉单列表（对应 Java `queryComplaints`）。
    async fn query_complaints(
        &self,
        request: &WxMaComplaintRequest,
    ) -> Result<WxMaComplaintResult, WxErrorException>;

    /// 查询投诉单详情（对应 Java `getComplaint`）。
    async fn get_complaint(
        &self,
        request: &WxMaComplaintDetailRequest,
    ) -> Result<WxMaComplaintDetailResult, WxErrorException>;

    /// 查询投诉协商历史（对应 Java `queryNegotiationHistorys`）。
    async fn query_negotiation_historys(
        &self,
        request: &WxMaNegotiationHistoryRequest,
    ) -> Result<WxMaNegotiationHistoryResult, WxErrorException>;

    /// 创建投诉通知回调地址（对应 Java `addComplaintNotifyUrl`）。
    async fn add_complaint_notify_url(
        &self,
        request: &WxMaComplaintNotifyUrlRequest,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException>;

    /// 查询投诉通知回调地址（对应 Java `getComplaintNotifyUrl`）。
    async fn get_complaint_notify_url(
        &self,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException>;

    /// 更新投诉通知回调地址（对应 Java `updateComplaintNotifyUrl`）。
    async fn update_complaint_notify_url(
        &self,
        request: &WxMaComplaintNotifyUrlRequest,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException>;

    /// 删除投诉通知回调地址（对应 Java `deleteComplaintNotifyUrl`）。
    async fn delete_complaint_notify_url(&self) -> Result<(), WxErrorException>;

    /// 提交回复（对应 Java `submitResponse`）。
    async fn submit_response(&self, request: &WxMaResponseRequest) -> Result<(), WxErrorException>;

    /// 反馈处理完成（对应 Java `complete`）。
    async fn complete(&self, request: &WxMaCompleteRequest) -> Result<(), WxErrorException>;

    /// 商户上传反馈图片（文件路径版，对应 Java `uploadResponseImage(File)`，
    /// 返回媒体文件标识 ID）。
    async fn upload_response_image(&self, image_path: &str) -> Result<String, WxErrorException>;

    /// 商户上传反馈图片（字节版，对应 Java
    /// `uploadResponseImage(InputStream, String)`）。
    async fn upload_response_image_bytes(
        &self,
        content: Vec<u8>,
        file_name: Option<&str>,
    ) -> Result<String, WxErrorException>;
}
