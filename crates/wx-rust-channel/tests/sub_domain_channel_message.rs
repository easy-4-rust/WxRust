//! 消息子系统测试（Wave 2 H2c：bean/message + 消息路由 + 消息服务）。
//!
//! 覆盖：
//! - JSON/XML 线格式 golden（对应 Java `WxChannelMessageRouterTest` 与
//!   回调文档示例；Java 测试 golden 中的 `close_timestamp` 字符串强转等
//!   Jackson 语义）；
//! - 嵌套对象 unpack（`ProductSpuAudit`/`BrandEvent`/`receive_info` 等）；
//! - 路由器分发：事件匹配、`next` 链、去重、异步规则、context 共享、
//!   格式探测（配置 msgDataFormat + 内容前缀猜测）；
//! - 消息服务默认规则（39 条，对应 Java `addDefaultRule`）。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use wx_rust_common::session::WxSessionManager;

use wx_rust_channel::api::WxChannelMessageService;
use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::api::r#impl::WxChannelMessageServiceImpl;
use wx_rust_channel::api::r#impl::WxChannelServiceImpl;
use wx_rust_channel::bean::message::coupon::CouponReceiveMessage;
use wx_rust_channel::bean::message::order::OrderIdMessage;
use wx_rust_channel::bean::message::product::{BrandMessage, SpuAuditMessage, SpuStockMessage};
use wx_rust_channel::bean::message::store::{CloseStoreMessage, NicknameUpdateMessage};
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_channel::constant::MessageEventConstants;
use wx_rust_channel::message::rule::WxChannelMessageHandlerFn;
use wx_rust_channel::message::{RouteContext, WxChannelMessage, WxChannelMessageRouter};

/// 测试用的门面服务（对应 Java 测试注入的 `WxChannelService`）。
fn test_service() -> Option<Arc<dyn WxChannelService>> {
    let config = Arc::new(WxChannelDefaultConfig::new("test-appid", "test-secret"));
    Some(WxChannelServiceImpl::new_arc(config))
}

/// Java `WxChannelMessageRouterTest.test1` 的商品审核 golden。
const SPU_AUDIT_JSON: &str = r#"{
    "ToUserName":"gh_*",
    "FromUserName":"OPENID",
    "CreateTime":1662480000,
    "MsgType":"event",
    "Event":"product_spu_audit",
    "ProductSpuAudit": {
        "product_id":"12345678",
        "status":3,
        "reason":"abc"
    }
}"#;

// ---------------------------------------------------------------- 线格式 golden

/// JSON 线格式 golden：基础字段 + 嵌套 order_info（对应 Java 回调文档）。
#[test]
fn json_wire_format_order_id_message() {
    let json = r#"{
        "ToUserName":"gh_*",
        "FromUserName":"OPENID",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_order_new",
        "MsgId":123456789,
        "order_info":{"order_id":"8888888"}
    }"#;
    let msg: OrderIdMessage = serde_json::from_str(json).expect("JSON 解析失败");
    assert_eq!(msg.to_user.as_deref(), Some("gh_*"));
    assert_eq!(msg.from_user.as_deref(), Some("OPENID"));
    assert_eq!(msg.create_time, Some(1662480000));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("channels_ec_order_new"));
    assert_eq!(msg.msg_id, Some(123456789));
    let order_info = msg.order_info.expect("order_info 缺失");
    assert_eq!(order_info.order_id.as_deref(), Some("8888888"));
}

/// `MsgID`（大写）兼容别名：对应 Java `msgIdFill` setter。
#[test]
fn json_msg_id_alias() {
    let json = r#"{"MsgID":1001,"FromUserName":"OPENID"}"#;
    let msg: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    assert_eq!(msg.msg_id, Some(1001));
}

/// Jackson 数字字符串强转：`close_timestamp` 为字符串（Java 测试 golden 原样）。
#[test]
fn json_string_number_coercion_close_store() {
    let json = r#"{
        "ToUserName": "gh_*",
        "FromUserName": "OPENID",
        "CreateTime": 1662480000,
        "MsgType": "event",
        "Event": "channels_ec_close_store",
        "appid": "APPID",
        "close_timestamp": "1662480000"
    }"#;
    let msg: CloseStoreMessage = serde_json::from_str(json).expect("解析失败");
    assert_eq!(msg.appid.as_deref(), Some("APPID"));
    assert_eq!(msg.close_timestamp, Some(1662480000));
}

/// 嵌套对象 unpack：`ProductSpuAudit` 合并到顶层字段（Java golden 同款）。
#[test]
fn json_unpack_spu_audit_golden() {
    let msg: SpuAuditMessage = serde_json::from_str(SPU_AUDIT_JSON).expect("解析失败");
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("product_spu_audit"));
    assert_eq!(msg.product_id.as_deref(), Some("12345678"));
    assert_eq!(msg.status, Some(3));
    assert_eq!(msg.reason.as_deref(), Some("abc"));
}

/// 嵌套对象 unpack：`receive_info`（对应 Java `CouponReceiveMessage` unpack）。
#[test]
fn json_unpack_coupon_receive_info() {
    let json = r#"{
        "ToUserName":"gh_*",
        "FromUserName":"OPENID",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_coupon_receive",
        "receive_info": {
            "coupon_id":"COUPON1",
            "user_coupon_id":"USER_COUPON1",
            "receive_time":"1662480000"
        }
    }"#;
    let msg: CouponReceiveMessage = serde_json::from_str(json).expect("解析失败");
    assert_eq!(msg.coupon_id.as_deref(), Some("COUPON1"));
    assert_eq!(msg.user_coupon_id.as_deref(), Some("USER_COUPON1"));
    assert_eq!(msg.receive_time.as_deref(), Some("1662480000"));
}

/// 顶层字段与嵌套对象同值时均可解析（顶层优先、嵌套覆盖语义不影响同值场景）。
#[test]
fn json_unpack_brand_event() {
    let json = r#"{
        "ToUserName":"gh_*",
        "FromUserName":"OPENID",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_brand",
        "BrandEvent": {
            "brand_id":"BRAND1",
            "audit_id":"AUDIT1",
            "status":4,
            "reason":"ok"
        }
    }"#;
    let msg: BrandMessage = serde_json::from_str(json).expect("解析失败");
    assert_eq!(msg.brand_id.as_deref(), Some("BRAND1"));
    assert_eq!(msg.audit_id.as_deref(), Some("AUDIT1"));
    assert_eq!(msg.status, Some(4));
    assert_eq!(msg.reason.as_deref(), Some("ok"));
}

/// XML 线格式 golden：CDATA 基础字段 + 嵌套 order_info（quick-xml serde）。
#[test]
fn xml_wire_format_order_id_message() {
    let xml = r#"<xml>
        <ToUserName><![CDATA[gh_*]]></ToUserName>
        <FromUserName><![CDATA[OPENID]]></FromUserName>
        <CreateTime>1662480000</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[channels_ec_order_new]]></Event>
        <order_info>
            <order_id><![CDATA[8888888]]></order_id>
        </order_info>
    </xml>"#;
    let msg: OrderIdMessage = quick_xml::de::from_str(xml).expect("XML 解析失败");
    assert_eq!(msg.to_user.as_deref(), Some("gh_*"));
    assert_eq!(msg.from_user.as_deref(), Some("OPENID"));
    assert_eq!(msg.create_time, Some(1662480000));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("channels_ec_order_new"));
    assert_eq!(msg.order_info.unwrap().order_id.as_deref(), Some("8888888"));
}

/// XML 嵌套对象 unpack：`receive_info` 元素。
#[test]
fn xml_unpack_coupon_receive_info() {
    let xml = r#"<xml>
        <ToUserName><![CDATA[gh_*]]></ToUserName>
        <FromUserName><![CDATA[OPENID]]></FromUserName>
        <CreateTime>1662480000</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[channels_ec_coupon_receive]]></Event>
        <receive_info>
            <coupon_id><![CDATA[COUPON1]]></coupon_id>
            <user_coupon_id><![CDATA[USER_COUPON1]]></user_coupon_id>
            <receive_time>1662480000</receive_time>
        </receive_info>
    </xml>"#;
    let msg: CouponReceiveMessage = quick_xml::de::from_str(xml).expect("XML 解析失败");
    assert_eq!(msg.coupon_id.as_deref(), Some("COUPON1"));
    assert_eq!(msg.user_coupon_id.as_deref(), Some("USER_COUPON1"));
    assert_eq!(msg.receive_time.as_deref(), Some("1662480000"));
}

/// `toJson` NON_NULL 语义：null 字段不输出（Jackson `JsonInclude.NON_NULL`）。
#[test]
fn to_json_skips_null_fields() {
    let mut msg = WxChannelMessage::default();
    msg.to_user = Some("gh_*".to_string());
    msg.msg_type = Some("event".to_string());
    let json = msg.to_json();
    assert_eq!(json, r#"{"ToUserName":"gh_*","MsgType":"event"}"#);
}

// ---------------------------------------------------------------- 路由分发

/// 事件匹配分发 + 类型化 handler（对应 Java `WxChannelMessageRouterTest.test1`）。
#[tokio::test]
async fn router_dispatches_spu_audit_golden() {
    let mut router = WxChannelMessageRouter::new();
    let captured = Arc::new(AtomicBool::new(false));
    {
        let captured = captured.clone();
        router
            .rule::<SpuAuditMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |message: &SpuAuditMessage,
                      _content: &str,
                      _app_id: &str,
                      _context: &mut RouteContext,
                      _session_manager: &dyn WxSessionManager| {
                    // 类型化消息：嵌套 ProductSpuAudit 已 unpack
                    assert_eq!(message.product_id.as_deref(), Some("12345678"));
                    assert_eq!(message.status, Some(3));
                    captured.store(true, Ordering::SeqCst);
                    Ok(Some("success".to_string()))
                },
            )))
            .end();
    }

    // 与 Java 测试一致：先按基类解析，再路由（content 用于重新反序列化）
    let base: WxChannelMessage = serde_json::from_str(SPU_AUDIT_JSON).expect("解析失败");
    let result = router
        .route(&base, SPU_AUDIT_JSON, "xxxWWQQxxx", test_service())
        .await;
    assert_eq!(result.as_deref(), Some("success"));
    assert!(captured.load(Ordering::SeqCst), "handler 未被调用");
}

/// 事件不匹配时返回 None（Java `route` 无匹配规则 → null）。
#[tokio::test]
async fn router_no_match_returns_none() {
    let mut router = WxChannelMessageRouter::new();
    router
        .rule::<SpuAuditMessage>()
        .async_exec(false)
        .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
        .handler(Arc::new(WxChannelMessageHandlerFn::new(
            |_m, _c, _a, _ctx, _sm| Ok(Some("success".to_string())),
        )))
        .end();

    let json = r#"{"MsgType":"event","Event":"unknown_event","FromUserName":"OPENID"}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let result = router.route(&base, json, "appid", None).await;
    assert_eq!(result, None);
}

/// 重复消息跳过（进程内单例去重器；Java `isMsgDuplicated` 语义）。
#[tokio::test]
async fn router_skips_duplicate_message() {
    let mut router = WxChannelMessageRouter::new();
    router
        .rule::<SpuAuditMessage>()
        .async_exec(false)
        .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
        .handler(Arc::new(WxChannelMessageHandlerFn::new(
            |_m, _c, _a, _ctx, _sm| Ok(Some("success".to_string())),
        )))
        .end();

    let json = r#"{"MsgType":"event","Event":"product_spu_audit","FromUserName":"DUP-USER-1","CreateTime":1662480000}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let first = router.route(&base, json, "appid", None).await;
    assert_eq!(first.as_deref(), Some("success"));
    // 第二次路由同一消息（同一 msgId 生成规则）→ 重复，跳过
    let second = router.route(&base, json, "appid", None).await;
    assert_eq!(second, None);
}

/// `next` 链：匹配规则依次执行，返回最后一个同步规则的结果（Java 同款）。
#[tokio::test]
async fn router_next_chain_returns_last_sync_result() {
    let mut router = WxChannelMessageRouter::new();
    let hits = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    {
        let hits = hits.clone();
        router
            .rule::<WxChannelMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |_m, _c, _a, _ctx, _sm| {
                    hits.fetch_add(1, Ordering::SeqCst);
                    Ok(Some("first".to_string()))
                },
            )))
            .next();
    }
    {
        let hits = hits.clone();
        router
            .rule::<WxChannelMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |_m, _c, _a, _ctx, _sm| {
                    hits.fetch_add(10, Ordering::SeqCst);
                    Ok(Some("last".to_string()))
                },
            )))
            .end();
    }

    let json = r#"{"MsgType":"event","Event":"product_spu_audit","FromUserName":"NEXT-USER-1","CreateTime":1662480001}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let result = router.route(&base, json, "appid", None).await;
    assert_eq!(result.as_deref(), Some("last"));
    // 非 next 规则停止匹配：第二条规则执行，第一条也执行（next=true）
    assert_eq!(hits.load(Ordering::SeqCst), 11);
}

/// 非 `next` 规则匹配后停止后续规则（Java 收集匹配规则遇非 next 即 break）。
#[tokio::test]
async fn router_stops_at_non_next_rule() {
    let mut router = WxChannelMessageRouter::new();
    let hits = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    {
        let hits = hits.clone();
        router
            .rule::<WxChannelMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |_m, _c, _a, _ctx, _sm| {
                    hits.fetch_add(1, Ordering::SeqCst);
                    Ok(None)
                },
            )))
            .end();
    }
    {
        let hits = hits.clone();
        router
            .rule::<WxChannelMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |_m, _c, _a, _ctx, _sm| {
                    hits.fetch_add(10, Ordering::SeqCst);
                    Ok(None)
                },
            )))
            .end();
    }

    let json = r#"{"MsgType":"event","Event":"product_spu_audit","FromUserName":"STOP-USER-1","CreateTime":1662480002}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let result = router.route(&base, json, "appid", None).await;
    assert_eq!(result, None);
    assert_eq!(hits.load(Ordering::SeqCst), 1, "第二条规则不应执行");
}

/// 同步规则共享 context（Java 同一 `Map<String, Object>`）。
#[tokio::test]
async fn router_context_shared_between_sync_rules() {
    let mut router = WxChannelMessageRouter::new();
    router
        .rule::<WxChannelMessage>()
        .async_exec(false)
        .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
        .handler(Arc::new(WxChannelMessageHandlerFn::new(
            |_m, _c, _a, context, _sm| {
                context.insert("step".to_string(), Box::new("handled"));
                Ok(None)
            },
        )))
        .next();
    let saw = Arc::new(AtomicBool::new(false));
    {
        let saw = saw.clone();
        router
            .rule::<WxChannelMessage>()
            .async_exec(false)
            .event(MessageEventConstants::PRODUCT_SPU_AUDIT)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |_m, _c, _a, context, _sm| {
                    let v = context
                        .get("step")
                        .and_then(|v| v.downcast_ref::<&str>())
                        .copied();
                    assert_eq!(v, Some("handled"));
                    saw.store(true, Ordering::SeqCst);
                    Ok(None)
                },
            )))
            .end();
    }

    let json = r#"{"MsgType":"event","Event":"product_spu_audit","FromUserName":"CTX-USER-1","CreateTime":1662480003}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let mut context: RouteContext = Default::default();
    router
        .route_with_context(
            &base,
            json,
            "appid",
            &mut context,
            None,
            Arc::new(wx_rust_common::session::StandardSessionManager::new()),
        )
        .await;
    assert!(saw.load(Ordering::SeqCst));
}

/// 格式探测：配置 msgDataFormat=XML 时按 XML 解析（不依赖 `<xml>` 前缀）。
#[tokio::test]
async fn router_honors_config_msg_data_format() {
    let mut router = WxChannelMessageRouter::new();
    let captured = Arc::new(AtomicBool::new(false));
    {
        let captured = captured.clone();
        router
            .rule::<OrderIdMessage>()
            .async_exec(false)
            .event(MessageEventConstants::ORDER_NEW)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |message: &OrderIdMessage,
                      _c: &str,
                      _a: &str,
                      _ctx: &mut RouteContext,
                      _sm: &dyn WxSessionManager| {
                    assert_eq!(
                        message.order_info.as_ref().unwrap().order_id.as_deref(),
                        Some("8888888")
                    );
                    captured.store(true, Ordering::SeqCst);
                    Ok(Some("success".to_string()))
                },
            )))
            .end();
    }

    // 根元素不是 `<xml>`，仅靠内容前缀猜测无法命中 → 必须走配置格式
    let xml = r#"<Message><ToUserName><![CDATA[gh_*]]></ToUserName><FromUserName><![CDATA[XMLFMT-USER-1]]></FromUserName><CreateTime>1662480004</CreateTime><MsgType><![CDATA[event]]></MsgType><Event><![CDATA[channels_ec_order_new]]></Event><order_info><order_id><![CDATA[8888888]]></order_id></order_info></Message>"#;
    let base: WxChannelMessage = serde_json::from_str(
        r#"{"MsgType":"event","Event":"channels_ec_order_new","FromUserName":"XMLFMT-USER-1","CreateTime":1662480004}"#,
    )
    .expect("解析失败");

    let mut config = WxChannelDefaultConfig::new("appid", "secret");
    config.set_msg_data_format("XML");
    let service: Arc<dyn WxChannelService> = WxChannelServiceImpl::new_arc(Arc::new(config));
    let result = router.route(&base, xml, "appid", Some(service)).await;
    assert_eq!(result.as_deref(), Some("success"));
    assert!(captured.load(Ordering::SeqCst));
}

/// 异步规则：提交 tokio 任务后台执行（对应 Java 线程池 `submit`），
/// 同步入口返回最后一个同步规则的结果。
#[tokio::test]
async fn router_async_rule_runs_in_background() {
    let mut router = WxChannelMessageRouter::new();
    let async_done = Arc::new(AtomicBool::new(false));
    {
        let async_done = async_done.clone();
        router
            .rule::<SpuStockMessage>()
            .async_exec(true)
            .event(MessageEventConstants::PRODUCT_STOCK_NO_ENOUGH)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |message: &SpuStockMessage,
                      _c: &str,
                      _a: &str,
                      _ctx: &mut RouteContext,
                      _sm: &dyn WxSessionManager| {
                    assert_eq!(message.product_id.as_deref(), Some("P1"));
                    async_done.store(true, Ordering::SeqCst);
                    Ok(Some("async".to_string()))
                },
            )))
            .end();
    }

    let json = r#"{"MsgType":"event","Event":"channels_ec_stock_no_enough","FromUserName":"ASYNC-USER-1","CreateTime":1662480005,"channels_ec_stock_no_enough":{"product_id":"P1","sku_id":"S1","remaining_stock_amount":5}}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let result = router.route(&base, json, "appid", None).await;
    // 异步规则结果不返回
    assert_eq!(result, None);
    // 等待后台任务执行完成
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert!(async_done.load(Ordering::SeqCst), "异步 handler 未执行");
}

/// `generateMessageId` golden（Java 语义：null → "null" 字面量、event trim）。
#[test]
fn generate_message_id_golden() {
    let router = WxChannelMessageRouter::new();

    // msgId 为空：createTime-fromUser-event(trimToEmpty)
    let msg = WxChannelMessage {
        create_time: Some(1662480000),
        from_user: Some("OPENID".to_string()),
        event: Some(" product_spu_audit ".to_string()),
        ..Default::default()
    };
    assert_eq!(
        router.generate_message_id(&msg),
        "1662480000-OPENID-product_spu_audit"
    );

    // 追加非空 toUser
    let msg = WxChannelMessage {
        create_time: Some(1662480000),
        from_user: Some("OPENID".to_string()),
        event: Some("product_spu_audit".to_string()),
        to_user: Some("gh_*".to_string()),
        ..Default::default()
    };
    assert_eq!(
        router.generate_message_id(&msg),
        "1662480000-OPENID-product_spu_audit-gh_*"
    );

    // 有 msgId：msgId-createTime-fromUser
    let msg = WxChannelMessage {
        msg_id: Some(123),
        create_time: Some(1662480000),
        from_user: Some("OPENID".to_string()),
        ..Default::default()
    };
    assert_eq!(router.generate_message_id(&msg), "123-1662480000-OPENID");
}

// ---------------------------------------------------------------- 消息服务

/// 消息服务默认规则：Java `addDefaultRule` 注册 39 条规则。
///
/// 默认规则均为异步（Java `addRule(clazz, event, consumer)` → async=true），
/// `route` 返回最后一个同步规则的结果 → `None`（Java 同款：异步规则
/// fire-and-forget，结果不入返回值）。分发行为由路由测试覆盖。
#[tokio::test]
async fn message_service_default_rules_dispatch() {
    let service = WxChannelMessageServiceImpl::new();
    assert_eq!(service.router().rules().len(), 39);

    // 商品审核事件命中默认规则（异步执行，不返回结果）；使用独立消息身份，
    // 避免与进程内单例去重器（`WxMessageInMemoryDuplicateCheckerSingleton`）
    // 中其他测试的消息 id 撞车
    let spu_audit_json = SPU_AUDIT_JSON.replace(
        "\"FromUserName\":\"OPENID\"",
        "\"FromUserName\":\"SVC-OPENID\"",
    );
    let base: WxChannelMessage = serde_json::from_str(&spu_audit_json).expect("解析失败");
    let result = service
        .route(&base, &spu_audit_json, "appid", test_service())
        .await;
    assert_eq!(result, None);

    // 未知事件不匹配任何默认规则
    let unknown = r#"{"MsgType":"event","Event":"unknown_event","FromUserName":"SVC-USER-1","CreateTime":1662480006}"#;
    let base: WxChannelMessage = serde_json::from_str(unknown).expect("解析失败");
    let result = service.route(&base, unknown, "appid", None).await;
    assert_eq!(result, None);
}

/// 小店注销/改名默认规则（对应 Java `WxChannelMessageRouterTest.closeStore` golden；
/// 默认规则异步，route 返回 None）。
#[tokio::test]
async fn message_service_close_store_and_nickname_rules() {
    let service = WxChannelMessageServiceImpl::new();

    let close_store_json = r#"{
        "ToUserName": "gh_*",
        "FromUserName": "OPENID",
        "CreateTime": 1662480000,
        "MsgType": "event",
        "Event": "channels_ec_close_store",
        "appid": "APPID",
        "close_timestamp": "1662480000"
    }"#;
    let base: WxChannelMessage = serde_json::from_str(close_store_json).expect("解析失败");
    let result = service.route(&base, close_store_json, "123456", None).await;
    assert_eq!(result, None);

    let nickname_json = r#"{
        "ToUserName": "gh_*",
        "FromUserName": "OPENID",
        "CreateTime": 1662480000,
        "MsgType": "event",
        "Event": "set_shop_nickname",
        "appid": "APPID",
        "old_nickname": "旧昵称",
        "new_nickname": "新昵称"
    }"#;
    let base: WxChannelMessage = serde_json::from_str(nickname_json).expect("解析失败");
    let result = service.route(&base, nickname_json, "123456", None).await;
    assert_eq!(result, None);
}

/// 自定义规则替换默认规则（Java 继承覆写事件的 Rust 扩展点）。
#[tokio::test]
async fn message_service_custom_rule_via_router() {
    let mut service = WxChannelMessageServiceImpl::new();
    // 移除默认规则，注册自定义事件 handler
    service.router_mut().rules_mut().clear();
    let captured = Arc::new(AtomicBool::new(false));
    {
        let captured = captured.clone();
        service
            .router_mut()
            .rule::<NicknameUpdateMessage>()
            .async_exec(false)
            .event(MessageEventConstants::SET_SHOP_NICKNAME)
            .handler(Arc::new(WxChannelMessageHandlerFn::new(
                move |message: &NicknameUpdateMessage,
                      _c: &str,
                      _a: &str,
                      _ctx: &mut RouteContext,
                      _sm: &dyn WxSessionManager| {
                    assert_eq!(message.new_nickname.as_deref(), Some("新昵称"));
                    captured.store(true, Ordering::SeqCst);
                    Ok(Some("custom-ok".to_string()))
                },
            )))
            .end();
    }

    let json = r#"{"MsgType":"event","Event":"set_shop_nickname","FromUserName":"CUSTOM-USER-1","CreateTime":1662480007,"new_nickname":"新昵称"}"#;
    let base: WxChannelMessage = serde_json::from_str(json).expect("解析失败");
    let result = service.route(&base, json, "appid", None).await;
    assert_eq!(result.as_deref(), Some("custom-ok"));
    assert!(captured.load(Ordering::SeqCst));
}
