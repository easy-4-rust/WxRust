//! WxChannelKfService（对应 Java `me.chanjar.weixin.channel.api.WxChannelKfService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::kf::{WxChannelKfSendMsgParam, WxChannelKfSendMsgResponse};

/// 商家客服服务（对应 Java `WxChannelKfService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_kf_service_impl` 的
/// `WxChannelKfServiceImpl`（Java `WxChannelKfServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelKfService: Send + Sync {
    /// 上传多媒体资源（对应 Java `WxChannelKfService#uploadMedia(String, String, String, byte[])`）。
    async fn upload_media(
        &self,
        open_id: String,
        msg_type: String,
        file_name: String,
        file: Vec<u8>,
    ) -> Result<String, WxErrorException>;

    /// 发送客服消息（对应 Java `WxChannelKfService#sendMessage(WxChannelKfSendMsgParam)`）。
    async fn send_message(
        &self,
        param: WxChannelKfSendMsgParam,
    ) -> Result<WxChannelKfSendMsgResponse, WxErrorException>;
}
