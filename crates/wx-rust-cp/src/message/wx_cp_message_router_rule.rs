//! 企业微信消息路由规则。
//!
//! 对应 Java `me.chanjar.weixin.cp.message.WxCpMessageRouterRule`：
//! 规则条件 + `test` 匹配 + `service` 拦截器/处理器链。Builder 链式配置在
//! `WxCpMessageRouter::rule()` 返回的 `RuleBuilder` 上完成。

use std::sync::Arc;

use regex::Regex;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxCpService;
use crate::bean::message::{WxCpXmlMessage, WxCpXmlOutMessage};
use crate::message::{
    RouteContext, WxCpMessageHandler, WxCpMessageInterceptor, WxCpMessageMatcher,
};

/// 消息路由规则（存储形态）。
#[derive(Clone)]
pub struct WxCpMessageRouterRule {
    /// 是否异步执行，默认 true（对应 Java `async`）。
    pub async_exec: bool,
    /// 消息来自指定用户时匹配（对应 Java `fromUser`，精确相等）。
    pub from_user: Option<String>,
    /// 消息类型等于某值时匹配，不区分大小写（对应 Java `msgType`）。
    pub msg_type: Option<String>,
    /// 事件等于某值时匹配，不区分大小写（对应 Java `event`）。
    pub event: Option<String>,
    /// 事件 key 等于某值时匹配，不区分大小写（对应 Java `eventKey`）。
    pub event_key: Option<String>,
    /// 事件 key 匹配该正则表达式时匹配（对应 Java `eventKeyRegex`，
    /// `Pattern.matches` 整串匹配，消息值为空串时不匹配）。
    pub event_key_regex: Option<String>,
    /// 内容等于某值时匹配（消息内容 trim 后比较，空串视为 null，
    /// 对应 Java `content` + `StringUtils.trimToNull`）。
    pub content: Option<String>,
    /// 内容匹配该正则表达式时匹配（对应 Java `rContent`）。
    pub r_content: Option<String>,
    /// 如果 agentId 匹配（对应 Java `agentId(Integer)`）。
    pub agent_id: Option<i32>,
    /// 自定义匹配器（对应 Java `matcher`）。
    pub matcher: Option<Arc<dyn WxCpMessageMatcher>>,
    /// 规则结束后消息是否继续进入其他规则（对应 Java `reEnter`）。
    pub re_enter: bool,
    /// 消息处理器列表（对应 Java `handlers`）。
    pub handlers: Vec<Arc<dyn WxCpMessageHandler>>,
    /// 消息拦截器列表（对应 Java `interceptors`）。
    pub interceptors: Vec<Arc<dyn WxCpMessageInterceptor>>,
}

impl WxCpMessageRouterRule {
    /// 新建空规则（`async` 默认 true，与 Java 字段初始化一致）。
    pub fn new() -> Self {
        Self {
            async_exec: true,
            from_user: None,
            msg_type: None,
            event: None,
            event_key: None,
            event_key_regex: None,
            content: None,
            r_content: None,
            agent_id: None,
            matcher: None,
            re_enter: false,
            handlers: Vec::new(),
            interceptors: Vec::new(),
        }
    }

    /// 测试消息是否匹配该规则（对应 Java `test`）。
    ///
    /// 组合逻辑照 Java `WxCpMessageRouterRule.test()`：
    /// - `fromUser` 精确相等（消息值为 null 时不匹配）；
    /// - `agentId` 与消息 `AgentID` 解析出的整数相等（Java
    ///   `Integer.valueOf(getAgentId())` 对非法值抛异常，Rust 解析失败
    ///   视为不匹配，ADAPTED）；
    /// - `msgType`/`event`/`eventKey` 不区分大小写比较；
    /// - `eventKeyRegex` 对 trim 后的事件 key 整串匹配（null 视为空串）；
    /// - `content` 与消息内容 trim 后比较（消息内容为 null/空/全空白时
    ///   视为 null 而不匹配，对应 `StringUtils.trimToNull`）；
    /// - `rContent` 对 trim 后的消息内容整串匹配；
    /// - `matcher` 自定义匹配。
    pub fn test(&self, wx_message: &WxCpXmlMessage) -> bool {
        // Java trimToNull：null/空/全空白 → null
        let content_trim_to_null = wx_message
            .content
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        (self.from_user.is_none()
            || self.from_user.as_deref() == wx_message.from_user_name.as_deref())
            && (self.agent_id.is_none()
                || wx_message
                    .agent_id
                    .as_deref()
                    .and_then(|s| s.trim().parse::<i32>().ok())
                    == self.agent_id)
            && (self.msg_type.is_none()
                || wx_message.msg_type.as_deref().is_some_and(|m| {
                    self.msg_type
                        .as_deref()
                        .is_some_and(|t| t.eq_ignore_ascii_case(m))
                }))
            && (self.event.is_none()
                || wx_message.event.as_deref().is_some_and(|e| {
                    self.event
                        .as_deref()
                        .is_some_and(|ev| ev.eq_ignore_ascii_case(e))
                }))
            && (self.event_key.is_none()
                || wx_message.event_key.as_deref().is_some_and(|k| {
                    self.event_key
                        .as_deref()
                        .is_some_and(|ek| ek.eq_ignore_ascii_case(k))
                }))
            && (self.event_key_regex.is_none()
                || regex_full_match(
                    self.event_key_regex.as_deref().unwrap_or_default(),
                    wx_message.event_key.as_deref().unwrap_or("").trim(),
                ))
            && (self.content.is_none() || self.content.as_deref() == content_trim_to_null)
            && (self.r_content.is_none()
                || regex_full_match(
                    self.r_content.as_deref().unwrap_or_default(),
                    content_trim_to_null.unwrap_or(""),
                ))
            && (self.matcher.is_none()
                || self
                    .matcher
                    .as_ref()
                    .is_some_and(|m| m.match_message(wx_message)))
    }

    /// 处理微信推送过来的消息（对应 Java `service`）。
    ///
    /// 拦截器全部通过后按序执行 handler，返回最后一个 handler 的结果；
    /// 任一拦截器返回 `false` 立即返回 `None`（对应 Java 提前 return null）。
    /// Java 在方法内部 catch `WxErrorException` 并交给 exceptionHandler
    /// （返回 null）；Rust 以 `Result` 上抛，由 router 按异常处理器语义处理。
    pub fn service(
        &self,
        wx_message: &WxCpXmlMessage,
        context: &mut RouteContext,
        wx_cp_service: Option<&dyn WxCpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxCpXmlOutMessage>, WxErrorException> {
        // 如果拦截器不通过，直接返回 null（对应 Java 提前 return）
        for interceptor in &self.interceptors {
            if !interceptor.intercept(wx_message, context, wx_cp_service, session_manager) {
                return Ok(None);
            }
        }

        // 交给 handler 处理，返回最后一个 handler 的结果
        let mut res: Option<WxCpXmlOutMessage> = None;
        for handler in &self.handlers {
            res = handler.handle(wx_message, context, wx_cp_service, session_manager)?;
        }
        Ok(res)
    }

    /// 是否为异步规则（对应 Java `isAsync()`）。
    pub fn is_async(&self) -> bool {
        self.async_exec
    }

    /// 规则结束后是否继续进入其他规则（对应 Java `isReEnter()`）。
    pub fn is_re_enter(&self) -> bool {
        self.re_enter
    }
}

impl Default for WxCpMessageRouterRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Java `Pattern.matches(regex, input)` 语义：整个字符串必须完全匹配。
/// 以 `^(?:...)$` 包裹实现（Java 隐式整串锚定；`regex` 本身含锚点时仍正确）。
fn regex_full_match(regex: &str, text: &str) -> bool {
    Regex::new(&format!("^(?:{regex})$")).is_ok_and(|re| re.is_match(text))
}
