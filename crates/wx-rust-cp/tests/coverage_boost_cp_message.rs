//! 企业微信应用消息（WxCpMessage）覆盖率提升测试（纯离线，无网络依赖）。
//!
//! 覆盖 `bean/message/wx_cp_message.rs`（`to_json` 各 msgtype 分支 + 模板
//! 卡片各子样式序列化）与 `message/messagebuilder/*`（text/textcard/image/
//! voice/video/news/mpnews/markdown/file/taskcard/templatecard/
//! miniprogram_notice 全部 builder 方法）。
//!
//! 断言策略：
//! - builder 路径：golden 字符串（键序与 Java `toJson()` 一致）；
//! - 手工组装路径：断言序列化 JSON 的字段名与类型（serde_json::Value 比较，
//!   不依赖键序）。

use std::collections::HashMap;

use wx_rust_cp::bean::article::{MpnewsArticle, NewArticle};
use wx_rust_cp::bean::message::WxCpMessage;
use wx_rust_cp::bean::taskcard::TaskCardButton;
use wx_rust_cp::bean::templatecard::{
    ActionMenuItem, CheckboxOption, HorizontalContent, MultipleSelect, QuoteArea,
    TemplateCardButton, TemplateCardButtonSelection, TemplateCardButtonSelectionOption,
    TemplateCardImageTextArea, TemplateCardJump, VerticalContent,
};

// ========================================================================
// 一、基础消息类型 builder → to_json golden（对应 Java WxCpMessageTest）
// ========================================================================

/// 对应 Java: WxCpMessageTest.testVoiceBuild
#[test]
fn voice_build_golden() {
    let reply = WxCpMessage::voice()
        .to_user("OPENID")
        .media_id("MEDIA_ID")
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"voice\",\"voice\":{\"media_id\":\"MEDIA_ID\"},\"safe\":\"0\"}"
    );
}

/// 对应 Java: WxCpMessageTest.testVideoBuild
#[test]
fn video_build_golden() {
    let reply = WxCpMessage::video()
        .to_user("OPENID")
        .title("TITLE")
        .media_id("MEDIA_ID")
        .thumb_media_id("MEDIA_ID")
        .description("DESCRIPTION")
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"video\",\"video\":{\"media_id\":\"MEDIA_ID\",\"thumb_media_id\":\"MEDIA_ID\",\"title\":\"TITLE\",\"description\":\"DESCRIPTION\"},\"safe\":\"0\"}"
    );
}

/// 对应 Java: WxCpMessageTest.testNewsBuild（NewArticle 四字段 + 空字段省略）。
#[test]
fn news_build_golden() {
    let article = |title: &str| NewArticle {
        title: title.to_string(),
        description: "Is Really A Happy Day".to_string(),
        url: "URL".to_string(),
        pic_url: "PIC_URL".to_string(),
        ..Default::default()
    };
    let reply = WxCpMessage::news()
        .to_user("OPENID")
        .add_article(article("Happy Day"))
        .add_article(article("Happy Day"))
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"news\",\"news\":{\"articles\":[{\"title\":\"Happy Day\",\"description\":\"Is Really A Happy Day\",\"url\":\"URL\",\"picurl\":\"PIC_URL\",\"appid\":\"\",\"pagepath\":\"\"},{\"title\":\"Happy Day\",\"description\":\"Is Really A Happy Day\",\"url\":\"URL\",\"picurl\":\"PIC_URL\",\"appid\":\"\",\"pagepath\":\"\"}]},\"safe\":\"0\"}"
    );
}

/// 对应 Java: WxCpMessageTest.testNewsBuild（articles(vec) 批量设置形态）。
#[test]
fn news_build_with_articles_vec() {
    let articles = vec![
        NewArticle {
            title: "t1".into(),
            description: "d1".into(),
            url: "u1".into(),
            pic_url: "p1".into(),
            appid: "wx_app".into(),
            pagepath: "/pages/index".into(),
            ..Default::default()
        },
        NewArticle {
            title: "t2".into(),
            description: "d2".into(),
            url: "u2".into(),
            pic_url: "p2".into(),
            ..Default::default()
        },
    ];
    let reply = WxCpMessage::news()
        .to_user("OPENID")
        .articles(articles)
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    // msgtype/news 外层键存在且类型正确
    assert_eq!(json["msgtype"], "news");
    let arr = json["news"]["articles"].as_array().unwrap();
    assert_eq!(arr.len(), 2);
    // 第一条带小程序字段（appid/pagepath 线格式键名）
    assert_eq!(arr[0]["appid"], "wx_app");
    assert_eq!(arr[0]["pagepath"], "/pages/index");
    // 第二条未设置小程序字段 → 空字符串
    assert_eq!(arr[1]["appid"], "");
    assert_eq!(arr[1]["title"], "t2");
    assert_eq!(arr[1]["picurl"], "p2");
}

/// 对应 Java: WxCpMessageTest.testMpnewsBuild_with_articles。
#[test]
fn mpnews_build_with_articles_golden() {
    let article = MpnewsArticle {
        title: "Happy Day".into(),
        thumb_media_id: "thumb".into(),
        author: "aaaaaa".into(),
        content_source_url: "nice url".into(),
        content: "hahaha".into(),
        digest: "digest".into(),
        show_cover_pic: "heihei".into(),
    };
    let reply = WxCpMessage::mpnews()
        .to_user("OPENID")
        .articles(vec![article.clone(), article])
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"mpnews\",\"mpnews\":{\"articles\":[{\"title\":\"Happy Day\",\"thumb_media_id\":\"thumb\",\"author\":\"aaaaaa\",\"content_source_url\":\"nice url\",\"content\":\"hahaha\",\"digest\":\"digest\",\"show_cover_pic\":\"heihei\"},{\"title\":\"Happy Day\",\"thumb_media_id\":\"thumb\",\"author\":\"aaaaaa\",\"content_source_url\":\"nice url\",\"content\":\"hahaha\",\"digest\":\"digest\",\"show_cover_pic\":\"heihei\"}]},\"safe\":\"0\"}"
    );
}

/// 对应 Java: WxCpMessageTest.testMpnewsBuild_with_media_id（media_id 优先于 articles）。
#[test]
fn mpnews_build_with_media_id_golden() {
    let mut reply = WxCpMessage::mpnews().to_user("OPENID").build();
    reply.media_id = Some("mmm".into());
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"mpnews\",\"mpnews\":{\"media_id\":\"mmm\"},\"safe\":\"0\"}"
    );
    // articles 与 media_id 同时存在时 media_id 分支生效
    reply.mpnews_articles = vec![MpnewsArticle::default()];
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    assert_eq!(json["mpnews"]["media_id"], "mmm");
    assert!(json["mpnews"].get("articles").is_none());
}

/// 对应 Java: messagebuilder.MarkdownMsgBuilder（MARKDOWN 分支）。
#[test]
fn markdown_build_golden() {
    let reply = WxCpMessage::markdown()
        .to_user("OPENID")
        .content("# 标题\n正文")
        .build();
    // JSON 序列化把换行转义为 \n
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"markdown\",\"markdown\":{\"content\":\"# 标题\\n正文\"},\"safe\":\"0\"}"
    );
}

/// 对应 Java: messagebuilder.FileBuilder（file 与 image/voice 共用 media 分支）。
#[test]
fn file_build_golden() {
    let reply = WxCpMessage::file()
        .to_user("OPENID")
        .media_id("FILE_ID")
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"file\",\"file\":{\"media_id\":\"FILE_ID\"},\"safe\":\"0\"}"
    );
}

// ========================================================================
// 二、顶层公共字段键序与开关（agentid/toparty/totag/id_trans/dup_check/safe）
// ========================================================================

/// 对应 Java: WxCpMessage.toJson 顶层键序（agentid 最先、msgtype 恒有）。
#[test]
fn to_json_top_level_key_order() {
    let mut msg = WxCpMessage::text()
        .agent_id(1000002)
        .to_user("OPENID")
        .to_party("PartyID1 | PartyID2")
        .to_tag("TagID1 | TagID2")
        .content("hello")
        .build();
    msg.enable_id_trans = true;
    msg.enable_duplicate_check = true;
    msg.duplicate_check_interval = Some(1800);
    let json: serde_json::Value = serde_json::from_str(&msg.to_json()).unwrap();
    assert_eq!(json["agentid"], 1000002);
    assert_eq!(json["touser"], "OPENID");
    assert_eq!(json["toparty"], "PartyID1 | PartyID2");
    assert_eq!(json["totag"], "TagID1 | TagID2");
    assert_eq!(json["enable_id_trans"], 1);
    assert_eq!(json["enable_duplicate_check"], 1);
    assert_eq!(json["duplicate_check_interval"], 1800);
    assert_eq!(json["safe"], "0");
    // 键序与 Java golden 一致（serde_json preserve_order）
    let keys: Vec<&str> = json
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.as_str())
        .collect();
    assert_eq!(
        keys,
        vec![
            "agentid",
            "touser",
            "msgtype",
            "toparty",
            "totag",
            "enable_id_trans",
            "enable_duplicate_check",
            "duplicate_check_interval",
            "text",
            "safe"
        ]
    );
}

/// 对应 Java: StringUtils.isNotBlank 语义（空白接收者省略、safe 显式覆盖）。
#[test]
fn to_json_blank_fields_omitted_and_safe_override() {
    let reply = WxCpMessage::text()
        .to_user("   ")
        .to_party("")
        .content("c")
        .safe("1")
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    assert!(json.get("touser").is_none());
    assert!(json.get("toparty").is_none());
    assert_eq!(json["safe"], "1");

    // BaseBuilder：safe 为纯空白时回退默认 "0"
    let blank_safe = WxCpMessage::text().content("c").safe("  ").build();
    let json2: serde_json::Value = serde_json::from_str(&blank_safe.to_json()).unwrap();
    assert_eq!(json2["safe"], "0");
}

/// 对应 Java: handleMsgType default 分支（未知/缺失 msgtype 不输出消息体）。
#[test]
fn to_json_unknown_msg_type_no_body() {
    let mut msg = WxCpMessage::default();
    msg.msg_type = Some("unknown".into());
    assert_eq!(msg.to_json(), "{\"msgtype\":\"unknown\"}");

    let mut none_type = WxCpMessage::default();
    none_type.msg_type = None;
    assert_eq!(none_type.to_json(), "{\"msgtype\":null}");
}

// ========================================================================
// 三、任务卡片（taskcard 分支 + btn2Json 全字段）
// ========================================================================

/// 对应 Java: WxCpMessageTest.testTaskCardBuilder（safe 置 null、btn 数组线格式）。
#[test]
fn taskcard_build_golden() {
    let button1 = TaskCardButton::new(
        "yes",
        "批准",
        Some("已批准".into()),
        Some("blue".into()),
        Some(true),
    );
    let button2 = TaskCardButton::new(
        "no",
        "拒绝",
        Some("已拒绝".into()),
        Some("red".into()),
        Some(false),
    );
    let reply = WxCpMessage::taskcard()
        .to_user("OPENID")
        .title("任务卡片")
        .description("有一条待处理任务")
        .url("http://www.qq.com")
        .task_id("task_123")
        .buttons(vec![button1, button2])
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"taskcard\",\"taskcard\":{\"title\":\"任务卡片\",\"description\":\"有一条待处理任务\",\"url\":\"http://www.qq.com\",\"task_id\":\"task_123\",\"btn\":[{\"key\":\"yes\",\"name\":\"批准\",\"replace_name\":\"已批准\",\"color\":\"blue\",\"is_bold\":true},{\"key\":\"no\",\"name\":\"拒绝\",\"replace_name\":\"已拒绝\",\"color\":\"red\",\"is_bold\":false}]}}"
    );
}

/// 对应 Java: btn2Json（key/name 恒输出 null、可选字段省略、url 空白省略）。
#[test]
fn taskcard_btn_minimal_and_blank_url() {
    let reply = WxCpMessage::taskcard()
        .title("t")
        .description("d")
        .task_id("tid")
        .buttons(vec![TaskCardButton::default()])
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    // url 未设置 → 键省略；btn 空字段 → null
    assert!(json["taskcard"].get("url").is_none());
    assert_eq!(json["taskcard"]["btn"][0]["key"], serde_json::Value::Null);
    assert_eq!(json["taskcard"]["btn"][0]["name"], serde_json::Value::Null);
    assert!(json["taskcard"]["btn"][0].get("replace_name").is_none());
    assert!(json["taskcard"]["btn"][0].get("color").is_none());
    assert!(json["taskcard"]["btn"][0].get("is_bold").is_none());

    // url 为空白字符串 → 键省略
    let blank_url = WxCpMessage::taskcard().url("   ").build();
    let json2: serde_json::Value = serde_json::from_str(&blank_url.to_json()).unwrap();
    assert!(json2["taskcard"].get("url").is_none());
}

// ========================================================================
// 四、小程序通知（miniprogram_notice 分支）
// ========================================================================

/// 对应 Java: messagebuilder.MiniProgramNoticeMsgBuilder（content_item/emphasis）。
#[test]
fn miniprogram_notice_build_wire_format() {
    let mut content_items = HashMap::new();
    content_items.insert("k1".to_string(), "v1".to_string());
    let reply = WxCpMessage::miniprogram_notice()
        .to_user("OPENID")
        .app_id("wx_appid")
        .page("pages/index")
        .title("title")
        .description("desc")
        .emphasis_first_item(true)
        .content_items(content_items)
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"miniprogram_notice\",\"miniprogram_notice\":{\"appid\":\"wx_appid\",\"page\":\"pages/index\",\"description\":\"desc\",\"title\":\"title\",\"emphasis_first_item\":true,\"content_item\":[{\"key\":\"k1\",\"value\":\"v1\"}]},\"safe\":\"0\"}"
    );

    // 未设置 emphasis_first_item → null；无 content_item → 空数组
    let plain = WxCpMessage::miniprogram_notice().app_id("a").build();
    let json: serde_json::Value = serde_json::from_str(&plain.to_json()).unwrap();
    assert_eq!(
        json["miniprogram_notice"]["emphasis_first_item"],
        serde_json::Value::Null
    );
    assert_eq!(
        json["miniprogram_notice"]["content_item"],
        serde_json::json!([])
    );
}

// ========================================================================
// 五、模板卡片（template_card 分支：text_notice 全量字段 golden）
// ========================================================================

/// 对应 Java: WxCpMessageTest.TestTemplateCardBuilder_text_notice
/// （horizontal 四形态 + jump 两形态 + card_action + quote_area）。
#[test]
fn template_card_text_notice_golden() {
    let h1 = HorizontalContent {
        keyname: "邀请人".into(),
        value: "张三".into(),
        ..Default::default()
    };
    let h2 = HorizontalContent {
        r#type: 1,
        keyname: "企业微信官网".into(),
        value: "点击访问".into(),
        url: "https://work.weixin.qq.com".into(),
        ..Default::default()
    };
    let h3 = HorizontalContent {
        r#type: 2,
        keyname: "企业微信下载".into(),
        value: "企业微信.apk".into(),
        media_id: "文件的media_id".into(),
        ..Default::default()
    };
    let h4 = HorizontalContent {
        r#type: 3,
        keyname: "员工信息".into(),
        value: "点击查看".into(),
        userid: "zhangsan".into(),
        ..Default::default()
    };
    let jump1 = TemplateCardJump {
        r#type: 1,
        title: "企业微信官网".into(),
        url: "https://work.weixin.qq.com".into(),
        ..Default::default()
    };
    let jump2 = TemplateCardJump {
        r#type: 2,
        title: "跳转小程序".into(),
        appid: "小程序的appid".into(),
        pagepath: "/index.html".into(),
        ..Default::default()
    };
    let quote_area = QuoteArea {
        r#type: 1,
        url: "https://work.weixin.qq.com".into(),
        appid: "小程序的appid".into(),
        pagepath: "/index.html".into(),
        title: "引用文献标题".into(),
        quote_text: "引用文献样式的引用文案".into(),
    };
    let actions = vec![
        ActionMenuItem {
            text: "接受推送".into(),
            key: "A".into(),
        },
        ActionMenuItem {
            text: "不再推送".into(),
            key: "B".into(),
        },
    ];
    let mut reply = WxCpMessage::templatecard()
        .to_user("OPENID")
        .to_party("PartyID1 | PartyID2")
        .to_tag("TagID1 | TagID2")
        .agent_id(1000002)
        .card_type("text_notice")
        .task_id("task_id")
        .source_icon_url("图片的url")
        .source_desc("企业微信")
        .source_desc_color(1)
        .action_menu_desc("卡片副交互辅助文本说明")
        .action_menu_action_list(actions)
        .main_title_title("欢迎使用企业微信")
        .main_title_desc("您的好友正在邀请您加入企业微信")
        .emphasis_content_title("100")
        .emphasis_content_desc("核心数据")
        .sub_title_text("下载企业微信还能抢红包！")
        .horizontal_contents(vec![h1, h2, h3, h4])
        .jumps(vec![jump1, jump2])
        .card_action_type(2)
        .card_action_appid("小程序的appid")
        .card_action_url("https://work.weixin.qq.com")
        .card_action_pagepath("/index.html")
        .quote_area(quote_area)
        .build();
    reply.enable_id_trans = false;
    reply.enable_duplicate_check = false;
    reply.duplicate_check_interval = Some(1800);
    assert_eq!(
        reply.to_json(),
        "{\"agentid\":1000002,\"touser\":\"OPENID\",\"msgtype\":\"template_card\",\"toparty\":\"PartyID1 | PartyID2\",\"totag\":\"TagID1 | TagID2\",\"duplicate_check_interval\":1800,\"template_card\":{\"card_type\":\"text_notice\",\"source\":{\"icon_url\":\"图片的url\",\"desc\":\"企业微信\",\"desc_color\":1},\"action_menu\":{\"desc\":\"卡片副交互辅助文本说明\",\"action_list\":[{\"text\":\"接受推送\",\"key\":\"A\"},{\"text\":\"不再推送\",\"key\":\"B\"}]},\"main_title\":{\"title\":\"欢迎使用企业微信\",\"desc\":\"您的好友正在邀请您加入企业微信\"},\"emphasis_content\":{\"title\":\"100\",\"desc\":\"核心数据\"},\"sub_title_text\":\"下载企业微信还能抢红包！\",\"task_id\":\"task_id\",\"horizontal_content_list\":[{\"keyname\":\"邀请人\",\"value\":\"张三\"},{\"type\":1,\"keyname\":\"企业微信官网\",\"value\":\"点击访问\",\"url\":\"https://work.weixin.qq.com\"},{\"type\":2,\"keyname\":\"企业微信下载\",\"value\":\"企业微信.apk\",\"media_id\":\"文件的media_id\"},{\"type\":3,\"keyname\":\"员工信息\",\"value\":\"点击查看\",\"userid\":\"zhangsan\"}],\"jump_list\":[{\"type\":1,\"title\":\"企业微信官网\",\"url\":\"https://work.weixin.qq.com\"},{\"type\":2,\"title\":\"跳转小程序\",\"appid\":\"小程序的appid\",\"pagepath\":\"/index.html\"}],\"card_action\":{\"type\":2,\"url\":\"https://work.weixin.qq.com\",\"appid\":\"小程序的appid\",\"pagepath\":\"/index.html\"},\"quote_area\":{\"type\":1,\"url\":\"https://work.weixin.qq.com\",\"appid\":\"小程序的appid\",\"pagepath\":\"/index.html\",\"title\":\"引用文献标题\",\"quote_text\":\"引用文献样式的引用文案\"}}}"
    );
}

/// 对应 Java: WxCpMessageTest.TestTemplateCardBuilder_news_notice
/// （vertical_content_list 分支 + source 无 desc_color → null）。
#[test]
fn template_card_news_notice_golden() {
    let v1 = VerticalContent {
        title: "惊喜红包等你来拿".into(),
        desc: "下载企业微信还能抢红包！".into(),
    };
    let v2 = VerticalContent {
        title: "二级垂直内容".into(),
        desc: "二级垂直内容！".into(),
    };
    let h1 = HorizontalContent {
        keyname: "邀请人".into(),
        value: "张三".into(),
        ..Default::default()
    };
    let h2 = HorizontalContent {
        r#type: 1,
        keyname: "企业微信官网".into(),
        value: "点击访问".into(),
        url: "https://work.weixin.qq.com".into(),
        ..Default::default()
    };
    let jump1 = TemplateCardJump {
        r#type: 1,
        title: "企业微信官网".into(),
        url: "https://work.weixin.qq.com".into(),
        ..Default::default()
    };
    let mut reply = WxCpMessage::templatecard()
        .to_user("OPENID")
        .agent_id(1000002)
        .card_type("news_notice")
        .source_icon_url("图片的url")
        .source_desc("企业微信")
        .main_title_title("欢迎使用企业微信")
        .main_title_desc("您的好友正在邀请您加入企业微信")
        .vertical_contents(vec![v1, v2])
        .horizontal_contents(vec![h1, h2])
        .jumps(vec![jump1])
        .card_action_type(2)
        .card_action_appid("小程序的appid")
        .card_action_url("https://work.weixin.qq.com")
        .card_action_pagepath("/index.html")
        .build();
    reply.duplicate_check_interval = Some(1800);
    assert_eq!(
        reply.to_json(),
        "{\"agentid\":1000002,\"touser\":\"OPENID\",\"msgtype\":\"template_card\",\"duplicate_check_interval\":1800,\"template_card\":{\"card_type\":\"news_notice\",\"source\":{\"icon_url\":\"图片的url\",\"desc\":\"企业微信\",\"desc_color\":null},\"main_title\":{\"title\":\"欢迎使用企业微信\",\"desc\":\"您的好友正在邀请您加入企业微信\"},\"vertical_content_list\":[{\"title\":\"惊喜红包等你来拿\",\"desc\":\"下载企业微信还能抢红包！\"},{\"title\":\"二级垂直内容\",\"desc\":\"二级垂直内容！\"}],\"horizontal_content_list\":[{\"keyname\":\"邀请人\",\"value\":\"张三\"},{\"type\":1,\"keyname\":\"企业微信官网\",\"value\":\"点击访问\",\"url\":\"https://work.weixin.qq.com\"}],\"jump_list\":[{\"type\":1,\"title\":\"企业微信官网\",\"url\":\"https://work.weixin.qq.com\"}],\"card_action\":{\"type\":2,\"url\":\"https://work.weixin.qq.com\",\"appid\":\"小程序的appid\",\"pagepath\":\"/index.html\"}}}"
    );
}

/// 对应 Java: WxCpMessageTest.TestTemplateCardBuilder_button_interaction
/// （button_list 线格式：text 恒有、style 非 0 才输出、type 恒输出、url 非空才输出）。
#[test]
fn template_card_button_interaction() {
    let b1 = TemplateCardButton {
        text: "按钮1".into(),
        style: 1,
        key: "button_key_1".into(),
        ..Default::default()
    };
    let b2 = TemplateCardButton {
        text: "按钮2".into(),
        style: 2,
        key: "button_key_2".into(),
        url: "https://work.weixin.qq.com".into(),
        ..Default::default()
    };
    let selection = TemplateCardButtonSelection {
        question_key: "question_key".into(),
        title: "选择器标题".into(),
        selected_id: "selection_id".into(),
        option_list: vec![
            TemplateCardButtonSelectionOption {
                id: "selection_id1".into(),
                text: "选项1".into(),
            },
            TemplateCardButtonSelectionOption {
                id: "selection_id2".into(),
                text: "选项2".into(),
            },
        ],
    };
    let reply = WxCpMessage::templatecard()
        .to_user("OPENID")
        .agent_id(1000002)
        .card_type("button_interaction")
        .source_desc("企业微信")
        .sub_title_text("下载企业微信还能抢红包！")
        .task_id("task_id")
        .buttons(vec![b1, b2])
        .button_selection(selection)
        .card_action_type(1)
        .card_action_url("https://work.weixin.qq.com")
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    let card = &json["template_card"];
    assert_eq!(card["card_type"], "button_interaction");
    // button_selection 三字段 + option_list
    assert_eq!(card["button_selection"]["question_key"], "question_key");
    assert_eq!(card["button_selection"]["title"], "选择器标题");
    assert_eq!(card["button_selection"]["selected_id"], "selection_id");
    assert_eq!(
        card["button_selection"]["option_list"][0]["id"],
        "selection_id1"
    );
    assert_eq!(card["button_selection"]["option_list"][1]["text"], "选项2");
    // button 线格式
    assert_eq!(
        card["button_list"][0],
        serde_json::json!({"text":"按钮1","style":1,"key":"button_key_1","type":0})
    );
    assert_eq!(
        card["button_list"][1],
        serde_json::json!({"text":"按钮2","style":2,"key":"button_key_2","type":0,"url":"https://work.weixin.qq.com"})
    );
    // card_action type=1 输出 url、省略 appid/pagepath
    assert_eq!(card["card_action"]["type"], 1);
    assert_eq!(card["card_action"]["url"], "https://work.weixin.qq.com");
    assert!(card["card_action"].get("appid").is_none());
    // TemplateCardButton 无 url 且 style=0 → 仅 text/key/type
    let plain = WxCpMessage::templatecard()
        .card_type("button_interaction")
        .buttons(vec![TemplateCardButton {
            text: "t".into(),
            key: "k".into(),
            ..Default::default()
        }])
        .build();
    let plain_json: serde_json::Value = serde_json::from_str(&plain.to_json()).unwrap();
    assert_eq!(
        plain_json["template_card"]["button_list"][0],
        serde_json::json!({"text":"t","key":"k","type":0})
    );
}

/// 对应 Java: WxCpMessageTest.TestTemplateCardBuilder_vote_interaction
/// （checkbox 分支：question_key/mode/option_list/submit_button）。
#[test]
fn template_card_vote_interaction_golden() {
    let reply = WxCpMessage::templatecard()
        .to_user("OPENID")
        .agent_id(1000002)
        .card_type("vote_interaction")
        .source_icon_url("图片的url")
        .source_desc("企业微信")
        .main_title_title("欢迎使用企业微信")
        .main_title_desc("您的好友正在邀请您加入企业微信")
        .task_id("task_id")
        .checkbox_question_key("question_key1")
        .checkbox_mode(1)
        .options(vec![
            CheckboxOption::new("option_id1", "选择题选项1", Some(true)),
            CheckboxOption::new("option_id2", "选择题选项2", Some(false)),
        ])
        .submit_button_key("key")
        .submit_button_text("提交")
        .build();
    assert_eq!(
        reply.to_json(),
        "{\"agentid\":1000002,\"touser\":\"OPENID\",\"msgtype\":\"template_card\",\"template_card\":{\"card_type\":\"vote_interaction\",\"source\":{\"icon_url\":\"图片的url\",\"desc\":\"企业微信\",\"desc_color\":null},\"main_title\":{\"title\":\"欢迎使用企业微信\",\"desc\":\"您的好友正在邀请您加入企业微信\"},\"task_id\":\"task_id\",\"checkbox\":{\"question_key\":\"question_key1\",\"mode\":1,\"option_list\":[{\"id\":\"option_id1\",\"text\":\"选择题选项1\",\"is_checked\":true},{\"id\":\"option_id2\",\"text\":\"选择题选项2\",\"is_checked\":false}]},\"submit_button\":{\"text\":\"提交\",\"key\":\"key\"}}}"
    );
}

/// 对应 Java: WxCpMessageTest.TestTemplateCardBuilder_multiple_interaction
/// （select_list 分支：question_key/title/selected_id/option_list）。
#[test]
fn template_card_multiple_interaction_golden() {
    let s1 = MultipleSelect {
        question_key: "question_key1".into(),
        title: "选择器标签1".into(),
        selected_id: "selection_id1".into(),
        options: vec![
            CheckboxOption::new("selection_id1", "选择器选项1", None),
            CheckboxOption::new("selection_id2", "选择题选项2", None),
        ],
    };
    let s2 = MultipleSelect {
        question_key: "question_key2".into(),
        title: "选择器标签2".into(),
        selected_id: "selection_id3".into(),
        options: vec![
            CheckboxOption::new("selection_id3", "选择器选项3", Some(true)),
            CheckboxOption::new("selection_id4", "选择题选项4", None),
        ],
    };
    let reply = WxCpMessage::templatecard()
        .to_user("OPENID")
        .agent_id(1000002)
        .card_type("multiple_interaction")
        .source_icon_url("图片的url")
        .source_desc("企业微信")
        .main_title_title("欢迎使用企业微信")
        .main_title_desc("您的好友正在邀请您加入企业微信")
        .task_id("task_id")
        .selects(vec![s1, s2])
        .submit_button_key("key")
        .submit_button_text("提交")
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    let card = &json["template_card"];
    assert_eq!(card["card_type"], "multiple_interaction");
    assert_eq!(
        card["submit_button"],
        serde_json::json!({"text":"提交","key":"key"})
    );
    let selects = card["select_list"].as_array().unwrap();
    assert_eq!(selects.len(), 2);
    assert_eq!(
        selects[0],
        serde_json::json!({
            "question_key":"question_key1","title":"选择器标签1","selected_id":"selection_id1",
            "option_list":[{"id":"selection_id1","text":"选择器选项1"},{"id":"selection_id2","text":"选择题选项2"}]
        })
    );
    // is_checked Some(true) → 输出；None → 省略
    assert_eq!(selects[1]["option_list"][0]["is_checked"], true);
    assert!(selects[1]["option_list"][1].get("is_checked").is_none());
}

// ========================================================================
// 六、模板卡片其余子样式（image_text_area/card_image/emphasis 可选分支）
// ========================================================================

/// 对应 Java: TemplateCardImageTextArea.toJson（type 非 0 才输出 + imageUrl 键名）。
#[test]
fn template_card_image_text_area_wire_format() {
    let area = TemplateCardImageTextArea {
        r#type: 1,
        url: "https://work.weixin.qq.com".into(),
        title: "左图右文标题".into(),
        desc: "左图右文描述".into(),
        image_url: "https://img.example.com".into(),
    };
    let reply = WxCpMessage::templatecard()
        .card_type("news_notice")
        .image_text_area(area)
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    assert_eq!(
        json["template_card"]["image_text_area"],
        serde_json::json!({
            "type":1,"url":"https://work.weixin.qq.com","title":"左图右文标题",
            "desc":"左图右文描述","image_url":"https://img.example.com"
        })
    );

    // type=0 且字段全空 → 空对象
    let empty = WxCpMessage::templatecard()
        .card_type("news_notice")
        .image_text_area(TemplateCardImageTextArea::default())
        .build();
    let empty_json: serde_json::Value = serde_json::from_str(&empty.to_json()).unwrap();
    assert_eq!(
        empty_json["template_card"]["image_text_area"],
        serde_json::json!({})
    );
}

/// 对应 Java: TemplateCardBuilder.cardImageUrl/cardImageAspectRatio（card_image 分支）。
#[test]
fn template_card_card_image_wire_format() {
    let reply = WxCpMessage::templatecard()
        .card_type("news_notice")
        .card_image_url("https://img.example.com/big.png")
        .card_image_aspect_ratio(1.5)
        .build();
    let json: serde_json::Value = serde_json::from_str(&reply.to_json()).unwrap();
    assert_eq!(
        json["template_card"]["card_image"]["url"],
        "https://img.example.com/big.png"
    );
    assert_eq!(json["template_card"]["card_image"]["aspect_ratio"], 1.5);

    // 仅 ratio（无 url）也输出 card_image
    let ratio_only = WxCpMessage::templatecard()
        .card_type("news_notice")
        .card_image_aspect_ratio(1.5)
        .build();
    let ratio_json: serde_json::Value = serde_json::from_str(&ratio_only.to_json()).unwrap();
    assert_eq!(
        ratio_json["template_card"]["card_image"],
        serde_json::json!({"aspect_ratio":1.5})
    );

    // 两者皆空 → card_image 键省略
    let none = WxCpMessage::templatecard().card_type("news_notice").build();
    let none_json: serde_json::Value = serde_json::from_str(&none.to_json()).unwrap();
    assert!(none_json["template_card"].get("card_image").is_none());
}

/// 对应 Java: emphasisContent（title/desc 单字段即可触发）+ 单侧 main_title。
#[test]
fn template_card_optional_single_field_branches() {
    let only_title = WxCpMessage::templatecard()
        .card_type("text_notice")
        .emphasis_content_title("100")
        .main_title_title("仅标题")
        .build();
    let json: serde_json::Value = serde_json::from_str(&only_title.to_json()).unwrap();
    assert_eq!(
        json["template_card"]["emphasis_content"],
        serde_json::json!({"title":"100"})
    );
    assert_eq!(
        json["template_card"]["main_title"],
        serde_json::json!({"title":"仅标题"})
    );

    let only_desc = WxCpMessage::templatecard()
        .card_type("text_notice")
        .emphasis_content_desc("核心数据")
        .main_title_desc("仅描述")
        .build();
    let json2: serde_json::Value = serde_json::from_str(&only_desc.to_json()).unwrap();
    assert_eq!(
        json2["template_card"]["emphasis_content"],
        serde_json::json!({"desc":"核心数据"})
    );
    assert_eq!(
        json2["template_card"]["main_title"],
        serde_json::json!({"desc":"仅描述"})
    );

    // 两侧皆空 → 键省略
    let none = WxCpMessage::templatecard().card_type("text_notice").build();
    let none_json: serde_json::Value = serde_json::from_str(&none.to_json()).unwrap();
    assert!(none_json["template_card"].get("emphasis_content").is_none());
    assert!(none_json["template_card"].get("main_title").is_none());
    assert!(none_json["template_card"].get("source").is_none());
    assert!(none_json["template_card"].get("action_menu").is_none());
}
