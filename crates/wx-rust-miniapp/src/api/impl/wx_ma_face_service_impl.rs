//! 微信小程序人脸核身服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaFaceServiceImpl`：
//! 请求经 `request.toJson()` 序列化，响应经 bean 的 `fromJson` 解析
//! （errcode!=0 由执行引擎抛错，Java 依赖微信返回报文语义，语义一致）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaFaceService;
use crate::bean::face::{
    WxMaFaceGetVerifyIdRequest, WxMaFaceGetVerifyIdResponse, WxMaFaceQueryVerifyInfoRequest,
    WxMaFaceQueryVerifyInfoResponse,
};
use crate::enums::g4_urls::url_g4_ability::face as face_url;

/// 微信小程序人脸核身服务实现。
pub struct WxMaFaceServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaFaceServiceImpl {
    /// 构建人脸核身服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaFaceService for WxMaFaceServiceImpl {
    /// 获取用户人脸核身会话唯一标识（对应 Java
    /// `WxMaFaceServiceImpl.getVerifyId`）。
    async fn get_verify_id(
        &self,
        request: &WxMaFaceGetVerifyIdRequest,
    ) -> Result<WxMaFaceGetVerifyIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &face_url::get_verify_id_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        WxMaFaceGetVerifyIdResponse::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    /// 查询用户人脸核身真实验证结果（对应 Java
    /// `WxMaFaceServiceImpl.queryVerifyInfo`）。
    async fn query_verify_info(
        &self,
        request: &WxMaFaceQueryVerifyInfoRequest,
    ) -> Result<WxMaFaceQueryVerifyInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &face_url::query_verify_info_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        WxMaFaceQueryVerifyInfoResponse::from_json(&response_content)
            .map_err(WxErrorException::Serde)
    }
}
