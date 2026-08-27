//! Top-15 未镜像 Java 测试类批量补测——miniapp 模块。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxMaMessageTest（576 行）

use wx_rust_miniapp::message::*;

// ═══════════════════════════════════════════════════════════════
// #3 WxMaMessageTest（576 行）—— 小程序消息解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMessageTest.testFromXml
#[test]
fn test_wx_ma_message_from_xml() {
    let xml = "<xml>\n\
        <ToUserName><![CDATA[toUser]]></ToUserName>\n\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\n\
        <CreateTime>1482048670</CreateTime>\n\
        <MsgType><![CDATA[text]]></MsgType>\n\
        <Content><![CDATA[this is a test]]></Content>\n\
        <MsgId>1234567890123456</MsgId>\n\
        <PicUrl><![CDATA[this is a url]]></PicUrl>\n\
        <MediaId><![CDATA[media_id]]></MediaId>\n\
        <Title><![CDATA[Title]]></Title>\n\
        <AppId><![CDATA[AppId]]></AppId>\n\
        <PagePath><![CDATA[PagePath]]></PagePath>\n\
        <ThumbUrl><![CDATA[ThumbUrl]]></ThumbUrl>\n\
        <ThumbMediaId><![CDATA[ThumbMediaId]]></ThumbMediaId>\n\
        <Event><![CDATA[user_enter_tempsession]]></Event>\n\
        <SessionFrom><![CDATA[sessionFrom]]></SessionFrom>\n\
        </xml>";
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user.as_deref(), Some("toUser"));
    assert_eq!(msg.from_user.as_deref(), Some("fromUser"));
    assert_eq!(msg.create_time, Some(1482048670));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.content.as_deref(), Some("this is a test"));
    assert_eq!(msg.msg_id, Some(1234567890123456));
    assert_eq!(msg.pic_url.as_deref(), Some("this is a url"));
    assert_eq!(msg.media_id.as_deref(), Some("media_id"));
    assert_eq!(msg.title.as_deref(), Some("Title"));
    assert_eq!(msg.page_path.as_deref(), Some("PagePath"));
    assert_eq!(msg.thumb_url.as_deref(), Some("ThumbUrl"));
    assert_eq!(msg.thumb_media_id.as_deref(), Some("ThumbMediaId"));
    assert_eq!(msg.app_id.as_deref(), Some("AppId"));
    assert_eq!(msg.event.as_deref(), Some("user_enter_tempsession"));
    assert_eq!(msg.session_from.as_deref(), Some("sessionFrom"));
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgPopupEvent（XML 格式）
#[test]
fn test_wx_ma_message_subscribe_msg_popup_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>",
        "<FromUserName><![CDATA[otFpruAK8D-E6EfStSYonYSBZ8_4]]></FromUserName>",
        "<CreateTime>1610969440</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[subscribe_msg_popup_event]]></Event>",
        "<SubscribeMsgPopupEvent>",
        " <List>",
        "   <TemplateId><![CDATA[VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc]]></TemplateId>",
        "   <SubscribeStatusString><![CDATA[accept]]></SubscribeStatusString>",
        "   <PopupScene>0</PopupScene>",
        " </List>",
        "</SubscribeMsgPopupEvent>",
        "</xml>"
    );
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user.as_deref(), Some("gh_123456789abc"));
    assert_eq!(
        msg.from_user.as_deref(),
        Some("otFpruAK8D-E6EfStSYonYSBZ8_4")
    );
    assert_eq!(msg.create_time, Some(1610969440));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("subscribe_msg_popup_event"));
    let popup = msg
        .subscribe_msg_popup_event
        .as_ref()
        .expect("SubscribeMsgPopupEvent 存在");
    assert_eq!(popup.list.len(), 1);
    assert_eq!(
        popup.list[0].template_id,
        "VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc"
    );
    assert_eq!(popup.list[0].subscribe_status_string, "accept");
    assert_eq!(popup.list[0].popup_scene, "0");
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgPopupEvent（JSON 格式，对象）
#[test]
fn test_wx_ma_message_subscribe_msg_popup_event_json_object() {
    let json = r#"{
        "ToUserName": "gh_123456789abc",
        "FromUserName": "otFpruAK8D-E6EfStSYonYSBZ8_4",
        "CreateTime": 1610969440,
        "MsgType": "event",
        "Event": "subscribe_msg_popup_event",
        "List": {
            "TemplateId": "VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc",
            "SubscribeStatusString": "accept",
            "PopupScene": "0"
        }
    }"#;
    let msg = WxMaMessage::from_json(json).expect("解析 JSON 成功");
    assert_eq!(msg.to_user.as_deref(), Some("gh_123456789abc"));
    assert_eq!(
        msg.from_user.as_deref(),
        Some("otFpruAK8D-E6EfStSYonYSBZ8_4")
    );
    assert_eq!(msg.create_time, Some(1610969440));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("subscribe_msg_popup_event"));
    let popup = msg
        .subscribe_msg_popup_event
        .as_ref()
        .expect("SubscribeMsgPopupEvent 存在");
    assert_eq!(popup.list.len(), 1);
    assert_eq!(
        popup.list[0].template_id,
        "VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc"
    );
    assert_eq!(popup.list[0].subscribe_status_string, "accept");
    assert_eq!(popup.list[0].popup_scene, "0");
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgPopupEvent（JSON 格式，数组）
#[test]
fn test_wx_ma_message_subscribe_msg_popup_event_json_array() {
    let json = r#"{
        "ToUserName": "gh_123456789abc",
        "FromUserName": "otFpruAK8D-E6EfStSYonYSBZ8_4",
        "CreateTime": 1610969440,
        "MsgType": "event",
        "Event": "subscribe_msg_popup_event",
        "List": [{
            "TemplateId": "VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc",
            "SubscribeStatusString": "accept",
            "PopupScene": "0"
        }]
    }"#;
    let msg = WxMaMessage::from_json(json).expect("解析 JSON 成功");
    let popup = msg
        .subscribe_msg_popup_event
        .as_ref()
        .expect("SubscribeMsgPopupEvent 存在");
    assert_eq!(popup.list.len(), 1);
    assert_eq!(
        popup.list[0].template_id,
        "VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc"
    );
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgChangeEvent（XML 格式）
#[test]
fn test_wx_ma_message_subscribe_msg_change_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>",
        "<FromUserName><![CDATA[o7esq5OI1Uej6Xixw1lA2H7XDVbc]]></FromUserName>",
        "<CreateTime>1610968440</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[subscribe_msg_change_event]]></Event>",
        "<SubscribeMsgChangeEvent>",
        "    <List>",
        "         <TemplateId><![CDATA[BEwX0BOT3MqK3Uc5oTU3CGBqzjpndk2jzUf7VfExd8]]></TemplateId>",
        "        <SubscribeStatusString><![CDATA[reject]]></SubscribeStatusString>",
        "    </List>",
        "</SubscribeMsgChangeEvent>",
        "</xml>"
    );
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user.as_deref(), Some("gh_123456789abc"));
    assert_eq!(
        msg.from_user.as_deref(),
        Some("o7esq5OI1Uej6Xixw1lA2H7XDVbc")
    );
    assert_eq!(msg.create_time, Some(1610968440));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("subscribe_msg_change_event"));
    let change = msg
        .subscribe_msg_change_event
        .as_ref()
        .expect("SubscribeMsgChangeEvent 存在");
    assert_eq!(change.list.len(), 1);
    assert_eq!(
        change.list[0].template_id,
        "BEwX0BOT3MqK3Uc5oTU3CGBqzjpndk2jzUf7VfExd8"
    );
    assert_eq!(change.list[0].subscribe_status_string, "reject");
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgChangeEvent（JSON 格式）
#[test]
fn test_wx_ma_message_subscribe_msg_change_event_json() {
    let json = r#"{
        "ToUserName": "gh_123456789abc",
        "FromUserName": "o7esq5OI1Uej6Xixw1lA2H7XDVbc",
        "CreateTime": 1610968440,
        "MsgType": "event",
        "Event": "subscribe_msg_change_event",
        "List": {
            "TemplateId": "BEwX0BOT3MqK3Uc5oTU3CGBqzjpndk2jzUf7VfExd8",
            "SubscribeStatusString": "reject"
        }
    }"#;
    let msg = WxMaMessage::from_json(json).expect("解析 JSON 成功");
    let change = msg
        .subscribe_msg_change_event
        .as_ref()
        .expect("SubscribeMsgChangeEvent 存在");
    assert_eq!(change.list.len(), 1);
    assert_eq!(
        change.list[0].template_id,
        "BEwX0BOT3MqK3Uc5oTU3CGBqzjpndk2jzUf7VfExd8"
    );
    assert_eq!(change.list[0].subscribe_status_string, "reject");
}

/// 对应 Java: WxMaMessageTest.testSubscribeMsgSentEvent（XML 格式）
#[test]
fn test_wx_ma_message_subscribe_msg_sent_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>",
        "<FromUserName><![CDATA[o7esq5PHRGBQYmeNyfG064wEFVpQ]]></FromUserName>",
        "<CreateTime>1620963428</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[subscribe_msg_sent_event]]></Event>",
        "<SubscribeMsgSentEvent>",
        "    <List>",
        "         <TemplateId><![CDATA[TEMPLATE001]]></TemplateId>",
        "        <SubscribeStatusString><![CDATA[accept]]></SubscribeStatusString>",
        "        <MsgId>MSG001</MsgId>",
        "        <ErrorCode>0</ErrorCode>",
        "        <ErrorStatus><![CDATA[success]]></ErrorStatus>",
        "    </List>",
        "</SubscribeMsgSentEvent>",
        "</xml>"
    );
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user.as_deref(), Some("gh_123456789abc"));
    assert_eq!(
        msg.from_user.as_deref(),
        Some("o7esq5PHRGBQYmeNyfG064wEFVpQ")
    );
    assert_eq!(msg.create_time, Some(1620963428));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("subscribe_msg_sent_event"));
    let sent = msg
        .subscribe_msg_sent_event
        .as_ref()
        .expect("SubscribeMsgSentEvent 存在");
    let event = sent.list.as_ref().expect("SentEvent 存在");
    assert_eq!(event.template_id, "TEMPLATE001");
    assert_eq!(event.error_code, "0");
    assert_eq!(event.error_status, "success");
}

/// 对应 Java: WxMaMessageTest（JSON 消息基础解析）
#[test]
fn test_wx_ma_message_from_json_basic() {
    let json = r#"{
        "ToUserName": "gh_test",
        "FromUserName": "user_test",
        "CreateTime": 1620000000,
        "MsgType": "text",
        "Content": "hello world",
        "MsgId": 12345
    }"#;
    let msg = WxMaMessage::from_json(json).expect("解析 JSON 成功");
    assert_eq!(msg.to_user.as_deref(), Some("gh_test"));
    assert_eq!(msg.from_user.as_deref(), Some("user_test"));
    assert_eq!(msg.create_time, Some(1620000000));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.content.as_deref(), Some("hello world"));
    assert_eq!(msg.msg_id, Some(12345));
}

/// 对应 Java: WxMaMessageTest（图片消息解析）
#[test]
fn test_wx_ma_message_image_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[gh_test]]></ToUserName>",
        "<FromUserName><![CDATA[user_test]]></FromUserName>",
        "<CreateTime>1620000000</CreateTime>",
        "<MsgType><![CDATA[image]]></MsgType>",
        "<PicUrl><![CDATA[https://example.com/pic.jpg]]></PicUrl>",
        "<MediaId><![CDATA[media_001]]></MediaId>",
        "<MsgId>67890</MsgId>",
        "</xml>"
    );
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.msg_type.as_deref(), Some("image"));
    assert_eq!(msg.pic_url.as_deref(), Some("https://example.com/pic.jpg"));
    assert_eq!(msg.media_id.as_deref(), Some("media_001"));
    assert_eq!(msg.msg_id, Some(67890));
}

/// 对应 Java: WxMaMessageTest（进入客服会话事件）
#[test]
fn test_wx_ma_message_user_enter_tempsession() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[gh_test]]></ToUserName>",
        "<FromUserName><![CDATA[user_test]]></FromUserName>",
        "<CreateTime>1620000000</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[user_enter_tempsession]]></Event>",
        "<SessionFrom><![CDATA[source_page]]></SessionFrom>",
        "</xml>"
    );
    let msg = WxMaMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("user_enter_tempsession"));
    assert_eq!(msg.session_from.as_deref(), Some("source_page"));
}
