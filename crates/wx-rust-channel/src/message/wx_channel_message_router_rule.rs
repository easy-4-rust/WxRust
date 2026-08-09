//! 消息路由规则。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.WxChannelMessageRouterRule<T
//! extends WxChannelMessage>`：规则条件（msgType/event/matcher）+ `isMatch`
//! 匹配 + `process` 拦截器/处理器链 + 按 `messageClass` 重新反序列化。
//!
//! Java 的 `List<WxChannelMessageRouterRule<? extends WxChannelMessage>>`
//! 以类型擦除 trait [`WxChannelMessageRouterRuleErased`] 承载异构泛型规则
//! （Rust 无通配符泛型，ADAPTED）；异步规则由路由器提交 tokio 任务。

use std::marker::PhantomData;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use wx_rust_common::api::WxErrorExceptionHandler;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxChannelService;
use crate::message::rule::{
    WxChannelMessageHandler, WxChannelMessageInterceptor, WxChannelMessageMatcher,
};
use crate::message::{RouteContext, WxChannelMessage};

/// 消息类型约束（对应 Java `T extends WxChannelMessage`：可反序列化的消息类型）。
pub trait WxChannelMessageLike: DeserializeOwned + Send + Sync + 'static {}
impl<T: DeserializeOwned + Send + Sync + 'static> WxChannelMessageLike for T {}

/// 类型擦除后的路由规则统一接口。
///
/// 对应 Java 规则列表 `List<WxChannelMessageRouterRule<? extends
/// WxChannelMessage>>` 的元素（Rust 以 trait 对象表达异构列表，ADAPTED）。
pub trait WxChannelMessageRouterRuleErased: Send + Sync + 'static {
    /// 测试消息是否匹配规则（对应 Java `isMatch`）。
    fn rule_is_match(&self, message: &WxChannelMessage) -> bool;

    /// 是否异步执行（对应 Java `isAsync`）。
    fn rule_is_async(&self) -> bool;

    /// 匹配后是否继续进入其他规则（对应 Java `isNext`）。
    fn rule_is_next(&self) -> bool;

    /// 处理微信推送过来的消息（对应 Java `process`）。
    ///
    /// 返回最后 handler 的结果；任一拦截器不通过返回 `None`；异常交给
    /// `exception_handler` 后返回 `None`（Java catch `WxErrorException` 语义）。
    #[allow(clippy::too_many_arguments)]
    fn rule_process(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        context: &mut RouteContext,
        service: Option<&dyn WxChannelService>,
        session_manager: &dyn WxSessionManager,
        exception_handler: &dyn WxErrorExceptionHandler,
    ) -> Option<String>;
}

/// 消息路由规则（对应 Java `WxChannelMessageRouterRule<T>`）。
///
/// 字段与 Java 一一对应；`async_exec` 默认 `true`（Java 字段初始化一致）。
/// 可通过 [`crate::message::WxChannelMessageRouter::rule`] 的链式 builder
/// 或直接构造（字段均为 `pub`）使用。
pub struct WxChannelMessageRouterRule<T: WxChannelMessageLike> {
    /// 是否异步, 默认是 true（对应 Java `async`）。
    pub async_exec: bool,
    /// 消息类型（对应 Java `msgType`；`setEvent` 时自动置为 "event"）。
    pub msg_type: Option<String>,
    /// 事件类型（对应 Java `event`）。
    pub event: Option<String>,
    /// 自定义匹配器（对应 Java `matcher`）。
    pub matcher: Option<Arc<dyn WxChannelMessageMatcher>>,
    /// 进入下一个rule，默认是 false（对应 Java `next`）。
    pub next: bool,
    /// 消息处理器（对应 Java `handlers`）。
    pub handlers: Vec<Arc<dyn WxChannelMessageHandler<T>>>,
    /// 消息拦截器（对应 Java `interceptors`）。
    pub interceptors: Vec<Arc<dyn WxChannelMessageInterceptor>>,
    /// 消息类型标记（对应 Java `messageClass`）。
    message_class: PhantomData<T>,
}

impl<T: WxChannelMessageLike> WxChannelMessageRouterRule<T> {
    /// 新建空规则（`async` 默认 true，与 Java 字段初始化一致）。
    pub fn new() -> Self {
        Self {
            async_exec: true,
            msg_type: None,
            event: None,
            matcher: None,
            next: false,
            handlers: Vec::new(),
            interceptors: Vec::new(),
            message_class: PhantomData,
        }
    }

    /// 设置事件（对应 Java `setEvent(String)`：同时把 msgType 置为 "event"）。
    pub fn set_event(&mut self, event: impl Into<String>) -> &mut Self {
        self.msg_type = Some(crate::enums::MessageType::Event.key().to_string());
        self.event = Some(event.into());
        self
    }

    /// 测试消息是否匹配规则（对应 Java `isMatch`）。
    ///
    /// msgType/event 双方 `toLowerCase` 后比较（Rust `to_ascii_lowercase`）；
    /// matcher 自定义匹配；规则条件为 `None` 时不参与判断。
    pub fn is_match(&self, message: &WxChannelMessage) -> bool {
        let msg_type = message.msg_type.as_deref().map(str::to_ascii_lowercase);
        let event = message.event.as_deref().map(str::to_ascii_lowercase);

        let match_msg_type = match &self.msg_type {
            None => true,
            Some(rule_type) => msg_type.as_deref() == Some(&rule_type.to_ascii_lowercase()),
        };
        let match_event = match &self.event {
            None => true,
            Some(rule_event) => event.as_deref() == Some(&rule_event.to_ascii_lowercase()),
        };
        let match_matcher = match &self.matcher {
            None => true,
            Some(matcher) => matcher.match_message(message),
        };

        match_msg_type && match_event && match_matcher
    }
}

impl<T: WxChannelMessageLike> Default for WxChannelMessageRouterRule<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: WxChannelMessageLike> WxChannelMessageRouterRuleErased for WxChannelMessageRouterRule<T> {
    fn rule_is_match(&self, message: &WxChannelMessage) -> bool {
        self.is_match(message)
    }

    fn rule_is_async(&self) -> bool {
        self.async_exec
    }

    fn rule_is_next(&self) -> bool {
        self.next
    }

    fn rule_process(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        context: &mut RouteContext,
        service: Option<&dyn WxChannelService>,
        session_manager: &dyn WxSessionManager,
        exception_handler: &dyn WxErrorExceptionHandler,
    ) -> Option<String> {
        // 重新反序列化消息
        let temp_message = match deserialize_message::<T>(content, service) {
            Some(m) => m,
            None => {
                // Java: log.error("消息重新反序列化失败，请检查消息格式是否正确或者指定正确的messageClass")
                return None;
            }
        };

        let out_message = (|| -> Result<Option<String>, WxErrorException> {
            // 如果拦截器不通过，返回 null
            for interceptor in &self.interceptors {
                if !interceptor.intercept(message, content, context, service, session_manager)? {
                    return Ok(None);
                }
            }

            // 交给 handler 处理，返回最后 handler 的结果
            let mut out_message: Option<String> = None;
            for handler in &self.handlers {
                out_message =
                    handler.handle(&temp_message, content, app_id, context, session_manager)?;
            }
            Ok(out_message)
        })();

        match out_message {
            Ok(m) => m,
            Err(e) => {
                // Java: catch (WxErrorException e) { exceptionHandler.handle(e); }
                exception_handler.handle(e);
                None
            }
        }
    }
}

/// 重新反序列化消息（对应 Java `deserialize(String, Class<T>, WxChannelService)`）。
///
/// 先按配置的 `msgDataFormat`（JSON 默认）反序列化；失败时按内容前缀猜测
/// （`<xml>` → XML，`{` → JSON）。
fn deserialize_message<T: WxChannelMessageLike>(
    content: &str,
    service: Option<&dyn WxChannelService>,
) -> Option<Box<T>> {
    let msg_format =
        service.and_then(|s| s.wx_channel_config().msg_data_format().map(str::to_string));
    if let Some(t) = deserialize_by_format::<T>(content, msg_format.as_deref()) {
        return Some(t);
    }
    // 如果指定的消息格式不正确，根据内容猜猜格式
    if content.starts_with("<xml>") {
        deserialize_xml::<T>(content)
    } else if content.starts_with('{') {
        deserialize_json::<T>(content)
    } else {
        None
    }
}

/// 按指定格式反序列化（对应 Java `deserialize(String, Class<T>, String)`）。
fn deserialize_by_format<T: WxChannelMessageLike>(
    content: &str,
    msg_format: Option<&str>,
) -> Option<Box<T>> {
    // Java: msgFormat == null || msgFormat.equalsIgnoreCase("JSON") → JSON，否则 XML
    if msg_format.is_none() || msg_format.unwrap().eq_ignore_ascii_case("JSON") {
        deserialize_json::<T>(content)
    } else {
        deserialize_xml::<T>(content)
    }
}

/// JSON 反序列化（对应 Java `JsonUtils.decode`）。
fn deserialize_json<T: WxChannelMessageLike>(content: &str) -> Option<Box<T>> {
    serde_json::from_str::<T>(content).ok().map(Box::new)
}

/// XML 反序列化（对应 Java `XmlUtils.decode`；quick-xml serde，CDATA 自动处理）。
fn deserialize_xml<T: WxChannelMessageLike>(content: &str) -> Option<Box<T>> {
    quick_xml::de::from_str::<T>(content).ok().map(Box::new)
}
