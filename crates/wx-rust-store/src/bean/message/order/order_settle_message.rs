//! 订单结算消息。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.order.OrderSettleMessage.java`
//! （继承 `WxStoreMessage`）。

use serde::{Deserialize, Serialize};

use crate::bean::message::order::OrderSettleInfo;

/// 订单结算消息（对应 Java `OrderSettleMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderSettleMessage {
    /// 开发者微信号（对应 Java `WxStoreMessage.toUser`）。
    #[serde(rename = "ToUserName", default)]
    pub to_user: Option<String>,
    /// 发送方帐号（对应 Java `WxStoreMessage.fromUser`）。
    #[serde(rename = "FromUserName", default)]
    pub from_user: Option<String>,
    /// 消息创建时间（整型，对应 Java `WxStoreMessage.createTime`）。
    #[serde(
        rename = "CreateTime",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub create_time: Option<i64>,
    /// 消息类型（对应 Java `WxStoreMessage.msgType`）。
    #[serde(rename = "MsgType", default)]
    pub msg_type: Option<String>,
    /// 事件类型（对应 Java `WxStoreMessage.event`）。
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 加密字段（对应 Java `WxStoreMessage.encrypt`）。
    #[serde(rename = "Encrypt", default)]
    pub encrypt: Option<String>,
    /// 消息id（对应 Java `WxStoreMessage.msgId`；`MsgID` 为兼容别名，
    /// 对应 Java `msgIdFill` setter）。
    #[serde(
        rename = "MsgId",
        alias = "MsgID",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub msg_id: Option<i64>,

    /// 订单信息（对应 Java `orderInfo`）。
    #[serde(rename = "order_info", default)]
    pub order_info: Option<OrderSettleInfo>,
}
