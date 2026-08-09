//! 小程序消息路由规则。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaMessageRouterRule`：
//! 规则条件 + `test` 匹配 + `service` 拦截器/处理器链。Builder 链式配置在
//! `WxMaMessageRouter::rule()` 返回的 `RuleBuilder` 上完成。

use std::sync::Arc;

use regex::Regex;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxMaService;
use crate::message::{
    RouteContext, WxMaMessage, WxMaMessageHandler, WxMaMessageInterceptor, WxMaMessageMatcher,
    WxMaOutMessage,
};

/// 消息路由规则（存储形态）。
#[derive(Clone)]
pub struct WxMaMessageRouterRule {
    /// 是否异步执行，默认 true（对应 Java `async`）。
    pub async_exec: bool,
    /// 消息来自指定用户时匹配（对应 Java `fromUser`）。
    pub from_user: Option<String>,
    /// 消息类型等于某值时匹配，不区分大小写（对应 Java `msgType`）。
    pub msg_type: Option<String>,
    /// 事件等于某值时匹配，不区分大小写（对应 Java `event`）。
    pub event: Option<String>,
    /// 事件 key 等于某值（对应 Java `eventKey`）。
    ///
    /// 注意：Java 的 `WxMaMessageRouterRule.test()` 实际并未使用该字段
    /// （字段存在、有 setter/getter 但条件组合里没有它），此处照抄 Java 语义。
    pub event_key: Option<String>,
    /// 内容等于某值时匹配（消息内容 trim 后比较，对应 Java `content`）。
    pub content: Option<String>,
    /// 内容匹配该正则表达式时匹配（对应 Java `rContent`）。
    pub r_content: Option<String>,
    /// 标题等于某值时匹配（消息标题 trim 后比较，对应 Java `title`，
    /// 发送小程序页卡时有效）。
    pub title: Option<String>,
    /// 自定义匹配器（对应 Java `matcher`）。
    pub matcher: Option<Arc<dyn WxMaMessageMatcher>>,
    /// 规则结束后消息是否继续进入其他规则（对应 Java `reEnter`）。
    pub re_enter: bool,
    /// 消息处理器列表（对应 Java `handlers`）。
    pub handlers: Vec<Arc<dyn WxMaMessageHandler>>,
    /// 消息拦截器列表（对应 Java `interceptors`）。
    pub interceptors: Vec<Arc<dyn WxMaMessageInterceptor>>,
}

impl WxMaMessageRouterRule {
    /// 新建空规则（`async` 默认 true，与 Java 字段初始化一致）。
    pub fn new() -> Self {
        Self {
            async_exec: true,
            from_user: None,
            msg_type: None,
            event: None,
            event_key: None,
            content: None,
            r_content: None,
            title: None,
            matcher: None,
            re_enter: false,
            handlers: Vec::new(),
            interceptors: Vec::new(),
        }
    }

    /// 测试消息是否匹配该规则（对应 Java `test`）。
    ///
    /// 组合逻辑照 Java `WxMaMessageRouterRule.test()`：
    /// - `fromUser` 精确相等；
    /// - `msgType`/`event` 双方 toLowerCase 后相等（Rust 用 `eq_ignore_ascii_case`）；
    /// - `content`/`title` 与消息值 `trim` 后比较（消息值为 null 时不匹配）；
    /// - `rContent` 按 Java `Pattern.matches` 语义整串匹配（消息内容 null 视为空串）；
    /// - `matcher` 自定义匹配；
    /// - `eventKey` 字段存在但 Java `test()` 不使用，故不参与判断。
    pub fn test(&self, wx_message: &WxMaMessage) -> bool {
        let msg_type = wx_message.msg_type.as_deref();
        let event = wx_message.event.as_deref();
        let content_trimmed = wx_message.content.as_deref().map(str::trim);
        let title_trimmed = wx_message.title.as_deref().map(str::trim);

        (self.from_user.is_none() || self.from_user.as_deref() == wx_message.from_user.as_deref())
            && (self.msg_type.is_none()
                || self
                    .msg_type
                    .as_deref()
                    .is_some_and(|m| msg_type.is_some_and(|mt| m.eq_ignore_ascii_case(mt))))
            && (self.event.is_none()
                || self
                    .event
                    .as_deref()
                    .is_some_and(|e| event.is_some_and(|ev| e.eq_ignore_ascii_case(ev))))
            && (self.content.is_none()
                || content_trimmed == Some(self.content.as_deref().unwrap_or_default()))
            && (self.r_content.is_none()
                || regex_full_match(
                    self.r_content.as_deref().unwrap_or_default(),
                    content_trimmed.unwrap_or(""),
                ))
            && (self.matcher.is_none()
                || self
                    .matcher
                    .as_ref()
                    .is_some_and(|m| m.match_message(wx_message)))
            && (self.title.is_none()
                || title_trimmed == Some(self.title.as_deref().unwrap_or_default()))
    }

    /// 处理微信推送过来的消息（对应 Java `service`）。
    ///
    /// 拦截器全部通过后按序执行 handler，返回最后一个 handler 的结果；
    /// 任一拦截器返回 `false` 立即返回 `None`。Java 在方法内部 catch
    /// `WxErrorException` 并交给 exceptionHandler（返回 null）；Rust 以
    /// `Result` 上抛，由 router 按异常处理器语义处理（观察语义一致）。
    pub fn service(
        &self,
        wx_message: &WxMaMessage,
        context: &mut RouteContext,
        wx_ma_service: Option<&dyn WxMaService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<Arc<dyn WxMaOutMessage + Send + Sync>>, WxErrorException> {
        // 如果拦截器不通过，直接返回 null（对应 Java 提前 return）
        for interceptor in &self.interceptors {
            if !interceptor.intercept(wx_message, context, wx_ma_service, session_manager) {
                return Ok(None);
            }
        }

        // 交给 handler 处理，返回最后一个 handler 的结果
        let mut res: Option<Arc<dyn WxMaOutMessage + Send + Sync>> = None;
        for handler in &self.handlers {
            res = handler.handle(wx_message, context, wx_ma_service, session_manager)?;
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

impl Default for WxMaMessageRouterRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Java `Pattern.matches(regex, input)` 语义：整个字符串必须完全匹配。
/// 以 `^(?:...)$` 包裹实现（Java 隐式整串锚定；`regex` 本身含锚点时仍正确）。
fn regex_full_match(regex: &str, text: &str) -> bool {
    Regex::new(&format!("^(?:{regex})$")).is_ok_and(|re| re.is_match(text))
}
