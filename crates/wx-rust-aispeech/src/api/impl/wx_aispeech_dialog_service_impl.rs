//! 对话机器人服务实现。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.impl.WxAispeechDialogServiceImpl`：
//! 通过门面执行引擎（`executeDialogPost`）调用对话 API，并对
//! `AispeechApiResponse` 做 `ensureSuccess` 校验（code != 0 抛错）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::{WxAispeechDialogService, WxAispeechService};
use crate::bean::dialog::{
    AispeechApiResponse, AsyncTaskResult, BotIntent, DialogQueryRequest, DialogResult,
    PublishProgress,
};
use crate::util::WxAispeechSignUtil;

/// 对话机器人服务实现。
pub struct WxAispeechDialogServiceImpl {
    /// 门面服务弱引用（对应 Java `WxAispeechServiceImpl service` 字段）
    service: Weak<dyn WxAispeechService>,
}

impl WxAispeechDialogServiceImpl {
    /// 构建实现。
    ///
    /// # 参数
    /// - `service`：门面服务弱引用（打破循环引用）
    pub fn new(service: Weak<dyn WxAispeechService>) -> Self {
        Self { service }
    }

    /// 门面服务引用（子服务生命周期内必然存在，对应 Java 强引用字段）。
    fn service(&self) -> Result<std::sync::Arc<dyn WxAispeechService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已释放"))
    }

    /// 响应校验（对应 Java `ensureSuccess`）：`code != 0`（含 null）抛错。
    fn ensure_success<T>(response: &AispeechApiResponse<T>) -> Result<(), WxErrorException> {
        match response.code {
            Some(0) => Ok(()),
            code => Err(WxErrorException::Wx(
                wx_rust_common::error::WxErrorError::new(WxError::new(
                    code.unwrap_or(-1),
                    response.msg.clone().unwrap_or_default(),
                )),
            )),
        }
    }
}

#[async_trait]
impl WxAispeechDialogService for WxAispeechDialogServiceImpl {
    async fn get_access_token(
        &self,
        appid: Option<&str>,
        account: Option<&str>,
    ) -> Result<String, WxErrorException> {
        // 对应 Java：account 非空时才放入请求体
        let mut request = serde_json::Map::new();
        if let Some(account) = account.filter(|a| !a.is_empty()) {
            request.insert(
                "account".to_string(),
                serde_json::Value::String(account.to_string()),
            );
        }
        let body = serde_json::Value::Object(request).to_string();

        let service = self.service()?;
        let response = service
            .execute_dialog_post("/v2/token", Some(&body), false, appid)
            .await?;
        let result: AispeechApiResponse<serde_json::Value> =
            AispeechApiResponse::from_json(&response)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;
        let token = result
            .data
            .as_ref()
            .and_then(|d| d.get("access_token"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| WxErrorException::from_code(-99, "access_token 字段缺失"))?
            .to_string();
        // 对应 Java：写入配置存储供后续 X-OPENAI-TOKEN 使用
        service.config_storage().set_open_ai_token(&token);
        Ok(token)
    }

    async fn import_bot_json(
        &self,
        mode: i32,
        data: &[BotIntent],
    ) -> Result<String, WxErrorException> {
        // 对应 Java：body 为 {"mode": mode, "data": data}
        let body = serde_json::json!({ "mode": mode, "data": data }).to_string();
        let service = self.service()?;
        let response = service
            .execute_dialog_post("/v2/bot/import/json", Some(&body), true, None)
            .await?;
        let result: AispeechApiResponse<serde_json::Value> =
            AispeechApiResponse::from_json(&response)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;
        result
            .data
            .as_ref()
            .and_then(|d| d.get("task_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "task_id 字段缺失"))
    }

    async fn publish_bot(&self) -> Result<String, WxErrorException> {
        // 对应 Java：body 为字面量 "{}"
        let service = self.service()?;
        let response = service
            .execute_dialog_post("/v2/bot/publish", Some("{}"), true, None)
            .await?;
        let result: AispeechApiResponse<serde_json::Value> =
            AispeechApiResponse::from_json(&response)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;
        // 对应 Java：返回 request_id
        result
            .request_id
            .ok_or_else(|| WxErrorException::from_code(-99, "request_id 字段缺失"))
    }

    async fn get_publish_progress(&self, env: &str) -> Result<PublishProgress, WxErrorException> {
        let body = serde_json::json!({ "env": env }).to_string();
        let service = self.service()?;
        let response = service
            .execute_dialog_post("/v2/bot/effective_progress", Some(&body), true, None)
            .await?;
        let result: AispeechApiResponse<PublishProgress> =
            AispeechApiResponse::from_json(&response)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;
        result
            .data
            .ok_or_else(|| WxErrorException::from_code(-99, "data 字段缺失"))
    }

    async fn query_async_task(&self, task_id: &str) -> Result<AsyncTaskResult, WxErrorException> {
        let body = serde_json::json!({ "task_id": task_id }).to_string();
        let service = self.service()?;
        let response = service
            .execute_dialog_post("/v2/async/fetch", Some(&body), true, None)
            .await?;
        let result: AispeechApiResponse<AsyncTaskResult> =
            AispeechApiResponse::from_json(&response)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;
        result
            .data
            .ok_or_else(|| WxErrorException::from_code(-99, "data 字段缺失"))
    }

    async fn query(&self, request: &DialogQueryRequest) -> Result<DialogResult, WxErrorException> {
        let service = self.service()?;
        // 对应 Java：请求体先经 AES-CBC 加密（密钥为配置 aesKey）
        let json =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = service.config_storage();
        let aes_key = config
            .aes_key()
            .ok_or_else(|| WxErrorException::from_code(-99, "AES 密钥未配置"))?
            .to_string();
        let encrypted =
            WxAispeechSignUtil::encrypt_aes_cbc_to_base64(&json, &aes_key).map_err(|e| {
                WxErrorException::Runtime(wx_rust_common::error::WxRuntimeError::new(e))
            })?;

        let response = service
            .execute_dialog_post("/v2/bot/query", Some(&encrypted), true, None)
            .await?;

        // 对应 Java：响应非 JSON（明文密文）时解密后解析
        let response_json = if looks_like_json(&response) {
            response
        } else {
            WxAispeechSignUtil::decrypt_aes_cbc_from_base64(&response, &aes_key).map_err(|e| {
                WxErrorException::Runtime(wx_rust_common::error::WxRuntimeError::new(e))
            })?
        };

        let result: AispeechApiResponse<DialogResult> =
            AispeechApiResponse::from_json(&response_json)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::ensure_success(&result)?;

        // 对应 Java：answer 形如 JSON 时解析为 rawAnswer
        let mut dialog_result = result
            .data
            .ok_or_else(|| WxErrorException::from_code(-99, "data 字段缺失"))?;
        if let Some(answer) = dialog_result.answer.as_deref() {
            if looks_like_json(answer) {
                dialog_result.raw_answer = serde_json::from_str(answer).ok();
            }
        }
        Ok(dialog_result)
    }
}

/// 形如 JSON 判断（对应 Java `looksLikeJson`：非空且以 `{`/`[` 开头）。
fn looks_like_json(value: &str) -> bool {
    !value.is_empty() && (value.starts_with('{') || value.starts_with('['))
}
