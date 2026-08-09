//! 群聊服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpChatServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpChatService, WxCpService};
use crate::bean::{WxCpAppChatMessage, WxCpChat};
use crate::enums::url_chat::*;

/// 群聊服务实现。
pub struct WxCpChatServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpChatServiceImpl {
    /// 构建群聊服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpChatService for WxCpChatServiceImpl {
    async fn create(
        &self,
        name: &str,
        owner: &str,
        users: &[&str],
        chat_id: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create`：HashMap(4) 按 `StringUtils.isNotBlank` 条件组装
        // `name`/`owner`/`userlist`/`chatid`，POST `APPCHAT_CREATE`，
        // 响应取 `chatid`
        let mut data = serde_json::Map::new();
        if is_not_blank(name) {
            data.insert("name".to_string(), serde_json::Value::from(name));
        }
        if is_not_blank(owner) {
            data.insert("owner".to_string(), serde_json::Value::from(owner));
        }
        if !users.is_empty() {
            data.insert(
                "userlist".to_string(),
                serde_json::Value::Array(
                    users.iter().map(|v| serde_json::Value::from(*v)).collect(),
                ),
            );
        }
        if let Some(chat_id) = chat_id {
            if is_not_blank(chat_id) {
                data.insert("chatid".to_string(), serde_json::Value::from(chat_id));
            }
        }
        let config = svc.wx_cp_config_storage();
        let result = svc
            .post(
                &config.api_url(APPCHAT_CREATE),
                &serde_json::Value::Object(data).to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&result).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("chatid")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "chatid 字段缺失"))
    }

    async fn update(
        &self,
        chat_id: &str,
        name: Option<&str>,
        owner: Option<&str>,
        users_to_add: &[&str],
        users_to_delete: &[&str],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：HashMap(5) 按 isNotBlank / 非空条件组装
        // `chatid`/`name`/`owner`/`add_user_list`/`del_user_list`，
        // POST `APPCHAT_UPDATE`
        let mut data = serde_json::Map::new();
        if is_not_blank(chat_id) {
            data.insert("chatid".to_string(), serde_json::Value::from(chat_id));
        }
        if let Some(name) = name {
            if is_not_blank(name) {
                data.insert("name".to_string(), serde_json::Value::from(name));
            }
        }
        if let Some(owner) = owner {
            if is_not_blank(owner) {
                data.insert("owner".to_string(), serde_json::Value::from(owner));
            }
        }
        if !users_to_add.is_empty() {
            data.insert(
                "add_user_list".to_string(),
                serde_json::Value::Array(
                    users_to_add
                        .iter()
                        .map(|v| serde_json::Value::from(*v))
                        .collect(),
                ),
            );
        }
        if !users_to_delete.is_empty() {
            data.insert(
                "del_user_list".to_string(),
                serde_json::Value::Array(
                    users_to_delete
                        .iter()
                        .map(|v| serde_json::Value::from(*v))
                        .collect(),
                ),
            );
        }
        let config = svc.wx_cp_config_storage();
        svc.post(
            &config.api_url(APPCHAT_UPDATE),
            &serde_json::Value::Object(data).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get(&self, chat_id: &str) -> Result<WxCpChat, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get`：GET `APPCHAT_GET_CHATID + chatId`，响应取 `chat_info`
        // 子对象解析为 `WxCpChat`
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{chat_id}", config.api_url(APPCHAT_GET_CHATID));
        let result = svc.get(&url, "").await?;
        let json: serde_json::Value =
            serde_json::from_str(&result).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let chat_info = json
            .get("chat_info")
            .ok_or_else(|| WxErrorException::from_code(-99, "chat_info 字段缺失"))?;
        serde_json::from_value(chat_info.clone())
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn send_msg(&self, message: &WxCpAppChatMessage) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `sendMsg`：POST `APPCHAT_SEND`，请求体 `message.toJson()`
        let config = svc.wx_cp_config_storage();
        svc.post(&config.api_url(APPCHAT_SEND), &message.to_json())
            .await?;
        Ok(())
    }
}

/// 是否非空白（对应 Java `StringUtils.isNotBlank`）。
fn is_not_blank(s: &str) -> bool {
    !s.trim().is_empty()
}
