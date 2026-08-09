//! 企业微信第三方应用（tp）消息路由器。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.message.WxCpTpMessageRouter`：
//! 代码化配置规则，把来自服务商的消息交给 handler 处理。与
//! `WxCpMessageRouter` 相比多了 `infoType`/`changeType` 维度的匹配
//! （`WxCpTpMessageRouterRule`），且去重 id 按服务商消息维度拼接。
//!
//! 语义要点（与 Java 一致）：
//! 1. 配置路由规则时要按照从细到粗的原则，否则消息可能被提前处理；
//! 2. 默认情况下消息只会被处理一次，除非使用
//!    `WxCpTpMessageRouterRule#next()`（`TpRuleBuilder::next`）；
//! 3. 规则的结束必须用 `end()` 或 `next()`，否则不会生效；
//! 4. 异步规则提交到 `tokio::task::spawn` 执行（对应 Java 线程池，
//!   默认 100 线程），并追加等待任务统一收尾。
//!
//! 路由入口：`route(wxCpMessage)` / `route_with_context` /
//! `route_with_suite_id`（对应 Java `route(suiteId, WxCpTpXmlMessage, Map)`
//! 与 `route(WxCpTpXmlMessage, Map)`，后者 suiteId 为 null）。

use std::collections::HashMap;
use std::sync::Arc;

use wx_rust_common::api::{WxErrorExceptionHandler, WxMessageDuplicateChecker};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::LogExceptionHandler;

use crate::bean::message::{WxCpTpXmlMessage, WxCpXmlOutMessage};
use crate::message::RouteContext;
use crate::tp::message::{
    WxCpTpMessageHandler, WxCpTpMessageInterceptor, WxCpTpMessageMatcher, WxCpTpMessageRouterRule,
};
use crate::tp::service::WxCpTpService;

/// 企业微信第三方应用消息路由器。
pub struct WxCpTpMessageRouter {
    /// 路由规则表（对应 Java `rules`）。
    rules: Vec<WxCpTpMessageRouterRule>,
    /// 第三方应用服务（对应 Java `wxCpTpService`；测试场景可为空）。
    tp_service: Option<Arc<dyn WxCpTpService>>,
    /// 消息重复检查器（对应 Java `messageDuplicateChecker`，
    /// 默认 `WxMessageInMemoryDuplicateCheckerSingleton`）。
    message_duplicate_checker: Arc<dyn WxMessageDuplicateChecker>,
    /// 会话管理器（对应 Java `sessionManager`，默认取服务的
    /// `getSessionManager()`；服务为空时新建 `StandardSessionManager`）。
    session_manager: Arc<dyn WxSessionManager>,
    /// 异常处理器（对应 Java `exceptionHandler`，默认
    /// `LogExceptionHandler`）。
    exception_handler: Arc<dyn WxErrorExceptionHandler>,
}

impl WxCpTpMessageRouter {
    /// 新建消息路由器（对应 Java 构造 `WxCpTpMessageRouter(WxCpTpService)`；
    /// 测试场景可传 `None`，与 Java 测试 `new WxCpTpMessageRouter(null)`
    /// 的语义一致）。
    pub fn new(tp_service: Option<Arc<dyn WxCpTpService>>) -> Self {
        let session_manager = match &tp_service {
            Some(svc) => svc.session_manager(),
            None => Arc::new(StandardSessionManager::new()) as Arc<dyn WxSessionManager>,
        };
        Self {
            rules: Vec::new(),
            tp_service,
            message_duplicate_checker: Arc::new(
                wx_rust_common::api::WxMessageInMemoryDuplicateCheckerSingleton,
            ),
            session_manager,
            exception_handler: Arc::new(LogExceptionHandler),
        }
    }

    /// 设置自定义的消息去重器（对应 Java `setMessageDuplicateChecker`）。
    pub fn set_message_duplicate_checker(&mut self, checker: Arc<dyn WxMessageDuplicateChecker>) {
        self.message_duplicate_checker = checker;
    }

    /// 设置自定义的会话管理器（对应 Java `setSessionManager`）。
    pub fn set_session_manager(&mut self, session_manager: Arc<dyn WxSessionManager>) {
        self.session_manager = session_manager;
    }

    /// 设置自定义的异常处理器（对应 Java `setExceptionHandler`）。
    pub fn set_exception_handler(&mut self, exception_handler: Arc<dyn WxErrorExceptionHandler>) {
        self.exception_handler = exception_handler;
    }

    /// 返回当前规则列表（对应 Java `getRules()`）。
    pub fn rules(&self) -> &[WxCpTpMessageRouterRule] {
        &self.rules
    }

    /// 开始一个新的路由规则（对应 Java `rule()`，返回链式 builder）。
    pub fn rule(&mut self) -> TpRuleBuilder<'_> {
        TpRuleBuilder::new(self)
    }

    /// 处理服务商推送的消息（对应 Java `route(WxCpTpXmlMessage)`，
    /// suiteId 为 null + 默认空 context）。
    pub async fn route(&mut self, wx_message: &WxCpTpXmlMessage) -> Option<WxCpXmlOutMessage> {
        self.route_with_suite_id(None, wx_message, &mut HashMap::new())
            .await
    }

    /// 处理服务商推送的消息（带上下文，对应 Java
    /// `route(WxCpTpXmlMessage, Map)`，suiteId 为 null）。
    pub async fn route_with_context(
        &mut self,
        wx_message: &WxCpTpXmlMessage,
        context: &mut RouteContext,
    ) -> Option<WxCpXmlOutMessage> {
        self.route_with_suite_id(None, wx_message, context).await
    }

    /// 处理服务商推送的消息（带 suiteId 与上下文，对应 Java
    /// `route(String suiteId, WxCpTpXmlMessage, Map)`）。
    ///
    /// 1. 重复消息直接返回 `None`；
    /// 2. 收集匹配的规则（遇到非 reEnter 规则即停止）；
    /// 3. 依次执行：异步规则提交后台执行，同步规则返回最后一个结果；
    /// 4. 存在异步规则时追加等待任务统一收尾（对应 Java 的 follow-up job）。
    pub async fn route_with_suite_id(
        &mut self,
        suite_id: Option<&str>,
        wx_message: &WxCpTpXmlMessage,
        context: &mut RouteContext,
    ) -> Option<WxCpXmlOutMessage> {
        if self.is_msg_duplicated(suite_id, wx_message) {
            // 如果是重复消息，那么就不做处理
            return None;
        }

        // 收集匹配的规则（遇到非 reEnter 规则即停止）
        let mut match_rules: Vec<WxCpTpMessageRouterRule> = Vec::new();
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

        let service = self.tp_service.clone();
        let session_manager = self.session_manager.clone();
        let exception_handler = self.exception_handler.clone();

        let mut res: Option<WxCpXmlOutMessage> = None;
        let mut async_spawned = false;

        for rule in match_rules {
            if rule.is_async() {
                // 异步执行：提交任务（对应 Java executorService.submit）
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
                    session_manager.end_access(msg.suite_id.as_deref().unwrap_or(""));
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
                session_manager.end_access(wx_message.suite_id.as_deref().unwrap_or(""));
            }
        }

        if async_spawned {
            // 追加等待任务：异步任务全部完成后统一收尾（对应 Java follow-up job）
            let session_manager = self.session_manager.clone();
            let suite_id = wx_message.suite_id.clone();
            tokio::spawn(async move {
                // 异步任务自身已做 endAccess；此处保留 Java 的收尾语义
                session_manager.end_access(suite_id.as_deref().unwrap_or(""));
            });
        }

        res
    }

    /// 判断消息是否重复（对应 Java `isMsgDuplicated`）。
    ///
    /// 构造去重 id（镜像 Java StringBuilder 拼接语义）：
    /// - 前缀固定为 `toUserName`；
    /// - `infoType` 非空时追加
    ///   `infoType-suiteId(trimToEmpty)-timeStamp-authCorpId(trimToEmpty)-
    ///   userID(trimToEmpty)-changeType(trimToEmpty)-serviceCorpId(trimToEmpty)-
    ///   externalUserID(trimToEmpty)`；
    /// - 否则若 suiteId 非空白追加 suiteId；
    /// - `msgType` 非空时：有 `msgId` 追加
    ///   `msgId-createTime-fromUserName`，否则追加
    ///   `msgType-createTime-fromUserName-event(trimToEmpty)-
    ///   eventKey(trimToEmpty)-externalUserID(trimToEmpty)`。
    pub fn is_msg_duplicated(&self, suite_id: Option<&str>, wx_message: &WxCpTpXmlMessage) -> bool {
        let mut message_id = String::new();
        // Java StringBuilder.append(null) 追加字面量 "null"，Rust 侧对齐
        message_id.push_str(wx_message.to_user_name.as_deref().unwrap_or("null"));

        if let Some(info_type) = wx_message.info_type.as_deref() {
            let trim_to_empty = |v: &Option<String>| v.as_deref().unwrap_or("").trim().to_string();
            message_id.push_str(info_type);
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.suite_id));
            message_id.push('-');
            message_id.push_str(wx_message.time_stamp.as_deref().unwrap_or("null"));
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.auth_corp_id));
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.user_id));
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.change_type));
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.service_corp_id));
            message_id.push('-');
            message_id.push_str(&trim_to_empty(&wx_message.external_user_id));
        } else if suite_id.is_some_and(|s| !s.trim().is_empty()) {
            message_id.push_str(suite_id.unwrap_or_default());
        }

        if let Some(msg_type) = wx_message.msg_type.as_deref() {
            let trim_to_empty = |v: &Option<String>| v.as_deref().unwrap_or("").trim().to_string();
            if let Some(msg_id) = wx_message.msg_id.as_deref() {
                message_id.push_str(msg_id);
                message_id.push('-');
                message_id.push_str(
                    &wx_message
                        .create_time
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "null".to_string()),
                );
                message_id.push('-');
                message_id.push_str(wx_message.from_user_name.as_deref().unwrap_or("null"));
            } else {
                message_id.push_str(msg_type);
                message_id.push('-');
                message_id.push_str(
                    &wx_message
                        .create_time
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "null".to_string()),
                );
                message_id.push('-');
                message_id.push_str(wx_message.from_user_name.as_deref().unwrap_or("null"));
                message_id.push('-');
                message_id.push_str(&trim_to_empty(&wx_message.event));
                message_id.push('-');
                message_id.push_str(&trim_to_empty(&wx_message.event_key));
                message_id.push('-');
                message_id.push_str(&trim_to_empty(&wx_message.external_user_id));
            }
        }

        self.message_duplicate_checker.is_duplicate(&message_id)
    }
}

/// 规则链式 builder（对应 Java `WxCpTpMessageRouterRule` 的链式配置方法）。
pub struct TpRuleBuilder<'a> {
    router: &'a mut WxCpTpMessageRouter,
    rule: WxCpTpMessageRouterRule,
}

impl<'a> TpRuleBuilder<'a> {
    /// 由 router 创建规则 builder。
    pub(crate) fn new(router: &'a mut WxCpTpMessageRouter) -> Self {
        Self {
            router,
            rule: WxCpTpMessageRouterRule::new(),
        }
    }

    /// 设置是否异步执行，默认是 true（对应 Java `async(boolean)`）。
    pub fn async_exec(mut self, async_exec: bool) -> Self {
        self.rule.async_exec = async_exec;
        self
    }

    /// 如果 agentId 匹配（对应 Java `agentId(Integer)`）。
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.rule.agent_id = Some(agent_id);
        self
    }

    /// 如果 msgType 等于某值（对应 Java `msgType(String)`）。
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.rule.msg_type = Some(msg_type.into());
        self
    }

    /// 如果 event 等于某值（对应 Java `event(String)`）。
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.rule.event = Some(event.into());
        self
    }

    /// 如果 eventKey 等于某值（对应 Java `eventKey(String)`）。
    pub fn event_key(mut self, event_key: impl Into<String>) -> Self {
        self.rule.event_key = Some(event_key.into());
        self
    }

    /// 如果 eventKey 匹配该正则表达式（对应 Java `eventKeyRegex(String)`）。
    pub fn event_key_regex(mut self, regex: impl Into<String>) -> Self {
        self.rule.event_key_regex = Some(regex.into());
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

    /// 匹配服务商推送消息的 InfoType（对应 Java `infoType(String)`）。
    pub fn info_type(mut self, info_type: impl Into<String>) -> Self {
        self.rule.info_type = Some(info_type.into());
        self
    }

    /// 匹配通讯录变更的 ChangeType（对应 Java `changeType(String)`）。
    pub fn change_type(mut self, change_type: impl Into<String>) -> Self {
        self.rule.change_type = Some(change_type.into());
        self
    }

    /// 匹配 suiteId（对应 Java `suiteId(String)`）。
    pub fn suite_id(mut self, suite_id: impl Into<String>) -> Self {
        self.rule.suite_id = Some(suite_id.into());
        self
    }

    /// 匹配授权码（对应 Java `authCode(String)`）。
    pub fn auth_code(mut self, auth_code: impl Into<String>) -> Self {
        self.rule.auth_code = Some(auth_code.into());
        self
    }

    /// 匹配 suiteTicket（对应 Java `suiteTicket(String)`）。
    pub fn suite_ticket(mut self, suite_ticket: impl Into<String>) -> Self {
        self.rule.suite_ticket = Some(suite_ticket.into());
        self
    }

    /// 如果消息匹配某个 matcher（对应 Java
    /// `matcher(WxCpTpMessageMatcher)`，用在用户需要自定义更复杂的匹配
    /// 规则的时候）。
    pub fn matcher(mut self, matcher: Arc<dyn WxCpTpMessageMatcher>) -> Self {
        self.rule.matcher = Some(matcher);
        self
    }

    /// 设置消息拦截器（对应 Java `interceptor(WxCpTpMessageInterceptor)`；
    /// 多次调用依次追加，等价于 Java 变参版本）。
    pub fn interceptor(mut self, interceptor: Arc<dyn WxCpTpMessageInterceptor>) -> Self {
        self.rule.interceptors.push(interceptor);
        self
    }

    /// 设置消息处理器（对应 Java `handler(WxCpTpMessageHandler)`；
    /// 多次调用依次追加，等价于 Java 变参版本）。
    pub fn handler(mut self, handler: Arc<dyn WxCpTpMessageHandler>) -> Self {
        self.rule.handlers.push(handler);
        self
    }

    /// 规则结束：消息匹配该规则后将不再进入其他规则（对应 Java `end()`）。
    pub fn end(self) -> &'a mut WxCpTpMessageRouter {
        self.router.rules.push(self.rule);
        self.router
    }

    /// 规则结束，但是消息还会进入其他规则（对应 Java `next()`）。
    pub fn next(mut self) -> &'a mut WxCpTpMessageRouter {
        self.rule.re_enter = true;
        self.end()
    }
}
