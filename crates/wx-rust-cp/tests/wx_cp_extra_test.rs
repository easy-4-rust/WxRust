#![allow(clippy::field_reassign_with_default)]
//! Wave 2 C2b 扩展测试：7 个手写 bean 线格式 golden + 消息路由 + 会话存档 RSA。
//!
//! 对照 Java：
//! - `WxCpUserGsonAdapterTest`（user JSON golden）
//! - `WxCpTagServiceImplTest.testGet` / `WxCpTpTagServiceImplTest`（tag golden）
//! - `WxCpMessageRouterTest`（路由分发语义，去重 id 用唯一 from_user 避免
//!   进程内单例去重器污染，与 wx-rust-mp `source_parity_router.rs` 同一模式）
//! - `WxCpMsgAuditServiceImpl.decryptChatData`（RSA 解密随机密钥 → AES 解密
//!   消息；官方 native SDK 的 `DecryptData` 部分以 Rust 纯实现替代，
//!   测试用 RSA/AES 自构造向量做往返验证）

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use wx_rust_common::api::WxMessageInMemoryDuplicateChecker;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::WxSessionManager;
use wx_rust_cp::bean::message::{WxCpXmlMessage, WxCpXmlOutMessage};
use wx_rust_cp::bean::{
    Gender, WxCpChat, WxCpDepart, WxCpKfGetCorpStatisticResp, WxCpTag, WxCpTagGetResult,
    WxCpTpTagGetResult, WxCpUser,
};
use wx_rust_cp::message::{
    RouteContext, WxCpMessageHandler, WxCpMessageInterceptor, WxCpMessageMatcher, WxCpMessageRouter,
};
use wx_rust_cp::util::crypto::{
    decrypt_chat_data, decrypt_encrypt_chat_msg, decrypt_pri_key, decrypt_pri_key_by_pkcs1,
    decrypt_pri_key_by_pkcs8,
};

// ===========================================================================
// 7 个 bean：线格式 golden
// ===========================================================================

/// Java `WxCpUserGsonAdapterTest.testDeserialize` golden（全字段断言）。
#[test]
fn user_from_json_golden() {
    let json = r#"{
      "errcode": 0,
      "errmsg": "ok",
      "userid": "zhangsan",
      "name": "李四",
      "department": [1, 2],
      "order": [1, 2],
      "position": "后台工程师",
      "mobile": "15913215421",
      "gender": "1",
      "email": "zhangsan@gzdev.com",
      "isleader": 1,
      "avatar": "http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/0",
      "telephone": "020-123456",
      "address": "广州市海珠区新港中路",
      "enable": 1,
      "alias": "jackzhang",
      "extattr": {
          "attrs": [
              { "type": 0, "name": "文本名称", "text": { "value": "文本" } },
              { "type": 1, "name": "网页名称", "web": { "url": "http://www.test.com", "title": "标题" } }
          ]
      },
      "status": 1,
      "qr_code": "https://open.work.weixin.qq.com/wwopen/userQRCode?vcode=xxx",
      "external_position": "高级产品经理",
      "external_profile": {
          "external_corp_name": "企业简称",
          "external_attr": [
              { "type": 0, "name": "文本名称", "text": { "value": "文本" } },
              { "type": 1, "name": "网页名称", "web": { "url": "http://www.test.com", "title": "标题" } },
              { "type": 2, "name": "测试app", "miniprogram": { "appid": "wx8bd8012614784fake", "pagepath": "/index", "title": "my miniprogram" } }
          ]
      }
    }"#;
    let user = WxCpUser::from_json(json).unwrap();

    assert_eq!(user.user_id.as_deref(), Some("zhangsan"));
    assert_eq!(user.name.as_deref(), Some("李四"));
    assert_eq!(user.depart_ids.as_deref(), Some(&[1, 2][..]));
    assert_eq!(user.orders.as_deref(), Some(&[1, 2][..]));
    assert_eq!(user.position.as_deref(), Some("后台工程师"));
    assert_eq!(user.mobile.as_deref(), Some("15913215421"));
    assert_eq!(user.gender, Some(Gender::Male)); // wire "1" → MALE
    assert_eq!(user.is_leader, Some(1));
    assert_eq!(user.telephone.as_deref(), Some("020-123456"));
    assert_eq!(user.address.as_deref(), Some("广州市海珠区新港中路"));
    assert_eq!(user.enable, Some(1));
    assert_eq!(user.alias.as_deref(), Some("jackzhang"));
    assert_eq!(user.status, Some(1));
    assert_eq!(
        user.qr_code.as_deref(),
        Some("https://open.work.weixin.qq.com/wwopen/userQRCode?vcode=xxx")
    );

    // extattr.attrs
    assert_eq!(user.ext_attrs.len(), 2);
    assert_eq!(user.ext_attrs[0].r#type, Some(0));
    assert_eq!(user.ext_attrs[0].name.as_deref(), Some("文本名称"));
    assert_eq!(user.ext_attrs[0].text_value.as_deref(), Some("文本"));
    assert_eq!(user.ext_attrs[1].r#type, Some(1));
    assert_eq!(user.ext_attrs[1].name.as_deref(), Some("网页名称"));
    assert_eq!(
        user.ext_attrs[1].web_url.as_deref(),
        Some("http://www.test.com")
    );
    assert_eq!(user.ext_attrs[1].web_title.as_deref(), Some("标题"));

    // external_profile
    assert_eq!(user.external_position.as_deref(), Some("高级产品经理"));
    assert_eq!(
        user.external_profile.external_corp_name.as_deref(),
        Some("企业简称")
    );
    assert_eq!(user.external_profile.external_attrs.len(), 3);
    let ea0 = &user.external_profile.external_attrs[0];
    assert_eq!(ea0.r#type, 0);
    assert_eq!(ea0.name.as_deref(), Some("文本名称"));
    assert_eq!(ea0.value.as_deref(), Some("文本"));
    let ea1 = &user.external_profile.external_attrs[1];
    assert_eq!(ea1.r#type, 1);
    assert_eq!(ea1.name.as_deref(), Some("网页名称"));
    assert_eq!(ea1.url.as_deref(), Some("http://www.test.com"));
    assert_eq!(ea1.title.as_deref(), Some("标题"));
    let ea2 = &user.external_profile.external_attrs[2];
    assert_eq!(ea2.r#type, 2);
    assert_eq!(ea2.name.as_deref(), Some("测试app"));
    assert_eq!(ea2.appid.as_deref(), Some("wx8bd8012614784fake"));
    assert_eq!(ea2.page_path.as_deref(), Some("/index"));
    assert_eq!(ea2.title.as_deref(), Some("my miniprogram"));
}

/// Java `WxCpUserGsonAdapterTest.testSerialize` golden（精确字符串比对）+
/// `testDirectLeaderEmptyArraySerialization`（direct_leader 空数组也输出）。
#[test]
fn user_to_json_golden() {
    let mut user = WxCpUser::default();
    user.orders = Some(vec![1, 2]);
    user.add_ext_attr("文本名称", "文本");
    user.add_external_attr(wx_rust_cp::bean::wx_cp_user::ExternalAttribute {
        r#type: 0,
        name: Some("文本名称".to_string()),
        value: Some("文本".to_string()),
        url: None,
        title: None,
        appid: None,
        page_path: None,
    });
    user.add_external_attr(wx_rust_cp::bean::wx_cp_user::ExternalAttribute {
        r#type: 1,
        name: Some("网页名称".to_string()),
        url: Some("http://www.test.com".to_string()),
        title: Some("标题".to_string()),
        ..Default::default()
    });
    user.add_external_attr(wx_rust_cp::bean::wx_cp_user::ExternalAttribute {
        r#type: 2,
        name: Some("测试app".to_string()),
        appid: Some("wx8bd80126147df384".to_string()),
        page_path: Some("/index".to_string()),
        title: Some("my miniprogram".to_string()),
        ..Default::default()
    });

    // 与 Java testSerialize 断言逐字一致（字段顺序/省略规则照 adapter）
    let expected = "{\"order\":[1,2],".to_string()
        + "\"extattr\":{\"attrs\":[{\"type\":0,\"name\":\"文本名称\",\"text\":{\"value\":\"文本\"}}]},"
        + "\"external_profile\":{\"external_attr\":"
        + "[{\"type\":0,\"name\":\"文本名称\",\"text\":{\"value\":\"文本\"}},"
        + "{\"type\":1,\"name\":\"网页名称\",\"web\":{\"url\":\"http://www.test.com\",\"title\":\"标题\"}},"
        + "{\"type\":2,\"name\":\"测试app\","
        + "\"miniprogram\":{\"appid\":\"wx8bd80126147df384\",\"pagepath\":\"/index\",\"title\":\"my miniprogram\"}}]}}";
    assert_eq!(user.to_json().unwrap(), expected);

    // direct_leader：空数组也输出（Java 用于清空直属上级）
    let mut u2 = WxCpUser::default();
    u2.user_id = Some("testuser".to_string());
    u2.name = Some("Test User".to_string());
    u2.direct_leader = Some(vec![]);
    assert!(u2.to_json().unwrap().contains("\"direct_leader\":[]"));
    u2.direct_leader = None;
    assert!(!u2.to_json().unwrap().contains("direct_leader"));
    u2.direct_leader = Some(vec!["leader1".to_string(), "leader2".to_string()]);
    assert!(
        u2.to_json()
            .unwrap()
            .contains("\"direct_leader\":[\"leader1\",\"leader2\"]")
    );
}

/// `WxCpTagGsonAdapter`：tagid/tagname 往返 + null 省略。
#[test]
fn tag_from_json_to_json() {
    let tag = WxCpTag::from_json(r#"{"tagid":"1","tagname":"标签A"}"#).unwrap();
    assert_eq!(tag.id.as_deref(), Some("1"));
    assert_eq!(tag.name.as_deref(), Some("标签A"));
    assert_eq!(tag.to_json().unwrap(), r#"{"tagid":"1","tagname":"标签A"}"#);

    let empty = WxCpTag::default();
    assert_eq!(empty.to_json().unwrap(), "{}"); // Java addPropertyIfNotNull 省略 null
}

/// Java `WxCpTagServiceImplTest.testGet` golden（含 userlist 内嵌 WxCpUser）。
#[test]
fn tag_get_result_golden() {
    let json = r#"{"errcode": 0,"errmsg": "ok","userlist": [{"userid": "0124035","name": "王五"},{"userid": "0114035","name": "梦雪"}],"partylist": [9576,9567,9566],"tagname": "测试标签-001"}"#;
    let result = WxCpTagGetResult::from_json(json).unwrap();
    assert_eq!(result.errcode, Some(0));
    assert_eq!(result.errmsg.as_deref(), Some("ok"));
    assert_eq!(result.tagname.as_deref(), Some("测试标签-001"));
    assert_eq!(result.partylist.as_deref(), Some(&[9576, 9567, 9566][..]));
    let users = result.userlist.as_ref().unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].user_id.as_deref(), Some("0124035"));
    assert_eq!(users[0].name.as_deref(), Some("王五"));
    assert_eq!(users[1].user_id.as_deref(), Some("0114035"));
    assert_eq!(users[1].name.as_deref(), Some("梦雪"));

    // 序列化往返：语义相等（用户对象含恒输出的 external_profile:{}，
    // 与 Java adapter 行为一致，故不做逐字比对）
    let re: serde_json::Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    let orig: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(re["errcode"], orig["errcode"]);
    assert_eq!(re["tagname"], orig["tagname"]);
    assert_eq!(re["partylist"], orig["partylist"]);
    assert_eq!(re["userlist"][0]["userid"], orig["userlist"][0]["userid"]);
}

/// Java `WxCpTpTagServiceImplTest` golden：`WxCpTpTagGetResult`（类型别名）。
#[test]
fn tp_tag_get_result_golden() {
    let json = r#"{"errcode":0,"errmsg":"ok","tagname":"乒乓球协会","userlist":[{"userid":"zhangsan","name":"李四"}],"partylist":[2]}"#;
    let result = WxCpTpTagGetResult::from_json(json).unwrap();
    assert_eq!(result.tagname.as_deref(), Some("乒乓球协会"));
    assert_eq!(result.partylist.as_deref(), Some(&[2][..]));
    let users = result.userlist.as_ref().unwrap();
    assert_eq!(users[0].user_id.as_deref(), Some("zhangsan"));
    assert_eq!(users[0].name.as_deref(), Some("李四"));
}

/// `WxCpDepartGsonAdapter`：id/name/name_en/department_leader/parentid/order
/// 往返（字段顺序照 adapter）。
#[test]
fn depart_roundtrip() {
    let json = r#"{"id":1,"name":"产品部","name_en":"pd","department_leader":["zhangsan","lisi"],"parentid":0,"order":2}"#;
    let depart = WxCpDepart::from_json(json).unwrap();
    assert_eq!(depart.id, Some(1));
    assert_eq!(depart.name.as_deref(), Some("产品部"));
    assert_eq!(depart.en_name.as_deref(), Some("pd"));
    assert_eq!(
        depart.department_leader.as_deref(),
        Some(&["zhangsan".to_string(), "lisi".to_string()][..])
    );
    assert_eq!(depart.parent_id, Some(0));
    assert_eq!(depart.order, Some(2));
    assert_eq!(depart.to_json().unwrap(), json);

    // 空 department_leader 省略
    let mut d2 = WxCpDepart::default();
    d2.id = Some(1);
    assert_eq!(d2.to_json().unwrap(), r#"{"id":1}"#);
}

/// `WxCpChatGsonAdapter`：chatid/name/owner/userlist 往返。
#[test]
fn chat_roundtrip() {
    let json = r#"{"chatid":"wrOgQhDgAAcwMTB7YmDkviVs6T1Mxxvw","name":"群聊1","owner":"zhangsan","userlist":["zhangsan","lisi"]}"#;
    let chat = WxCpChat::from_json(json).unwrap();
    assert_eq!(chat.id.as_deref(), Some("wrOgQhDgAAcwMTB7YmDkviVs6T1Mxxvw"));
    assert_eq!(chat.name.as_deref(), Some("群聊1"));
    assert_eq!(chat.owner.as_deref(), Some("zhangsan"));
    assert_eq!(
        chat.users.as_deref(),
        Some(&["zhangsan".to_string(), "lisi".to_string()][..])
    );
    assert_eq!(chat.to_json().unwrap(), json);

    // userlist 为空时省略（Java adapter 判断非空才输出）
    let mut c2 = WxCpChat::default();
    c2.id = Some("wx1".to_string());
    assert_eq!(c2.to_json().unwrap(), r#"{"chatid":"wx1"}"#);
}

/// `WxCpKfGetCorpStatisticResp`（继承 WxCpBaseResp）：全字段解析 + 往返。
#[test]
fn kf_corp_statistic_resp_golden() {
    let json = r#"{"errcode":0,"errmsg":"ok","statistic_list":[{"stat_time":1650000000,"statistic":{"session_cnt":1,"customer_cnt":2,"customer_msg_cnt":3,"upgrade_service_customer_cnt":4,"ai_session_reply_cnt":5,"ai_transfer_rate":0.5,"ai_knowledge_hit_rate":0.6,"msg_rejected_customer_cnt":7}}]}"#;
    let resp = WxCpKfGetCorpStatisticResp::from_json(json).unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
    assert_eq!(resp.statistic_list.len(), 1);
    let item = &resp.statistic_list[0];
    assert_eq!(item.stat_time, 1650000000);
    assert_eq!(item.statistic.session_cnt, 1);
    assert_eq!(item.statistic.customer_cnt, 2);
    assert_eq!(item.statistic.customer_msg_cnt, 3);
    assert_eq!(item.statistic.upgrade_service_customer_cnt, 4);
    assert_eq!(item.statistic.ai_session_reply_cnt, 5);
    assert_eq!(item.statistic.ai_transfer_rate, 0.5);
    assert_eq!(item.statistic.ai_knowledge_hit_rate, 0.6);
    assert_eq!(item.statistic.msg_rejected_customer_cnt, 7);
    assert_eq!(resp.to_json().unwrap(), json);
}

// ===========================================================================
// 消息路由（镜像 Java `WxCpMessageRouterTest`）
// ===========================================================================

/// 回声处理器（对应 Java `WxEchoCpMessageHandler`）。
struct EchoHandler {
    sb: Arc<Mutex<String>>,
    echo: &'static str,
}

impl WxCpMessageHandler for EchoHandler {
    fn handle(
        &self,
        _wx_message: &WxCpXmlMessage,
        _context: &mut RouteContext,
        _wx_cp_service: Option<&dyn wx_rust_cp::api::WxCpService>,
        _session_manager: &dyn WxSessionManager,
    ) -> Result<Option<WxCpXmlOutMessage>, WxErrorException> {
        let mut sb = self.sb.lock().unwrap();
        sb.push_str(self.echo);
        sb.push(',');
        Ok(None)
    }
}

fn echo_handler(sb: &Arc<Mutex<String>>, echo: &'static str) -> Arc<dyn WxCpMessageHandler> {
    Arc::new(EchoHandler {
        sb: Arc::clone(sb),
        echo,
    })
}

/// 自定义匹配器（对应 Java 匿名 matcher：format == "strangeformat"）。
struct FormatMatcher;

impl WxCpMessageMatcher for FormatMatcher {
    fn match_message(&self, message: &WxCpXmlMessage) -> bool {
        message.format.as_deref() == Some("strangeformat")
    }
}

/// 拦截器：format == "blockme" 时放行，其余全部阻断。
struct BlockUnlessStrangeFormat;

impl WxCpMessageInterceptor for BlockUnlessStrangeFormat {
    fn intercept(
        &self,
        wx_message: &WxCpXmlMessage,
        _context: &mut RouteContext,
        _wx_cp_service: Option<&dyn wx_rust_cp::api::WxCpService>,
        _session_manager: &dyn WxSessionManager,
    ) -> bool {
        wx_message.format.as_deref() == Some("blockme")
    }
}

/// 构建与 Java `prepare()` 相同的 10 条规则（从细到粗）。
fn prepare(router: &mut WxCpMessageRouter, sb: &Arc<Mutex<String>>) {
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

/// 消息夹具（对应 Java `messages-1` dataProvider 的 10 个消息）。
///
/// 每条消息设唯一 from_user：Java 测试进程内单例去重器（TTL 15s）在
/// msgId/createTime 均为 null 时会对 10 条消息生成相同去重 id；Rust 侧
/// 用唯一 from_user 规避（与 wx-rust-mp `source_parity_router.rs` 同一模式），
/// 且每个测试使用独立去重器，测试间互不污染。
fn messages() -> Vec<(WxCpXmlMessage, &'static str)> {
    let mut v = Vec::new();
    let base = |msg_type: Option<&str>,
                event: Option<&str>,
                event_key: Option<&str>,
                content: Option<&str>,
                format: Option<&str>| {
        let mut m = WxCpXmlMessage::default();
        m.msg_type = msg_type.map(str::to_string);
        m.event = event.map(str::to_string);
        m.event_key = event_key.map(str::to_string);
        m.content = content.map(str::to_string);
        m.format = format.map(str::to_string);
        m
    };

    let mut m1 = base(Some("text"), None, None, None, None);
    m1.from_user_name = Some("m1".to_string());
    v.push((m1, "text,"));

    let mut m2 = base(None, Some("CLICK"), None, None, None);
    m2.from_user_name = Some("m2".to_string());
    v.push((m2, "CLICK,"));

    let mut m3 = base(None, None, Some("KEY_1"), None, None);
    m3.from_user_name = Some("m3".to_string());
    v.push((m3, "KEY_1,"));

    let mut m4 = base(None, None, None, Some("CONTENT_1"), None);
    m4.from_user_name = Some("m4".to_string());
    v.push((m4, "CONTENT_1,"));

    let mut m5 = base(None, None, None, Some("BLA"), None);
    m5.from_user_name = Some("m5".to_string());
    v.push((m5, "ALL,"));

    let mut m6 = base(None, None, None, Some("abcd"), None);
    m6.from_user_name = Some("m6".to_string());
    v.push((m6, "abcd,"));

    let mut m7 = base(None, None, None, None, Some("strangeformat"));
    m7.from_user_name = Some("m7".to_string());
    v.push((m7, "matcher,"));

    let mut c2 = base(Some("text"), Some("CLICK"), None, None, None);
    c2.from_user_name = Some("c2".to_string());
    v.push((c2, "COMBINE_2,"));

    let mut c3 = base(Some("text"), Some("CLICK"), Some("KEY_1"), None, None);
    c3.from_user_name = Some("c3".to_string());
    v.push((c3, "COMBINE_3,"));

    let mut c4 = base(
        Some("text"),
        Some("CLICK"),
        Some("KEY_1"),
        Some("CONTENT_1"),
        None,
    );
    c4.from_user_name = Some("c4".to_string());
    v.push((c4, "COMBINE_4,"));

    v
}

/// 新建路由器：独立去重器 + 标准会话管理器（对应 Java 默认组件）。
fn new_router() -> WxCpMessageRouter {
    let mut router = WxCpMessageRouter::new(None);
    router.set_message_duplicate_checker(Arc::new(WxMessageInMemoryDuplicateChecker::new()));
    router
}

/// Java `WxCpMessageRouterTest.testSync`：10 条消息 × 同步规则分发 golden。
#[tokio::test]
async fn router_sync_dispatch_matches_java() {
    for (msg, expected) in messages() {
        let sb = Arc::new(Mutex::new(String::new()));
        let mut router = new_router();
        prepare(&mut router, &sb);
        let res = router.route(&msg).await;
        assert!(res.is_none());
        assert_eq!(*sb.lock().unwrap(), expected, "消息: {msg:?}");
    }
}

/// Java 测试语义：content 匹配时消息内容 trim 后比较（trimToNull），
/// rContent/eventKeyRegex 整串匹配。
#[tokio::test]
async fn router_content_and_regex_semantics() {
    // content("CONTENT_1") 匹配带空白的内容（Java trimToNull）
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = new_router();
    router
        .rule()
        .async_exec(false)
        .content("CONTENT_1")
        .handler(echo_handler(&sb, "hit"))
        .end();
    let mut m = WxCpXmlMessage::default();
    m.from_user_name = Some("c1".to_string());
    m.content = Some("  CONTENT_1  ".to_string());
    router.route(&m).await;
    assert_eq!(*sb.lock().unwrap(), "hit,");

    // rContent 整串匹配（Java Pattern.matches）
    let sb2 = Arc::new(Mutex::new(String::new()));
    let mut router2 = new_router();
    router2
        .rule()
        .async_exec(false)
        .r_content(".*bc.*")
        .handler(echo_handler(&sb2, "abcd"))
        .end()
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb2, "ALL"))
        .end();
    let mut m2 = WxCpXmlMessage::default();
    m2.from_user_name = Some("c2".to_string());
    m2.content = Some("xbcx".to_string());
    router2.route(&m2).await;
    assert_eq!(*sb2.lock().unwrap(), "abcd,");

    // eventKeyRegex 整串匹配（Java Pattern.matches，trimToEmpty）
    let sb3 = Arc::new(Mutex::new(String::new()));
    let mut router3 = new_router();
    router3
        .rule()
        .async_exec(false)
        .event_key_regex("KEY_1.*")
        .handler(echo_handler(&sb3, "k1"))
        .end();
    let mut m3 = WxCpXmlMessage::default();
    m3.from_user_name = Some("c3".to_string());
    m3.event_key = Some("KEY_123".to_string());
    router3.route(&m3).await;
    assert_eq!(*sb3.lock().unwrap(), "k1,");
}

/// 拦截器阻断 + agentId 匹配 + next() 重入语义。
#[tokio::test]
async fn router_interceptor_agent_id_and_reenter() {
    // 拦截器阻断：format != "blockme" 时 handler 不被调用
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = new_router();
    router
        .rule()
        .async_exec(false)
        .interceptor(Arc::new(BlockUnlessStrangeFormat))
        .handler(echo_handler(&sb, "blocked"))
        .end();
    let mut m = WxCpXmlMessage::default();
    m.from_user_name = Some("b1".to_string());
    m.format = Some("other".to_string());
    router.route(&m).await;
    assert_eq!(*sb.lock().unwrap(), "");

    // 拦截器放行
    let sb2 = Arc::new(Mutex::new(String::new()));
    let mut router2 = new_router();
    router2
        .rule()
        .async_exec(false)
        .interceptor(Arc::new(BlockUnlessStrangeFormat))
        .handler(echo_handler(&sb2, "ok"))
        .end();
    let mut m2 = WxCpXmlMessage::default();
    m2.from_user_name = Some("b2".to_string());
    m2.format = Some("blockme".to_string());
    router2.route(&m2).await;
    assert_eq!(*sb2.lock().unwrap(), "ok,");

    // agentId 匹配（Java `agentId.equals(Integer.valueOf(getAgentId()))`）
    let sb3 = Arc::new(Mutex::new(String::new()));
    let mut router3 = new_router();
    router3
        .rule()
        .async_exec(false)
        .agent_id(1000004)
        .handler(echo_handler(&sb3, "agent"))
        .end();
    let mut m3 = WxCpXmlMessage::default();
    m3.from_user_name = Some("b3".to_string());
    m3.agent_id = Some("1000004".to_string());
    router3.route(&m3).await;
    assert_eq!(*sb3.lock().unwrap(), "agent,");
    let mut m3b = WxCpXmlMessage::default();
    m3b.from_user_name = Some("b3b".to_string());
    m3b.agent_id = Some("1".to_string());
    router3.route(&m3b).await;
    assert_eq!(*sb3.lock().unwrap(), "agent,"); // 不匹配，不再追加

    // next()：匹配后继续进入其他规则（Java `isReEnter()`）
    let sb4 = Arc::new(Mutex::new(String::new()));
    let mut router4 = new_router();
    router4
        .rule()
        .async_exec(false)
        .content("x")
        .handler(echo_handler(&sb4, "first"))
        .next()
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb4, "second"))
        .end();
    let mut m4 = WxCpXmlMessage::default();
    m4.from_user_name = Some("b4".to_string());
    m4.content = Some("x".to_string());
    router4.route(&m4).await;
    assert_eq!(*sb4.lock().unwrap(), "first,second,");
}

/// 重复消息跳过（Java `isMsgDuplicated` → route 返回 null）。
#[tokio::test]
async fn router_duplicate_message_skipped() {
    let sb = Arc::new(Mutex::new(String::new()));
    let mut router = new_router();
    router
        .rule()
        .async_exec(false)
        .handler(echo_handler(&sb, "once"))
        .end();

    let mut m = WxCpXmlMessage::default();
    m.msg_id = Some("123456789".to_string());
    m.create_time = Some(1348831860);
    m.from_user_name = Some("dup1".to_string());

    let res1 = router.route(&m).await;
    assert!(res1.is_none());
    // 同一消息再次路由：去重 → 直接返回 null，handler 不再执行
    let res2 = router.route(&m).await;
    assert!(res2.is_none());
    assert_eq!(*sb.lock().unwrap(), "once,");
}

/// 异步规则执行（Java `testAsync`：async=true 时规则提交线程池执行，
/// route 返回 null，handler 异步追加）。
#[tokio::test]
async fn router_async_execution() {
    let done = Arc::new(AtomicBool::new(false));
    struct AsyncHandler {
        done: Arc<AtomicBool>,
    }
    impl WxCpMessageHandler for AsyncHandler {
        fn handle(
            &self,
            _wx_message: &WxCpXmlMessage,
            _context: &mut RouteContext,
            _wx_cp_service: Option<&dyn wx_rust_cp::api::WxCpService>,
            _session_manager: &dyn WxSessionManager,
        ) -> Result<Option<WxCpXmlOutMessage>, WxErrorException> {
            self.done.store(true, Ordering::SeqCst);
            Ok(None)
        }
    }

    let mut router = new_router();
    router
        .rule()
        .async_exec(true) // 默认即 true，显式声明
        .handler(Arc::new(AsyncHandler {
            done: Arc::clone(&done),
        }))
        .end();

    let mut m = WxCpXmlMessage::default();
    m.from_user_name = Some("async1".to_string());
    let res = router.route(&m).await;
    // 异步规则：route 立即返回 null（对应 Java 异步不返回结果）
    assert!(res.is_none());

    // 轮询等待异步任务完成（避免固定 sleep 抖动）
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while !done.load(Ordering::SeqCst) {
        assert!(
            tokio::time::Instant::now() < deadline,
            "异步 handler 未在超时内执行"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

// ===========================================================================
// 会话存档 RSA/AES 解密（对应 Java WxCpCryptUtil.decryptPriKey +
// WxCpMsgAuditServiceImpl.decryptChatData）
// ===========================================================================

/// 2048-bit RSA 私钥（PKCS1 传统格式，单行 base64，Java 测试配置形态）。
const MSG_AUDIT_PRI_KEY_PKCS1: &str = "MIIEowIBAAKCAQEAliNgp88WTEs0Y9UPISI1JVZ9piXq7sXJCfgzD4MJuzyFa8QQRjA8z/tfbFuo54ck18CSBaKybgs1wHPHSjKj4TUYywZhbpog0fG3WXOlV3iZWBlblC6uo05dj2K/Nl0529o3ihx3gjV1l0u5RqLHuz3y8mlhP1sMnXQqP9+a25F+30UIo9vnPIKxfPFako79QHeUqPrWhDjYxqwQN3PaNMtepgqJ+VgNxmE6jQlotjL5UCesdx0pu6vJoEhh8Gu3giEejGQLHI5SWnmCFsZYATY+B+caed6aVU6TbCdhoJW9cNMqHYk+DYbLiXhqUyNjM4s664U/MP8seVBNHnkUZwIDAQABAoIBAALD3gjZKIj5LZrVFrU+gwoQALppol0JaX10g8Q6XpiDMdXdQ4t3YfdWothFC+RPud77XfUanPyn2jKUL0xviFMifjVnBa5aylXzhLPyDE6DGxfjR6t//JlnXRPB9tGSf/lLmJBDm/Pa9jJ2I5BoeRM6vdm3FSJsk9vmgsywWpLGoPogURt5wDe73yylZqlXN7ldpd5ymwiEa7geQgVD+5gzXJpgnhvcDroeET0N+vt4b2eVlrkl9YTY6R+k9rmXV3nhC/06CA+UC0/ZkKjlkJWRpEYRXLazORYqq/WIEZl9tTZ7aUs56XKl0speXucWB1WmNTA8bJPVHNfz7XtzCZECgYEA0O1hh0XIEzpSfGKQNEVXT0/JYyXuMlJ987V7puNEifeXvTsmXLAXtyqGEotzvDoNffChww0dlXcwhHzWDdvO2w+OniPH+Vp8JsMcOxhBmDanf55XHn6Qw5Xtos/mp7YY0RsxB4kQIC5SxkbWegNzNBpG743Zq/lmg4Iyp9UcRUkCgYEAt/cg9rNew1y7/4uEStZB2KjpvhyBnc4pInXwioRUdb29vyqUU1y6l+W1ii5KXy+0CXkwKOL15LFoZlHgikJpTrhz/NndVX/f7UHqbNyJGeda0Eo/27rXHDuBk3UCV5jECcaMVjI6nhJCxjodUjPrDrdgHQjp+ItwpH9UfHvxfC8CgYEAz3Hj1JzE+9J89GQpKr1u46ElJP0UdYLS7JjWHe6qrxLX51xDyuRNp+DhHqi4UbySiOnUtAbhyPZ/hg0tvLTzvy7DPOHqPPxLKXGxJjv/ZVNfHKfreKvSsWC8g1xKeVulaHrHQ+QjztAwAA92BdBHIwS0SkdeShNoZktwYJSJbYECgYBmeVh5U20j8dIfBP5KGZNQNrcNWR+yYZZ0RbIZ1qCdfXXmuLcqwGqIFkLLmAGBiDcHV+RFwjcqjtASvuUO3vN+EC072bf62mJH7QHCZg5QyZjCkbmEkk0kwtjo5LLLTKNWtqyRjyUrzAw6E0El+xmidu8o/UAnXLUATJUpSnk3rQKBgCfYzrHXrV9/eksbCkZD70emfPjEXLGVZ+Xu0mdjboCrrRbWe6CAb+LtMv/4cqUjb6ewDneNousVm0dG9XXGoekpB/MWxj6+AiwE1kHBk4Z3nz7jXkQnQ5Jb/N3Ltp+rNHVrXelD9XWBgPg9gFD+KmQ7CisxsgVJnWm9VRwIIDuw";

/// 2048-bit RSA 私钥（PKCS8 格式，单行 base64）。
const MSG_AUDIT_PRI_KEY_PKCS8: &str = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCWI2CnzxZMSzRj1Q8hIjUlVn2mJeruxckJ+DMPgwm7PIVrxBBGMDzP+19sW6jnhyTXwJIForJuCzXAc8dKMqPhNRjLBmFumiDR8bdZc6VXeJlYGVuULq6jTl2PYr82XTnb2jeKHHeCNXWXS7lGose7PfLyaWE/WwyddCo/35rbkX7fRQij2+c8grF88VqSjv1Ad5So+taEONjGrBA3c9o0y16mCon5WA3GYTqNCWi2MvlQJ6x3HSm7q8mgSGHwa7eCIR6MZAscjlJaeYIWxlgBNj4H5xp53ppVTpNsJ2Gglb1w0yodiT4NhsuJeGpTI2MzizrrhT8w/yx5UE0eeRRnAgMBAAECggEAAsPeCNkoiPktmtUWtT6DChAAummiXQlpfXSDxDpemIMx1d1Di3dh91ai2EUL5E+53vtd9Rqc/KfaMpQvTG+IUyJ+NWcFrlrKVfOEs/IMToMbF+NHq3/8mWddE8H20ZJ/+UuYkEOb89r2MnYjkGh5Ezq92bcVImyT2+aCzLBaksag+iBRG3nAN7vfLKVmqVc3uV2l3nKbCIRruB5CBUP7mDNcmmCeG9wOuh4RPQ36+3hvZ5WWuSX1hNjpH6T2uZdXeeEL/ToID5QLT9mQqOWQlZGkRhFctrM5Fiqr9YgRmX21NntpSznpcqXSyl5e5xYHVaY1MDxsk9Uc1/Pte3MJkQKBgQDQ7WGHRcgTOlJ8YpA0RVdPT8ljJe4yUn3ztXum40SJ95e9OyZcsBe3KoYSi3O8Og198KHDDR2VdzCEfNYN287bD46eI8f5Wnwmwxw7GEGYNqd/nlcefpDDle2iz+anthjRGzEHiRAgLlLGRtZ6A3M0Gkbvjdmr+WaDgjKn1RxFSQKBgQC39yD2s17DXLv/i4RK1kHYqOm+HIGdzikidfCKhFR1vb2/KpRTXLqX5bWKLkpfL7QJeTAo4vXksWhmUeCKQmlOuHP82d1Vf9/tQeps3IkZ51rQSj/butccO4GTdQJXmMQJxoxWMjqeEkLGOh1SM+sOt2AdCOn4i3Ckf1R8e/F8LwKBgQDPcePUnMT70nz0ZCkqvW7joSUk/RR1gtLsmNYd7qqvEtfnXEPK5E2n4OEeqLhRvJKI6dS0BuHI9n+GDS28tPO/LsM84eo8/EspcbEmO/9lU18cp+t4q9KxYLyDXEp5W6VoesdD5CPO0DAAD3YF0EcjBLRKR15KE2hmS3BglIltgQKBgGZ5WHlTbSPx0h8E/koZk1A2tw1ZH7JhlnRFshnWoJ19dea4tyrAaogWQsuYAYGINwdX5EXCNyqO0BK+5Q7e834QLTvZt/raYkftAcJmDlDJmMKRuYSSTSTC2OjksstMo1a2rJGPJSvMDDoTQSX7GaJ27yj9QCdctQBMlSlKeTetAoGAJ9jOsdetX396SxsKRkPvR6Z8+MRcsZVn5e7SZ2NugKutFtZ7oIBv4u0y//hypSNvp7AOd42i6xWbR0b1dcah6SkH8xbGPr4CLATWQcGThnefPuNeRCdDklv83cu2n6s0dWtd6UP1dYGA+D2AUP4qZDsKKzGyBUmdab1VHAggO7A=";

/// 确定性 RNG（xorshift64*，实现 rand_core 0.6 的 RngCore/CryptoRng，
/// 用于 PKCS1 v1.5 填充的随机源；测试向量自构造，无需系统随机源）。
struct XorShift(u64);

impl rand_core::RngCore for XorShift {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl rand_core::CryptoRng for XorShift {}

/// 用测试私钥的公钥按 PKCS1 v1.5 加密 32 字节 AES 密钥（构造
/// `encrypt_random_key`，对应 Java 侧管理端公钥加密）。
fn rsa_encrypt_random_key(aes_key: &[u8], private_key_der: &[u8], pkcs1: bool) -> String {
    use base64::Engine as _;
    use rsa::pkcs1::DecodeRsaPrivateKey as _;
    use rsa::pkcs8::DecodePrivateKey as _;

    let private_key = if pkcs1 {
        rsa::RsaPrivateKey::from_pkcs1_der(private_key_der).unwrap()
    } else {
        rsa::RsaPrivateKey::from_pkcs8_der(private_key_der).unwrap()
    };
    let public_key = rsa::RsaPublicKey::from(&private_key);
    let mut rng = XorShift(0x9E3779B97F4A7C15);
    let encrypted = public_key
        .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, aes_key)
        .unwrap();
    base64::engine::general_purpose::STANDARD.encode(encrypted)
}

/// 用 AES-256-CBC（key 为 32 字节、IV = key 前 16 字节、PKCS7）加密明文
/// （构造 `encrypt_chat_msg`，与解密函数互为逆运算）。
fn aes_encrypt_chat_msg(aes_key: &[u8], plaintext: &[u8]) -> String {
    use aes::Aes256;
    use base64::Engine as _;
    use cbc::cipher::block_padding::Pkcs7;
    use cbc::cipher::{BlockModeEncrypt, KeyIvInit};
    type Aes256CbcEnc = cbc::Encryptor<Aes256>;

    let cipher = Aes256CbcEnc::new_from_slices(&aes_key[..32], &aes_key[..16]).unwrap();
    let mut buf = vec![0u8; plaintext.len() + 32];
    let out = cipher
        .encrypt_padded_b2b::<Pkcs7>(plaintext, &mut buf)
        .unwrap();
    base64::engine::general_purpose::STANDARD.encode(out)
}

/// 单行私钥字符串 → DER 字节（与 Java 测试配置的私钥形态一致）。
fn single_line_to_der(single_line: &str) -> Vec<u8> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD
        .decode(single_line)
        .unwrap()
}

/// Java `WxCpCryptUtil.decryptPriKeyByPKCS8` 往返：公钥加密 → 私钥解密。
#[test]
fn decrypt_pri_key_pkcs8_roundtrip() {
    let aes_key = b"0123456789abcdef0123456789abcdef"; // 32 字节
    let der = single_line_to_der(MSG_AUDIT_PRI_KEY_PKCS8);
    let encrypt_random_key = rsa_encrypt_random_key(aes_key, &der, false);

    let decrypted = decrypt_pri_key(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS8, Some(2)).unwrap();
    assert_eq!(decrypted.as_bytes(), aes_key);
    // 显式 PKCS8 入口等价
    assert_eq!(
        decrypt_pri_key_by_pkcs8(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS8).unwrap(),
        decrypted
    );
}

/// Java `WxCpCryptUtil.decryptPriKeyByPKCS1` 往返（PKCS1 传统格式私钥）。
#[test]
fn decrypt_pri_key_pkcs1_roundtrip() {
    let aes_key = b"abcdef0123456789abcdef0123456789"; // 32 字节
    let der = single_line_to_der(MSG_AUDIT_PRI_KEY_PKCS1);
    let encrypt_random_key = rsa_encrypt_random_key(aes_key, &der, true);

    let decrypted = decrypt_pri_key(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS1, Some(1)).unwrap();
    assert_eq!(decrypted.as_bytes(), aes_key);
    assert_eq!(
        decrypt_pri_key_by_pkcs1(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS1).unwrap(),
        decrypted
    );
}

/// Java `WxCpMsgAuditServiceImpl.decryptChatData` 全流程往返：
/// RSA 解密随机密钥 → AES-256-CBC 解密消息明文，结果可解析为
/// `WxCpChatModel`（对应 Java `getDecryptChatData`）。
#[test]
fn decrypt_chat_data_full_flow_roundtrip() {
    // Java `WxCpMsgAuditTest` 的文本消息明文（与官方文档样例同构）
    let plaintext = r#"{"msgid":"CAQQluDa4QUY0On2rYSAgAMgzPrShAE=","action":"send","from":"XuJinSheng","tolist":["icefog"],"roomid":"","msgtime":1547087894783,"msgtype":"text","text":{"content":"这是一条引用/回复消息：\"\n------\n@nick777"}}"#;

    let aes_key = b"msgaudit-aes-key-0123456789abcdef"; // 32 字节
    let der = single_line_to_der(MSG_AUDIT_PRI_KEY_PKCS8);
    let encrypt_random_key = rsa_encrypt_random_key(aes_key, &der, false);
    let encrypt_chat_msg = aes_encrypt_chat_msg(aes_key, plaintext.as_bytes());

    let decrypted = decrypt_chat_data(
        &encrypt_random_key,
        &encrypt_chat_msg,
        MSG_AUDIT_PRI_KEY_PKCS8,
        Some(2),
    )
    .unwrap();
    assert_eq!(decrypted, plaintext);

    // 拆解两段式入口等价：RSA 解出密钥后直接 AES 解密
    let key = decrypt_pri_key(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS8, Some(2)).unwrap();
    assert_eq!(
        decrypt_encrypt_chat_msg(&key, &encrypt_chat_msg).unwrap(),
        plaintext
    );

    // 明文可解析为 WxCpChatModel（对应 Java `WxCpChatModel.fromJson`）
    let model = wx_rust_cp::bean::msgaudit::WxCpChatModel::from_json(&decrypted).unwrap();
    assert_eq!(model.msg_type, "text");
    assert_eq!(model.from, "XuJinSheng");
    assert_eq!(
        model.text.content,
        "这是一条引用/回复消息：\"\n------\n@nick777"
    );
    assert_eq!(model.msg_time, 1547087894783);
}

/// 未配置解密方式（pkcs1=None）报错，对应 Java
/// `WxErrorException("请配置会话存档解密方式")`。
#[test]
fn decrypt_pri_key_requires_pkcs_mode() {
    let err = decrypt_pri_key("any", "any", None).unwrap_err();
    assert_eq!(err, "请配置会话存档解密方式");
}

/// 私钥格式错误 / 密文错误 → 返回可读错误而非 panic。
#[test]
fn decrypt_pri_key_error_paths() {
    assert!(decrypt_pri_key_by_pkcs8("AAAA", "not-a-key").is_err());
    assert!(decrypt_pri_key_by_pkcs1("AAAA", "not-a-key").is_err());

    let der = single_line_to_der(MSG_AUDIT_PRI_KEY_PKCS8);
    let aes_key = b"0123456789abcdef0123456789abcdef";
    let encrypt_random_key = rsa_encrypt_random_key(aes_key, &der, false);
    // 错误的私钥 → 解密失败
    assert!(decrypt_pri_key_by_pkcs8(&encrypt_random_key, MSG_AUDIT_PRI_KEY_PKCS1).is_err());

    // AES 密钥不足 32 字节 → 明确报错
    let err = decrypt_encrypt_chat_msg("too-short", "AAAA").unwrap_err();
    assert!(err.contains("AES 密钥长度不足"));
}
