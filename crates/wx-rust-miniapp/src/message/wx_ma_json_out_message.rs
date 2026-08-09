//! 输出给微信服务器的 JSON 格式消息。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaJsonOutMessage`。Gson
//! 线格式：`toUserName`/`fromUserName`/`createTime`/`msgType`（null 省略）。

use serde::{Deserialize, Serialize};

use crate::config::WxMaConfig;
use crate::message::WxMaOutMessage;
use crate::util::crypto::WxMaCryptUtils;

/// 微信小程序输出给微信服务器的 JSON 格式消息（对应 Java `WxMaJsonOutMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaJsonOutMessage {
    /// 接收方帐号（收到的 OpenID）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_name: Option<String>,
    /// 开发者微信号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_user_name: Option<String>,
    /// 消息创建时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 消息类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

impl WxMaJsonOutMessage {
    /// 转换成 JSON 格式（对应 Java `toJson()`）。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl WxMaOutMessage for WxMaJsonOutMessage {
    /// 转换成 XML 格式（对于 JSON 消息类型，返回 JSON 格式，与 Java 一致）。
    fn to_xml(&self) -> String {
        self.to_json()
    }

    /// 转换成 JSON 格式。
    fn to_json(&self) -> String {
        WxMaJsonOutMessage::to_json(self)
    }

    /// 转换成加密的 JSON 格式（对应 Java `toEncryptedJson`）。
    fn to_encrypted_json(&self, config: &dyn WxMaConfig) -> Result<String, String> {
        let plain_json = self.to_json();
        let crypt_util = WxMaCryptUtils::new(config)?;
        crypt_util.encrypt(&plain_json)
    }

    /// 转换成加密的 XML 格式（对于 JSON 消息类型，返回加密的 JSON 格式，与 Java 一致）。
    fn to_encrypted_xml(&self, config: &dyn WxMaConfig) -> Result<String, String> {
        self.to_encrypted_json(config)
    }
}
