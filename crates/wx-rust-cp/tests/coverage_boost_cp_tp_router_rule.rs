//! Coverage boost: `wx_cp_tp_message_router_rule.rs` (120 lines, 0% covered)
//! and `wx_cp_tp_message_router.rs` (241 lines, 0% covered).
//!
//! Exercises:
//! - `WxCpTpMessageRouterRule::new()` / `Default` / field defaults
//! - `test()` matching logic: suiteId, fromUser, agentId, msgType, event,
//!   infoType, suiteTicket, eventKeyRegex, content, rContent, changeType,
//!   authCode, matcher
//! - `service()` with handlers and interceptors
//! - `WxCpTpMessageRouter::new()` / rule / route

use std::sync::Arc;

use wx_rust_cp::bean::message::WxCpTpXmlMessage;
use wx_rust_cp::message::RouteContext;
use wx_rust_cp::tp::message::{
    WxCpTpMessageHandler, WxCpTpMessageInterceptor, WxCpTpMessageMatcher, WxCpTpMessageRouter,
    WxCpTpMessageRouterRule,
};

fn sample_message() -> WxCpTpXmlMessage {
    let mut m = WxCpTpXmlMessage::default();
    m.suite_id = Some("suite_123".into());
    m.from_user_name = Some("user_a".into());
    m.agent_id = Some("1000002".into());
    m.msg_type = Some("text".into());
    m.event = Some("subscribe".into());
    m.info_type = Some("change_auth".into());
    m.change_type = Some("add_authorized".into());
    m.suite_ticket = Some("ticket_xyz".into());
    m.auth_code = Some("auth_code_1".into());
    m.event_key = Some("KEY_123".into());
    m.content = Some("  hello world  ".into());
    m
}

// ---- new / default ----

#[test]
fn rule_new_defaults() {
    let r = WxCpTpMessageRouterRule::new();
    assert!(r.async_exec);
    assert!(!r.re_enter);
    assert!(r.from_user.is_none());
    assert!(r.handlers.is_empty());
    assert!(r.interceptors.is_empty());
}

#[test]
fn rule_default_same_as_new() {
    let r = WxCpTpMessageRouterRule::default();
    assert!(r.async_exec);
    assert!(!r.re_enter);
}

#[test]
fn rule_helpers() {
    let r = WxCpTpMessageRouterRule::new();
    assert!(r.is_async());
    assert!(!r.is_re_enter());
    let mut r2 = WxCpTpMessageRouterRule::new();
    r2.re_enter = true;
    assert!(r2.is_re_enter());
}

// ---- test() matching ----

#[test]
fn test_suite_id_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.suite_id = Some("suite_123".into());
    assert!(r.test(&m));
}

#[test]
fn test_suite_id_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.suite_id = Some("other_suite".into());
    assert!(!r.test(&m));
}

#[test]
fn test_from_user_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.from_user = Some("user_a".into());
    assert!(r.test(&m));
}

#[test]
fn test_from_user_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.from_user = Some("user_b".into());
    assert!(!r.test(&m));
}

#[test]
fn test_agent_id_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.agent_id = Some(1000002);
    assert!(r.test(&m));
}

#[test]
fn test_agent_id_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.agent_id = Some(9999999);
    assert!(!r.test(&m));
}

#[test]
fn test_agent_id_invalid_in_message() {
    let m = WxCpTpXmlMessage {
        agent_id: Some("not_a_number".into()),
        ..Default::default()
    };
    let mut r = WxCpTpMessageRouterRule::new();
    r.agent_id = Some(1);
    assert!(!r.test(&m));
}

#[test]
fn test_msg_type_case_insensitive() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.msg_type = Some("TEXT".into());
    assert!(r.test(&m));
}

#[test]
fn test_msg_type_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.msg_type = Some("image".into());
    assert!(!r.test(&m));
}

#[test]
fn test_event_case_insensitive() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.event = Some("SUBSCRIBE".into());
    assert!(r.test(&m));
}

#[test]
fn test_info_type_exact() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.info_type = Some("change_auth".into());
    assert!(r.test(&m));
}

#[test]
fn test_info_type_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.info_type = Some("create_auth".into());
    assert!(!r.test(&m));
}

#[test]
fn test_change_type_exact() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.change_type = Some("add_authorized".into());
    assert!(r.test(&m));
}

#[test]
fn test_suite_ticket_case_insensitive() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.suite_ticket = Some("TICKET_XYZ".into());
    assert!(r.test(&m));
}

#[test]
fn test_auth_code_case_insensitive() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.auth_code = Some("AUTH_CODE_1".into());
    assert!(r.test(&m));
}

#[test]
fn test_auth_code_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.auth_code = Some("wrong".into());
    assert!(!r.test(&m));
}

#[test]
fn test_event_key_regex_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.event_key_regex = Some(r"KEY_\d+".into());
    assert!(r.test(&m));
}

#[test]
fn test_event_key_regex_no_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.event_key_regex = Some(r"NOPE_\d+".into());
    assert!(!r.test(&m));
}

#[test]
fn test_event_key_regex_empty_key() {
    let m = WxCpTpXmlMessage {
        event_key: Some("".into()),
        ..Default::default()
    };
    let mut r = WxCpTpMessageRouterRule::new();
    r.event_key_regex = Some(r".*".into());
    assert!(r.test(&m));
}

#[test]
fn test_content_trim_match() {
    let m = sample_message(); // content = "  hello world  " → trim = "hello world"
    let mut r = WxCpTpMessageRouterRule::new();
    r.content = Some("hello world".into());
    assert!(r.test(&m));
}

#[test]
fn test_content_mismatch() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.content = Some("goodbye".into());
    assert!(!r.test(&m));
}

#[test]
fn test_content_none_in_message() {
    let m = WxCpTpXmlMessage::default();
    let mut r = WxCpTpMessageRouterRule::new();
    r.content = Some("anything".into());
    assert!(!r.test(&m));
}

#[test]
fn test_r_content_regex() {
    let m = sample_message(); // trimmed content = "hello world"
    let mut r = WxCpTpMessageRouterRule::new();
    r.r_content = Some(r"hello \w+".into());
    assert!(r.test(&m));
}

#[test]
fn test_r_content_no_match() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.r_content = Some(r"goodbye.*".into());
    assert!(!r.test(&m));
}

#[test]
fn test_all_conditions_combined() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.suite_id = Some("suite_123".into());
    r.from_user = Some("user_a".into());
    r.msg_type = Some("text".into());
    r.info_type = Some("change_auth".into());
    r.change_type = Some("add_authorized".into());
    assert!(r.test(&m));
}

#[test]
fn test_one_condition_fails_all_fail() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.suite_id = Some("suite_123".into());
    r.from_user = Some("user_a".into());
    r.msg_type = Some("image".into()); // mismatch
    assert!(!r.test(&m));
}

#[test]
fn test_no_conditions_always_matches() {
    let m = sample_message();
    let r = WxCpTpMessageRouterRule::new();
    assert!(r.test(&m));
}

// ---- custom matcher ----

struct AlwaysMatcher;
impl WxCpTpMessageMatcher for AlwaysMatcher {
    fn match_message(&self, _: &WxCpTpXmlMessage) -> bool {
        true
    }
}

struct NeverMatcher;
impl WxCpTpMessageMatcher for NeverMatcher {
    fn match_message(&self, _: &WxCpTpXmlMessage) -> bool {
        false
    }
}

#[test]
fn test_custom_matcher_true() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.matcher = Some(Arc::new(AlwaysMatcher));
    assert!(r.test(&m));
}

#[test]
fn test_custom_matcher_false() {
    let m = sample_message();
    let mut r = WxCpTpMessageRouterRule::new();
    r.matcher = Some(Arc::new(NeverMatcher));
    assert!(!r.test(&m));
}

// ---- service() with handlers and interceptors ----

struct TestHandler {
    pub called: std::sync::atomic::AtomicBool,
}

impl WxCpTpMessageHandler for TestHandler {
    fn handle(
        &self,
        _message: &WxCpTpXmlMessage,
        _context: &mut RouteContext,
        _service: Option<&dyn wx_rust_cp::tp::service::WxCpTpService>,
        _session_manager: &dyn wx_rust_common::session::WxSessionManager,
    ) -> Result<
        Option<wx_rust_cp::bean::message::WxCpXmlOutMessage>,
        wx_rust_common::error::WxErrorException,
    > {
        self.called.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(None)
    }
}

struct PassInterceptor;
impl WxCpTpMessageInterceptor for PassInterceptor {
    fn intercept(
        &self,
        _: &WxCpTpXmlMessage,
        _: &mut RouteContext,
        _: Option<&dyn wx_rust_cp::tp::service::WxCpTpService>,
        _: &dyn wx_rust_common::session::WxSessionManager,
    ) -> bool {
        true
    }
}

struct BlockInterceptor;
impl WxCpTpMessageInterceptor for BlockInterceptor {
    fn intercept(
        &self,
        _: &WxCpTpXmlMessage,
        _: &mut RouteContext,
        _: Option<&dyn wx_rust_cp::tp::service::WxCpTpService>,
        _: &dyn wx_rust_common::session::WxSessionManager,
    ) -> bool {
        false
    }
}

#[test]
fn service_with_handler_executes() {
    let handler = Arc::new(TestHandler {
        called: std::sync::atomic::AtomicBool::new(false),
    });
    let mut rule = WxCpTpMessageRouterRule::new();
    rule.handlers.push(handler.clone());

    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let session = wx_rust_common::session::StandardSessionManager::new();

    let result = rule.service(&msg, &mut ctx, None, &session).unwrap();
    assert!(result.is_none());
    assert!(handler.called.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn service_with_passing_interceptor() {
    let handler = Arc::new(TestHandler {
        called: std::sync::atomic::AtomicBool::new(false),
    });
    let mut rule = WxCpTpMessageRouterRule::new();
    rule.interceptors.push(Arc::new(PassInterceptor));
    rule.handlers.push(handler.clone());

    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let session = wx_rust_common::session::StandardSessionManager::new();

    let _ = rule.service(&msg, &mut ctx, None, &session).unwrap();
    assert!(handler.called.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn service_with_blocking_interceptor() {
    let handler = Arc::new(TestHandler {
        called: std::sync::atomic::AtomicBool::new(false),
    });
    let mut rule = WxCpTpMessageRouterRule::new();
    rule.interceptors.push(Arc::new(BlockInterceptor));
    rule.handlers.push(handler.clone());

    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let session = wx_rust_common::session::StandardSessionManager::new();

    let result = rule.service(&msg, &mut ctx, None, &session).unwrap();
    assert!(result.is_none());
    assert!(!handler.called.load(std::sync::atomic::Ordering::SeqCst));
}

// ---- WxCpTpMessageRouter ----

#[test]
fn router_new_with_none() {
    let router = WxCpTpMessageRouter::new(None);
    // Should not panic
    let _ = router;
}

#[test]
fn router_rule_builder() {
    let mut router = WxCpTpMessageRouter::new(None);
    router.rule().info_type("change_auth").end();
    // At least one rule should be registered
}

#[tokio::test]
async fn router_route_with_none_service() {
    let mut router = WxCpTpMessageRouter::new(None);
    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let result = router.route_with_context(&msg, &mut ctx).await;
    // No rules → no match → None
    assert!(result.is_none());
}

#[tokio::test]
async fn router_route_matching_rule() {
    let mut router = WxCpTpMessageRouter::new(None);
    router
        .rule()
        .msg_type("text")
        .async_exec(false)
        .handler(Arc::new(TestHandler {
            called: std::sync::atomic::AtomicBool::new(false),
        }))
        .end();
    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let result = router.route_with_context(&msg, &mut ctx).await;
    // sync handler returned None
    assert!(result.is_none());
}

#[tokio::test]
async fn router_route_no_match() {
    let mut router = WxCpTpMessageRouter::new(None);
    router
        .rule()
        .msg_type("image")
        .async_exec(false)
        .handler(Arc::new(TestHandler {
            called: std::sync::atomic::AtomicBool::new(false),
        }))
        .end();
    let msg = sample_message(); // msg_type = "text"
    let mut ctx = RouteContext::default();
    let result = router.route_with_context(&msg, &mut ctx).await;
    assert!(result.is_none());
}

#[tokio::test]
async fn router_route_with_suite_id_and_info_type() {
    let mut router = WxCpTpMessageRouter::new(None);
    router
        .rule()
        .info_type("change_auth")
        .async_exec(false)
        .handler(Arc::new(TestHandler {
            called: std::sync::atomic::AtomicBool::new(false),
        }))
        .end();
    let msg = sample_message();
    let mut ctx = RouteContext::default();
    let result = router
        .route_with_suite_id(Some("suite_123"), &msg, &mut ctx)
        .await;
    assert!(result.is_none());
}

#[tokio::test]
async fn router_builder_all_conditions() {
    let mut router = WxCpTpMessageRouter::new(None);
    router
        .rule()
        .agent_id(1000002)
        .msg_type("text")
        .event("subscribe")
        .event_key("KEY_123")
        .event_key_regex(r"KEY_\d+")
        .content("hello world")
        .r_content(r"hello \w+")
        .from_user("user_a")
        .info_type("change_auth")
        .change_type("add_authorized")
        .suite_id("suite_123")
        .auth_code("auth_code_1")
        .suite_ticket("ticket_xyz")
        .async_exec(false)
        .handler(Arc::new(TestHandler {
            called: std::sync::atomic::AtomicBool::new(false),
        }))
        .end();
    assert_eq!(router.rules().len(), 1);
}

#[test]
fn router_builder_next_sets_re_enter() {
    let mut router = WxCpTpMessageRouter::new(None);
    router
        .rule()
        .msg_type("text")
        .async_exec(false)
        .handler(Arc::new(TestHandler {
            called: std::sync::atomic::AtomicBool::new(false),
        }))
        .next();
    assert_eq!(router.rules().len(), 1);
    assert!(router.rules()[0].is_re_enter());
}

#[test]
fn router_setters() {
    let mut router = WxCpTpMessageRouter::new(None);
    router.set_message_duplicate_checker(Arc::new(
        wx_rust_common::api::WxMessageInMemoryDuplicateCheckerSingleton,
    ));
    router.set_session_manager(Arc::new(
        wx_rust_common::session::StandardSessionManager::new(),
    ));
    router.set_exception_handler(Arc::new(wx_rust_common::util::LogExceptionHandler));
}
