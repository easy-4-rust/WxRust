//! WxMpAiOpenService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpAiOpenService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::enums::wx_mp_api_url::ai_open;

/// 公众号AiOpenService。
#[async_trait]
pub trait WxMpAiOpenService: Send + Sync {
    async fn upload_voice(
        &self,
        voice_id: &str,
        lang: &str,
        file_path: &str,
    ) -> Result<(), WxErrorException>;

    async fn query_recognition_result(
        &self,
        voice_id: &str,
        lang: &str,
    ) -> Result<String, WxErrorException>;

    async fn translate(
        &self,
        lang_from: &str,
        lang_to: &str,
        content: &str,
    ) -> Result<String, WxErrorException>;
}
