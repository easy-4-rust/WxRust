//! WxMpAiOpenService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpAiOpenServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpAiOpenService, WxMpService};
use crate::enums::wx_mp_api_url::ai_open;

/// 公众号AiOpenService实现。
pub struct WxMpAiOpenServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpAiOpenServiceImpl {
    /// 构建 公众号AiOpenService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpAiOpenService for WxMpAiOpenServiceImpl {
    async fn upload_voice(
        &self,
        voice_id: &str,
        lang: &str,
        file_path: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let token = svc.get_access_token().await?;
        // Java：POST voice 文件（multipart），URL 带 format/voice_id/lang
        let url = ai_open::voice_upload(config.as_ref(), "amr", voice_id, lang);
        let url = format!("{url}?access_token={token}");
        let part = reqwest::multipart::Part::bytes(
            std::fs::read(file_path)
                .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?,
        )
        .file_name("voice");
        let form = reqwest::multipart::Form::new().part("media", part);
        let text = svc
            .http_client()
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &text,
            Some(wx_rust_common::enums::WxType::Mp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    async fn query_recognition_result(
        &self,
        voice_id: &str,
        lang: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"voice_id": voice_id, "lang": lang});
        let response = svc
            .post(
                &ai_open::voice_query_result(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("result")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "result 缺失"))
    }

    async fn translate(
        &self,
        lang_from: &str,
        lang_to: &str,
        content: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"content": content});
        let url = ai_open::translate(config.as_ref(), lang_from, lang_to);
        let response = svc.post(&url, &body.to_string()).await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("to_content")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "to_content 缺失"))
    }
}
