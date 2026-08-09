//! SPU状态消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.product.SpuStatusMessage.java`
//! （继承 `WxChannelMessage`）。
//!
//! Java 的 `OpenProductSpuStatusUpdate` unpack setter 把嵌套对象字段合并到
//! 顶层字段；Rust 以手动 `Deserialize` 表达同一合并语义（先顶层后嵌套、
//! 嵌套覆盖，ADAPTED）。

use std::collections::HashMap;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use crate::bean::message::serde_helpers::AnyScalar;

/// SPU状态消息（对应 Java `SpuStatusMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SpuStatusMessage {
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

    /// 商家自定义商品id（对应 Java `outProductId`）。
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: Option<String>,
    /// 平台商品id（对应 Java `productId`）。
    #[serde(rename = "product_id", default)]
    pub product_id: Option<String>,
    /// 当前商品上下架状态（对应 Java `status`，见 `SpuStatus` 枚举）。
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

impl<'de> Deserialize<'de> for SpuStatusMessage {
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

            #[serde(rename = "out_product_id")]
            out_product_id: Option<AnyScalar>,
            #[serde(rename = "product_id")]
            product_id: Option<AnyScalar>,
            #[serde(rename = "status")]
            status: Option<AnyScalar>,
            #[serde(rename = "reason")]
            reason: Option<AnyScalar>,
            /// 嵌套对象（对应 Java unpack setter 的 `Map<String, Object>` 入参）。
            #[serde(rename = "OpenProductSpuStatusUpdate")]
            open_product_spu_status_update: Option<HashMap<String, AnyScalar>>,
        }

        let raw = Raw::deserialize(deserializer)?;
        let mut msg = SpuStatusMessage::default();
        msg.to_user = raw.to_user.map(|v| v.0);
        msg.from_user = raw.from_user.map(|v| v.0);
        msg.create_time = raw.create_time.and_then(|v| v.0.parse().ok());
        msg.msg_type = raw.msg_type.map(|v| v.0);
        msg.event = raw.event.map(|v| v.0);
        msg.encrypt = raw.encrypt.map(|v| v.0);
        msg.msg_id = raw.msg_id.and_then(|v| v.0.parse().ok());
        msg.out_product_id = raw.out_product_id.map(|v| v.0);
        msg.product_id = raw.product_id.map(|v| v.0);
        msg.status = raw.status.and_then(|v| v.0.parse().ok());
        msg.reason = raw.reason.map(|v| v.0);
        // 嵌套 unpack（对应 Java `unpackNameFromNestedObject`，后应用者覆盖）
        if let Some(map) = raw.open_product_spu_status_update {
            if let Some(v) = map.get("out_product_id") {
                msg.out_product_id = Some(v.0.clone());
            }
            if let Some(v) = map.get("product_id") {
                msg.product_id = Some(v.0.clone());
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
