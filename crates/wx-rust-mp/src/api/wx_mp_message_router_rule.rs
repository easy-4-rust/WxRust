//! 微信消息路由规则。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMessageRouterRule`：规则条件 +
//! `test` 匹配 + `service` 拦截器/处理器链。Builder 链式配置在
//! `WxMpMessageRouter::rule()` 返回的 `RuleBuilder` 上完成。

use std::sync::Arc;

use regex::Regex;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::{WxMpMessageHandler, WxMpMessageInterceptor, WxMpMessageMatcher, WxMpService};
use crate::bean::message::{WxMpXmlMessage, WxMpXmlOutMessage};

/// 消息路由规则（存储形态）。
#[derive(Clone)]
pub struct WxMpMessageRouterRule {
    /// 是否异步执行，默认 true。
    pub async_exec: bool,
    /// 消息来自指定用户时匹配。
    pub from_user: Option<String>,
    /// 消息类型等于某值时匹配（不区分大小写）。
    pub msg_type: Option<String>,
    /// 事件等于某值时匹配（不区分大小写）。
    pub event: Option<String>,
    /// 事件匹配该正则表达式时匹配。
    pub event_regex: Option<String>,
    /// 事件 key 等于某值时匹配（不区分大小写）。
    pub event_key: Option<String>,
    /// 事件 key 匹配该正则表达式时匹配。
    pub event_key_regex: Option<String>,
    /// 内容等于某值时匹配（内容 trim 后比较，空串视为 null）。
    pub content: Option<String>,
    /// 内容匹配该正则表达式时匹配。
    pub r_content: Option<String>,
    /// 自定义匹配器。
    pub matcher: Option<Arc<dyn WxMpMessageMatcher>>,
    /// 规则结束后消息是否继续进入其他规则。
    pub re_enter: bool,
    /// 消息处理器列表。
    pub handlers: Vec<Arc<dyn WxMpMessageHandler>>,
    /// 消息拦截器列表。
    pub interceptors: Vec<Arc<dyn WxMpMessageInterceptor>>,
}

impl WxMpMessageRouterRule {
    /// 新建空规则。
    pub fn new() -> Self {
        Self {
            async_exec: true,
            from_user: None,
            msg_type: None,
            event: None,
            event_regex: None,
            event_key: None,
            event_key_regex: None,
            content: None,
            r_content: None,
            matcher: None,
            re_enter: false,
            handlers: Vec::new(),
            interceptors: Vec::new(),
        }
    }

    /// 测试消息是否匹配该规则（对应 Java `test`）。
    ///
    /// 事件类比较不区分大小写；正则匹配对 `trim` 后的值执行；
    /// `content` 按 `trim` 后与规则值比较（空串视为 null）。
    pub fn test(&self, wx_message: &WxMpXmlMessage) -> bool {
        let event_trimmed = wx_message.event.as_deref().unwrap_or("").trim();
        let event_key_trimmed = wx_message.event_key.as_deref().unwrap_or("").trim();
        let content_trimmed = wx_message.content.as_deref().unwrap_or("").trim();
        let content_null = if content_trimmed.is_empty() {
            None
        } else {
            Some(content_trimmed)
        };

        (self.from_user.is_none()
            || self
                .from_user
                .as_deref()
                .is_some_and(|f| Some(f) == wx_message.from_user.as_deref()))
            && (self.msg_type.is_none()
                || self.msg_type.as_deref().is_some_and(|m| {
                    wx_message
                        .msg_type
                        .as_deref()
                        .is_some_and(|mt| m.eq_ignore_ascii_case(mt))
                }))
            && (self.event.is_none()
                || self.event.as_deref().is_some_and(|e| {
                    e.eq_ignore_ascii_case(wx_message.event.as_deref().unwrap_or(""))
                }))
            && (self.event_regex.is_none()
                || self
                    .event_regex
                    .as_deref()
                    .is_some_and(|r| Regex::new(r).is_ok_and(|re| re.is_match(event_trimmed))))
            && (self.event_key.is_none()
                || self.event_key.as_deref().is_some_and(|k| {
                    k.eq_ignore_ascii_case(wx_message.event_key.as_deref().unwrap_or(""))
                }))
            && (self.event_key_regex.is_none()
                || self
                    .event_key_regex
                    .as_deref()
                    .is_some_and(|r| Regex::new(r).is_ok_and(|re| re.is_match(event_key_trimmed))))
            && (self.content.is_none()
                || self
                    .content
                    .as_deref()
                    .is_some_and(|c| Some(c) == content_null))
            && (self.r_content.is_none()
                || self
                    .r_content
                    .as_deref()
                    .is_some_and(|r| Regex::new(r).is_ok_and(|re| re.is_match(content_trimmed))))
            && (self.matcher.is_none()
                || self
                    .matcher
                    .as_ref()
                    .is_some_and(|m| m.match_message(wx_message)))
    }

    /// 处理微信推送过来的消息（对应 Java `service`）。
    ///
    /// 拦截器全部通过后按序执行 handler，返回最后一个 handler 的结果；
    /// 拦截器不通过返回 `None`；handler 抛 `WxErrorException` 时上抛由
    /// 调用方（router）按异常处理器语义处理。
    pub fn service(
        &self,
        wx_message: &WxMpXmlMessage,
        context: &mut crate::api::RouteContext,
        wx_mp_service: Option<&dyn WxMpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxMpXmlOutMessage>, WxErrorException> {
        for interceptor in &self.interceptors {
            if !interceptor.intercept(wx_message, context, wx_mp_service, session_manager) {
                return Ok(None);
            }
        }

        // 交给 handler 处理，返回最后一个 handler 的结果
        let mut res: Option<WxMpXmlOutMessage> = None;
        for handler in &self.handlers {
            res = handler.handle(wx_message, context, wx_mp_service, session_manager)?;
        }
        Ok(res)
    }

    /// 是否为异步规则。
    pub fn is_async(&self) -> bool {
        self.async_exec
    }

    /// 规则结束后是否继续进入其他规则。
    pub fn is_re_enter(&self) -> bool {
        self.re_enter
    }
}

impl Default for WxMpMessageRouterRule {
    fn default() -> Self {
        Self::new()
    }
}
