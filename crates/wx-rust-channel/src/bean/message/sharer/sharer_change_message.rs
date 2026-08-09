//! 分享员变更消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.sharer.SharerChangeMessage.java`
//! （继承 `WxChannelMessage`）。
//! 文档：https://developers.weixin.qq.com/doc/channels/API/sharer/callback/channels_ec_sharer_change.html

use serde::{Deserialize, Serialize};

/// 分享员变更消息（对应 Java `SharerChangeMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SharerChangeMessage {
    /// 开发者微信号（对应 Java `WxChannelMessage.toUser`）。
    #[serde(rename = "ToUserName", default)]
    pub to_user: Option<String>,
    /// 发送方帐号（对应 Java `WxChannelMessage.fromUser`）。
    #[serde(rename = "FromUserName", default)]
    pub from_user: Option<String>,
    /// 消息创建时间（整型，对应 Java `WxChannelMessage.createTime`）。
    #[serde(
        rename = "CreateTime",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub create_time: Option<i64>,
    /// 消息类型（对应 Java `WxChannelMessage.msgType`）。
    #[serde(rename = "MsgType", default)]
    pub msg_type: Option<String>,
    /// 事件类型（对应 Java `WxChannelMessage.event`）。
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 加密字段（对应 Java `WxChannelMessage.encrypt`）。
    #[serde(rename = "Encrypt", default)]
    pub encrypt: Option<String>,
    /// 消息id（对应 Java `WxChannelMessage.msgId`；`MsgID` 为兼容别名，
    /// 对应 Java `msgIdFill` setter）。
    #[serde(
        rename = "MsgId",
        alias = "MsgID",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub msg_id: Option<i64>,

    /// 分享员OpenID（对应 Java `openid`）。
    #[serde(rename = "openid", default)]
    pub openid: Option<String>,
    /// 分享员类型：0-普通分享员，1-店铺分享员（对应 Java `sharerType`）。
    #[serde(
        rename = "sharer_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub sharer_type: Option<i32>,
    /// 分享员绑定状态：1-绑定，2-解绑（对应 Java `bindStatus`）。
    #[serde(
        rename = "bind_status",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub bind_status: Option<i32>,
}
