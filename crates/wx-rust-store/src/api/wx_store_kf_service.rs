//! WxStoreKfService（对应 Java `com.binarywang.wxjava.store.api.WxStoreKfService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::kf::{WxStoreKfSendMsgParam, WxStoreKfSendMsgResponse};

/// 商家客服服务（对应 Java `WxStoreKfService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_kf_service_impl` 的
/// `WxStoreKfServiceImpl`（Java `WxStoreKfServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreKfService: Send + Sync {
    /// 上传多媒体资源（对应 Java `WxStoreKfService#uploadMedia(String, String, String, byte[])`）。
    async fn upload_media(
        &self,
        open_id: String,
        msg_type: String,
        file_name: String,
        file: Vec<u8>,
    ) -> Result<String, WxErrorException>;

    /// 发送客服消息（对应 Java `WxStoreKfService#sendMessage(WxStoreKfSendMsgParam)`）。
    async fn send_message(
        &self,
        param: WxStoreKfSendMsgParam,
    ) -> Result<WxStoreKfSendMsgResponse, WxErrorException>;
}
