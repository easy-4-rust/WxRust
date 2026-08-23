//! 企业微信群机器人消息与成员信息覆盖率提升测试（纯离线，无网络依赖）。
//!
//! 覆盖：
//! - `bean/message/wx_cp_group_robot_message.rs`：`to_json` 全部 msgtype 分支
//!   （text/markdown/markdown_v2/image/news/file/template_card）；
//! - `bean/wx_cp_user.rs`：`WxCpUser` 自定义 serde（序列化键序 + 反序列化
//!   golden + extattr/external_profile 各 type 形态 + Gender 码表）。
//!
//! 断言策略：builder 路径用 golden 字符串；字段较散的路径断言 serde_json::Value
//! 的字段名与类型（不依赖键序）。

use wx_rust_cp::bean::article::NewArticle;
use wx_rust_cp::bean::gender::Gender;
use wx_rust_cp::bean::message::WxCpGroupRobotMessage;
use wx_rust_cp::bean::taskcard::TaskCardButton;
use wx_rust_cp::bean::templatecard::{
    ActionMenuItem, CheckboxOption, HorizontalContent, MultipleSelect, QuoteArea,
    TemplateCardButton, TemplateCardJump, VerticalContent,
};
use wx_rust_cp::bean::wx_cp_user::{Attr, ExternalAttribute, WxCpUser};

// ========================================================================
// 一、群机器人基础消息类型（对应 Java WxCpGroupRobotMessage.toJson）
// ========================================================================

/// 对应 Java: WxCpGroupRobotServiceImplTest（text + mentioned 双列表）。
#[test]
fn robot_text_with_mentioned_lists_golden() {
    let mut msg = WxCpGroupRobotMessage::default();
    msg.msg_type = Some("text".into());
    msg.content = Some("hello".into());
    msg.mentioned_list = vec!["userid1".into(), "userid2".into()];
    msg.mentioned_mobile_list = vec!["13800000000".into()];
    assert_eq!(
        msg.to_json(),
        "{\"msgtype\":\"text\",\"text\":{\"content\":\"hello\",\"mentioned_list\":[\"userid1\",\"userid2\"],\"mentioned_mobile_list\":[\"13800000000\"]}}"
    );

    // 空列表 → 空数组；未设置 content → null
    let empty = WxCpGroupRobotMessage::default();
    let _ = empty;
    let mut plain = WxCpGroupRobotMessage::default();
    plain.msg_type = Some("text".into());
    assert_eq!(
        plain.to_json(),
        "{\"msgtype\":\"text\",\"text\":{\"content\":null,\"mentioned_list\":[],\"mentioned_mobile_list\":[]}}"
    );
}

/// 对应 Java: WxCpGroupRobotMessage.toJson（markdown / markdown_v2 分支）。
#[test]
fn robot_markdown_and_markdown_v2_golden() {
    let mut md = WxCpGroupRobotMessage::default();
    md.msg_type = Some("markdown".into());
    md.content = Some("**加粗**".into());
    assert_eq!(
        md.to_json(),
        "{\"msgtype\":\"markdown\",\"markdown\":{\"content\":\"**加粗**\"}}"
    );

    let mut md2 = WxCpGroupRobotMessage::default();
    md2.msg_type = Some("markdown_v2".into());
    md2.content = Some("<md>实时天气</md>".into());
    assert_eq!(
        md2.to_json(),
        "{\"msgtype\":\"markdown_v2\",\"markdown_v2\":{\"content\":\"<md>实时天气</md>\"}}"
    );
}

/// 对应 Java: WxCpGroupRobotMessage.toJson（image base64/md5 分支）。
#[test]
fn robot_image_base64_md5_golden() {
    let mut msg = WxCpGroupRobotMessage::default();
    msg.msg_type = Some("image".into());
    msg.base64 = Some("QkFTRTY0".into());
    msg.md5 = Some("MD5SUM".into());
    assert_eq!(
        msg.to_json(),
        "{\"msgtype\":\"image\",\"image\":{\"base64\":\"QkFTRTY0\",\"md5\":\"MD5SUM\"}}"
    );
}

/// 对应 Java: WxCpGroupRobotMessage.toJson（news 四字段 + file media_id）。
#[test]
fn robot_news_and_file_golden() {
    let mut news = WxCpGroupRobotMessage::default();
    news.msg_type = Some("news".into());
    news.articles = vec![NewArticle {
        title: "中秋节礼品领取".into(),
        description: "今年中秋节公司发放的礼品有五仁月饼、辣条".into(),
        url: "https://work.weixin.qq.com".into(),
        pic_url: "https://pic.example.com/a.png".into(),
        ..Default::default()
    }];
    assert_eq!(
        news.to_json(),
        "{\"msgtype\":\"news\",\"news\":{\"articles\":[{\"title\":\"中秋节礼品领取\",\"description\":\"今年中秋节公司发放的礼品有五仁月饼、辣条\",\"url\":\"https://work.weixin.qq.com\",\"picurl\":\"https://pic.example.com/a.png\"}]}}"
    );

    let mut file = WxCpGroupRobotMessage::default();
    file.msg_type = Some("file".into());
    file.media_id = Some("FILE_MEDIA_ID".into());
    assert_eq!(
        file.to_json(),
        "{\"msgtype\":\"file\",\"file\":{\"media_id\":\"FILE_MEDIA_ID\"}}"
    );
}

/// 对应 Java: WxCpGroupRobotMessage.toJson（agentid 键 + 未知/缺失 msgtype）。
#[test]
fn robot_agent_id_and_unknown_msg_type() {
    let mut msg = WxCpGroupRobotMessage::default();
    msg.msg_type = Some("text".into());
    msg.agent_id = Some(1000002);
    msg.content = Some("c".into());
    let json: serde_json::Value = serde_json::from_str(&msg.to_json()).unwrap();
    assert_eq!(json["agentid"], 1000002);

    let mut unknown = WxCpGroupRobotMessage::default();
    unknown.msg_type = Some("unknown".into());
    assert_eq!(unknown.to_json(), "{\"msgtype\":\"unknown\"}");
    assert_eq!(
        WxCpGroupRobotMessage::default().to_json(),
        "{\"msgtype\":null}"
    );
}

// ========================================================================
// 二、群机器人模板卡片（template_card 分支全量子样式）
// ========================================================================

/// 对应 Java: WxCpGroupRobotMessage.toJson template_card 全量字段 golden。
#[test]
fn robot_template_card_full_golden() {
    let msg = WxCpGroupRobotMessage {
        msg_type: Some("template_card".into()),
        agent_id: Some(1000002),
        card_type: Some("text_notice".into()),
        source_icon_url: Some("图片的url".into()),
        source_desc: Some("企业微信".into()),
        source_desc_color: Some(1),
        action_menu_desc: Some("卡片副交互辅助文本说明".into()),
        action_menu_action_list: vec![ActionMenuItem {
            text: "接受推送".into(),
            key: "A".into(),
        }],
        main_title_title: Some("欢迎使用企业微信".into()),
        main_title_desc: Some("您的好友正在邀请您加入企业微信".into()),
        card_image_url: Some("https://img.example.com".into()),
        card_image_aspect_ratio: Some(1.5),
        emphasis_content_title: Some("100".into()),
        emphasis_content_desc: Some("核心数据".into()),
        sub_title_text: Some("下载企业微信还能抢红包！".into()),
        vertical_contents: vec![VerticalContent {
            title: "惊喜红包等你来拿".into(),
            desc: "下载企业微信还能抢红包！".into(),
        }],
        horizontal_contents: vec![HorizontalContent {
            keyname: "邀请人".into(),
            value: "张三".into(),
            ..Default::default()
        }],
        jumps: vec![TemplateCardJump {
            r#type: 1,
            title: "企业微信官网".into(),
            url: "https://work.weixin.qq.com".into(),
            ..Default::default()
        }],
        card_action_type: Some(1),
        card_action_url: Some("https://work.weixin.qq.com".into()),
        buttons: vec![TemplateCardButton {
            text: "按钮1".into(),
            style: 1,
            key: "button_key_1".into(),
            ..Default::default()
        }],
        checkbox_question_key: Some("question_key1".into()),
        checkbox_mode: Some(1),
        options: vec![CheckboxOption::new("option_id1", "选择题选项1", Some(true))],
        submit_button_text: Some("提交".into()),
        submit_button_key: Some("key".into()),
        selects: vec![MultipleSelect {
            question_key: "select_qk".into(),
            title: "选择器标签".into(),
            selected_id: "selection_id1".into(),
            options: vec![CheckboxOption::new("selection_id1", "选择器选项1", None)],
        }],
        quote_area: Some(QuoteArea {
            r#type: 1,
            url: "https://work.weixin.qq.com".into(),
            title: "引用文献标题".into(),
            quote_text: "引用文献样式的引用文案".into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let json: serde_json::Value = serde_json::from_str(&msg.to_json()).unwrap();
    let card = &json["template_card"];
    // 顶层键序：msgtype → agentid → template_card
    let top: Vec<&str> = json
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.as_str())
        .collect();
    assert_eq!(top, vec!["msgtype", "agentid", "template_card"]);
    assert_eq!(json["msgtype"], "template_card");
    // 各子样式字段名与类型
    assert_eq!(card["card_type"], "text_notice");
    assert_eq!(
        card["source"],
        serde_json::json!({"icon_url":"图片的url","desc":"企业微信","desc_color":1})
    );
    assert_eq!(card["action_menu"]["desc"], "卡片副交互辅助文本说明");
    assert_eq!(card["action_menu"]["action_list"][0]["key"], "A");
    assert_eq!(card["main_title"]["title"], "欢迎使用企业微信");
    assert_eq!(card["card_image"]["aspect_ratio"], 1.5);
    assert_eq!(
        card["emphasis_content"],
        serde_json::json!({"title":"100","desc":"核心数据"})
    );
    assert_eq!(card["sub_title_text"], "下载企业微信还能抢红包！");
    assert_eq!(
        card["vertical_content_list"][0]["title"],
        "惊喜红包等你来拿"
    );
    assert_eq!(
        card["horizontal_content_list"][0],
        serde_json::json!({"keyname":"邀请人","value":"张三"})
    );
    assert_eq!(card["jump_list"][0]["url"], "https://work.weixin.qq.com");
    assert_eq!(card["card_action"]["type"], 1);
    assert_eq!(card["button_list"][0]["key"], "button_key_1");
    assert_eq!(card["checkbox"]["question_key"], "question_key1");
    assert_eq!(card["checkbox"]["option_list"][0]["is_checked"], true);
    assert_eq!(
        card["submit_button"],
        serde_json::json!({"text":"提交","key":"key"})
    );
    assert_eq!(card["select_list"][0]["question_key"], "select_qk");
    assert_eq!(
        card["select_list"][0]["option_list"][0]["id"],
        "selection_id1"
    );
    assert_eq!(card["quote_area"]["quote_text"], "引用文献样式的引用文案");

    // 仅 card_type：其余键全部省略
    let mut bare = WxCpGroupRobotMessage::default();
    bare.msg_type = Some("template_card".into());
    bare.card_type = Some("news_notice".into());
    assert_eq!(
        bare.to_json(),
        "{\"msgtype\":\"template_card\",\"template_card\":{\"card_type\":\"news_notice\"}}"
    );
    // TaskCardButton 仅用于类型对齐（机器人卡片不带 taskcard 按钮）
    let _btn = TaskCardButton::new("k", "n", None, None, None);
    let _ = &_btn;
}

// ========================================================================
// 三、WxCpUser 序列化（Java WxCpUserGsonAdapter 键序）
// ========================================================================

/// 对应 Java: WxCpUserGsonAdapter.serialize 全字段 golden（键序一致）。
#[test]
fn user_to_json_full_golden() {
    let mut user = WxCpUser {
        user_id: Some("zhangsan".into()),
        new_user_id: Some("newzs".into()),
        name: Some("张三".into()),
        depart_ids: Some(vec![1, 2]),
        orders: Some(vec![10, 20]),
        position: Some("工程师".into()),
        positions: Some(vec!["职位1".into(), "职位2".into()]),
        mobile: Some("13800000000".into()),
        gender: Some(Gender::Male),
        email: Some("z@e.com".into()),
        biz_mail: Some("b@corp.com".into()),
        avatar: Some("https://a.png".into()),
        thumb_avatar: Some("https://t.png".into()),
        address: Some("北京市海淀区".into()),
        avatar_media_id: Some("AMID".into()),
        status: Some(1),
        enable: Some(1),
        alias: Some("zs".into()),
        is_leader: Some(1),
        is_leader_in_dept: Some(vec![1, 0]),
        hide_mobile: Some(0),
        english_name: Some("San".into()),
        telephone: Some("01012345678".into()),
        qr_code: Some("https://qr".into()),
        to_invite: Some(true),
        open_user_id: Some("ou123".into()),
        main_department: Some("1".into()),
        direct_leader: Some(vec!["leader1".into(), "leader2".into()]),
        external_position: Some("外部职位".into()),
        ..Default::default()
    };
    user.add_ext_attr("爱好", "旅游");
    user.add_external_attr(ExternalAttribute {
        r#type: 0,
        name: Some("文本属性".into()),
        value: Some("tv".into()),
        ..Default::default()
    });
    user.add_external_attr(ExternalAttribute {
        r#type: 1,
        name: Some("网页属性".into()),
        url: Some("https://w".into()),
        title: Some("wt".into()),
        ..Default::default()
    });
    user.add_external_attr(ExternalAttribute {
        r#type: 2,
        name: Some("小程序属性".into()),
        appid: Some("wx".into()),
        page_path: Some("/p".into()),
        title: Some("mt".into()),
        ..Default::default()
    });
    user.external_profile.external_corp_name = Some("EC".into());
    user.external_profile.wechat_channels = Some(wx_rust_cp::bean::wx_cp_user::WechatChannels {
        nickname: Some("视频号".into()),
        status: Some(1),
    });
    assert_eq!(
        user.to_json().unwrap(),
        "{\"userid\":\"zhangsan\",\"new_userid\":\"newzs\",\"name\":\"张三\",\"department\":[1,2],\"order\":[10,20],\"position\":\"工程师\",\"positions\":[\"职位1\",\"职位2\"],\"mobile\":\"13800000000\",\"gender\":\"1\",\"email\":\"z@e.com\",\"biz_mail\":\"b@corp.com\",\"avatar\":\"https://a.png\",\"thumb_avatar\":\"https://t.png\",\"address\":\"北京市海淀区\",\"avatar_mediaid\":\"AMID\",\"status\":1,\"enable\":1,\"alias\":\"zs\",\"isleader\":1,\"is_leader_in_dept\":[1,0],\"hide_mobile\":0,\"english_name\":\"San\",\"telephone\":\"01012345678\",\"qr_code\":\"https://qr\",\"to_invite\":true,\"open_userid\":\"ou123\",\"main_department\":\"1\",\"direct_leader\":[\"leader1\",\"leader2\"],\"extattr\":{\"attrs\":[{\"type\":0,\"name\":\"爱好\",\"text\":{\"value\":\"旅游\"}}]},\"external_position\":\"外部职位\",\"external_profile\":{\"external_corp_name\":\"EC\",\"wechat_channels\":{\"nickname\":\"视频号\",\"status\":1},\"external_attr\":[{\"type\":0,\"name\":\"文本属性\",\"text\":{\"value\":\"tv\"}},{\"type\":1,\"name\":\"网页属性\",\"web\":{\"url\":\"https://w\",\"title\":\"wt\"}},{\"type\":2,\"name\":\"小程序属性\",\"miniprogram\":{\"appid\":\"wx\",\"pagepath\":\"/p\",\"title\":\"mt\"}}]}}"
    );
}

/// 对应 Java: directLeader 非 null（含空数组）必输出（清空直连上级）。
#[test]
fn user_to_json_direct_leader_empty_array_and_profile_always_output() {
    let mut user = WxCpUser::default();
    user.user_id = Some("u1".into());
    user.direct_leader = Some(vec![]);
    assert_eq!(
        user.to_json().unwrap(),
        "{\"userid\":\"u1\",\"direct_leader\":[],\"external_profile\":{}}"
    );

    // 空数组 department/order/positions/is_leader_in_dept → 键省略
    let mut sparse = WxCpUser::default();
    sparse.depart_ids = Some(vec![]);
    sparse.orders = Some(vec![]);
    sparse.positions = Some(vec![]);
    sparse.is_leader_in_dept = Some(vec![]);
    assert_eq!(sparse.to_json().unwrap(), "{\"external_profile\":{}}");
}

// ========================================================================
// 四、Attr / ExternalAttribute 线格式（type null/0/1/2/其他）
// ========================================================================

/// 对应 Java: WxCpUser.Attr 三种线格式 + 其他 type 忽略。
#[test]
fn attr_wire_formats() {
    // type==null → value 直挂
    let null_type = Attr {
        name: Some("n0".into()),
        text_value: Some("v0".into()),
        ..Default::default()
    };
    let mut user = WxCpUser::default();
    user.ext_attrs.push(null_type);
    assert_eq!(
        user.to_json().unwrap(),
        "{\"extattr\":{\"attrs\":[{\"name\":\"n0\",\"value\":\"v0\"}]},\"external_profile\":{}}"
    );

    // type==1（网页）
    let web = Attr {
        r#type: Some(1),
        name: Some("n1".into()),
        web_url: Some("https://w".into()),
        web_title: Some("t1".into()),
        ..Default::default()
    };
    let mut user2 = WxCpUser::default();
    user2.ext_attrs.push(web);
    assert_eq!(
        user2.to_json().unwrap(),
        "{\"extattr\":{\"attrs\":[{\"type\":1,\"name\":\"n1\",\"web\":{\"url\":\"https://w\",\"title\":\"t1\"}}]},\"external_profile\":{}}"
    );

    // type==2（其他）→ 仅 type+name
    let other = Attr {
        r#type: Some(2),
        name: Some("n2".into()),
        ..Default::default()
    };
    let mut user3 = WxCpUser::default();
    user3.ext_attrs.push(other);
    assert_eq!(
        user3.to_json().unwrap(),
        "{\"extattr\":{\"attrs\":[{\"type\":2,\"name\":\"n2\"}]},\"external_profile\":{}}"
    );
}

/// 对应 Java: ExternalAttribute type 3（未知）→ 仅 type+name。
#[test]
fn external_attribute_unknown_type_ignored() {
    let mut user = WxCpUser::default();
    user.add_external_attr(ExternalAttribute {
        r#type: 3,
        name: Some("n3".into()),
        value: Some("ignored".into()),
        ..Default::default()
    });
    assert_eq!(
        user.to_json().unwrap(),
        "{\"external_profile\":{\"external_attr\":[{\"type\":3,\"name\":\"n3\"}]}}"
    );
}

// ========================================================================
// 五、WxCpUser 反序列化（golden JSON → 结构体字段断言）
// ========================================================================

/// 对应 Java: WxCpUserGsonAdapter.deserialize 全字段 golden。
#[test]
fn user_from_json_full_fields() {
    let json = r#"{
        "userid":"zhangsan","new_userid":"newzs","name":"张三",
        "department":[1,2],"order":[10,20],"position":"工程师",
        "positions":["职位1","职位2"],"mobile":"13800000000","gender":"2",
        "email":"z@e.com","biz_mail":"b@corp.com","avatar":"https://a.png",
        "thumb_avatar":"https://t.png","address":"北京市海淀区",
        "avatar_mediaid":"AMID","status":1,"enable":1,"alias":"zs",
        "isleader":1,"is_leader_in_dept":[1,0],"hide_mobile":0,
        "english_name":"San","telephone":"01012345678","qr_code":"https://qr",
        "to_invite":true,"open_userid":"ou123","main_department":"1",
        "direct_leader":["leader1"],
        "extattr":{"attrs":[
            {"name":"n0","value":"v0"},
            {"type":0,"name":"n1","text":{"value":"tv"}},
            {"type":1,"name":"n2","web":{"url":"https://w","title":"wt"}}
        ]},
        "external_position":"外部职位",
        "external_profile":{
            "external_corp_name":"EC",
            "wechat_channels":{"nickname":"视频号","status":1},
            "external_attr":[
                {"type":0,"name":"文本属性","text":{"value":"tv"}},
                {"type":1,"name":"网页属性","web":{"url":"https://w","title":"wt"}},
                {"type":2,"name":"小程序属性","miniprogram":{"appid":"wx","pagepath":"/p","title":"mt"}},
                {"name":"无类型跳过"}
            ]
        }
    }"#;
    let user = WxCpUser::from_json(json).unwrap();
    assert_eq!(user.user_id.as_deref(), Some("zhangsan"));
    assert_eq!(user.new_user_id.as_deref(), Some("newzs"));
    assert_eq!(user.name.as_deref(), Some("张三"));
    assert_eq!(user.depart_ids, Some(vec![1, 2]));
    assert_eq!(user.orders, Some(vec![10, 20]));
    assert_eq!(user.position.as_deref(), Some("工程师"));
    assert_eq!(
        user.positions,
        Some(vec!["职位1".to_string(), "职位2".to_string()])
    );
    assert_eq!(user.mobile.as_deref(), Some("13800000000"));
    assert_eq!(user.gender, Some(Gender::Female));
    assert_eq!(user.email.as_deref(), Some("z@e.com"));
    assert_eq!(user.biz_mail.as_deref(), Some("b@corp.com"));
    assert_eq!(user.avatar.as_deref(), Some("https://a.png"));
    assert_eq!(user.thumb_avatar.as_deref(), Some("https://t.png"));
    assert_eq!(user.address.as_deref(), Some("北京市海淀区"));
    assert_eq!(user.avatar_media_id.as_deref(), Some("AMID"));
    assert_eq!(user.status, Some(1));
    assert_eq!(user.enable, Some(1));
    assert_eq!(user.alias.as_deref(), Some("zs"));
    assert_eq!(user.is_leader, Some(1));
    assert_eq!(user.is_leader_in_dept, Some(vec![1, 0]));
    assert_eq!(user.hide_mobile, Some(0));
    assert_eq!(user.english_name.as_deref(), Some("San"));
    assert_eq!(user.telephone.as_deref(), Some("01012345678"));
    assert_eq!(user.qr_code.as_deref(), Some("https://qr"));
    assert_eq!(user.to_invite, Some(true));
    assert_eq!(user.open_user_id.as_deref(), Some("ou123"));
    assert_eq!(user.main_department.as_deref(), Some("1"));
    assert_eq!(user.direct_leader, Some(vec!["leader1".to_string()]));
    assert_eq!(user.external_position.as_deref(), Some("外部职位"));
    // extattr：value 直挂 + text + web 三形态
    assert_eq!(user.ext_attrs.len(), 3);
    assert_eq!(user.ext_attrs[0].r#type, None);
    assert_eq!(user.ext_attrs[0].text_value.as_deref(), Some("v0"));
    assert_eq!(user.ext_attrs[1].r#type, Some(0));
    assert_eq!(user.ext_attrs[1].text_value.as_deref(), Some("tv"));
    assert_eq!(user.ext_attrs[2].r#type, Some(1));
    assert_eq!(user.ext_attrs[2].web_url.as_deref(), Some("https://w"));
    assert_eq!(user.ext_attrs[2].web_title.as_deref(), Some("wt"));
    // external_profile：三类属性 + 无 type 项跳过
    let profile = &user.external_profile;
    assert_eq!(profile.external_corp_name.as_deref(), Some("EC"));
    assert_eq!(
        profile
            .wechat_channels
            .as_ref()
            .unwrap()
            .nickname
            .as_deref(),
        Some("视频号")
    );
    assert_eq!(profile.wechat_channels.as_ref().unwrap().status, Some(1));
    assert_eq!(profile.external_attrs.len(), 3);
    assert_eq!(profile.external_attrs[0].value.as_deref(), Some("tv"));
    assert_eq!(profile.external_attrs[1].url.as_deref(), Some("https://w"));
    assert_eq!(profile.external_attrs[1].title.as_deref(), Some("wt"));
    assert_eq!(profile.external_attrs[2].appid.as_deref(), Some("wx"));
    assert_eq!(profile.external_attrs[2].page_path.as_deref(), Some("/p"));
    assert_eq!(profile.external_attrs[2].title.as_deref(), Some("mt"));
}

/// 对应 Java: Gender.fromCode（"0"/"1"/"2" + 未知码 → None）。
#[test]
fn user_from_json_gender_codes() {
    for (code, gender) in [
        ("0", Some(Gender::Undefined)),
        ("1", Some(Gender::Male)),
        ("2", Some(Gender::Female)),
        ("9", None),
    ] {
        let user = WxCpUser::from_json(&format!(r#"{{"gender":"{code}"}}"#)).unwrap();
        assert_eq!(user.gender, gender, "gender code {code}");
    }
    // 数字形态 gender（非字符串）→ None
    let user = WxCpUser::from_json(r#"{"gender":1}"#).unwrap();
    assert_eq!(user.gender, None);
}

/// 对应 Java: WxCpUser.fromJson 错误路径（非 JSON 对象）。
#[test]
fn user_from_json_non_object_error() {
    assert!(WxCpUser::from_json("[1,2]").is_err());
    assert!(WxCpUser::from_json("\"str\"").is_err());
    assert!(WxCpUser::from_json("not json").is_err());
}

/// 对应 Java: Gson 平铺 roundtrip（to_json → from_json 等价）。
#[test]
fn user_roundtrip_to_json_from_json() {
    let mut user = WxCpUser {
        user_id: Some("u1".into()),
        name: Some("张三".into()),
        depart_ids: Some(vec![1]),
        gender: Some(Gender::Male),
        status: Some(1),
        to_invite: Some(false),
        direct_leader: Some(vec![]),
        ..Default::default()
    };
    user.add_ext_attr("爱好", "旅游");
    user.add_external_attr(ExternalAttribute {
        r#type: 0,
        name: Some("文本属性".into()),
        value: Some("tv".into()),
        ..Default::default()
    });
    let wire = user.to_json().unwrap();
    let parsed = WxCpUser::from_json(&wire).unwrap();
    assert_eq!(parsed.user_id, user.user_id);
    assert_eq!(parsed.name, user.name);
    assert_eq!(parsed.depart_ids, user.depart_ids);
    assert_eq!(parsed.gender, user.gender);
    assert_eq!(parsed.status, user.status);
    assert_eq!(parsed.to_invite, user.to_invite);
    assert_eq!(parsed.direct_leader, user.direct_leader);
    assert_eq!(parsed.ext_attrs, user.ext_attrs);
    assert_eq!(parsed.external_profile, user.external_profile);
    assert_eq!(parsed.to_json().unwrap(), wire);
}
