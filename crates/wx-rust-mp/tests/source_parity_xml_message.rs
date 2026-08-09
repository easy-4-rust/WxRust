#![allow(clippy::field_reassign_with_default)]
//! 镜像 Java `WxMpXmlMessageTest` / `WxMpXmlOut*MessageTest` / `WxMpJsAPITest`。
//!
//! Java 源：
//! - `bean/message/WxMpXmlMessageTest.java`（fromXml 全字段 + 群发 + 订阅事件）
//! - `bean/message/WxMpXmlOutTextMessageTest.java`（golden XML）
//! - `api/WxMpJsAPITest.java`（jsapi 签名黄金向量）

use wx_rust_common::util::crypto::Sha1;
use wx_rust_mp::bean::message::{
    WxMpXmlMessage, WxMpXmlOutImageMessage, WxMpXmlOutMessage, WxMpXmlOutMusic,
    WxMpXmlOutMusicMessage, WxMpXmlOutNewsMessage, WxMpXmlOutNewsMessageItem,
    WxMpXmlOutTextMessage, WxMpXmlOutTransferBizAiIvrMessage, WxMpXmlOutTransferKefuMessage,
    WxMpXmlOutVideo, WxMpXmlOutVideoMessage, WxMpXmlOutVoiceMessage,
};

// ---- 镜像 testFromXml：完整字段断言 ----

#[test]
fn xml_message_from_xml_full_fields() {
    // 与 Java testFromXml 同一 XML 夹具（逐字节一致）
    let xml = "<xml>".to_string()
        + "<ToUserName><![CDATA[toUser]]></ToUserName>"
        + "<FromUserName><![CDATA[fromUser]]></FromUserName> "
        + "<CreateTime>1348831860</CreateTime>"
        + "<MsgType><![CDATA[text]]></MsgType>"
        + "<Content><![CDATA[this is a test]]></Content>"
        + "<MsgId>1234567890123456</MsgId>"
        + "<PicUrl><![CDATA[this is a url]]></PicUrl>"
        + "<MediaId><![CDATA[media_id]]></MediaId>"
        + "<Format><![CDATA[Format]]></Format>"
        + "<ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>"
        + "<Location_X>23.134521</Location_X>"
        + "<Location_Y>113.358803</Location_Y>"
        + "<Scale>20</Scale>"
        + "<Label><![CDATA[位置信息]]></Label>"
        + "<Description><![CDATA[公众平台官网链接]]></Description>"
        + "<Url><![CDATA[url]]></Url>"
        + "<Title><![CDATA[公众平台官网链接]]></Title>"
        + "<Event><![CDATA[subscribe]]></Event>"
        + "<EventKey><![CDATA[qrscene_123123]]></EventKey>"
        + "<Ticket><![CDATA[TICKET]]></Ticket>"
        + "<Latitude>23.137466</Latitude>"
        + "<Longitude>113.352425</Longitude>"
        + "<Precision>119.385040</Precision>"
        + "<ScanCodeInfo>"
        + " <ScanType><![CDATA[qrcode]]></ScanType>"
        + " <ScanResult><![CDATA[1]]></ScanResult>"
        + "</ScanCodeInfo>"
        + "<SendPicsInfo>"
        + " <Count>1</Count>"
        + " <PicList>"
        + "  <item>"
        + "   <PicMd5Sum><![CDATA[1b5f7c23b5bf75682a53e7b6d163e185]]></PicMd5Sum>"
        + "  </item>"
        + " </PicList>"
        + "</SendPicsInfo>"
        + "<SendLocationInfo>"
        + "  <Location_X><![CDATA[23]]></Location_X>"
        + "  <Location_Y><![CDATA[113]]></Location_Y>"
        + "  <Scale><![CDATA[15]]></Scale>"
        + "  <Label><![CDATA[ 广州市海珠区客村艺苑路 106号]]></Label>"
        + "  <Poiname><![CDATA[wo de poi]]></Poiname>"
        + "</SendLocationInfo>"
        + "<KeyStandard><![CDATA[ean13]]></KeyStandard>"
        + "<KeyStr><![CDATA[6901481811083]]></KeyStr>"
        + "<Country><![CDATA[中国]]></Country>"
        + "<Province><![CDATA[广东]]></Province>"
        + "<City><![CDATA[揭阳]]></City>"
        + "<Sex>1</Sex>"
        + "<Scene>2</Scene>"
        + "<ExtInfo><![CDATA[123]]></ExtInfo>"
        + "<RegionCode><![CDATA[440105]]></RegionCode>"
        + "<ReasonMsg><![CDATA[]]></ReasonMsg>"
        + "</xml>";

    let m = WxMpXmlMessage::from_xml(&xml).expect("解析成功");
    assert_eq!(m.to_user.as_deref(), Some("toUser"));
    assert_eq!(m.from_user.as_deref(), Some("fromUser"));
    assert_eq!(m.create_time, Some(1348831860));
    assert_eq!(m.msg_type.as_deref(), Some("text"));
    assert_eq!(m.content.as_deref(), Some("this is a test"));
    assert_eq!(m.msg_id, Some(1234567890123456));
    assert_eq!(m.pic_url.as_deref(), Some("this is a url"));
    assert_eq!(m.media_id.as_deref(), Some("media_id"));
    assert_eq!(m.format.as_deref(), Some("Format"));
    assert_eq!(m.thumb_media_id.as_deref(), Some("thumb_media_id"));
    assert_eq!(m.location_x, Some(23.134521));
    assert_eq!(m.location_y, Some(113.358803));
    assert_eq!(m.scale, Some(20.0));
    assert_eq!(m.label.as_deref(), Some("位置信息"));
    assert_eq!(m.description.as_deref(), Some("公众平台官网链接"));
    assert_eq!(m.url.as_deref(), Some("url"));
    assert_eq!(m.title.as_deref(), Some("公众平台官网链接"));
    assert_eq!(m.event.as_deref(), Some("subscribe"));
    assert_eq!(m.event_key.as_deref(), Some("qrscene_123123"));
    assert_eq!(m.ticket.as_deref(), Some("TICKET"));
    assert_eq!(m.latitude, Some(23.137466));
    assert_eq!(m.longitude, Some(113.352425));
    assert_eq!(m.precision, Some(119.385040));
    let scan = m.scan_code_info.as_ref().expect("ScanCodeInfo 存在");
    assert_eq!(scan.scan_type.as_deref(), Some("qrcode"));
    assert_eq!(scan.scan_result.as_deref(), Some("1"));
    let pics = m.send_pics_info.as_ref().expect("SendPicsInfo 存在");
    assert_eq!(pics.count, Some(1));
    assert_eq!(pics.pic_list.len(), 1);
    assert_eq!(
        pics.pic_list[0].pic_md5_sum.as_deref(),
        Some("1b5f7c23b5bf75682a53e7b6d163e185")
    );
    let loc = m
        .send_location_info
        .as_ref()
        .expect("SendLocationInfo 存在");
    assert_eq!(loc.location_x.as_deref(), Some("23"));
    assert_eq!(loc.location_y.as_deref(), Some("113"));
    assert_eq!(loc.scale.as_deref(), Some("15"));
    assert_eq!(loc.label.as_deref(), Some(" 广州市海珠区客村艺苑路 106号"));
    assert_eq!(loc.poi_name.as_deref(), Some("wo de poi"));
    assert_eq!(m.key_standard.as_deref(), Some("ean13"));
    assert_eq!(m.key_str.as_deref(), Some("6901481811083"));
    assert_eq!(m.country.as_deref(), Some("中国"));
    assert_eq!(m.province.as_deref(), Some("广东"));
    assert_eq!(m.city.as_deref(), Some("揭阳"));
    assert_eq!(m.sex.as_deref(), Some("1"));
    assert_eq!(m.scene.as_deref(), Some("2"));
    assert_eq!(m.ext_info.as_deref(), Some("123"));
    assert_eq!(m.region_code.as_deref(), Some("440105"));
    assert_eq!(m.reason_msg.as_deref(), Some(""));
}

// ---- 镜像 testFromXml2：MsgID 变体（群发消息 ID 字段名） ----

#[test]
fn xml_message_from_xml_msgid_variant() {
    // Java testFromXml2：群发结果中 MsgID 映射到 mass_msg_id
    let xml = "<xml><ToUserName><![CDATA[gh_4d00ed8d6399]]></ToUserName>".to_string()
        + "<FromUserName><![CDATA[oV5CrjpxgaGXNHIQigzNlgLTnwic]]></FromUserName>"
        + "<CreateTime>1481013459</CreateTime>"
        + "<MsgType><![CDATA[event]]></MsgType>"
        + "<Event><![CDATA[MASSSENDJOBFINISH]]></Event>"
        + "<MsgID>1000001625</MsgID>"
        + "<Status><![CDATA[err(30003)]]></Status>"
        + "<TotalCount>0</TotalCount>"
        + "<FilterCount>0</FilterCount>"
        + "<SentCount>0</SentCount>"
        + "<ErrorCount>0</ErrorCount>"
        + "<CopyrightCheckResult>"
        + "<Count>2</Count>"
        + "<ResultList>"
        + "<item><ArticleIdx>1</ArticleIdx><UserDeclareState>0</UserDeclareState>"
        + "<AuditState>2</AuditState><OriginalArticleUrl><![CDATA[Url_1]]></OriginalArticleUrl>"
        + "<OriginalArticleType>1</OriginalArticleType><CanReprint>1</CanReprint>"
        + "<NeedReplaceContent>1</NeedReplaceContent><NeedShowReprintSource>1</NeedShowReprintSource></item>"
        + "<item><ArticleIdx>2</ArticleIdx><UserDeclareState>0</UserDeclareState>"
        + "<AuditState>2</AuditState><OriginalArticleUrl><![CDATA[Url_2]]></OriginalArticleUrl>"
        + "<OriginalArticleType>1</OriginalArticleType><CanReprint>1</CanReprint>"
        + "<NeedReplaceContent>1</NeedReplaceContent><NeedShowReprintSource>1</NeedShowReprintSource></item>"
        + "</ResultList>"
        + "<CheckState>2</CheckState>"
        + "</CopyrightCheckResult>"
        + "</xml>";

    let m = WxMpXmlMessage::from_xml(&xml).expect("解析成功");
    assert_eq!(m.to_user.as_deref(), Some("gh_4d00ed8d6399"));
    assert_eq!(m.from_user.as_deref(), Some("oV5CrjpxgaGXNHIQigzNlgLTnwic"));
    assert_eq!(m.create_time, Some(1481013459));
    assert_eq!(m.msg_type.as_deref(), Some("event"));
    assert_eq!(m.event.as_deref(), Some("MASSSENDJOBFINISH"));
    assert_eq!(m.mass_msg_id, Some(1000001625));
    assert_eq!(m.status.as_deref(), Some("err(30003)"));
    assert_eq!(m.total_count, Some(0));
    assert_eq!(m.filter_count, Some(0));
    assert_eq!(m.sent_count, Some(0));
    assert_eq!(m.error_count, Some(0));

    // allFieldsMap 嵌套结构（对应 Java 断言 CopyrightCheckResult.Count/CheckState/ResultList）
    let all = m.all_fields_map.as_ref().expect("allFieldsMap 存在");
    let copyright = all
        .get("CopyrightCheckResult")
        .expect("CopyrightCheckResult 存在")
        .as_node()
        .expect("节点");
    assert_eq!(
        copyright.get("Count").and_then(|v| v.as_scalar()),
        Some("2")
    );
    assert_eq!(
        copyright.get("CheckState").and_then(|v| v.as_scalar()),
        Some("2")
    );
    let result_list = copyright
        .get("ResultList")
        .and_then(|v| v.as_node())
        .expect("ResultList 节点");
    let items = result_list
        .get("item")
        .and_then(|v| v.as_array())
        .expect("item 数组");
    assert_eq!(items.len(), 2);
    let item0 = items[0].as_node().expect("item 节点");
    assert_eq!(
        item0.get("ArticleIdx").and_then(|v| v.as_scalar()),
        Some("1")
    );
    assert_eq!(
        item0.get("OriginalArticleUrl").and_then(|v| v.as_scalar()),
        Some("Url_1")
    );
    let item1 = items[1].as_node().expect("item 节点");
    assert_eq!(
        item1.get("ArticleIdx").and_then(|v| v.as_scalar()),
        Some("2")
    );
    assert_eq!(
        item1.get("OriginalArticleUrl").and_then(|v| v.as_scalar()),
        Some("Url_2")
    );
}

// ---- 镜像 testSubMsgPopupFromXml / Change / Sent ----

#[test]
fn xml_message_subscribe_popup_event() {
    let xml = "<xml>".to_string()
        + "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>"
        + "<FromUserName><![CDATA[otFpruAK8D-E6EfStSYonYSBZ8_4]]></FromUserName>"
        + "<CreateTime>1610969440</CreateTime>"
        + "<MsgType><![CDATA[event]]></MsgType>"
        + "<Event><![CDATA[subscribe_msg_popup_event]]></Event>"
        + "<SubscribeMsgPopupEvent>"
        + "<List><TemplateId><![CDATA[VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc]]></TemplateId>"
        + "<SubscribeStatusString><![CDATA[accept]]></SubscribeStatusString>"
        + "<PopupScene>2</PopupScene></List>"
        + "<List><TemplateId><![CDATA[9nLIlbOQZC5Y89AZteFEux3WCXRRRG5Wfzkpssu4bLI]]></TemplateId>"
        + "<SubscribeStatusString><![CDATA[reject]]></SubscribeStatusString>"
        + "<PopupScene>2</PopupScene></List>"
        + "</SubscribeMsgPopupEvent>"
        + "</xml>";

    let m = WxMpXmlMessage::from_xml(&xml).expect("解析成功");
    let popup = m.subscribe_msg_popup_event.expect("弹窗事件存在");
    assert_eq!(popup.list.len(), 2);
    assert_eq!(
        popup.list[0].template_id.as_deref(),
        Some("VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc")
    );
    assert_eq!(
        popup.list[0].subscribe_status_string.as_deref(),
        Some("accept")
    );
    assert_eq!(popup.list[0].popup_scene.as_deref(), Some("2"));
    assert_eq!(
        popup.list[1].template_id.as_deref(),
        Some("9nLIlbOQZC5Y89AZteFEux3WCXRRRG5Wfzkpssu4bLI")
    );
    assert_eq!(
        popup.list[1].subscribe_status_string.as_deref(),
        Some("reject")
    );
    assert_eq!(popup.list[1].popup_scene.as_deref(), Some("2"));
}

#[test]
fn xml_message_subscribe_change_event() {
    let xml = "<xml>".to_string()
        + "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>"
        + "<FromUserName><![CDATA[otFpruAK8D-E6EfStSYonYSBZ8_4]]></FromUserName>"
        + "<CreateTime>1610969440</CreateTime>"
        + "<MsgType><![CDATA[event]]></MsgType>"
        + "<Event><![CDATA[subscribe_msg_change_event]]></Event>"
        + "<SubscribeMsgChangeEvent>"
        + "<List><TemplateId><![CDATA[VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc]]></TemplateId>"
        + "<SubscribeStatusString><![CDATA[reject]]></SubscribeStatusString></List>"
        + "</SubscribeMsgChangeEvent>"
        + "</xml>";

    let m = WxMpXmlMessage::from_xml(&xml).expect("解析成功");
    let change = m.subscribe_msg_change_event.expect("变更事件存在");
    assert_eq!(change.list.len(), 1);
    assert_eq!(
        change.list[0].template_id.as_deref(),
        Some("VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc")
    );
    assert_eq!(
        change.list[0].subscribe_status_string.as_deref(),
        Some("reject")
    );
}

#[test]
fn xml_message_subscribe_sent_event() {
    let xml = "<xml>".to_string()
        + "<ToUserName><![CDATA[gh_123456789abc]]></ToUserName>"
        + "<FromUserName><![CDATA[otFpruAK8D-E6EfStSYonYSBZ8_4]]></FromUserName>"
        + "<CreateTime>1610969440</CreateTime>"
        + "<MsgType><![CDATA[event]]></MsgType>"
        + "<Event><![CDATA[subscribe_msg_sent_event]]></Event>"
        + "<SubscribeMsgSentEvent>"
        + "<List><TemplateId><![CDATA[VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc]]></TemplateId>"
        + "<MsgID>1700827132819554304</MsgID>"
        + "<ErrorCode>0</ErrorCode>"
        + "<ErrorStatus><![CDATA[success]]></ErrorStatus></List>"
        + "</SubscribeMsgSentEvent>"
        + "</xml>";

    let m = WxMpXmlMessage::from_xml(&xml).expect("解析成功");
    let sent = m.subscribe_msg_sent_event.expect("发送事件存在");
    assert_eq!(sent.list.len(), 1);
    assert_eq!(
        sent.list[0].template_id.as_deref(),
        Some("VRR0UEO9VJOLs0MHlU0OilqX6MVFDwH3_3gz3Oc0NIc")
    );
    assert_eq!(sent.list[0].msg_id.as_deref(), Some("1700827132819554304"));
    assert_eq!(sent.list[0].error_code.as_deref(), Some("0"));
    assert_eq!(sent.list[0].error_status.as_deref(), Some("success"));
}

// ---- 镜像 WxMpXmlOutTextMessageTest：golden XML ----

#[test]
fn out_text_message_to_xml_golden() {
    // Java test()：固定字段 + 断言 toXml（忽略空白）
    let mut m = WxMpXmlOutTextMessage::new();
    m.content = Some("content".to_string());
    m.base.create_time = Some(1122);
    m.base.from_user_name = Some("from".to_string());
    m.base.to_user_name = Some("to".to_string());

    let expected = "<xml>".to_string()
        + "<ToUserName><![CDATA[to]]></ToUserName>"
        + "<FromUserName><![CDATA[from]]></FromUserName>"
        + "<CreateTime>1122</CreateTime>"
        + "<MsgType><![CDATA[text]]></MsgType>"
        + "<Content><![CDATA[content]]></Content>"
        + "</xml>";

    let actual: String = m.to_xml().chars().filter(|c| !c.is_whitespace()).collect();
    let expected: String = expected.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(actual, expected);
}

#[test]
fn out_text_message_build_chain() {
    // Java testBuild()：WxMpXmlOutMessage.TEXT().content().fromUser().toUser().build()
    let m = WxMpXmlOutMessage::text()
        .content("content")
        .from_user("from")
        .to_user("to")
        .build();
    assert_eq!(m.content.as_deref(), Some("content"));
    assert_eq!(m.base.from_user_name.as_deref(), Some("from"));
    assert_eq!(m.base.to_user_name.as_deref(), Some("to"));
    assert_eq!(m.base.msg_type.as_deref(), Some("text"));
    assert!(m.base.create_time.is_some(), "build 应设置创建时间");
}

// ---- 其余 out 消息 golden ----

#[test]
fn out_image_message_to_xml() {
    let mut m = WxMpXmlOutImageMessage::new();
    m.media_id = Some("media_id".to_string());
    m.base.to_user_name = Some("to".to_string());
    m.base.from_user_name = Some("from".to_string());
    m.base.create_time = Some(1122);
    let xml: String = m.to_xml().chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        xml.contains("<MsgType><![CDATA[image]]></MsgType>"),
        "实际: {xml}"
    );
    assert!(
        xml.contains("<Image><MediaId><![CDATA[media_id]]></MediaId></Image>"),
        "实际: {xml}"
    );
}

#[test]
fn out_voice_message_to_xml() {
    let mut m = WxMpXmlOutVoiceMessage::new();
    m.media_id = Some("media_id".to_string());
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[voice]]></MsgType>"));
    assert!(xml.contains("<Voice><MediaId><![CDATA[media_id]]></MediaId></Voice>"));
}

#[test]
fn out_video_message_to_xml() {
    let mut m = WxMpXmlOutVideoMessage::new();
    m.video = Some(WxMpXmlOutVideo {
        media_id: Some("m1".to_string()),
        title: Some("标题".to_string()),
        description: Some("描述".to_string()),
    });
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[video]]></MsgType>"));
    assert!(xml.contains("<Video>"));
    assert!(xml.contains("<MediaId><![CDATA[m1]]></MediaId>"));
    assert!(xml.contains("<Title><![CDATA[标题]]></Title>"));
    assert!(xml.contains("<Description><![CDATA[描述]]></Description>"));
    assert!(xml.contains("</Video>"));
}

#[test]
fn out_music_message_to_xml() {
    let mut m = WxMpXmlOutMusicMessage::new();
    m.music = Some(WxMpXmlOutMusic {
        title: Some("t".to_string()),
        description: Some("d".to_string()),
        thumb_media_id: Some("thumb".to_string()),
        music_url: Some("http://music".to_string()),
        hq_music_url: Some("http://hq".to_string()),
    });
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[music]]></MsgType>"));
    assert!(xml.contains("<Music>"));
    assert!(xml.contains("<Title><![CDATA[t]]></Title>"));
    assert!(xml.contains("<HQMusicUrl><![CDATA[http://hq]]></HQMusicUrl>"));
    assert!(xml.contains("</Music>"));
}

#[test]
fn out_news_message_to_xml() {
    let mut m = WxMpXmlOutNewsMessage::new();
    m.add_article(WxMpXmlOutNewsMessageItem {
        title: Some("t1".to_string()),
        description: Some("d1".to_string()),
        pic_url: Some("http://pic".to_string()),
        url: Some("http://url".to_string()),
    });
    m.add_article(WxMpXmlOutNewsMessageItem {
        title: Some("t2".to_string()),
        ..Default::default()
    });
    let xml: String = m.to_xml().chars().filter(|c| !c.is_whitespace()).collect();
    // XStream 声明序：Articles 在前，ArticleCount 在后（与 Java 测试二一致）
    assert!(xml.contains("<Articles><item>"), "实际: {xml}");
    assert!(
        xml.contains("</Articles><ArticleCount>2</ArticleCount></xml>"),
        "实际: {xml}"
    );
    assert!(xml.contains("<Title><![CDATA[t1]]></Title>"));
    assert!(xml.contains("<Url><![CDATA[http://url]]></Url>"));
}

#[test]
fn out_transfer_kefu_message_to_xml() {
    let mut m = WxMpXmlOutTransferKefuMessage::new();
    m.kf_account = Some("kf@test".to_string());
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[transfer_customer_service]]></MsgType>"));
    assert!(xml.contains("<TransInfo><KfAccount><![CDATA[kf@test]]></KfAccount></TransInfo>"));
}

#[test]
fn out_transfer_biz_ai_ivr_message_to_xml() {
    let m = WxMpXmlOutTransferBizAiIvrMessage::new();
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[transfer_biz_ai_ivr]]></MsgType>"));
}

#[test]
fn out_message_null_fields_omitted() {
    // XStream 语义：null 字段不输出
    let m = WxMpXmlOutTextMessage::new();
    let xml = m.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[text]]></MsgType>"));
    assert!(
        !xml.contains("Content"),
        "null Content 不应输出，实际: {xml}"
    );
    assert!(
        !xml.contains("ToUserName"),
        "null ToUserName 不应输出，实际: {xml}"
    );
    assert!(
        !xml.contains("CreateTime"),
        "null CreateTime 不应输出，实际: {xml}"
    );
}

// ---- 镜像 WxMpJsAPITest：jsapi 签名黄金向量 ----

#[test]
fn jsapi_signature_golden_vector() {
    // Java WxMpJsAPITest：固定输入 → 固定签名 c6f04b64d6351d197b71bd23fb7dd2d44c0db486
    let timestamp = 1419835025_i64;
    let url = "http://omstest.vmall.com:23568/thirdparty/wechat/vcode/gotoshare?quantity=1&batchName=MATE7";
    let noncestr = "82693e11-b9bc-448e-892f-f5289f46cd0f";
    let jsapi_ticket =
        "bxLdikRXVbTPdHSM05e5u4RbEYQn7pNQMPrfzl8lJNb1foLDa3HIwI3BRMkQmSO_5F64VFa75uURcq6Uz7QHgA";
    let result = Sha1::digest_with_amp(&[
        &format!("jsapi_ticket={jsapi_ticket}"),
        &format!("noncestr={noncestr}"),
        &format!("timestamp={timestamp}"),
        &format!("url={url}"),
    ])
    .expect("签名成功");
    assert_eq!(result, "c6f04b64d6351d197b71bd23fb7dd2d44c0db486");
}
