#![allow(clippy::field_reassign_with_default)]
//! 镜像 Java `WxMpTemplateMessageTest` / `WxMpSubscribeMessageTest` /
//! `WxMpKefuMessageTest` / `WxMpMenuTest` 的 bean 序列化黄金。

use wx_rust_mp::bean::kefu::WxMpKefuMessage;
use wx_rust_mp::bean::menu::WxMpMenu;
use wx_rust_mp::bean::subscribe::MiniProgram as SubscribeMiniProgram;
use wx_rust_mp::bean::subscribe::WxMpSubscribeMessage;
use wx_rust_mp::bean::template::{MiniProgram, WxMpTemplateData, WxMpTemplateMessage};

// ---- 镜像 testToJson：模板消息黄金 JSON ----

#[test]
fn template_message_to_json_golden() {
    let tm = WxMpTemplateMessage::builder()
        .to_user("OPENID")
        .template_id("ngqIpbwh8bUfcSsECmogfXcV14J0tQlEpBO27izEYtY")
        .mini_program(MiniProgram::new("xiaochengxuappid12345", "index?foo=bar"))
        .url("http://weixin.qq.com/download")
        .client_msg_id("MSG_000001")
        .add_data(WxMpTemplateData::with_color("first", "haahah", "#FF00FF"))
        .add_data(WxMpTemplateData::with_color("remark", "heihei", "#FF00FF"));

    let expected = "{\"touser\":\"OPENID\",\"template_id\":\"ngqIpbwh8bUfcSsECmogfXcV14J0tQlEpBO27izEYtY\",\"client_msg_id\":\"MSG_000001\",\"url\":\"http://weixin.qq.com/download\",\"miniprogram\":{\"appid\":\"xiaochengxuappid12345\",\"path\":\"index?foo=bar\"},\"data\":{\"first\":{\"value\":\"haahah\",\"color\":\"#FF00FF\"},\"remark\":{\"value\":\"heihei\",\"color\":\"#FF00FF\"}}}";
    assert_eq!(tm.to_json().expect("序列化成功"), expected);
}

// ---- 镜像 testAddData：字段类型截断规则 ----

#[test]
fn template_message_add_data_truncation() {
    let tm = WxMpTemplateMessage::builder()
        .add_data(WxMpTemplateData::new(
            "thing01",
            "张三李四王麻子张三李四王麻子张三李四王麻子张三李四王麻子",
        ))
        .add_data(WxMpTemplateData::new("time01", "2019年10月1日 15:01"))
        .add_data(WxMpTemplateData::new(
            "character_string01",
            "1234567890123456789012345678901234567890",
        ))
        .add_data(WxMpTemplateData::new("amount01", "￥100.21"))
        .add_data(WxMpTemplateData::new(
            "phone_number01",
            "+86-0766-668888661111",
        ))
        .add_data(WxMpTemplateData::new("car_number01", "粤A8Z888挂9"))
        .add_data(WxMpTemplateData::new(
            "const01",
            "支付状态、排队状态、天气状态、物流状态、用药提醒、还款提醒",
        ));

    let data = tm.get_data();
    assert_eq!(data.len(), 7);
    // thing*: >20 → 前 17 字 + "..."
    assert_eq!(data[0].value, "张三李四王麻子张三李四王麻子张三李...");
    // time01: 不截断
    assert_eq!(data[1].value, "2019年10月1日 15:01");
    // character_string*: >32 → 前 29 字 + "..."
    assert_eq!(data[2].value, "12345678901234567890123456789...");
    assert_eq!(data[3].value, "￥100.21");
    // phone_number*: >17 → 前 14 字 + "..."
    assert_eq!(data[4].value, "+86-0766-66888...");
    // car_number*: >8 → 前 5 字 + "..."
    assert_eq!(data[5].value, "粤A8Z8...");
    // const*: >20 → 前 17 字 + "..."
    assert_eq!(data[6].value, "支付状态、排队状态、天气状态、物流...");
}

// ---- WxMpSubscribeMessage：toJson 字段输出 ----

#[test]
fn subscribe_message_to_json() {
    // Java WxMpSubscribeMessageGsonAdapter 线格式：touser/template_id + data map
    let mut m = WxMpSubscribeMessage::default();
    m.to_user = Some("OPENID".to_string());
    m.template_id = Some("TPL_ID".to_string());
    m.page = Some("pages/index".to_string());
    m.data_map.insert("thing1".to_string(), "值1".to_string());
    let json = m.to_json().expect("序列化成功");
    assert!(json.contains("\"touser\":\"OPENID\""), "实际: {json}");
    assert!(json.contains("\"template_id\":\"TPL_ID\""), "实际: {json}");
    assert!(json.contains("\"page\":\"pages/index\""), "实际: {json}");
    assert!(
        json.contains("\"data\":{\"thing1\":{\"value\":\"值1\"}}"),
        "实际: {json}"
    );
}

#[test]
fn subscribe_message_content_to_json() {
    // dataMap 为空 → data.content{value,color}（Java golden 结构）
    let m = WxMpSubscribeMessage::builder()
        .to_user("OPENID")
        .template_id("TEMPLATE_ID")
        .url("URL")
        .mini_program(SubscribeMiniProgram::new(
            "xiaochengxuappid12345",
            "index?foo=bar",
            false,
        ))
        .scene("SCENE")
        .title("TITLE")
        .content_value("VALUE")
        .content_color("COLOR");
    let json = m.to_json().expect("序列化成功");
    assert_eq!(
        json,
        "{\"touser\":\"OPENID\",\"template_id\":\"TEMPLATE_ID\",\"url\":\"URL\",\"miniprogram\":{\"appid\":\"xiaochengxuappid12345\",\"pagepath\":\"index?foo=bar\"},\"scene\":\"SCENE\",\"title\":\"TITLE\",\"data\":{\"content\":{\"value\":\"VALUE\",\"color\":\"COLOR\"}}}"
    );
}

// ---- WxMpKefuMessage：text 客服消息 ----

#[test]
fn kefu_message_text_to_json() {
    let mut m = WxMpKefuMessage::default();
    m.to_user = Some("OPENID".to_string());
    m.msg_type = Some("text".to_string());
    m.content = Some("hello".to_string());
    let json = m.to_json().expect("序列化成功");
    assert_eq!(
        json,
        "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"hello\"}}"
    );
}

#[test]
fn kefu_message_image_to_json() {
    let mut m = WxMpKefuMessage::default();
    m.to_user = Some("OPENID".to_string());
    m.msg_type = Some("image".to_string());
    m.media_id = Some("MEDIA_ID".to_string());
    let json = m.to_json().expect("序列化成功");
    assert_eq!(
        json,
        "{\"touser\":\"OPENID\",\"msgtype\":\"image\",\"image\":{\"media_id\":\"MEDIA_ID\"}}"
    );
}

#[test]
fn kefu_message_music_to_json() {
    let mut m = WxMpKefuMessage::default();
    m.to_user = Some("OPENID".to_string());
    m.msg_type = Some("music".to_string());
    m.title = Some("标题".to_string());
    m.music_url = Some("http://m".to_string());
    m.thumb_media_id = Some("thumb".to_string());
    let json = m.to_json().expect("序列化成功");
    assert!(json.contains("\"msgtype\":\"music\""), "实际: {json}");
    assert!(json.contains("\"musicurl\":\"http://m\""), "实际: {json}");
    assert!(
        json.contains("\"thumb_media_id\":\"thumb\""),
        "实际: {json}"
    );
}

#[test]
fn kefu_message_from_json() {
    let json = r#"{"touser":"OPENID","msgtype":"text","text":{"content":"hi"}}"#;
    let m = WxMpKefuMessage::from_json(json).expect("解析成功");
    assert_eq!(m.to_user.as_deref(), Some("OPENID"));
    assert_eq!(m.msg_type.as_deref(), Some("text"));
    // Java Gson 平铺映射：嵌套 text.content 不落入 content 字段（行为一致）
    assert_eq!(m.content, None);
}

// ---- WxMpMenu：fromJson / toJson ----

#[test]
fn menu_from_json() {
    let json = r#"{
        "menu": {
            "button": [
                {"type": "view", "name": "阅读记录", "sub_button": []},
                {"name": "签到送礼", "sub_button": [
                    {"type": "view", "name": "书城首页", "sub_button": []}
                ]}
            ],
            "matchrule": {"tag_id": "100"},
            "menuid": "208396938"
        }
    }"#;
    let menu = WxMpMenu::from_json(json).expect("解析成功");
    let m = menu.menu.expect("menu 存在");
    assert_eq!(m.buttons.len(), 2);
    assert_eq!(m.buttons[0].name, "阅读记录");
    assert_eq!(m.buttons[1].sub_buttons.len(), 1);
    assert_eq!(m.buttons[1].sub_buttons[0].name, "书城首页");
    assert!(m.rule.is_some());
    assert_eq!(m.menu_id.as_deref(), Some("208396938"));
}

#[test]
fn menu_to_json_roundtrip() {
    let json = r#"{"menu":{"button":[{"type":"view","name":"阅读记录","sub_button":[]}]}}"#;
    let menu = WxMpMenu::from_json(json).expect("解析成功");
    let out = menu.to_json();
    let again = WxMpMenu::from_json(&out).expect("再次解析成功");
    assert_eq!(again.menu.unwrap().buttons.len(), 1);
}
