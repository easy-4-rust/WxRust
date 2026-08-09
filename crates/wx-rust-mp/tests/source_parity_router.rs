#![allow(clippy::field_reassign_with_default)]
//! 镜像 Java `WxMpMessageRouterTest`：规则链匹配、同步/异步执行、去重、会话清理。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_mp::api::{WxMpMessageHandler, WxMpMessageRouter, WxMpService};
use wx_rust_mp::bean::message::{WxMpXmlMessage, WxMpXmlOutMessage};

/// 回声处理器（对应 Java WxEchoMpMessageHandler）。
struct EchoHandler {
    sb: Arc<Mutex<String>>,
    echo: &'static str,
}

impl WxMpMessageHandler for EchoHandler {
    fn handle(
        &self,
        _wx_message: &WxMpXmlMessage,
        _context: &mut HashMap<String, Box<dyn std::any::Any + Send>>,
        _wx_mp_service: Option<&dyn WxMpService>,
        _session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxMpXmlOutMessage>, wx_rust_common::error::WxErrorException> {
        self.sb.lock().unwrap().push_str(self.echo);
        self.sb.lock().unwrap().push(',');
        Ok(None)
    }
}

fn echo_handler(sb: &Arc<Mutex<String>>, echo: &'static str) -> Arc<dyn WxMpMessageHandler> {
    Arc::new(EchoHandler {
        sb: sb.clone(),
        echo,
    })
}

/// 会话处理器（对应 Java WxSessionMessageHandler）：访问会话。
struct SessionHandler;

impl WxMpMessageHandler for SessionHandler {
    fn handle(
        &self,
        wx_message: &WxMpXmlMessage,
        _context: &mut HashMap<String, Box<dyn std::any::Any + Send>>,
        _wx_mp_service: Option<&dyn WxMpService>,
        session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxMpXmlOutMessage>, wx_rust_common::error::WxErrorException> {
        session_manager.get_session(wx_message.from_user.as_deref().unwrap_or(""));
        Ok(None)
    }
}

/// 构建与 Java prepare() 相同的 13 条规则。
fn prepare(router: &mut WxMpMessageRouter, sb: &Arc<Mutex<String>>) {
    router
        .rule()
        .async_exec(false)
        .msg_type("text")
        .event("CLICK")
        .event_key("KEY_1")
        .content("CONTENT_1")
        .handler(echo_handler(sb, "COMBINE_4"))
        .end()
        .rule()
        .async_exec(false)
        .msg_type("text")
        .event("CLICK")
        .event_key("KEY_1")
        .handler(echo_handler(sb, "COMBINE_3"))
        .end()
        .rule()
        .async_exec(false)
        .msg_type("text")
        .event("CLICK")
        .handler(echo_handler(sb, "COMBINE_2"))
        .end()
        .rule()
        .async_exec(false)
        .msg_type("text")
        .handler(echo_handler(sb, "text"))
        .end()
        .rule()
        .async_exec(false)
        .event("CLICK")
        .handler(echo_handler(sb, "CLICK"))
        .end()
        .rule()
        .async_exec(false)
        .event_key("KEY_1")
        .handler(echo_handler(sb, "KEY_1"))
        .end()
        .rule()
        .async_exec(false)
        .event_key_regex("KEY_1*")
        .handler(echo_handler(sb, "KEY_123"))
        .end()
        .rule()
        .async_exec(false)
        .content("CONTENT_1")
        .handler(echo_handler(sb, "CONTENT_1"))
        .end()
        .rule()
        .async_exec(false)
        .r_content(".*bc.*")
        .handler(echo_handler(sb, "abcd"))
        .end()
        .rule()
        .async_exec(false)
        .matcher(Arc::new(FormatMatcher))
        .handler(echo_handler(sb, "matcher"))
        .end()
        .rule()
        .async_exec(false)
        .handler(echo_handler(sb, "ALL"))
        .end();
}

/// 自定义匹配器（对应 Java 匿名 matcher：format == "strangeformat"）。
struct FormatMatcher;

impl wx_rust_mp::api::WxMpMessageMatcher for FormatMatcher {
    fn match_message(&self, message: &WxMpXmlMessage) -> bool {
        message.format.as_deref() == Some("strangeformat")
    }
}

/// 消息夹具（对应 Java messages-1 dataProvider 的 10 个消息）。
fn messages(prefix: &str) -> Vec<(WxMpXmlMessage, &'static str)> {
    let mut v = Vec::new();
    let mut idx = 0;

    // 每条消息设唯一 from_user：全局单例去重器按
    // createTime-fromUser-eventKey-event 构造去重 id，重复 id 会被跳过；
    // 唯一 from_user 使路由测试聚焦规则匹配，且测试间互不污染。
    let mut m1 = WxMpXmlMessage::default();
    idx += 1;
    m1.msg_id = Some(1000 + idx as i64);
    m1.from_user = Some(format!("{prefix}{idx}"));
    m1.msg_type = Some("text".to_string());
    v.push((m1, "text,"));

    let mut m2 = WxMpXmlMessage::default();
    idx += 1;
    m2.msg_id = Some(2000 + idx as i64);
    m2.from_user = Some(format!("{prefix}{idx}"));
    m2.event = Some("CLICK".to_string());
    v.push((m2, "CLICK,"));

    let mut m3 = WxMpXmlMessage::default();
    idx += 1;
    m3.msg_id = Some(3000 + idx as i64);
    m3.from_user = Some(format!("{prefix}{idx}"));
    m3.event_key = Some("KEY_1".to_string());
    v.push((m3, "KEY_1,"));

    let mut m4 = WxMpXmlMessage::default();
    idx += 1;
    m4.msg_id = Some(4000 + idx as i64);
    m4.from_user = Some(format!("{prefix}{idx}"));
    m4.content = Some("CONTENT_1".to_string());
    v.push((m4, "CONTENT_1,"));

    let mut m5 = WxMpXmlMessage::default();
    idx += 1;
    m5.msg_id = Some(5000 + idx as i64);
    m5.from_user = Some(format!("{prefix}{idx}"));
    m5.content = Some("BLA".to_string());
    v.push((m5, "ALL,"));

    let mut m6 = WxMpXmlMessage::default();
    idx += 1;
    m6.msg_id = Some(6000 + idx as i64);
    m6.from_user = Some(format!("{prefix}{idx}"));
    m6.content = Some("abcd".to_string());
    v.push((m6, "abcd,"));

    let mut m7 = WxMpXmlMessage::default();
    idx += 1;
    m7.msg_id = Some(7000 + idx as i64);
    m7.from_user = Some(format!("{prefix}{idx}"));
    m7.format = Some("strangeformat".to_string());
    v.push((m7, "matcher,"));

    let mut c2 = WxMpXmlMessage::default();
    idx += 1;
    c2.msg_id = Some(8000 + idx as i64);
    c2.from_user = Some(format!("{prefix}{idx}"));
    c2.msg_type = Some("text".to_string());
    c2.event = Some("CLICK".to_string());
    v.push((c2, "COMBINE_2,"));

    let mut c3 = WxMpXmlMessage::default();
    idx += 1;
    c3.msg_id = Some(9000 + idx as i64);
    c3.from_user = Some(format!("{prefix}{idx}"));
    c3.msg_type = Some("text".to_string());
    c3.event = Some("CLICK".to_string());
    c3.event_key = Some("KEY_1".to_string());
    v.push((c3, "COMBINE_3,"));

    let mut c4 = WxMpXmlMessage::default();
    idx += 1;
    c4.msg_id = Some(10000 + idx as i64);
    c4.from_user = Some(format!("{prefix}{idx}"));
    c4.msg_type = Some("text".to_string());
    c4.event = Some("CLICK".to_string());
    c4.event_key = Some("KEY_1".to_string());
    c4.content = Some("CONTENT_1".to_string());
    v.push((c4, "COMBINE_4,"));

    v
}

// ---- 镜像 testSync：10 个消息的同步路由 ----

#[tokio::test]
async fn router_sync_rules_matching() {
    let sb = Arc::new(Mutex::new(String::new()));
    for (msg, expected) in messages("sync") {
        let sb = sb.clone();
        let mut router = WxMpMessageRouter::new(None);
        prepare(&mut router, &sb);
        let _ = router.route(&msg).await;
        let actual = sb.lock().unwrap().clone();
        assert_eq!(actual, expected, "消息 {msg:?} 期望 {expected}");
        sb.lock().unwrap().clear();
    }
}

// ---- 镜像 testAsync：异步规则 + 等待 ----

#[tokio::test]
async fn router_async_rules_matching() {
    let sb = Arc::new(Mutex::new(String::new()));
    for (msg, expected) in messages("async") {
        let sb = sb.clone();
        let mut router = WxMpMessageRouter::new(None);
        // 异步规则：全部规则 async_exec(true)（Java prepare(async=true)）
        router
            .rule()
            .async_exec(true)
            .msg_type("text")
            .event("CLICK")
            .event_key("KEY_1")
            .content("CONTENT_1")
            .handler(echo_handler(&sb, "COMBINE_4"))
            .end()
            .rule()
            .async_exec(true)
            .msg_type("text")
            .event("CLICK")
            .event_key("KEY_1")
            .handler(echo_handler(&sb, "COMBINE_3"))
            .end()
            .rule()
            .async_exec(true)
            .msg_type("text")
            .event("CLICK")
            .handler(echo_handler(&sb, "COMBINE_2"))
            .end()
            .rule()
            .async_exec(true)
            .msg_type("text")
            .handler(echo_handler(&sb, "text"))
            .end()
            .rule()
            .async_exec(true)
            .event("CLICK")
            .handler(echo_handler(&sb, "CLICK"))
            .end()
            .rule()
            .async_exec(true)
            .event_key("KEY_1")
            .handler(echo_handler(&sb, "KEY_1"))
            .end()
            .rule()
            .async_exec(true)
            .event_key_regex("KEY_1*")
            .handler(echo_handler(&sb, "KEY_123"))
            .end()
            .rule()
            .async_exec(true)
            .content("CONTENT_1")
            .handler(echo_handler(&sb, "CONTENT_1"))
            .end()
            .rule()
            .async_exec(true)
            .r_content(".*bc.*")
            .handler(echo_handler(&sb, "abcd"))
            .end()
            .rule()
            .async_exec(true)
            .matcher(Arc::new(FormatMatcher))
            .handler(echo_handler(&sb, "matcher"))
            .end()
            .rule()
            .async_exec(true)
            .handler(echo_handler(&sb, "ALL"))
            .end();
        let _ = router.route(&msg).await;
        // Java testAsync：sleep(500) 等待异步完成
        tokio::time::sleep(Duration::from_millis(300)).await;
        let actual = sb.lock().unwrap().clone();
        assert_eq!(actual, expected, "异步消息 {msg:?} 期望 {expected}");
        sb.lock().unwrap().clear();
    }
}

// ---- 镜像 next() 语义：reEnter 规则继续匹配 ----

#[tokio::test]
async fn router_next_continues_matching() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMpMessageRouter::new(None);
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

    let mut m = WxMpXmlMessage::default();
    m.msg_id = Some(11111);
    m.from_user = Some("next1".to_string());
    m.msg_type = Some("text".to_string());
    let _ = router.route(&m).await;
    assert_eq!(sb.lock().unwrap().as_str(), "FIRST,ALL,");
}

// ---- 镜像 testSessionClean1：同步规则处理完毕 session 清理 ----

#[tokio::test]
async fn router_session_clean_sync() {
    // Java: maxInactiveInterval=1s + backgroundProcessorDelay=1s → 2s 后清理
    // Rust: 无后台线程（ADAPTED），显式调用 expire_inactive_sessions 模拟处理器
    let ism = StandardSessionManager::with_max_inactive(Duration::from_secs(1));
    let ism = Arc::new(ism);
    let mut router = WxMpMessageRouter::new(None);
    router.set_session_manager(ism.clone());
    router
        .rule()
        .async_exec(false)
        .handler(Arc::new(SessionHandler))
        .next()
        .rule()
        .async_exec(false)
        .handler(Arc::new(SessionHandler))
        .end();

    let mut msg = WxMpXmlMessage::default();
    msg.from_user = Some("abc".to_string());
    let _ = router.route(&msg).await;
    assert_eq!(ism.active_sessions(), 1, "处理期间会话活跃");

    // 模拟后台处理器清理（Java 2s 后断言 activeSessions == 0）
    tokio::time::sleep(Duration::from_millis(1100)).await;
    ism.expire_inactive_sessions();
    assert_eq!(ism.active_sessions(), 0, "超过不活动时间应被清理");
}

// ---- 重复消息：同一 msgId 只处理一次 ----

#[tokio::test]
async fn router_duplicate_message_skipped() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMpMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "H"))
        .end();

    let mut m = WxMpXmlMessage::default();
    m.from_user = Some("dup1".to_string());
    m.msg_id = Some(1001);
    m.create_time = Some(1348831860);
    m.from_user = Some("oUser".to_string());

    let _ = router.route(&m).await;
    let _ = router.route(&m).await;
    assert_eq!(
        sb.lock().unwrap().as_str(),
        "H,",
        "重复消息应只处理一次（Java isMsgDuplicated 语义）"
    );
}

// ---- 无 msgId 时按 createTime-fromUser-eventKey-event 构造去重 id ----

#[tokio::test]
async fn router_duplicate_id_without_msg_id() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = WxMpMessageRouter::new(None);
    router
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "H"))
        .end();

    let mut m = WxMpXmlMessage::default();
    m.from_user = Some("dup2".to_string());
    m.msg_id = None;
    m.create_time = Some(123);
    m.from_user = Some("u1".to_string());
    m.event = Some("CLICK".to_string());
    m.event_key = Some("KEY_1".to_string());

    let _ = router.route(&m).await;
    let _ = router.route(&m).await;
    assert_eq!(sb.lock().unwrap().as_str(), "H,");
}
