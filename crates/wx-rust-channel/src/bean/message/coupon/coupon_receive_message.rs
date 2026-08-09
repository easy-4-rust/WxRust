//! 用户领券 消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.coupon.CouponReceiveMessage.java`
//! （继承 `WxChannelMessage`）。
//!
//! Java 的 `receive_info` unpack setter 把嵌套对象字段合并到顶层字段
//! （Jackson 按文档顺序应用 setter，后出现者覆盖）；Rust 以手动 `Deserialize`
//! 表达同一合并语义：先应用顶层字段，再应用嵌套对象（嵌套覆盖）。
//! 实际回调中两种形式互斥，差异仅在同时出现时可见（ADAPTED）。

use std::collections::HashMap;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use crate::bean::message::serde_helpers::AnyScalar;

/// 用户领券 消息（对应 Java `CouponReceiveMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct CouponReceiveMessage {
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

    /// 领取的优惠券ID（对应 Java `couponId`）。
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: Option<String>,
    /// 生成的用户券ID（对应 Java `userCouponId`）。
    #[serde(rename = "user_coupon_id", default)]
    pub user_coupon_id: Option<String>,
    /// 领券时间（对应 Java `receiveTime`）。
    #[serde(rename = "receive_time", default)]
    pub receive_time: Option<String>,
}

impl<'de> Deserialize<'de> for CouponReceiveMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Default)]
        #[serde(default)]
        struct Raw {
            #[serde(rename = "ToUserName", default)]
            to_user: Option<AnyScalar>,
            #[serde(rename = "FromUserName", default)]
            from_user: Option<AnyScalar>,
            #[serde(rename = "CreateTime", default)]
            create_time: Option<AnyScalar>,
            #[serde(rename = "MsgType", default)]
            msg_type: Option<AnyScalar>,
            #[serde(rename = "Event", default)]
            event: Option<AnyScalar>,
            #[serde(rename = "Encrypt", default)]
            encrypt: Option<AnyScalar>,
            #[serde(rename = "MsgId", alias = "MsgID", default)]
            msg_id: Option<AnyScalar>,

            #[serde(rename = "coupon_id")]
            coupon_id: Option<AnyScalar>,
            #[serde(rename = "user_coupon_id")]
            user_coupon_id: Option<AnyScalar>,
            #[serde(rename = "receive_time")]
            receive_time: Option<AnyScalar>,
            /// 嵌套对象（对应 Java unpack setter 的 `Map<String, Object>` 入参）。
            #[serde(rename = "receive_info")]
            receive_info: Option<HashMap<String, AnyScalar>>,
        }

        let raw = Raw::deserialize(deserializer)?;
        let mut msg = CouponReceiveMessage::default();
        msg.to_user = raw.to_user.map(|v| v.0);
        msg.from_user = raw.from_user.map(|v| v.0);
        msg.create_time = raw.create_time.and_then(|v| v.0.parse().ok());
        msg.msg_type = raw.msg_type.map(|v| v.0);
        msg.event = raw.event.map(|v| v.0);
        msg.encrypt = raw.encrypt.map(|v| v.0);
        msg.msg_id = raw.msg_id.and_then(|v| v.0.parse().ok());
        msg.coupon_id = raw.coupon_id.map(|v| v.0);
        msg.user_coupon_id = raw.user_coupon_id.map(|v| v.0);
        msg.receive_time = raw.receive_time.map(|v| v.0);
        // 嵌套 unpack（对应 Java `unpackNameFromNestedObject`，后应用者覆盖）
        if let Some(map) = raw.receive_info {
            if let Some(v) = map.get("coupon_id") {
                msg.coupon_id = Some(v.0.clone());
            }
            if let Some(v) = map.get("user_coupon_id") {
                msg.user_coupon_id = Some(v.0.clone());
            }
            if let Some(v) = map.get("receive_time") {
                msg.receive_time = Some(v.0.clone());
            }
        }
        Ok(msg)
    }
}
