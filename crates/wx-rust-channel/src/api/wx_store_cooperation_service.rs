//! WxStoreCooperationService（对应 Java `me.chanjar.weixin.channel.api.WxStoreCooperationService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::cooperation::{
    CooperationListResponse, CooperationQrCodeResponse, CooperationStatusResponse,
};

/// 微信小店 合作账号相关接口（对应 Java `WxStoreCooperationService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_store_cooperation_service_impl` 的
/// `WxStoreCooperationServiceImpl`（Java `WxStoreCooperationServiceImpl`）。
#[async_trait]
pub trait WxStoreCooperationService: Send + Sync {
    /// 获取合作账号列表（对应 Java `WxStoreCooperationService#listCooperation`；
    /// `sharer_type`：2 公众号，3 小程序）。
    async fn list_cooperation(
        &self,
        sharer_type: Option<i32>,
    ) -> Result<CooperationListResponse, WxErrorException>;

    /// 获取合作账号状态（对应 Java `WxStoreCooperationService#getCooperationStatus`；
    /// `sharer_id`：公众号 gh_ 开头 id / 小程序 appid）。
    async fn get_cooperation_status(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<CooperationStatusResponse, WxErrorException>;

    /// 生成合作账号邀请二维码（对应 Java `WxStoreCooperationService#generateQrCode`）。
    async fn generate_qr_code(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<CooperationQrCodeResponse, WxErrorException>;

    /// 取消合作账号邀请（对应 Java `WxStoreCooperationService#cancelInvitation`）。
    async fn cancel_invitation(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 解绑合作账号（对应 Java `WxStoreCooperationService#unbind`）。
    async fn unbind(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
