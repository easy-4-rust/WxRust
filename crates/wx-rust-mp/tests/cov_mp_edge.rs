//! 边界覆盖测试：menu 按钮类型、template message、kefu bean。
//!
//! 对应 Java `WxMpMenuServiceImpl`、`WxMpTemplateMsgServiceImpl`、
//! `WxMpKefuServiceImpl` 等边界场景。

use wx_rust_common::bean::menu::{WxMenuButton, WxMenuRule};
use wx_rust_mp::bean::kefu::{
    MsgMenu, WxMpKefuMessage, WxMpKfAccountRequest, WxMpKfInfo, WxMpKfList, WxMpKfMsgList,
    WxMpKfMsgRecord, WxMpKfOnlineList, WxMpKfSessionGetResult, WxMpKfSessionList,
    WxMpKfSessionRequest, WxMpKfSessionWaitCaseList,
};
use wx_rust_mp::bean::menu::{WxMpConditionalMenu, WxMpMenu};
use wx_rust_mp::bean::template::{
    MiniProgram, WxMpTemplate, WxMpTemplateData, WxMpTemplateIndustry, WxMpTemplateIndustryEnum,
    WxMpTemplateMessage,
};

// ========== Menu: 各 button 类型构造 ==========

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — view 类型按钮
fn menu_button_type_view() {
    let btn = WxMenuButton {
        r#type: "view".to_string(),
        name: "Search".to_string(),
        url: "https://example.com".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "view");
    assert!(!btn.url.is_empty());
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — click 类型按钮
fn menu_button_type_click() {
    let btn = WxMenuButton {
        r#type: "click".to_string(),
        name: "Like".to_string(),
        key: "V1001_LIKE".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "click");
    assert_eq!(btn.key, "V1001_LIKE");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — scancode_push 类型按钮
fn menu_button_type_scancode_push() {
    let btn = WxMenuButton {
        r#type: "scancode_push".to_string(),
        name: "Scan".to_string(),
        key: "SCAN_PUSH".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "scancode_push");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — scancode_waitmsg 类型按钮
fn menu_button_type_scancode_waitmsg() {
    let btn = WxMenuButton {
        r#type: "scancode_waitmsg".to_string(),
        name: "ScanWait".to_string(),
        key: "SCAN_WAIT".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "scancode_waitmsg");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — pic_sysphoto 类型按钮
fn menu_button_type_pic_sysphoto() {
    let btn = WxMenuButton {
        r#type: "pic_sysphoto".to_string(),
        name: "TakePhoto".to_string(),
        key: "PIC_SYSPHOTO".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "pic_sysphoto");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — pic_photo_or_album 类型按钮
fn menu_button_type_pic_photo_or_album() {
    let btn = WxMenuButton {
        r#type: "pic_photo_or_album".to_string(),
        name: "PhotoOrAlbum".to_string(),
        key: "PIC_ALBUM".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "pic_photo_or_album");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — pic_weixin 类型按钮
fn menu_button_type_pic_weixin() {
    let btn = WxMenuButton {
        r#type: "pic_weixin".to_string(),
        name: "WeixinPhoto".to_string(),
        key: "PIC_WEIXIN".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "pic_weixin");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — location_select 类型按钮
fn menu_button_type_location_select() {
    let btn = WxMenuButton {
        r#type: "location_select".to_string(),
        name: "Location".to_string(),
        key: "LOCATION".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "location_select");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — media_id 类型按钮
fn menu_button_type_media_id() {
    let btn = WxMenuButton {
        r#type: "media_id".to_string(),
        name: "Media".to_string(),
        media_id: "MEDIA_ID_123".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "media_id");
    assert_eq!(btn.media_id, "MEDIA_ID_123");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — view_limited 类型按钮
fn menu_button_type_view_limited() {
    let btn = WxMenuButton {
        r#type: "view_limited".to_string(),
        name: "ViewLimited".to_string(),
        media_id: "MEDIA_456".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "view_limited");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — article_id 类型按钮
fn menu_button_type_article_id() {
    let btn = WxMenuButton {
        r#type: "article_id".to_string(),
        name: "Article".to_string(),
        article_id: "ARTICLE_789".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "article_id");
    assert_eq!(btn.article_id, "ARTICLE_789");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — article_view_limited 类型按钮
fn menu_button_type_article_view_limited() {
    let btn = WxMenuButton {
        r#type: "article_view_limited".to_string(),
        name: "ArticleLimited".to_string(),
        article_id: "ARTICLE_ABC".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "article_view_limited");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuCreate — miniprogram 类型按钮
fn menu_button_type_miniprogram() {
    let btn = WxMenuButton {
        r#type: "miniprogram".to_string(),
        name: "MiniApp".to_string(),
        app_id: "wx123456".to_string(),
        page_path: "pages/index/index".to_string(),
        url: "https://fallback.com".to_string(),
        ..Default::default()
    };
    assert_eq!(btn.r#type, "miniprogram");
    assert_eq!(btn.app_id, "wx123456");
    assert_eq!(btn.page_path, "pages/index/index");
}

#[test]
/// 对应 Java: WxMpMenuServiceImpl#menuDelete — 菜单序列化后可删除（语义验证）
fn menu_delete_semantics() {
    // 删除菜单即调用 delete 接口，此处验证空菜单可序列化
    let menu = WxMpMenu::default();
    let json = menu.to_json();
    assert!(json.contains("[]") || json.contains("button"));
}

// ========== Menu: WxMpMenu JSON 往返 ==========

#[test]
/// 对应 Java: WxMpMenu#fromJson — 含子按钮的菜单
fn mp_menu_with_sub_buttons_roundtrip() {
    let mut parent = WxMenuButton {
        r#type: String::new(),
        name: "Parent".to_string(),
        ..Default::default()
    };
    parent.sub_buttons.push(WxMenuButton {
        r#type: "click".to_string(),
        name: "Child1".to_string(),
        key: "CHILD1".to_string(),
        ..Default::default()
    });
    parent.sub_buttons.push(WxMenuButton {
        r#type: "view".to_string(),
        name: "Child2".to_string(),
        url: "https://example.com".to_string(),
        ..Default::default()
    });

    let mut menu = WxMpMenu::default();
    menu.menu = Some(WxMpConditionalMenu {
        buttons: vec![parent],
        rule: None,
        menu_id: None,
    });

    let json = menu.to_json();
    let parsed = WxMpMenu::from_json(&json).unwrap();
    let buttons = &parsed.menu.unwrap().buttons;
    assert_eq!(buttons.len(), 1);
    assert_eq!(buttons[0].sub_buttons.len(), 2);
    assert_eq!(buttons[0].sub_buttons[0].r#type, "click");
    assert_eq!(buttons[0].sub_buttons[1].r#type, "view");
}

#[test]
/// 对应 Java: WxMpMenu — 个性化菜单带匹配规则
fn mp_conditional_menu_with_rule() {
    let rule = WxMenuRule {
        tag_id: "100".to_string(),
        sex: "1".to_string(),
        country: "中国".to_string(),
        province: "广东".to_string(),
        city: "深圳".to_string(),
        client_platform_type: "2".to_string(),
        language: "zh_CN".to_string(),
    };
    let menu = WxMpConditionalMenu {
        buttons: vec![WxMenuButton {
            r#type: "click".to_string(),
            name: "ConditionalBtn".to_string(),
            key: "COND_KEY".to_string(),
            ..Default::default()
        }],
        rule: Some(rule),
        menu_id: Some("MENU_001".to_string()),
    };
    let json = serde_json::to_string(&menu).unwrap();
    let parsed: WxMpConditionalMenu = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.menu_id.as_deref(), Some("MENU_001"));
    assert!(parsed.rule.is_some());
    assert_eq!(parsed.rule.unwrap().tag_id, "100");
}

#[test]
/// 对应 Java: WxMpMenu — menu_id 为数字时统一转字符串
fn mp_menu_id_from_number() {
    let json = r#"{"button":[],"menuid":12345}"#;
    let menu: WxMpConditionalMenu = serde_json::from_str(json).unwrap();
    assert_eq!(menu.menu_id.as_deref(), Some("12345"));
}

// ========== Template Message ==========

#[test]
/// 对应 Java: WxMpTemplateMsgService#sendTemplateMsg — 基本模板消息构造
fn template_message_basic() {
    let msg = WxMpTemplateMessage::builder()
        .to_user("openid_123")
        .template_id("tpl_id_456")
        .url("https://example.com");
    assert_eq!(msg.to_user.as_deref(), Some("openid_123"));
    assert_eq!(msg.template_id.as_deref(), Some("tpl_id_456"));
    assert!(msg.mini_program.is_none());
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — 模板消息带 miniprogram 字段
fn template_message_with_miniprogram() {
    let msg = WxMpTemplateMessage::builder()
        .to_user("openid_123")
        .template_id("tpl_id_456")
        .mini_program(MiniProgram::new("wx_appid", "pages/index/index"))
        .add_data(WxMpTemplateData::new("first", "hello"));
    let mp = msg.mini_program.as_ref().unwrap();
    assert_eq!(mp.appid, "wx_appid");
    assert_eq!(mp.path, "pages/index/index");
    assert_eq!(msg.data.len(), 1);
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — to_json 含 miniprogram
fn template_message_to_json_with_miniprogram() {
    let msg = WxMpTemplateMessage::builder()
        .to_user("user1")
        .template_id("tpl1")
        .mini_program(MiniProgram::new("wx123", "pages/home"))
        .add_data(WxMpTemplateData::new("first", "Welcome"));
    let json = msg.to_json().unwrap();
    assert!(json.contains("miniprogram"));
    assert!(json.contains("wx123"));
    assert!(json.contains("pages/home"));
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 截断规则 thing* > 20 字
fn template_data_thing_truncation() {
    let long_value = "a".repeat(25); // 25 > 20
    let msg = WxMpTemplateMessage::builder().add_data(WxMpTemplateData::new("thing01", long_value));
    let val = &msg.data[0].value;
    // 截断到 17 字 + "..."
    assert!(val.ends_with("..."));
    assert_eq!(val.len(), 20); // 17 chars + "..."
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 截断规则 character_string* > 32 字
fn template_data_character_string_truncation() {
    let long_value = "b".repeat(40); // 40 > 32
    let msg = WxMpTemplateMessage::builder()
        .add_data(WxMpTemplateData::new("character_string01", long_value));
    let val = &msg.data[0].value;
    assert!(val.ends_with("..."));
    assert_eq!(val.len(), 32); // 29 chars + "..."
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 截断规则 phone_number* > 17 字
fn template_data_phone_number_truncation() {
    let long_value = "1".repeat(20); // 20 > 17
    let msg = WxMpTemplateMessage::builder()
        .add_data(WxMpTemplateData::new("phone_number01", long_value));
    let val = &msg.data[0].value;
    assert!(val.ends_with("..."));
    assert_eq!(val.len(), 17); // 14 chars + "..."
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 截断规则 car_number* > 8 字
fn template_data_car_number_truncation() {
    let long_value = "ABCDEFGHJ"; // 9 > 8
    let msg =
        WxMpTemplateMessage::builder().add_data(WxMpTemplateData::new("car_number01", long_value));
    let val = &msg.data[0].value;
    assert!(val.ends_with("..."));
    assert_eq!(val.len(), 8); // 5 chars + "..."
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 截断规则 const* > 20 字
fn template_data_const_truncation() {
    let long_value = "x".repeat(25);
    let msg = WxMpTemplateMessage::builder().add_data(WxMpTemplateData::new("const01", long_value));
    let val = &msg.data[0].value;
    assert!(val.ends_with("..."));
}

#[test]
/// 对应 Java: WxMpTemplateMsgService — add_data 短值不截断
fn template_data_short_value_no_truncation() {
    let msg = WxMpTemplateMessage::builder().add_data(WxMpTemplateData::new("thing01", "hello"));
    assert_eq!(msg.data[0].value, "hello");
}

#[test]
/// 对应 Java: WxMpTemplateData — with_color 带颜色
fn template_data_with_color() {
    let d = WxMpTemplateData::with_color("key", "val", "#FF0000");
    assert_eq!(d.color.as_deref(), Some("#FF0000"));
}

// ========== Template Industry ==========

#[test]
/// 对应 Java: WxMpTemplateIndustry — JSON 往返
fn template_industry_roundtrip() {
    let industry = WxMpTemplateIndustry {
        primary_industry: Some(WxMpTemplateIndustryEnum::ECommerce),
        second_industry: Some(WxMpTemplateIndustryEnum::Bank),
    };
    let json = serde_json::to_string(&industry).unwrap();
    let parsed: WxMpTemplateIndustry = serde_json::from_str(&json).unwrap();
    assert_eq!(
        parsed.primary_industry,
        Some(WxMpTemplateIndustryEnum::ECommerce)
    );
    assert_eq!(parsed.second_industry, Some(WxMpTemplateIndustryEnum::Bank));
}

#[test]
/// 对应 Java: WxMpTemplateIndustryEnum#find_by_class — 查找行业
fn template_industry_find_by_class() {
    let found = WxMpTemplateIndustryEnum::find_by_class("IT科技", "电子商务");
    assert_eq!(found, Some(WxMpTemplateIndustryEnum::ECommerce));
}

#[test]
/// 对应 Java: WxMpTemplateIndustryEnum#find_by_code — 按编码查找
fn template_industry_find_by_code() {
    let found = WxMpTemplateIndustryEnum::find_by_code(7);
    assert_eq!(found, Some(WxMpTemplateIndustryEnum::Bank));
}

#[test]
/// 对应 Java: WxMpTemplateIndustryEnum#ALL — 共 41 个行业
fn template_industry_all_count() {
    assert_eq!(WxMpTemplateIndustryEnum::ALL.len(), 41);
}

// ========== Template bean ==========

#[test]
/// 对应 Java: WxMpTemplate — JSON 往返
fn wx_mp_template_roundtrip() {
    let tpl = WxMpTemplate {
        template_id: "tpl_001".to_string(),
        title: "Order Notification".to_string(),
        primary_industry: "IT科技".to_string(),
        deputy_industry: "互联网|电子商务".to_string(),
        content: "order {{name.DATA}}".to_string(),
        example: "order example".to_string(),
    };
    let json = serde_json::to_string(&tpl).unwrap();
    let parsed: WxMpTemplate = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.template_id, "tpl_001");
    assert_eq!(parsed.title, "Order Notification");
}

// ========== Kefu: 客服账号 CRUD ==========

#[test]
/// 对应 Java: WxMpKefuService#kfAccountAdd — 请求 bean 构造
fn kf_account_request_add() {
    let req = WxMpKfAccountRequest {
        kf_account: "test@test".to_string(),
        nick_name: "TestAgent".to_string(),
        invite_wx: "wx_invite".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("test@test"));
    assert!(json.contains("TestAgent"));
}

#[test]
/// 对应 Java: WxMpKefuService#kfAccountUpdate — 请求 bean 更新
fn kf_account_request_update() {
    let req = WxMpKfAccountRequest {
        kf_account: "test@test".to_string(),
        nick_name: "Updated".to_string(),
        invite_wx: String::new(),
    };
    assert_eq!(req.kf_account, "test@test");
    assert_eq!(req.nick_name, "Updated");
}

#[test]
/// 对应 Java: WxMpKefuService#kfAccountInviteWorker — 邀请绑定
fn kf_account_request_invite() {
    let req = WxMpKfAccountRequest {
        kf_account: "worker@test".to_string(),
        nick_name: String::new(),
        invite_wx: "wx_worker_123".to_string(),
    };
    assert_eq!(req.invite_wx, "wx_worker_123");
}

#[test]
/// 对应 Java: WxMpKefuService#kfSessionCreate — 会话请求
fn kf_session_request_create() {
    let req = WxMpKfSessionRequest {
        kf_account: "agent@test".to_string(),
        openid: "user_openid".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("agent@test"));
    assert!(json.contains("user_openid"));
}

// ========== Kefu: result bean JSON 解析 ==========

#[test]
/// 对应 Java: WxMpKfInfo — JSON 解析含所有字段
fn kf_info_from_json() {
    let json = r#"{
        "kf_account": "test@test",
        "kf_headimgurl": "https://img.example.com/avatar.jpg",
        "kf_id": "1001",
        "kf_nick": "Agent",
        "kf_wx": "wx_agent",
        "invite_wx": "",
        "invite_expire_time": 0,
        "invite_status": "waiting",
        "status": 1,
        "accepted_case": 5
    }"#;
    let info: WxMpKfInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.account, "test@test");
    assert_eq!(info.head_img_url, "https://img.example.com/avatar.jpg");
    assert_eq!(info.status, 1);
    assert_eq!(info.accepted_case, 5);
}

#[test]
/// 对应 Java: WxMpKfList#fromJson — 客服列表
fn kf_list_from_json() {
    let json = r#"{"kf_list":[{"kf_account":"a@test","kf_nick":"A","kf_id":"1"},{"kf_account":"b@test","kf_nick":"B","kf_id":"2"}]}"#;
    let list = WxMpKfList::from_json(json).unwrap();
    assert_eq!(list.kf_list.len(), 2);
    assert_eq!(list.kf_list[0].account, "a@test");
    assert_eq!(list.kf_list[1].nick, "B");
}

#[test]
/// 对应 Java: WxMpKfOnlineList#fromJson — 在线客服列表
fn kf_online_list_from_json() {
    let json = r#"{"kf_online_list":[{"kf_account":"online@test","status":1}]}"#;
    let list = WxMpKfOnlineList::from_json(json).unwrap();
    assert_eq!(list.kf_online_list.len(), 1);
    assert_eq!(list.kf_online_list[0].account, "online@test");
}

#[test]
/// 对应 Java: WxMpKfSessionGetResult#fromJson — 会话获取结果
fn kf_session_get_result_from_json() {
    let json = r#"{"kf_account":"agent@test","createtime":1234567890}"#;
    let result = WxMpKfSessionGetResult::from_json(json).unwrap();
    assert_eq!(result.kf_account, "agent@test");
    assert_eq!(result.create_time, 1234567890);
}

#[test]
/// 对应 Java: WxMpKfSessionList#fromJson — 会话列表
fn kf_session_list_from_json() {
    let json = r#"{"sessionlist":[{"kf_account":"a@test","openid":"u1","createtime":100,"latest_time":200}]}"#;
    let list = WxMpKfSessionList::from_json(json).unwrap();
    assert_eq!(list.kf_session_list.len(), 1);
    assert_eq!(list.kf_session_list[0].kf_account, "a@test");
    assert_eq!(list.kf_session_list[0].openid, "u1");
}

#[test]
/// 对应 Java: WxMpKfSessionWaitCaseList#fromJson — 未接入会话列表
fn kf_session_wait_case_list_from_json() {
    let json = r#"{"count":2,"waitcaselist":[{"kf_account":"","openid":"u1","createtime":0,"latest_time":0},{"kf_account":"","openid":"u2","createtime":0,"latest_time":0}]}"#;
    let list = WxMpKfSessionWaitCaseList::from_json(json).unwrap();
    assert_eq!(list.count, 2);
    assert_eq!(list.kf_session_wait_case_list.len(), 2);
}

#[test]
/// 对应 Java: WxMpKfMsgList#fromJson — 聊天记录列表
fn kf_msg_list_from_json() {
    let json = r#"{"recordlist":[{"worker":"agent@test","openid":"u1","opercode":0,"text":"hello","time":1234}],"number":1,"msgid":999}"#;
    let list = WxMpKfMsgList::from_json(json).unwrap();
    assert_eq!(list.records.len(), 1);
    assert_eq!(list.number, 1);
    assert_eq!(list.msg_id, 999);
    assert_eq!(list.records[0].text, "hello");
}

#[test]
/// 对应 Java: WxMpKfMsgRecord — 各字段正确
fn kf_msg_record_fields() {
    let rec = WxMpKfMsgRecord {
        worker: "agent@test".to_string(),
        openid: "user1".to_string(),
        operate_code: 2,
        text: "bye".to_string(),
        time: 5678,
    };
    assert_eq!(rec.operate_code, 2);
    assert_eq!(rec.time, 5678);
}

// ========== Kefu: Message builder 各消息类型 ==========

#[test]
/// 对应 Java: WxMpKefuMessage — text 消息 JSON
fn kefu_message_text_to_json() {
    let msg = WxMpKefuMessage::text()
        .to_user("user1")
        .content("hello world")
        .build();
    let json = msg.to_json().unwrap();
    assert!(json.contains("\"text\""));
    assert!(json.contains("hello world"));
    assert_eq!(msg.get_msg_type(), "text");
}

#[test]
/// 对应 Java: WxMpKefuMessage — image 消息 JSON
fn kefu_message_image_to_json() {
    let msg = WxMpKefuMessage::image()
        .to_user("user1")
        .media_id("img_media_123")
        .build();
    let json = msg.to_json().unwrap();
    assert!(json.contains("\"image\""));
    assert!(json.contains("img_media_123"));
}

#[test]
/// 对应 Java: WxMpKefuMessage — miniprogrampage 消息 JSON
fn kefu_message_miniprogrampage_to_json() {
    let msg = WxMpKefuMessage::miniprogrampage()
        .to_user("user1")
        .title("Mini App")
        .app_id("wx_appid_123")
        .page_path("pages/index")
        .thumb_media_id("thumb_123")
        .build();
    let json = msg.to_json().unwrap();
    assert!(json.contains("miniprogrampage"));
    assert!(json.contains("wx_appid_123"));
    assert!(json.contains("pages/index"));
}

#[test]
/// 对应 Java: WxMpKefuMessage — msgmenu 消息 JSON
fn kefu_message_msgmenu_to_json() {
    let menus = vec![MsgMenu::new("1", "Option A"), MsgMenu::new("2", "Option B")];
    let msg = WxMpKefuMessage::msgmenu()
        .to_user("user1")
        .head_content("Please choose:")
        .tail_content("Thank you")
        .add_menus(menus)
        .build();
    let json = msg.to_json().unwrap();
    assert!(json.contains("msgmenu"));
    assert!(json.contains("Option A"));
    assert!(json.contains("Please choose:"));
}

#[test]
/// 对应 Java: WxMpKefuMessage — mpnewsarticle 消息 JSON
fn kefu_message_mpnewsarticle_to_json() {
    let msg = WxMpKefuMessage::mpnewsarticle()
        .to_user("user1")
        .article_id("ARTICLE_001")
        .build();
    let json = msg.to_json().unwrap();
    assert!(json.contains("mpnewsarticle"));
    assert!(json.contains("ARTICLE_001"));
}

#[test]
/// 对应 Java: WxMpKefuMessage — 非法消息类型返回错误
fn kefu_message_invalid_type_returns_error() {
    let mut msg = WxMpKefuMessage::default();
    msg.msg_type = Some("unknown_type".to_string());
    msg.to_user = Some("user1".to_string());
    let result = msg.to_json();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("暂不支持"));
}

#[test]
/// 对应 Java: WxMpKefuMessage — kf_account 路由到指定客服
fn kefu_message_with_kf_account() {
    let msg = WxMpKefuMessage::text()
        .to_user("user1")
        .content("hello")
        .build();
    // 手动设置 kf_account
    let mut msg = msg;
    msg.kf_account = Some("agent@test".to_string());
    let json = msg.to_json().unwrap();
    assert!(json.contains("customservice"));
    assert!(json.contains("agent@test"));
}

#[test]
/// 对应 Java: WxMpKefuMessage#toJson — 空 kf_account 不输出 customservice
fn kefu_message_empty_kf_account_omitted() {
    let msg = WxMpKefuMessage::text()
        .to_user("user1")
        .content("hello")
        .build();
    let json = msg.to_json().unwrap();
    assert!(!json.contains("customservice"));
}
