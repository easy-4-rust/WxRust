//! 企业微信智能机器人服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpIntelligentRobotServiceImpl`：
//! 以 `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `cpService`），全部方法经门面 `post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - 请求/响应均以 bean 的 `to_json`/`from_json` 表达（对应 Java
//!   `WxCpGsonBuilder`/`fromJson`）；
//! - `deleteRobot`/`getRobot`/`resetSession` 为固定 JSON 对象请求体
//!   （`robot_id`/`userid`/`session_id`，对应 Java `JsonObject` 组装）；
//! - `parseCallbackMessage` 仅解析不发起请求（对应 Java 直接
//!   `fromJson(callbackMessageJson)`）；
//! - `parseEncryptedCallbackMessage` 先验签解密再解析（对应 Java default
//!   方法，以 `WxCpIntelligentRobotCryptUtil` 完成加解密）；
//! - `replyMessage` 加密后 POST 到 `responseUrl`（对应 Java default
//!   方法，以 `postWithoutToken` 发起请求）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpIntelligentRobotService, WxCpService};
use crate::bean::{
    WxCpIntelligentRobot, WxCpIntelligentRobotChatRequest, WxCpIntelligentRobotChatResponse,
    WxCpIntelligentRobotCreateRequest, WxCpIntelligentRobotCreateResponse,
    WxCpIntelligentRobotMessage, WxCpIntelligentRobotSendMessageRequest,
    WxCpIntelligentRobotSendMessageResponse, WxCpIntelligentRobotUpdateRequest,
};
use crate::enums::url_intelligent_robot;
use crate::util::crypto::WxCpIntelligentRobotCryptUtil;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 企业微信智能机器人服务实现。
pub struct WxCpIntelligentRobotServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpIntelligentRobotServiceImpl {
    /// 构建智能机器人服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 序列化请求对象（对应 Java `toJson`）。
    fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpIntelligentRobotService for WxCpIntelligentRobotServiceImpl {
    /// 创建智能机器人（对应 Java `createRobot`）。
    async fn create_robot(
        &self,
        request: &WxCpIntelligentRobotCreateRequest,
    ) -> Result<WxCpIntelligentRobotCreateResponse, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::CREATE_ROBOT);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpIntelligentRobotCreateResponse::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除智能机器人（对应 Java `deleteRobot`）。
    async fn delete_robot(&self, robot_id: &str) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "robot_id".to_string(),
            serde_json::Value::String(robot_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::DELETE_ROBOT);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }

    /// 更新智能机器人（对应 Java `updateRobot`）。
    async fn update_robot(
        &self,
        request: &WxCpIntelligentRobotUpdateRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::UPDATE_ROBOT);
        svc.post(&url, &Self::to_json(request)?).await?;
        Ok(())
    }

    /// 查询智能机器人（对应 Java `getRobot`）。
    async fn get_robot(&self, robot_id: &str) -> Result<WxCpIntelligentRobot, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "robot_id".to_string(),
            serde_json::Value::String(robot_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::GET_ROBOT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpIntelligentRobot::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 智能机器人会话（对应 Java `chat`）。
    async fn chat(
        &self,
        request: &WxCpIntelligentRobotChatRequest,
    ) -> Result<WxCpIntelligentRobotChatResponse, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::CHAT);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpIntelligentRobotChatResponse::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 重置智能机器人会话（对应 Java `resetSession`）。
    async fn reset_session(
        &self,
        robot_id: &str,
        userid: &str,
        session_id: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "robot_id".to_string(),
            serde_json::Value::String(robot_id.to_string()),
        );
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        obj.insert(
            "session_id".to_string(),
            serde_json::Value::String(session_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::RESET_SESSION);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }

    /// 智能机器人主动发送消息（对应 Java `sendMessage`）。
    async fn send_message(
        &self,
        request: &WxCpIntelligentRobotSendMessageRequest,
    ) -> Result<WxCpIntelligentRobotSendMessageResponse, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_intelligent_robot::SEND_MESSAGE);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpIntelligentRobotSendMessageResponse::from_json(&response)
            .map_err(WxErrorException::Serde)
    }

    /// 解析智能机器人 API 模式回调消息（对应 Java `parseCallbackMessage`；
    /// 仅本地解析，无网络请求）。
    async fn parse_callback_message(
        &self,
        callback_message_json: &str,
    ) -> Result<WxCpIntelligentRobotMessage, WxErrorException> {
        WxCpIntelligentRobotMessage::from_json(callback_message_json)
            .map_err(WxErrorException::Serde)
    }

    /// 解析加密的回调消息（对应 Java default 方法
    /// `parseEncryptedCallbackMessage`）。
    ///
    /// 先以 `WxCpIntelligentRobotCryptUtil` 验签解密，再调用
    /// `parse_callback_message` 解析明文 JSON。
    async fn parse_encrypted_callback_message(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypted_json: &str,
        token: &str,
        encoding_aes_key: &str,
        ai_bot_id: &str,
    ) -> Result<WxCpIntelligentRobotMessage, WxErrorException> {
        let crypt_util =
            WxCpIntelligentRobotCryptUtil::from_params(token, encoding_aes_key, ai_bot_id)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let plain_json = crypt_util
            .decrypt(msg_signature, timestamp, nonce, encrypted_json)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        WxCpIntelligentRobotMessage::from_json(&plain_json).map_err(WxErrorException::Serde)
    }

    /// 回复智能机器人消息（对应 Java default 方法 `replyMessage`）。
    ///
    /// 将明文 JSON 加密为 JSON 格式后 POST 到 `response_url`
    /// （对应 Java `postWithoutToken(responseUrl, cryptUtil.encrypt(...))`）。
    async fn reply_message(
        &self,
        response_url: &str,
        plain_json: &str,
        token: &str,
        encoding_aes_key: &str,
        ai_bot_id: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String, WxErrorException> {
        let crypt_util =
            WxCpIntelligentRobotCryptUtil::from_params(token, encoding_aes_key, ai_bot_id)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let encrypted_body = crypt_util
            .encrypt_json(plain_json, timestamp, nonce)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let svc = self.service()?;
        // 对应 Java `postWithoutToken`：不注入 access_token，直接 POST
        svc.post_without_token(response_url, &encrypted_body).await
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：智能机器人创建/查询/重置会话的请求路径/请求体/响应解析
    //! 与回调消息解析。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testCreateRobot`/`testGetRobot`/`testResetSession`：
    /// 请求路径与请求体、响应解析。
    #[tokio::test]
    async fn test_robot_create_get_reset() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/intelligent_robot/create") {
                json(r#"{"errcode":0,"errmsg":"ok","robot_id":"ROBOT_1"}"#)
            } else if path.contains("/cgi-bin/intelligent_robot/get") {
                json(r#"{"errcode":0,"errmsg":"ok","robot_id":"ROBOT_1","name":"客服机器人"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpIntelligentRobotServiceImpl::new(weak_service(&service));

        // createRobot：请求体序列化 + 响应解析
        let mut create = WxCpIntelligentRobotCreateRequest::default();
        create.name = "客服机器人".to_string();
        let created = svc_impl
            .create_robot(&create)
            .await
            .expect("创建机器人成功");
        assert_eq!(created.robot_id, "ROBOT_1");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/intelligent_robot/create")
        );
        assert!(
            server.last_body().contains(r#""name":"客服机器人""#),
            "body: {}",
            server.last_body()
        );

        // getRobot：{"robot_id": ...} 请求体 + 响应解析
        let robot = svc_impl.get_robot("ROBOT_1").await.expect("查询机器人成功");
        assert_eq!(robot.robot_id, "ROBOT_1");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/intelligent_robot/get")
        );
        let body = server.last_body();
        assert!(body.contains(r#""robot_id":"ROBOT_1""#), "body: {body}");

        // resetSession：三个字段请求体
        svc_impl
            .reset_session("ROBOT_1", "zhangsan", "SESSION_1")
            .await
            .expect("重置会话成功");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/intelligent_robot/reset_session")
        );
        let body = server.last_body();
        assert!(body.contains(r#""robot_id":"ROBOT_1""#), "body: {body}");
        assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
        assert!(body.contains(r#""session_id":"SESSION_1""#), "body: {body}");
    }

    /// 镜像 Java `testParseCallbackMessage`：回调消息 JSON 解析（无网络
    /// 请求）。
    #[tokio::test]
    async fn test_robot_parse_callback_message() {
        let service = service_with_host("http://127.0.0.1:1");
        let svc_impl = WxCpIntelligentRobotServiceImpl::new(weak_service(&service));

        let json = r#"{"msgid":"MSG_1","aibotid":"ROBOT_1","chatid":"CHAT_1","chattype":"single","from":{"userid":"zhangsan"},"msgtype":"text","text":{"content":"你好"}}"#;
        let msg = svc_impl
            .parse_callback_message(json)
            .await
            .expect("解析回调消息成功");
        assert_eq!(msg.msg_id, "MSG_1");
        assert_eq!(msg.ai_bot_id, "ROBOT_1");
        assert_eq!(msg.from.userid, "zhangsan");
        assert_eq!(msg.msg_type, "text");
    }

    /// 镜像 Java `testParseEncryptedCallbackMessage`：加密回调消息的
    /// 验签解密 + 解析。
    #[tokio::test]
    async fn test_robot_parse_encrypted_callback_message() {
        use crate::util::crypto::WxCpIntelligentRobotCryptUtil;

        let service = service_with_host("http://127.0.0.1:1");
        let svc_impl = WxCpIntelligentRobotServiceImpl::new(weak_service(&service));

        let token = "test_token";
        let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG"; // 43 chars
        let ai_bot_id = "test_bot_id";

        let crypt_util = WxCpIntelligentRobotCryptUtil::from_params(token, aes_key, ai_bot_id)
            .expect("创建加密工具成功");

        // 先加密一条消息
        let plain_json = r#"{"msgid":"MSG_1","aibotid":"ROBOT_1","chatid":"CHAT_1","chattype":"single","from":{"userid":"zhangsan"},"msgtype":"text","text":{"content":"你好"}}"#;
        let timestamp = "1700000000";
        let nonce = "nonce_123";
        let encrypted_json = crypt_util
            .encrypt_json(plain_json, timestamp, nonce)
            .expect("加密成功");

        // 解析加密的 JSON 获取 encrypt 和 msg_signature
        let parsed: serde_json::Value = serde_json::from_str(&encrypted_json).unwrap();
        let encrypt = parsed["encrypt"].as_str().unwrap();
        let msg_signature = parsed["msg_signature"].as_str().unwrap();

        // 调用 parse_encrypted_callback_message
        let msg = svc_impl
            .parse_encrypted_callback_message(
                msg_signature,
                timestamp,
                nonce,
                encrypt,
                token,
                aes_key,
                ai_bot_id,
            )
            .await
            .expect("解析加密回调消息成功");
        assert_eq!(msg.msg_id, "MSG_1");
        assert_eq!(msg.ai_bot_id, "ROBOT_1");
        assert_eq!(msg.from.userid, "zhangsan");
        assert_eq!(msg.msg_type, "text");
    }

    /// 镜像 Java `testReplyMessage`：加密后 POST 到 responseUrl。
    #[tokio::test]
    async fn test_robot_reply_message() {
        use crate::util::crypto::WxCpIntelligentRobotCryptUtil;

        let server =
            MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpIntelligentRobotServiceImpl::new(weak_service(&service));

        let token = "test_token";
        let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG";
        let ai_bot_id = "test_bot_id";
        let response_url = &server.url("/cgi-bin/intelligent_robot/reply");
        let plain_json = r#"{"content":"hello"}"#;
        let timestamp = "1700000000";
        let nonce = "nonce_123";

        let result = svc_impl
            .reply_message(
                response_url,
                plain_json,
                token,
                aes_key,
                ai_bot_id,
                timestamp,
                nonce,
            )
            .await
            .expect("回复消息成功");
        // 验证 POST 已发出
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/intelligent_robot/reply"),
            "path: {}",
            server.last_path()
        );
        // 验证请求体包含 encrypt/msg_signature/timestamp/nonce
        let body = server.last_body();
        assert!(body.contains(r#""encrypt""#), "body: {body}");
        assert!(body.contains(r#""msg_signature""#), "body: {body}");
        assert!(body.contains(r#""timestamp":"1700000000""#), "body: {body}");
        assert!(body.contains(r#""nonce":"nonce_123""#), "body: {body}");
    }
}
