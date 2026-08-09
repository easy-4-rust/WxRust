//! 小程序消息路由器。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaMessageRouter`：
//! 代码化配置规则，把来自微信的消息交给 handler 处理。
//!
//! 语义要点（与 Java 一致）：
//! 1. 配置规则按从细到粗，否则消息可能被提前处理；
//! 2. 默认消息只被处理一次，除非规则使用 `next()`；
//! 3. 规则必须以 `end()` 或 `next()` 结束，否则不生效；
//! 4. 异步规则提交到 `tokio::task::spawn` 执行（对应 Java 线程池），
//!    并追加等待任务统一收尾。

use std::collections::HashMap;
use std::sync::Arc;

use wx_rust_common::api::{
    WxErrorExceptionHandler, WxMessageDuplicateChecker, WxMessageInMemoryDuplicateCheckerSingleton,
};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::LogExceptionHandler;

use crate::api::WxMaService;
use crate::message::{
    WxMaMessage, WxMaMessageHandler, WxMaMessageInterceptor, WxMaMessageMatcher,
    WxMaMessageRouterRule, WxMaOutMessage,
};

/// 路由上下文类型（对应 Java `Map<String, Object>`）。
pub type RouteContext = HashMap<String, Box<dyn std::any::Any + Send>>;

/// 小程序消息路由器。
pub struct WxMaMessageRouter {
    /// 路由规则表（对应 Java `rules`）。
    rules: Vec<WxMaMessageRouterRule>,
    /// 小程序服务（对应 Java `wxMaService`；测试场景可为空）。
    wx_ma_service: Option<Arc<dyn WxMaService>>,
    /// 消息重复检查器（对应 Java `messageDuplicateChecker`）。
    message_duplicate_checker: Arc<dyn WxMessageDuplicateChecker>,
    /// 会话管理器（对应 Java `sessionManager`）。
    session_manager: Arc<dyn WxSessionManager>,
    /// 异常处理器（对应 Java `exceptionHandler`）。
    exception_handler: Arc<dyn WxErrorExceptionHandler>,
}

impl WxMaMessageRouter {
    /// 新建消息路由器。
    ///
    /// # 参数
    /// - `wx_ma_service`：小程序服务（可为 `None`，与 mp 镜像的测试语义一致）
    ///
    /// 默认使用进程内单例内存去重器、`StandardSessionManager` 与
    /// `LogExceptionHandler`（对应 Java 默认构造的组件）。
    pub fn new(wx_ma_service: Option<Arc<dyn WxMaService>>) -> Self {
        Self {
            rules: Vec::new(),
            wx_ma_service,
            message_duplicate_checker: Arc::new(WxMessageInMemoryDuplicateCheckerSingleton),
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

    /// 返回当前规则列表（对应 Java `getRules()`）。
    pub fn rules(&self) -> &[WxMaMessageRouterRule] {
        &self.rules
    }

    /// 开始一个新的路由规则（对应 Java `rule()`，返回链式 builder）。
    pub fn rule(&mut self) -> RuleBuilder<'_> {
        RuleBuilder::new(self)
    }

    /// 处理微信消息（对应 Java `route(WxMaMessage)`，默认空 context）。
    pub async fn route(
        &mut self,
        wx_message: &WxMaMessage,
    ) -> Option<Arc<dyn WxMaOutMessage + Send + Sync>> {
        self.route_with_context(wx_message, &mut HashMap::new())
            .await
    }

    /// 处理微信消息（带上下文，对应 Java `route(WxMaMessage, Map)`）。
    ///
    /// 1. 重复消息直接返回 `None`；
    /// 2. 收集匹配的规则（遇到非 reEnter 规则即停止）；
    /// 3. 依次执行：异步规则提交后台执行，同步规则返回最后一个结果；
    /// 4. 存在异步规则时追加等待任务统一收尾（对应 Java 的 follow-up job）。
    pub async fn route_with_context(
        &mut self,
        wx_message: &WxMaMessage,
        context: &mut RouteContext,
    ) -> Option<Arc<dyn WxMaOutMessage + Send + Sync>> {
        if self.is_msg_duplicated(wx_message) {
            // 如果是重复消息，那么就不做处理
            return None;
        }

        // 收集匹配的规则（遇到非 reEnter 规则即停止）
        let mut match_rules: Vec<WxMaMessageRouterRule> = Vec::new();
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

        let service = self.wx_ma_service.clone();
        let session_manager = self.session_manager.clone();
        let exception_handler = self.exception_handler.clone();

        let mut res: Option<Arc<dyn WxMaOutMessage + Send + Sync>> = None;
        let mut async_spawned = false;

        for rule in match_rules {
            if rule.is_async() {
                // 异步执行：提交任务（对应 Java executorService.submit）。
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
                    // 异步操作结束，session 访问结束
                    session_manager.end_access(msg.from_user.as_deref().unwrap_or(""));
                });
                async_spawned = true;
            } else {
                // 同步执行：返回最后一个非异步规则的结果
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
    ///
    /// 构造去重 id：有 `msgId` 时 `msgId-createTime-fromUser`，否则
    /// `createTime-fromUser-event(trimToEmpty)`；之后追加非空的
    /// `toUser`、`traceId`。Java `StringBuilder.append(null)` 追加
    /// 字面量 `"null"`，Rust 侧对齐该语义。
    pub fn is_msg_duplicated(&self, wx_message: &WxMaMessage) -> bool {
        let mut message_id = String::new();
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
                "{create_time}-{from_user}-{}",
                wx_message.event.as_deref().unwrap_or("").trim()
            ));
        } else {
            message_id.push_str(&format!(
                "{}-{create_time}-{from_user}",
                wx_message.msg_id.unwrap_or_default()
            ));
        }

        if let Some(to_user) = wx_message.to_user.as_deref() {
            if !to_user.is_empty() {
                message_id.push('-');
                message_id.push_str(to_user);
            }
        }

        if let Some(trace_id) = wx_message.trace_id.as_deref() {
            if !trace_id.is_empty() {
                message_id.push('-');
                message_id.push_str(trace_id);
            }
        }

        self.message_duplicate_checker.is_duplicate(&message_id)
    }
}

/// 规则链式 builder（对应 Java `WxMaMessageRouterRule` 的链式配置方法）。
pub struct RuleBuilder<'a> {
    router: &'a mut WxMaMessageRouter,
    rule: WxMaMessageRouterRule,
}

impl<'a> RuleBuilder<'a> {
    /// 由 router 创建规则 builder。
    pub(crate) fn new(router: &'a mut WxMaMessageRouter) -> Self {
        Self {
            router,
            rule: WxMaMessageRouterRule::new(),
        }
    }

    /// 设置是否异步执行，默认是 true（对应 Java `async(boolean)`）。
    pub fn async_exec(mut self, async_exec: bool) -> Self {
        self.rule.async_exec = async_exec;
        self
    }

    /// 如果 msgType 等于某值（对应 Java `msgType(String)`）。
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.rule.msg_type = Some(msg_type.into());
        self
    }

    /// 标题，发送小程序页卡时有效（对应 Java `title(String)`）。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.rule.title = Some(title.into());
        self
    }

    /// 如果 event 等于某值（对应 Java `event(String)`）。
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.rule.event = Some(event.into());
        self
    }

    /// 如果 eventKey 等于某值（对应 Java `eventKey(String)`；
    /// 注意 Java `test()` 不使用该字段）。
    pub fn event_key(mut self, event_key: impl Into<String>) -> Self {
        self.rule.event_key = Some(event_key.into());
        self
    }

    /// 如果 content 等于某值（对应 Java `content(String)`）。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.rule.content = Some(content.into());
        self
    }

    /// 如果 content 匹配该正则表达式（对应 Java `rContent(String)`）。
    pub fn r_content(mut self, regex: impl Into<String>) -> Self {
        self.rule.r_content = Some(regex.into());
        self
    }

    /// 如果 fromUser 等于某值（对应 Java `fromUser(String)`）。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.rule.from_user = Some(from_user.into());
        self
    }

    /// 如果消息匹配某个 matcher（对应 Java `matcher(WxMaMessageMatcher)`，
    /// 用在用户需要自定义更复杂的匹配规则的时候）。
    pub fn matcher(mut self, matcher: Arc<dyn WxMaMessageMatcher>) -> Self {
        self.rule.matcher = Some(matcher);
        self
    }

    /// 设置微信消息拦截器（对应 Java `interceptor(WxMaMessageInterceptor)`；
    /// 多次调用依次追加，等价于 Java 变参版本）。
    pub fn interceptor(mut self, interceptor: Arc<dyn WxMaMessageInterceptor>) -> Self {
        self.rule.interceptors.push(interceptor);
        self
    }

    /// 设置微信消息处理器（对应 Java `handler(WxMaMessageHandler)`；
    /// 多次调用依次追加，等价于 Java 变参版本）。
    pub fn handler(mut self, handler: Arc<dyn WxMaMessageHandler>) -> Self {
        self.rule.handlers.push(handler);
        self
    }

    /// 规则结束：消息匹配该规则后将不再进入其他规则（对应 Java `end()`）。
    pub fn end(self) -> &'a mut WxMaMessageRouter {
        self.router.rules.push(self.rule);
        self.router
    }

    /// 规则结束，但是消息还会进入其他规则（对应 Java `next()`）。
    pub fn next(mut self) -> &'a mut WxMaMessageRouter {
        self.rule.re_enter = true;
        self.end()
    }
}
