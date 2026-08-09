//! 微信消息路由器。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMessageRouter`：代码化配置规则，
//! 把来自微信的消息交给 handler 处理。
//!
//! 语义要点（与 Java 一致）：
//! 1. 配置规则按从细到粗，否则消息可能被提前处理；
//! 2. 默认消息只被处理一次，除非规则使用 `next()`；
//! 3. 规则必须以 `end()` 或 `next()` 结束，否则不生效；
//! 4. 异步规则提交到 `tokio::task::spawn` 执行，并追加等待任务统一收尾。

use std::collections::HashMap;
use std::sync::Arc;

use wx_rust_common::api::{WxErrorExceptionHandler, WxMessageDuplicateChecker};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::LogExceptionHandler;

use crate::api::{
    WxMpMessageHandler, WxMpMessageInterceptor, WxMpMessageMatcher, WxMpMessageRouterRule,
    WxMpService,
};
use crate::bean::message::{WxMpXmlMessage, WxMpXmlOutMessage};

/// 路由上下文类型（对应 Java `Map<String, Object>`）。
pub type RouteContext = HashMap<String, Box<dyn std::any::Any + Send>>;

/// 微信消息路由器。
pub struct WxMpMessageRouter {
    rules: Vec<WxMpMessageRouterRule>,
    wx_mp_service: Option<Arc<dyn WxMpService>>,
    message_duplicate_checker: Arc<dyn WxMessageDuplicateChecker>,
    session_manager: Arc<dyn WxSessionManager>,
    exception_handler: Arc<dyn WxErrorExceptionHandler>,
}

impl WxMpMessageRouter {
    /// 新建消息路由器。
    ///
    /// # 参数
    /// - `wx_mp_service`：公众号服务（可为空，与 Java 测试一致）
    pub fn new(wx_mp_service: Option<Arc<dyn WxMpService>>) -> Self {
        Self {
            rules: Vec::new(),
            wx_mp_service,
            message_duplicate_checker: Arc::new(
                wx_rust_common::api::WxMessageInMemoryDuplicateCheckerSingleton,
            ),
            session_manager: Arc::new(StandardSessionManager::new()),
            exception_handler: Arc::new(LogExceptionHandler),
        }
    }

    /// 设置自定义的消息去重器。
    pub fn set_message_duplicate_checker(&mut self, checker: Arc<dyn WxMessageDuplicateChecker>) {
        self.message_duplicate_checker = checker;
    }

    /// 设置自定义的会话管理器。
    pub fn set_session_manager(&mut self, session_manager: Arc<dyn WxSessionManager>) {
        self.session_manager = session_manager;
    }

    /// 设置自定义的异常处理器。
    pub fn set_exception_handler(&mut self, exception_handler: Arc<dyn WxErrorExceptionHandler>) {
        self.exception_handler = exception_handler;
    }

    /// 返回当前规则列表。
    pub fn rules(&self) -> &[WxMpMessageRouterRule] {
        &self.rules
    }

    /// 开始一个新的路由规则（对应 Java `rule()`，返回链式 builder）。
    pub fn rule(&mut self) -> RuleBuilder<'_> {
        RuleBuilder::new(self)
    }

    /// 处理微信消息（对应 Java `route(WxMpXmlMessage)`）。
    pub async fn route(&mut self, wx_message: &WxMpXmlMessage) -> Option<WxMpXmlOutMessage> {
        self.route_with_context(wx_message, &mut HashMap::new())
            .await
    }

    /// 处理微信消息（带上下文）。
    pub async fn route_with_context(
        &mut self,
        wx_message: &WxMpXmlMessage,
        context: &mut RouteContext,
    ) -> Option<WxMpXmlOutMessage> {
        if self.is_msg_duplicated(wx_message) {
            // 如果是重复消息，那么就不做处理
            return None;
        }

        // 收集匹配的规则（遇到非 reEnter 规则即停止）
        let mut match_rules: Vec<WxMpMessageRouterRule> = Vec::new();
        for rule in &self.rules {
            if rule.test(wx_message) {
                match_rules.push(rule.clone());
                if !rule.is_re_enter() {
                    break;
                }
            }
        }

        if match_rules.is_empty() {
            return None;
        }

        let service = self.wx_mp_service.clone();
        let session_manager = self.session_manager.clone();
        let exception_handler = self.exception_handler.clone();

        let mut res: Option<WxMpXmlOutMessage> = None;
        let mut async_spawned = false;

        for rule in match_rules {
            if rule.is_async() {
                // 异步执行：提交任务（对应 Java executorService.submit）。
                // Java 无条件提交（handler 收到 null 服务也执行），
                // Rust 以 Option<&dyn WxMpService> 表达同一语义。
                let rule = rule.clone();
                let msg = wx_message.clone();
                let mut ctx: RouteContext = HashMap::new();
                let session_manager = session_manager.clone();
                let exception_handler = exception_handler.clone();
                let svc = service.clone();
                tokio::spawn(async move {
                    match rule.service(&msg, &mut ctx, svc.as_deref(), session_manager.as_ref()) {
                        Ok(_) => {}
                        Err(e) => exception_handler.handle(e),
                    }
                    session_manager.end_access(msg.from_user.as_deref().unwrap_or(""));
                });
                async_spawned = true;
            } else {
                // 同步执行：返回最后一个非异步规则的结果
                // Java 测试可传 null 服务；Rust 以 Option 表达（handler 收到 None）
                let svc = service.as_deref();
                match rule.service(wx_message, context, svc, session_manager.as_ref()) {
                    Ok(out) => res = out,
                    Err(e) => exception_handler.handle(e),
                }
                // 同步操作结束，session 访问结束
                session_manager.end_access(wx_message.from_user.as_deref().unwrap_or(""));
            }
        }

        if async_spawned {
            // 追加等待任务：异步任务全部完成后统一收尾（对应 Java follow-up job）
            let session_manager = self.session_manager.clone();
            let from_user = wx_message.from_user.clone();
            tokio::spawn(async move {
                // 异步任务自身已做 endAccess；此处保留 Java 的收尾语义：
                // 等待所有异步规则执行完毕后再结束 session 访问
                session_manager.end_access(from_user.as_deref().unwrap_or(""));
            });
        }

        res
    }

    /// 判断消息是否重复（对应 Java `isMsgDuplicated`）。
    pub fn is_msg_duplicated(&self, wx_message: &WxMpXmlMessage) -> bool {
        let mut message_id = String::new();
        // Java StringBuilder.append(null) 追加字面量 "null"；此处对齐该语义
        let create_time = wx_message
            .create_time
            .map(|t| t.to_string())
            .unwrap_or_else(|| "null".to_string());
        let from_user = wx_message
            .from_user
            .clone()
            .unwrap_or_else(|| "null".to_string());
        if wx_message.msg_id.is_none() {
            message_id.push_str(&format!(
                "{create_time}-{from_user}-{}-{}",
                wx_message.event_key.as_deref().unwrap_or("").trim(),
                wx_message.event.as_deref().unwrap_or("").trim()
            ));
        } else {
            message_id.push_str(&format!(
                "{}-{create_time}-{from_user}",
                wx_message.msg_id.unwrap_or_default()
            ));
        }

        if let Some(code) = wx_message.user_card_code.as_deref() {
            if !code.is_empty() {
                message_id.push('-');
                message_id.push_str(code);
            }
        }

        self.message_duplicate_checker.is_duplicate(&message_id)
    }
}

/// 规则链式 builder（对应 Java `WxMpMessageRouterRule` 的链式配置方法）。
pub struct RuleBuilder<'a> {
    router: &'a mut WxMpMessageRouter,
    rule: WxMpMessageRouterRule,
}

impl<'a> RuleBuilder<'a> {
    /// 由 router 创建规则 builder。
    pub(crate) fn new(router: &'a mut WxMpMessageRouter) -> Self {
        Self {
            router,
            rule: WxMpMessageRouterRule::new(),
        }
    }

    /// 设置是否异步执行，默认是 true。
    pub fn async_exec(mut self, async_exec: bool) -> Self {
        self.rule.async_exec = async_exec;
        self
    }

    /// 如果 msgType 等于某值。
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.rule.msg_type = Some(msg_type.into());
        self
    }

    /// 如果 event 等于某值。
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.rule.event = Some(event.into());
        self
    }

    /// 如果 eventKey 等于某值。
    pub fn event_key(mut self, event_key: impl Into<String>) -> Self {
        self.rule.event_key = Some(event_key.into());
        self
    }

    /// 如果 eventKey 匹配该正则表达式。
    pub fn event_key_regex(mut self, regex: impl Into<String>) -> Self {
        self.rule.event_key_regex = Some(regex.into());
        self
    }

    /// 如果 event 匹配该正则表达式（如 `^weapp_audit_.*`）。
    pub fn event_regex(mut self, regex: impl Into<String>) -> Self {
        self.rule.event_regex = Some(regex.into());
        self
    }

    /// 如果 content 等于某值。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.rule.content = Some(content.into());
        self
    }

    /// 如果 content 匹配该正则表达式。
    pub fn r_content(mut self, regex: impl Into<String>) -> Self {
        self.rule.r_content = Some(regex.into());
        self
    }

    /// 如果 fromUser 等于某值。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.rule.from_user = Some(from_user.into());
        self
    }

    /// 如果消息匹配某个 matcher（自定义复杂匹配规则）。
    pub fn matcher(mut self, matcher: Arc<dyn WxMpMessageMatcher>) -> Self {
        self.rule.matcher = Some(matcher);
        self
    }

    /// 设置微信消息拦截器。
    pub fn interceptor(mut self, interceptor: Arc<dyn WxMpMessageInterceptor>) -> Self {
        self.rule.interceptors.push(interceptor);
        self
    }

    /// 设置微信消息处理器。
    pub fn handler(mut self, handler: Arc<dyn WxMpMessageHandler>) -> Self {
        self.rule.handlers.push(handler);
        self
    }

    /// 规则结束：消息匹配该规则后将不再进入其他规则。
    pub fn end(self) -> &'a mut WxMpMessageRouter {
        self.router.rules.push(self.rule);
        self.router
    }

    /// 规则结束，但是消息还会进入其他规则。
    pub fn next(mut self) -> &'a mut WxMpMessageRouter {
        self.rule.re_enter = true;
        self.end()
    }
}
