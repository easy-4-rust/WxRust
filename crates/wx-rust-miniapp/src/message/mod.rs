//! 小程序消息模型。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean`（消息类 WxMaMessage/
//! WxMaKefuMessage/WxMaSubscribeMsgEvent 位于 bean 根包）与
//! `cn.binarywang.wx.miniapp.message`（输出消息接口/XML/JSON 输出消息）。
//! WxMaRunStepInfo 为 bean 根包生成类，此处转发。

pub mod wx_ma_json_out_message;
pub mod wx_ma_kefu_message;
pub mod wx_ma_message;
pub mod wx_ma_message_handler;
pub mod wx_ma_message_interceptor;
pub mod wx_ma_message_matcher;
pub mod wx_ma_message_router;
pub mod wx_ma_message_router_rule;
pub mod wx_ma_out_message;
pub mod wx_ma_subscribe_msg_event;
pub mod wx_ma_xml_out_message;

pub use wx_ma_json_out_message::WxMaJsonOutMessage;
pub use wx_ma_kefu_message::{AiMsgContext, KfImage, KfLink, KfMaPage, KfText, WxMaKefuMessage};
pub use wx_ma_message::{WxMaMessage, XmlValue};
pub use wx_ma_message_handler::WxMaMessageHandler;
pub use wx_ma_message_interceptor::WxMaMessageInterceptor;
pub use wx_ma_message_matcher::WxMaMessageMatcher;
pub use wx_ma_message_router::{RouteContext, WxMaMessageRouter};
pub use wx_ma_message_router_rule::WxMaMessageRouterRule;
pub use wx_ma_out_message::WxMaOutMessage;
pub use wx_ma_subscribe_msg_event::{
    ChangeEvent, PopupEvent, SentEvent, SubscribeMsgChangeEvent, SubscribeMsgPopupEvent,
    SubscribeMsgSentEvent, WxMaSubscribeMsgEvent, WxMaSubscribeMsgEventJson,
};
pub use wx_ma_xml_out_message::WxMaXmlOutMessage;

/// 微信运动步数信息（对应 Java `cn.binarywang.wx.miniapp.bean.WxMaRunStepInfo`，
/// 生成于 bean 根包，`from_json` 取 `stepInfoList` 数组）。
pub use crate::bean::wx_ma_run_step_info::WxMaRunStepInfo;
