//! 微信小程序人脸核身服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaFaceService`
//! （`impl.WxMaFaceServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::face::{
    WxMaFaceGetVerifyIdRequest, WxMaFaceGetVerifyIdResponse, WxMaFaceQueryVerifyInfoRequest,
    WxMaFaceQueryVerifyInfoResponse,
};

/// 微信小程序人脸核身服务。
///
/// 对应 Java `WxMaFaceService`：获取用户人脸核身会话唯一标识与查询真实验证结果。
#[async_trait]
pub trait WxMaFaceService: Send + Sync {
    /// 获取用户人脸核身会话唯一标识（对应 Java `getVerifyId`）。
    async fn get_verify_id(
        &self,
        request: &WxMaFaceGetVerifyIdRequest,
    ) -> Result<WxMaFaceGetVerifyIdResponse, WxErrorException>;

    /// 查询用户人脸核身真实验证结果（对应 Java `queryVerifyInfo`）。
    async fn query_verify_info(
        &self,
        request: &WxMaFaceQueryVerifyInfoRequest,
    ) -> Result<WxMaFaceQueryVerifyInfoResponse, WxErrorException>;
}
