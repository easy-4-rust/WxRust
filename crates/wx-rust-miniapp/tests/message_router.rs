//! 镜像 Java `WxMaMessageRouter` 语义：规则链匹配、同步/异步执行、
//! interceptor 阻断、matcher 匹配、去重、next 链。
//!
//! Java 无 `WxMaMessageRouterTest`，用例按 `WxMaMessageRouter`/
//! `WxMaMessageRouterRule` 源码语义构造（并参考 mp 的
//! `WxMpMessageRouterTest` 镜像风格）。

use std::sync::{Arc, Mutex};
use std::time::Duration;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;
use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::message::{
    RouteContext, WxMaMessage, WxMaMessageHandler, WxMaMessageInterceptor, WxMaMessageMatcher,
    WxMaMessageRouter, WxMaOutMessage, WxMaXmlOutMessage,
};

/// 回声处理器：把 echo 追加到共享字符串（用于断言哪个 handler 被调用）。
struct EchoHandler {
    sb: Arc<Mutex<String>>,
    echo: &'static str,
}

impl WxMaMessageHandler for EchoHandler {
    fn handle(
        &self,
        _wx_message: &WxMaMessage,
        _context: &mut RouteContext,
        _wx_ma_service: Option<&dyn WxMaService>,
        _session_manager: &dyn WxSessionManager,
    ) -> Result<Option<Arc<dyn WxMaOutMessage + Send + Sync>>, WxErrorException> {
        self.sb.lock().unwrap().push_str(self.echo);
        self.sb.lock().unwrap().push(',');
        Ok(None)
    }
}

fn echo_handler(sb: &Arc<Mutex<String>>, echo: &'static str) -> Arc<dyn WxMaMessageHandler> {
    Arc::new(EchoHandler {
        sb: sb.clone(),
        echo,
    })
}

/// 固定输出一条 XML 文本消息的处理器（验证 route 返回出站消息）。
struct TextOutHandler;

impl WxMaMessageHandler for TextOutHandler {
    fn handle(
        &self,
        _wx_message: &WxMaMessage,
        _context: &mut RouteContext,
        _wx_ma_service: Option<&dyn WxMaService>,
        _session_manager: &dyn WxSessionManager,
    ) -> Result<Option<Arc<dyn WxMaOutMessage + Send + Sync>>, WxErrorException> {
        let mut out = WxMaXmlOutMessage::default();
        out.msg_type = Some("text".to_string());
        out.to_user_name = Some("openid-xxx".to_string());
        Ok(Some(Arc::new(out)))
    }
}

/// 拦截器：按 allow 放行/阻断（对应 Java 匿名 interceptor 返回 boolean）。
struct AllowInterceptor {
    allow: bool,
}

impl WxMaMessageInterceptor for AllowInterceptor {
    fn intercept(
        &self,
        _wx_message: &WxMaMessage,
        _context: &mut RouteContext,
        _wx_ma_service: Option<&dyn WxMaService>,
        _session_manager: &dyn WxSessionManager,
    ) -> bool {
        self.allow
    }
}

/// 自定义匹配器：消息 content 等于 expected 时匹配。
struct ContentMatcher {
    expected: &'static str,
}

impl WxMaMessageMatcher for ContentMatcher {
    fn match_message(&self, message: &WxMaMessage) -> bool {
        message.content.as_deref() == Some(self.expected)
    }
}

/// 消息夹具：唯一 msg_id + from_user（全局去重单例按 msgId 构造去重 id，
/// 唯一 id 保证测试互不污染）。
fn msg(id: i64, from_user: &str) -> WxMaMessage {
    WxMaMessage {
        msg_id: Some(id),
        from_user: Some(from_user.to_string()),
        ..Default::default()
    }
}

// ---- 消息类型/事件/fromUser 路由分发 ----

#[tokio::test]
async fn router_dispatches_by_msg_type_event_and_from_user() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .msg_type("text")
        .handler(echo_handler(&sb, "TEXT"))
        .end()
        .rule()
        .async_exec(false)
        .event("subscribe")
        .handler(echo_handler(&sb, "EVENT"))
        .end()
        .rule()
        .async_exec(false)
        .from_user("u-other")
        .handler(echo_handler(&sb, "OTHER"))
        .end();

    // text 消息 → 第一条规则
    let mut m1 = msg(1001, "u1");
    m1.msg_type = Some("text".to_string());
    let _ = router.route(&m1).await;
    assert_eq!(sb.lock().unwrap().as_str(), "TEXT,");

    // event 消息 → 第二条规则（事件比较不区分大小写）
    sb.lock().unwrap().clear();
    let mut m2 = msg(1002, "u2");
    m2.event = Some("SUBSCRIBE".to_string());
    let _ = router.route(&m2).await;
    assert_eq!(sb.lock().unwrap().as_str(), "EVENT,");

    // fromUser 匹配 → 第三条规则
    sb.lock().unwrap().clear();
    let mut m3 = msg(1003, "u-other");
    m3.content = Some("任意内容".to_string());
    let _ = router.route(&m3).await;
    assert_eq!(sb.lock().unwrap().as_str(), "OTHER,");

    // 无任何规则匹配 → 不调用 handler
    sb.lock().unwrap().clear();
    let mut m4 = msg(1004, "u4");
    m4.msg_type = Some("image".to_string());
    let _ = router.route(&m4).await;
    assert_eq!(sb.lock().unwrap().as_str(), "");
}

// ---- interceptor 返回 false 阻断，不调用 handler ----

#[tokio::test]
async fn router_interceptor_blocks_and_allows() {
    let sb = Arc::new(Mutex::new(String::new()));

    // 阻断：拦截器 false → service 提前返回，handler 不执行
    let mut blocked = WxMaMessageRouter::new(None);
    blocked
        .rule()
        .async_exec(false)
        .interceptor(Arc::new(AllowInterceptor { allow: false }))
        .handler(echo_handler(&sb, "H"))
        .end();
    let m = msg(2001, "b1");
    let res = blocked.route(&m).await;
    assert!(res.is_none(), "拦截不通过时不应返回输出消息");
    assert_eq!(
        sb.lock().unwrap().as_str(),
        "",
        "拦截器 false 时 handler 不应被调用"
    );

    // 放行：拦截器 true → handler 正常执行
    let mut allowed = WxMaMessageRouter::new(None);
    allowed
        .rule()
        .async_exec(false)
        .interceptor(Arc::new(AllowInterceptor { allow: true }))
        .handler(echo_handler(&sb, "H"))
        .end();
    let m = msg(2002, "a1");
    let _ = allowed.route(&m).await;
    assert_eq!(sb.lock().unwrap().as_str(), "H,");
}

// ---- matcher 匹配 / 不匹配 ----

#[tokio::test]
async fn router_matcher_match_and_fallthrough() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .matcher(Arc::new(ContentMatcher { expected: "hello" }))
        .handler(echo_handler(&sb, "MATCH"))
        .end()
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "FALLBACK"))
        .end();

    // matcher 匹配 → 第一条规则
    let mut m1 = msg(3001, "m1");
    m1.content = Some("hello".to_string());
    let _ = router.route(&m1).await;
    assert_eq!(sb.lock().unwrap().as_str(), "MATCH,");

    // matcher 不匹配 → 落入下一条规则
    sb.lock().unwrap().clear();
    let mut m2 = msg(3002, "m2");
    m2.content = Some("world".to_string());
    let _ = router.route(&m2).await;
    assert_eq!(sb.lock().unwrap().as_str(), "FALLBACK,");
}

// ---- rule 链 next 语义：reEnter 规则继续匹配 ----

#[tokio::test]
async fn router_next_continues_other_rules() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .msg_type("text")
        .handler(echo_handler(&sb, "FIRST"))
        .next()
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "ALL"))
        .end();

    // text 消息：第一条（next → reEnter）命中后继续进入第二条
    let mut m1 = msg(4001, "n1");
    m1.msg_type = Some("text".to_string());
    let _ = router.route(&m1).await;
    assert_eq!(sb.lock().unwrap().as_str(), "FIRST,ALL,");

    // 非 text 消息：只命中第二条
    sb.lock().unwrap().clear();
    let mut m2 = msg(4002, "n2");
    m2.msg_type = Some("image".to_string());
    let _ = router.route(&m2).await;
    assert_eq!(sb.lock().unwrap().as_str(), "ALL,");
}

// ---- 同步规则：返回最后一个 handler 的输出消息 ----

#[tokio::test]
async fn router_returns_last_sync_handler_out_message() {
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .handler(Arc::new(TextOutHandler))
        .end();

    let m = msg(5001, "o1");
    let res = router.route(&m).await;
    let out = res.expect("同步规则应返回 handler 的输出消息");
    let xml = out.to_xml();
    assert!(
        xml.contains("<MsgType><![CDATA[text]]></MsgType>"),
        "输出消息应为 XML 文本消息，实际: {xml}"
    );
}

// ---- 重复消息（isMsgDuplicated）：同一 msgId 只处理一次 ----

#[tokio::test]
async fn router_duplicate_message_skipped() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "H"))
        .end();

    let m = msg(6001, "dup1");
    let _ = router.route(&m).await;
    let _ = router.route(&m).await;
    assert_eq!(
        sb.lock().unwrap().as_str(),
        "H,",
        "重复消息应只处理一次（Java isMsgDuplicated 语义）"
    );
}

// ---- 异步规则：handler 在后台执行 ----

#[tokio::test]
async fn router_async_rule_runs_in_background() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(true)
        .handler(echo_handler(&sb, "ASYNC"))
        .end();

    let m = msg(7001, "a1");
    // 同步返回 None（异步规则结果不返回），handler 在后台任务中执行
    let res = router.route(&m).await;
    assert!(res.is_none());
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(sb.lock().unwrap().as_str(), "ASYNC,");
}

// ---- content trim / rContent 整串匹配（Java Pattern.matches）与 eventKey 不参与匹配 ----

#[tokio::test]
async fn router_content_trim_r_content_and_event_key_quirk() {
    // content 按 trim 后比较：消息内容带首尾空格仍可命中
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .content("CONTENT_1")
        .handler(echo_handler(&sb, "C"))
        .end();
    let mut m1 = msg(8001, "c1");
    m1.content = Some("  CONTENT_1  ".to_string());
    let _ = router.route(&m1).await;
    assert_eq!(sb.lock().unwrap().as_str(), "C,");

    // rContent 是 Java Pattern.matches 整串匹配：`.*bc.*` 命中 "abcd"
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .r_content(".*bc.*")
        .handler(echo_handler(&sb, "R"))
        .end();
    let mut m2 = msg(8002, "c2");
    m2.content = Some("abcd".to_string());
    let _ = router.route(&m2).await;
    assert_eq!(sb.lock().unwrap().as_str(), "R,");

    // 裸 `bc` 无法整串命中 "abcd"（Java matches() 全串匹配，find 语义不成立）
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .r_content("bc")
        .handler(echo_handler(&sb, "R"))
        .end()
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "F"))
        .end();
    let mut m3 = msg(8003, "c3");
    m3.content = Some("abcd".to_string());
    let _ = router.route(&m3).await;
    assert_eq!(
        sb.lock().unwrap().as_str(),
        "F,",
        "rContent 为整串匹配（Pattern.matches），裸 bc 不应命中 abcd"
    );

    // Java 怪癖：eventKey 字段存在但 test() 不参与判断，
    // 只设 eventKey 的规则会匹配任意消息
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMaMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .event_key("KEY_1")
        .handler(echo_handler(&sb, "K"))
        .end();
    let m4 = msg(8004, "c4");
    let _ = router.route(&m4).await;
    assert_eq!(
        sb.lock().unwrap().as_str(),
        "K,",
        "Java test() 不使用 eventKey，仅设 eventKey 的规则应匹配任意消息"
    );
}
