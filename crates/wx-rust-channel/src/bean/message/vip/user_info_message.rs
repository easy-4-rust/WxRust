//! 用户信息消息（会员）。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.vip.UserInfoMessage.java`
//! （继承 `WxChannelMessage`）。
//!
//! 注意 Java 的 `userInfo` 字段 JSON 属性名为 `user_info` 而
//! `@JacksonXmlProperty(localName = "order_info")`（XML 名为 order_info，Java
//! 原样 bug）；serde 单一 rename 无法区分两种格式，Rust 以 JSON 名为准
//! （ADAPTED，实际回调为 JSON）。

use serde::{Deserialize, Serialize};

use crate::bean::message::vip::UserInfo;

/// 用户信息消息（对应 Java `UserInfoMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserInfoMessage {
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

    /// 用户信息（对应 Java `userInfo`）。
    #[serde(rename = "user_info", default)]
    pub user_info: Option<UserInfo>,
}
