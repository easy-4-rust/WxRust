//! 品牌消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.product.BrandMessage.java`
//! （继承 `WxChannelMessage`）。
//!
//! Java 的 `BrandEvent` unpack setter 把嵌套对象字段合并到顶层字段；Rust 以
//! 手动 `Deserialize` 表达同一合并语义（先顶层后嵌套、嵌套覆盖，ADAPTED）。

use std::collections::HashMap;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use crate::bean::message::serde_helpers::AnyScalar;

/// 品牌消息（对应 Java `BrandMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct BrandMessage {
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

    /// 品牌库中的品牌编号（对应 Java `brandId`）。
    #[serde(rename = "brand_id", default)]
    pub brand_id: Option<String>,
    /// 审核id（对应 Java `auditId`）。
    #[serde(rename = "audit_id", default)]
    pub audit_id: Option<String>,
    /// 审核状态, 1新增品牌 2更新品牌 3撤回品牌审核 4审核成功 5审核失败
    /// 6删除品牌 7品牌资质被系统撤销（对应 Java `status`）。
    #[serde(
        rename = "status",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub status: Option<i32>,
    /// 相关信息（对应 Java `reason`）。
    #[serde(rename = "reason", default)]
    pub reason: Option<String>,
}

impl<'de> Deserialize<'de> for BrandMessage {
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

            #[serde(rename = "brand_id")]
            brand_id: Option<AnyScalar>,
            #[serde(rename = "audit_id")]
            audit_id: Option<AnyScalar>,
            #[serde(rename = "status")]
            status: Option<AnyScalar>,
            #[serde(rename = "reason")]
            reason: Option<AnyScalar>,
            /// 嵌套对象（对应 Java unpack setter 的 `Map<String, Object>` 入参）。
            #[serde(rename = "BrandEvent")]
            brand_event: Option<HashMap<String, AnyScalar>>,
        }

        let raw = Raw::deserialize(deserializer)?;
        let mut msg = BrandMessage::default();
        msg.to_user = raw.to_user.map(|v| v.0);
        msg.from_user = raw.from_user.map(|v| v.0);
        msg.create_time = raw.create_time.and_then(|v| v.0.parse().ok());
        msg.msg_type = raw.msg_type.map(|v| v.0);
        msg.event = raw.event.map(|v| v.0);
        msg.encrypt = raw.encrypt.map(|v| v.0);
        msg.msg_id = raw.msg_id.and_then(|v| v.0.parse().ok());
        msg.brand_id = raw.brand_id.map(|v| v.0);
        msg.audit_id = raw.audit_id.map(|v| v.0);
        msg.status = raw.status.and_then(|v| v.0.parse().ok());
        msg.reason = raw.reason.map(|v| v.0);
        // 嵌套 unpack（对应 Java `unpackNameFromNestedObject`，后应用者覆盖）
        if let Some(map) = raw.brand_event {
            if let Some(v) = map.get("brand_id") {
                msg.brand_id = Some(v.0.clone());
            }
            if let Some(v) = map.get("audit_id") {
                msg.audit_id = Some(v.0.clone());
            }
            if let Some(v) = map.get("status") {
                msg.status = v.0.parse().ok();
            }
            if let Some(v) = map.get("reason") {
                msg.reason = Some(v.0.clone());
            }
        }
        Ok(msg)
    }
}
