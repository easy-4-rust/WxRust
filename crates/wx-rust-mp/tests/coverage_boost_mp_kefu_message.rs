//! 公众号客服消息（WxMpKefuMessage）覆盖率提升测试（纯离线，无网络依赖）。
//!
//! 覆盖 `bean/kefu/wx_mp_kefu_message.rs`：
//! - `to_json` 全部 msgtype 分支（text/image/voice/video/music/news/mpnews/
//!   wxcard/miniprogrampage/msgmenu/mpnewsarticle + 非法类型错误路径）；
//! - `KefuMessageBuilder` 全部 setter 方法与 `customservice` 会话拼接；
//! - `from_json` 平铺反序列化 + `WxArticle`/`MsgMenu` 构造器。
//!
//! 断言策略：builder 路径用 golden 字符串（键序与 Java
//! `WxMpKefuMessageGsonAdapter` 一致）；错误路径断言错误信息。

use wx_rust_mp::bean::kefu::{KefuMessageBuilder, MsgMenu, WxArticle, WxMpKefuMessage};

// ========================================================================
// 一、基础消息类型 builder → to_json golden（对应 Java WxMpKefuMessageTest）
// ========================================================================

/// 对应 Java: WxMpKefuMessageTest.testTextBuild
#[test]
fn kefu_text_build_golden() {
    let reply = WxMpKefuMessage::text()
        .to_user("OPENID")
        .content("sfsfdsdf")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"sfsfdsdf\"}}"
    );
    assert_eq!(reply.get_msg_type(), "text");
}

/// 对应 Java: WxMpKefuMessageTest.testImageBuild / testVoiceBuild。
#[test]
fn kefu_image_and_voice_build_golden() {
    let image = WxMpKefuMessage::image()
        .to_user("OPENID")
        .media_id("MEDIA_ID")
        .build();
    assert_eq!(
        image.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"image\",\"image\":{\"media_id\":\"MEDIA_ID\"}}"
    );

    let voice = WxMpKefuMessage::voice()
        .to_user("OPENID")
        .media_id("MEDIA_ID")
        .build();
    assert_eq!(
        voice.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"voice\",\"voice\":{\"media_id\":\"MEDIA_ID\"}}"
    );
    assert_eq!(voice.get_msg_type(), "voice");
}

/// 对应 Java: WxMpKefuMessageTest.testVideoBuild
#[test]
fn kefu_video_build_golden() {
    let reply = WxMpKefuMessage::video()
        .to_user("OPENID")
        .title("TITLE")
        .media_id("MEDIA_ID")
        .thumb_media_id("MEDIA_ID")
        .description("DESCRIPTION")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"video\",\"video\":{\"media_id\":\"MEDIA_ID\",\"thumb_media_id\":\"MEDIA_ID\",\"title\":\"TITLE\",\"description\":\"DESCRIPTION\"}}"
    );
}

/// 对应 Java: WxMpKefuMessageTest.testMusicBuild（musicurl/hqmusicurl 键名）。
#[test]
fn kefu_music_build_golden() {
    let reply = WxMpKefuMessage::music()
        .to_user("OPENID")
        .title("TITLE")
        .thumb_media_id("MEDIA_ID")
        .description("DESCRIPTION")
        .music_url("MUSIC_URL")
        .hq_music_url("HQ_MUSIC_URL")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"music\",\"music\":{\"title\":\"TITLE\",\"description\":\"DESCRIPTION\",\"thumb_media_id\":\"MEDIA_ID\",\"musicurl\":\"MUSIC_URL\",\"hqmusicurl\":\"HQ_MUSIC_URL\"}}"
    );
}

/// 对应 Java: WxMpKefuMessageTest.testNewsBuild（add_article 逐条 + WxArticle::new）。
#[test]
fn kefu_news_build_golden() {
    let article = || WxArticle::new("Happy Day", "Is Really A Happy Day", "URL", "PIC_URL");
    let reply = WxMpKefuMessage::news()
        .to_user("OPENID")
        .add_article(article())
        .add_article(article())
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"news\",\"news\":{\"articles\":[{\"title\":\"Happy Day\",\"description\":\"Is Really A Happy Day\",\"url\":\"URL\",\"picurl\":\"PIC_URL\"},{\"title\":\"Happy Day\",\"description\":\"Is Really A Happy Day\",\"url\":\"URL\",\"picurl\":\"PIC_URL\"}]}}"
    );
    assert_eq!(reply.articles.len(), 2);
    assert_eq!(reply.articles[0].url, "URL");
}

/// 对应 Java: WxMpKefuMessageTest.testMiniProgramPageBuild
#[test]
fn kefu_miniprogrampage_build_golden() {
    let reply = WxMpKefuMessage::miniprogrampage()
        .to_user("OPENID")
        .title("title")
        .app_id("appid")
        .page_path("pagepath")
        .thumb_media_id("thumb_media_id")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"miniprogrampage\",\"miniprogrampage\":{\"title\":\"title\",\"appid\":\"appid\",\"pagepath\":\"pagepath\",\"thumb_media_id\":\"thumb_media_id\"}}"
    );
}

/// 对应 Java: WxMpKefuMessageTest.testMsgMenuBuild（head/list/tail 三段式）。
#[test]
fn kefu_msgmenu_build_golden() {
    let reply = WxMpKefuMessage::msgmenu()
        .to_user("OPENID")
        .add_menus(vec![
            MsgMenu::new("101", "msgmenu1"),
            MsgMenu::new("102", "msgmenu2"),
        ])
        .head_content("head_content")
        .tail_content("tail_content")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"msgmenu\",\"msgmenu\":{\"head_content\":\"head_content\",\"list\":[{\"id\":\"101\",\"content\":\"msgmenu1\"},{\"id\":\"102\",\"content\":\"msgmenu2\"}],\"tail_content\":\"tail_content\"}}"
    );
    // MsgMenu 构造器字段
    let menu = MsgMenu::new("101", "msgmenu1");
    assert_eq!(menu.id, "101");
    assert_eq!(menu.content, "msgmenu1");
    assert_eq!(reply.msg_menus.len(), 2);
}

/// 对应 Java: WxMpKefuMessageTest.testMpNewsArticleBuilder
#[test]
fn kefu_mpnewsarticle_build_golden() {
    let reply = WxMpKefuMessage::mpnewsarticle()
        .to_user("OPENID")
        .article_id("ARTICLE_ID")
        .build();
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"mpnewsarticle\",\"mpnewsarticle\":{\"article_id\":\"ARTICLE_ID\"}}"
    );
}

/// 对应 Java: WxConsts.KefuMsgType.MPNEWS / WXCARD（builder 组装）。
#[test]
fn kefu_mpnews_and_wxcard_build_golden() {
    // mpnews 使用 mp_news_media_id 字段（builder 未提供 setter，直接组装）
    let mut mpnews = WxMpKefuMessage::mpnews().to_user("OPENID").build();
    mpnews.mp_news_media_id = Some("MPNEWS_MEDIA_ID".into());
    assert_eq!(
        mpnews.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"mpnews\",\"mpnews\":{\"media_id\":\"MPNEWS_MEDIA_ID\"}}"
    );

    let wxcard = WxMpKefuMessage::wxcard()
        .to_user("OPENID")
        .card_id("CARD_ID")
        .build();
    assert_eq!(
        wxcard.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"wxcard\",\"wxcard\":{\"card_id\":\"CARD_ID\"}}"
    );
}

// ========================================================================
// 二、customservice 会话拼接与空对象分支
// ========================================================================

/// 对应 Java: WxMpKefuMessageGsonAdapter（kf_account 非空追加 customservice）。
#[test]
fn kefu_customservice_appended_when_kf_account_present() {
    let mut reply = WxMpKefuMessage::text()
        .to_user("OPENID")
        .content("hi")
        .build();
    reply.kf_account = Some("test1@kfbiz".into());
    assert_eq!(
        reply.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"hi\"},\"customservice\":{\"kf_account\":\"test1@kfbiz\"}}"
    );

    // kf_account 为空字符串 → 不追加 customservice
    let mut empty_kf = WxMpKefuMessage::text()
        .to_user("OPENID")
        .content("hi")
        .build();
    empty_kf.kf_account = Some(String::new());
    assert_eq!(
        empty_kf.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"hi\"}}"
    );
}

/// 对应 Java: 各分支字段缺省形态（子对象字段省略 → 空对象）。
#[test]
fn kefu_empty_subobject_forms() {
    let text = WxMpKefuMessage::text().build();
    assert_eq!(
        text.to_json().unwrap(),
        "{\"msgtype\":\"text\",\"text\":{}}"
    );

    let image = WxMpKefuMessage::image().to_user("OPENID").build();
    assert_eq!(
        image.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"image\",\"image\":{}}"
    );

    // touser/msgtype 均缺省 → 仅空消息体
    let voice = WxMpKefuMessage::voice().build();
    assert_eq!(
        voice.to_json().unwrap(),
        "{\"msgtype\":\"voice\",\"voice\":{}}"
    );

    let news = WxMpKefuMessage::news().build();
    assert_eq!(
        news.to_json().unwrap(),
        "{\"msgtype\":\"news\",\"news\":{\"articles\":[]}}"
    );

    let msgmenu = WxMpKefuMessage::msgmenu().build();
    assert_eq!(
        msgmenu.to_json().unwrap(),
        "{\"msgtype\":\"msgmenu\",\"msgmenu\":{\"list\":[]}}"
    );
}

// ========================================================================
// 三、非法消息类型错误路径
// ========================================================================

/// 对应 Java: WxMpKefuMessageGsonAdapter.serialize（未知 msgtype 抛错语义）。
#[test]
fn kefu_illegal_msg_type_error() {
    let msg = KefuMessageBuilder::new("illegal").to_user("OPENID").build();
    let err = msg.to_json().unwrap_err();
    assert_eq!(err, "非法消息类型，暂不支持: illegal");

    // msg_type 缺省 → 空串同样落入非法分支
    let default_msg = WxMpKefuMessage::default();
    assert_eq!(default_msg.get_msg_type(), "");
    assert_eq!(
        default_msg.to_json().unwrap_err(),
        "非法消息类型，暂不支持: "
    );
}

// ========================================================================
// 四、from_json 反序列化（Gson 平铺映射语义）
// ========================================================================

/// 对应 Java: Gson 反序列化（顶层平铺字段 + articles/msg_menus 容器）。
#[test]
fn kefu_from_json_flat_fields() {
    let json = r#"{
        "touser":"OPENID",
        "msgtype":"text",
        "content":"hello",
        "media_id":"MEDIA_ID",
        "thumb_media_id":"THUMB_ID",
        "title":"TITLE",
        "description":"DESC",
        "music_url":"MUSIC_URL",
        "hq_music_url":"HQ_MUSIC_URL",
        "kf_account":"kf@test",
        "card_id":"CARD_ID",
        "mp_news_media_id":"MP_ID",
        "mini_program_app_id":"wx_app",
        "mini_program_page_path":"/pages/i",
        "head_content":"HEAD",
        "tail_content":"TAIL",
        "articles":[{"title":"t","description":"d","url":"u","pic_url":"p"}],
        "mp_news_article_id":"ART_ID",
        "msg_menus":[{"id":"101","content":"m1"}]
    }"#;
    let msg = WxMpKefuMessage::from_json(json).unwrap();
    assert_eq!(msg.to_user.as_deref(), Some("OPENID"));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.get_msg_type(), "text");
    assert_eq!(msg.content.as_deref(), Some("hello"));
    assert_eq!(msg.media_id.as_deref(), Some("MEDIA_ID"));
    assert_eq!(msg.thumb_media_id.as_deref(), Some("THUMB_ID"));
    assert_eq!(msg.title.as_deref(), Some("TITLE"));
    assert_eq!(msg.description.as_deref(), Some("DESC"));
    assert_eq!(msg.music_url.as_deref(), Some("MUSIC_URL"));
    assert_eq!(msg.hq_music_url.as_deref(), Some("HQ_MUSIC_URL"));
    assert_eq!(msg.kf_account.as_deref(), Some("kf@test"));
    assert_eq!(msg.card_id.as_deref(), Some("CARD_ID"));
    assert_eq!(msg.mp_news_media_id.as_deref(), Some("MP_ID"));
    assert_eq!(msg.mini_program_app_id.as_deref(), Some("wx_app"));
    assert_eq!(msg.mini_program_page_path.as_deref(), Some("/pages/i"));
    assert_eq!(msg.head_content.as_deref(), Some("HEAD"));
    assert_eq!(msg.tail_content.as_deref(), Some("TAIL"));
    assert_eq!(msg.mp_news_article_id.as_deref(), Some("ART_ID"));
    assert_eq!(msg.articles, vec![WxArticle::new("t", "d", "u", "p")]);
    assert_eq!(msg.msg_menus, vec![MsgMenu::new("101", "m1")]);
}

/// 对应 Java: Gson 平铺语义（嵌套 text.content 不落入顶层 content）。
#[test]
fn kefu_from_json_nested_object_not_flattened() {
    let msg = WxMpKefuMessage::from_json(
        r#"{"touser":"OPENID","msgtype":"text","text":{"content":"inner"}}"#,
    )
    .unwrap();
    assert_eq!(msg.to_user.as_deref(), Some("OPENID"));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    // 嵌套对象不参与顶层字段映射
    assert_eq!(msg.content, None);

    // serde 默认容器：articles/msg_menus 缺省为空
    let bare = WxMpKefuMessage::from_json(r#"{"msgtype":"image"}"#).unwrap();
    assert!(bare.articles.is_empty());
    assert!(bare.msg_menus.is_empty());
}

/// 对应 Java: Gson fromJson 错误语义（非法 JSON → Err）。
#[test]
fn kefu_from_json_error() {
    assert!(WxMpKefuMessage::from_json("not json").is_err());
    assert!(WxMpKefuMessage::from_json("{").is_err());
}

/// roundtrip：顶层平铺字段可往返；嵌套消息体不回填顶层字段（Gson 平铺语义）。
#[test]
fn kefu_roundtrip() {
    let reply = WxMpKefuMessage::mpnewsarticle()
        .to_user("OPENID")
        .article_id("ARTICLE_ID")
        .build();
    let wire = reply.to_json().unwrap();
    let parsed = WxMpKefuMessage::from_json(&wire).unwrap();
    assert_eq!(parsed.to_user, reply.to_user);
    assert_eq!(parsed.msg_type, reply.msg_type);
    // 嵌套 mpnewsarticle.article_id 不落入顶层 mp_news_article_id
    assert_eq!(parsed.mp_news_article_id, None);
    // 再序列化得到空子对象形态
    assert_eq!(
        parsed.to_json().unwrap(),
        "{\"touser\":\"OPENID\",\"msgtype\":\"mpnewsarticle\",\"mpnewsarticle\":{}}"
    );
}
