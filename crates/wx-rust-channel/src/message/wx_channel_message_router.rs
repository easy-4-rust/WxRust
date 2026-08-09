//! 消息路由器。
//!
//! 对应 Java `me.chanjar.weixin.channel.message.WxChannelMessageRouter`：
//! 代码化配置规则，把来自微信的消息交给 handler 处理。
//!
//! 语义要点（与 Java 一致，参考 miniapp `WxMaMessageRouter` 模式）：
//! 1. 规则按从细到粗配置，否则消息可能被提前处理；
//! 2. 默认消息只被处理一次，除非规则 `next()`；
//! 3. 异步规则提交 `tokio::spawn` 执行（对应 Java 线程池
//!    `executorService.submit`），同步规则返回最后一个结果；
//! 4. 重复消息直接跳过（默认进程内单例内存去重器）。

use std::collections::HashMap;
use std::sync::Arc;

use wx_rust_common::api::{
    WxErrorExceptionHandler, WxMessageDuplicateChecker, WxMessageInMemoryDuplicateCheckerSingleton,
};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::LogExceptionHandler;

use crate::api::WxChannelService;
use crate::message::rule::{
    WxChannelMessageHandler, WxChannelMessageInterceptor, WxChannelMessageMatcher,
};
use crate::message::{
    RouteContext, WxChannelMessage, WxChannelMessageLike, WxChannelMessageRouterRule,
    WxChannelMessageRouterRuleErased,
};

/// 消息路由器。
pub struct WxChannelMessageRouter {
    /// 规则列表（对应 Java `rules`）。
    rules: Vec<Arc<dyn WxChannelMessageRouterRuleErased>>,
    /// 异常处理器（对应 Java `exceptionHandler`，默认 `LogExceptionHandler`）。
    exception_handler: Arc<dyn WxErrorExceptionHandler>,
    /// 消息重复检查器（对应 Java `messageDuplicateChecker`，
    /// 默认 `WxMessageInMemoryDuplicateCheckerSingleton`）。
    message_duplicate_checker: Arc<dyn WxMessageDuplicateChecker>,
}

impl WxChannelMessageRouter {
    /// 新建消息路由器。
    ///
    /// 默认使用进程内单例内存去重器与 `LogExceptionHandler`
    /// （对应 Java 默认构造的组件；Java 的线程池以 `tokio::spawn` 表达）。
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            exception_handler: Arc::new(LogExceptionHandler),
            message_duplicate_checker: Arc::new(WxMessageInMemoryDuplicateCheckerSingleton),
        }
    }

    /// 设置自定义的异常处理器。
    pub fn set_exception_handler(&mut self, exception_handler: Arc<dyn WxErrorExceptionHandler>) {
        self.exception_handler = exception_handler;
    }

    /// 设置自定义的消息去重器。
    pub fn set_message_duplicate_checker(&mut self, checker: Arc<dyn WxMessageDuplicateChecker>) {
        self.message_duplicate_checker = checker;
    }

    /// 返回当前规则列表（对应 Java `getRules()`）。
    pub fn rules(&self) -> &[Arc<dyn WxChannelMessageRouterRuleErased>] {
        &self.rules
    }

    /// 返回当前规则列表（可变，对应 Java `getRules()` 后 `add`）。
    pub fn rules_mut(&mut self) -> &mut Vec<Arc<dyn WxChannelMessageRouterRuleErased>> {
        &mut self.rules
    }

    /// 添加一条规则进入路由器（对应 Java `addRule`）。
    pub fn add_rule(&mut self, rule: Arc<dyn WxChannelMessageRouterRuleErased>) {
        self.rules.push(rule);
    }

    /// 开始一个新的路由规则（链式 builder）。
    pub fn rule<T: WxChannelMessageLike>(&mut self) -> RuleBuilder<'_, T> {
        RuleBuilder::new(self)
    }

    /// 消息路由入口（对应 Java `route(WxChannelMessage, String, String, WxChannelService)`）。
    ///
    /// 内部创建空 context 与新的 `StandardSessionManager`（Java 同款：
    /// `new HashMap<>(2)` + `new StandardSessionManager()`）。
    pub async fn route(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        service: Option<Arc<dyn WxChannelService>>,
    ) -> Option<String> {
        let mut context: RouteContext = HashMap::new();
        let session_manager: Arc<dyn WxSessionManager> = Arc::new(StandardSessionManager::new());
        self.route_with_context(
            message,
            content,
            app_id,
            &mut context,
            service,
            session_manager,
        )
        .await
    }

    /// 路由微信消息（带上下文与自定义会话管理器，对应 Java 6 参 `route`）。
    ///
    /// 1. 重复消息直接返回 `None`；
    /// 2. 收集匹配的规则（遇到非 `next` 规则即停止）；
    /// 3. 依次执行：异步规则 `tokio::spawn` 提交后台执行，同步规则返回
    ///    最后一个结果；
    /// 4. 异步规则使用独立的 context（`Box<dyn Any>` 无法跨任务共享，ADAPTED：
    ///    Java 共享同一 Map）。
    pub async fn route_with_context(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        context: &mut RouteContext,
        service: Option<Arc<dyn WxChannelService>>,
        session_manager: Arc<dyn WxSessionManager>,
    ) -> Option<String> {
        // 如果是重复消息，那么就不做处理
        if self.is_msg_duplicated(message) {
            return None;
        }

        // 收集匹配的规则（遇到非 next 规则即停止）
        let mut match_rules: Vec<Arc<dyn WxChannelMessageRouterRuleErased>> = Vec::new();
        for rule in &self.rules {
            if rule.rule_is_match(message) {
                match_rules.push(rule.clone());
                if !rule.rule_is_next() {
                    break;
                }
            }
        }

        if match_rules.is_empty() {
            return None;
        }

        let mut result: Option<String> = None;
        for rule in match_rules {
            // 返回最后一个非异步的rule的执行结果
            if rule.rule_is_async() {
                // 异步执行：提交任务（对应 Java executorService.submit）
                let rule = rule.clone();
                let message = message.clone();
                let content = content.to_string();
                let app_id = app_id.to_string();
                let mut ctx: RouteContext = HashMap::new();
                let session_manager = session_manager.clone();
                let exception_handler = self.exception_handler.clone();
                let service = service.clone();
                tokio::spawn(async move {
                    rule.rule_process(
                        &message,
                        &content,
                        &app_id,
                        &mut ctx,
                        service.as_deref(),
                        session_manager.as_ref(),
                        exception_handler.as_ref(),
                    );
                    // 异步操作结束，session 访问结束
                    session_manager.end_access(message.from_user.as_deref().unwrap_or(""));
                });
            } else {
                // 同步执行：返回最后一个非异步规则的结果
                result = rule.rule_process(
                    message,
                    content,
                    app_id,
                    context,
                    service.as_deref(),
                    session_manager.as_ref(),
                    self.exception_handler.as_ref(),
                );
                // 在同步操作结束，session访问结束
                session_manager.end_access(message.from_user.as_deref().unwrap_or(""));
            }
        }
        result
    }

    /// 判断消息是否重复（对应 Java `isMsgDuplicated`）。
    pub fn is_msg_duplicated(&self, message: &WxChannelMessage) -> bool {
        let message_id = self.generate_message_id(message);
        self.message_duplicate_checker.is_duplicate(&message_id)
    }

    /// 生成消息id（对应 Java `generateMessageId`）。
    ///
    /// `msgId` 为空时 `createTime-fromUser-event(trimToEmpty)`，否则
    /// `msgId-createTime-fromUser`；之后追加非空的 `toUser`。
    /// Java `StringBuilder.append(null)` 追加字面量 `"null"`，Rust 侧对齐该语义。
    pub fn generate_message_id(&self, message: &WxChannelMessage) -> String {
        let mut sb = String::new();
        let create_time = message
            .create_time
            .map(|t| t.to_string())
            .unwrap_or_else(|| "null".to_string());
        let from_user = message
            .from_user
            .clone()
            .unwrap_or_else(|| "null".to_string());

        if message.msg_id.is_none() {
            sb.push_str(&create_time);
            sb.push('-');
            sb.push_str(&from_user);
            sb.push('-');
            sb.push_str(message.event.as_deref().unwrap_or("").trim());
        } else {
            sb.push_str(&message.msg_id.unwrap_or_default().to_string());
            sb.push('-');
            sb.push_str(&create_time);
            sb.push('-');
            sb.push_str(&from_user);
        }

        if let Some(to_user) = message.to_user.as_deref() {
            if !to_user.is_empty() {
                sb.push('-');
                sb.push_str(to_user);
            }
        }
        sb
    }
}

impl Default for WxChannelMessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// 规则链式 builder（对应 Java `WxChannelMessageRouterRule` 链式 setter）。
pub struct RuleBuilder<'a, T: WxChannelMessageLike> {
    router: &'a mut WxChannelMessageRouter,
    rule: WxChannelMessageRouterRule<T>,
}

impl<'a, T: WxChannelMessageLike> RuleBuilder<'a, T> {
    /// 由 router 创建规则 builder。
    pub(crate) fn new(router: &'a mut WxChannelMessageRouter) -> Self {
        Self {
            router,
            rule: WxChannelMessageRouterRule::new(),
        }
    }

    /// 设置是否异步执行，默认是 true（对应 Java `async(boolean)`）。
    pub fn async_exec(mut self, async_exec: bool) -> Self {
        self.rule.async_exec = async_exec;
        self
    }

    /// 设置消息类型（对应 Java `msgType(String)`）。
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.rule.msg_type = Some(msg_type.into());
        self
    }

    /// 设置事件（同时把 msgType 置为 "event"，对应 Java `setEvent(String)`）。
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.rule.set_event(event);
        self
    }

    /// 自定义匹配器（对应 Java `matcher(WxChannelMessageMatcher)`）。
    pub fn matcher(mut self, matcher: Arc<dyn WxChannelMessageMatcher>) -> Self {
        self.rule.matcher = Some(matcher);
        self
    }

    /// 设置微信消息拦截器（对应 Java `interceptor(WxChannelMessageInterceptor)`；
    /// 多次调用依次追加）。
    pub fn interceptor(mut self, interceptor: Arc<dyn WxChannelMessageInterceptor>) -> Self {
        self.rule.interceptors.push(interceptor);
        self
    }

    /// 设置微信消息处理器（对应 Java `handler(WxChannelMessageHandler)`；
    /// 多次调用依次追加，返回最后 handler 的结果）。
    pub fn handler(mut self, handler: Arc<dyn WxChannelMessageHandler<T>>) -> Self {
        self.rule.handlers.push(handler);
        self
    }

    /// 规则结束：消息匹配该规则后将不再进入其他规则（对应 Java `end()`）。
    pub fn end(self) -> &'a mut WxChannelMessageRouter {
        self.router.rules.push(Arc::new(self.rule));
        self.router
    }

    /// 规则结束，但是消息还会进入其他规则（对应 Java `next()`）。
    pub fn next(mut self) -> &'a mut WxChannelMessageRouter {
        self.rule.next = true;
        self.end()
    }
}
