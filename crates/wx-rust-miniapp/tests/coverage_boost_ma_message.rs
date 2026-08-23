//! 小程序覆盖率提升：推送消息 `WxMaMessage`（xml/json 双格式全字段）。
//!
//! 对应 Java `WxMaMessage` + `WxMaSubscribeMsgEvent`（XStream/Gson 双线格式）。
//! 覆盖：XML 树解析（嵌套/重复元素/CDATA/空元素/混合文本）、全字段
//! `from_xml`/`to_xml` 往返、JSON `from_json`/`to_json` 往返（`List`
//! 对象/数组歧义按 event 归集）、加密 xml/json 解密重解析、getter、
//! `XmlValue` 树值语义与错误路径。全程离线（加密为本地 AES 往返）。

use std::collections::HashMap;

use wx_rust_miniapp::bean::{DetailBean, ResultBean, WxMaXPayTeamInfo};
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::message::{
    ChangeEvent, PopupEvent, SentEvent, SubscribeMsgChangeEvent, SubscribeMsgPopupEvent,
    SubscribeMsgSentEvent, WxMaMessage, XmlValue,
};
use wx_rust_miniapp::util::crypto::WxMaCryptUtils;

/// 构建带 token/aesKey 的本地配置（加密往返用）。
fn crypto_config() -> WxMaDefaultConfig {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_token("tokentoken");
    config.set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    config
}

/// 全字段 XML 夹具（覆盖 `from_xml` 的全部字段提取分支）。
fn full_xml() -> String {
    "<xml>\
     <Encrypt><![CDATA[enc-1]]></Encrypt>\
     <ToUserName><![CDATA[gh_123]]></ToUserName>\
     <FromUserName><![CDATA[oABC]]></FromUserName>\
     <CreateTime>1700000000</CreateTime>\
     <MsgType><![CDATA[text]]></MsgType>\
     <MsgDataFormat><![CDATA[json]]></MsgDataFormat>\
     <Content><![CDATA[你好]]></Content>\
     <MsgId>8234567890123456</MsgId>\
     <PicUrl><![CDATA[https://pic/example.png]]></PicUrl>\
     <MediaId><![CDATA[MEDIA_1]]></MediaId>\
     <Event><![CDATA[user_enter_tempsession]]></Event>\
     <Title><![CDATA[标题]]></Title>\
     <AppId><![CDATA[wxappid]]></AppId>\
     <PagePath><![CDATA[pages/index/index]]></PagePath>\
     <ThumbUrl><![CDATA[https://thumb/1.jpg]]></ThumbUrl>\
     <ThumbMediaId><![CDATA[THUMB_1]]></ThumbMediaId>\
     <SessionFrom><![CDATA[proxy]]></SessionFrom>\
     <isrisky><![CDATA[0]]></isrisky>\
     <extra_info_json><![CDATA[{}]]></extra_info_json>\
     <appid><![CDATA[wxappid]]></appid>\
     <trace_id><![CDATA[trace-1]]></trace_id>\
     <status_code><![CDATA[200]]></status_code>\
     <version>2</version>\
     <result><suggest><![CDATA[pass]]></suggest><label><![CDATA[100]]></label></result>\
     <detail>\
     <strategy><![CDATA[content_model]]></strategy>\
     <errcode>0</errcode><suggest><![CDATA[pass]]></suggest>\
     <label><![CDATA[100]]></label><prob>90</prob>\
     </detail>\
     <Scene>1037</Scene>\
     <Query><![CDATA[a=1]]></Query>\
     <AppID><![CDATA[wxappid-upper]]></AppID>\
     <RevokeInfo><![CDATA[revoke-1]]></RevokeInfo>\
     <OpenID><![CDATA[openid-1]]></OpenID>\
     <PluginID><![CDATA[plugin-1]]></PluginID>\
     <OpenPID><![CDATA[openpid-1]]></OpenPID>\
     <ret><![CDATA[0]]></ret><first><![CDATA[1]]></first><second><![CDATA[2]]></second>\
     <reason><![CDATA[驳回]]></reason><Reason><![CDATA[审核驳回]]></Reason>\
     <nickname><![CDATA[小程序昵称]]></nickname>\
     <transaction_id><![CDATA[TXN_1]]></transaction_id>\
     <merchant_id><![CDATA[MCH_1]]></merchant_id>\
     <sub_merchant_id><![CDATA[SUB_MCH_1]]></sub_merchant_id>\
     <merchant_trade_no><![CDATA[NO_1]]></merchant_trade_no>\
     <pay_time>1700000001</pay_time>\
     <msg><![CDATA[发货消息]]></msg>\
     <shipped_time>1700000002</shipped_time>\
     <estimated_settlement_time>1700000003</estimated_settlement_time>\
     <confirm_receive_method>1</confirm_receive_method>\
     <confirm_receive_time>1700000004</confirm_receive_time>\
     <settlement_time>1700000005</settlement_time>\
     <WxRefundId><![CDATA[WR_1]]></WxRefundId>\
     <MchRefundId><![CDATA[MR_1]]></MchRefundId>\
     <WxOrderId><![CDATA[WO_1]]></WxOrderId>\
     <MchOrderId><![CDATA[MO_1]]></MchOrderId>\
     <RefundFee>100</RefundFee><RetCode>0</RetCode>\
     <RetMsg><![CDATA[退款成功]]></RetMsg>\
     <RefundStartTimestamp>1700000006</RefundStartTimestamp>\
     <RefundSuccTimestamp>1700000007</RefundSuccTimestamp>\
     <WxpayRefundTransactionId><![CDATA[WRT_1]]></WxpayRefundTransactionId>\
     <RetryTimes>1</RetryTimes>\
     <TeamInfo>\
     <ActivityId><![CDATA[ACT_1]]></ActivityId><TeamId><![CDATA[TEAM_1]]></TeamId>\
     <TeamType>1</TeamType><TeamAction>2</TeamAction>\
     </TeamInfo>\
     <TransactionId><![CDATA[COMPLAIN_TXN_1]]></TransactionId>\
     <ComplaintId><![CDATA[COMPLAINT_1]]></ComplaintId>\
     <ComplaintDetail><![CDATA[投诉详情]]></ComplaintDetail>\
     <ComplaintTime>1700000008</ComplaintTime>\
     <RequestId><![CDATA[REQ_1]]></RequestId>\
     <refund_time><![CDATA[2024-01-01]]></refund_time>\
     <order_time><![CDATA[2024-01-02]]></order_time>\
     <channel_bill><![CDATA[BILL_1]]></channel_bill>\
     <bundleid><![CDATA[com.example.app]]></bundleid>\
     <product_id><![CDATA[PROD_1]]></product_id>\
     <p_count><![CDATA[10]]></p_count>\
     <refund_request_reason><![CDATA[误购买]]></refund_request_reason>\
     <provide_status><![CDATA[1]]></provide_status>\
     </xml>"
        .to_string()
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: from_xml 全字段提取（镜像 Java WxMaMessage XStream 反序列化）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMessage.fromXml —— 全字段（字符串/整型/长整型/嵌套节点）。
#[test]
fn from_xml_extracts_all_fields() {
    let message = WxMaMessage::from_xml(&full_xml()).expect("全字段 XML 解析成功");
    assert_eq!(message.encrypt.as_deref(), Some("enc-1"));
    assert_eq!(message.to_user.as_deref(), Some("gh_123"));
    assert_eq!(message.from_user.as_deref(), Some("oABC"));
    assert_eq!(message.create_time, Some(1700000000));
    assert_eq!(message.msg_type.as_deref(), Some("text"));
    assert_eq!(message.msg_data_format.as_deref(), Some("json"));
    assert_eq!(message.content.as_deref(), Some("你好"));
    assert_eq!(message.msg_id, Some(8234567890123456));
    assert_eq!(message.pic_url.as_deref(), Some("https://pic/example.png"));
    assert_eq!(message.media_id.as_deref(), Some("MEDIA_1"));
    assert_eq!(message.event.as_deref(), Some("user_enter_tempsession"));
    assert_eq!(message.title.as_deref(), Some("标题"));
    assert_eq!(message.app_id.as_deref(), Some("wxappid"));
    assert_eq!(message.page_path.as_deref(), Some("pages/index/index"));
    assert_eq!(message.thumb_url.as_deref(), Some("https://thumb/1.jpg"));
    assert_eq!(message.thumb_media_id.as_deref(), Some("THUMB_1"));
    assert_eq!(message.session_from.as_deref(), Some("proxy"));
    assert_eq!(message.is_risky.as_deref(), Some("0"));
    assert_eq!(message.extra_info_json.as_deref(), Some("{}"));
    assert_eq!(message.appid.as_deref(), Some("wxappid"));
    assert_eq!(message.trace_id.as_deref(), Some("trace-1"));
    assert_eq!(message.status_code.as_deref(), Some("200"));
    assert_eq!(message.version, Some(2));
    assert_eq!(
        message.result,
        Some(ResultBean {
            suggest: "pass".to_string(),
            label: "100".to_string(),
        })
    );
    assert_eq!(
        message.detail,
        vec![DetailBean {
            strategy: "content_model".to_string(),
            errcode: 0,
            suggest: "pass".to_string(),
            label: "100".to_string(),
            prob: 90,
        }]
    );
    assert_eq!(message.scene, Some(1037));
    assert_eq!(message.query.as_deref(), Some("a=1"));
    assert_eq!(message.app_i_d.as_deref(), Some("wxappid-upper"));
    assert_eq!(message.revoke_info.as_deref(), Some("revoke-1"));
    assert_eq!(message.open_id.as_deref(), Some("openid-1"));
    assert_eq!(message.plugin_id.as_deref(), Some("plugin-1"));
    assert_eq!(message.open_pid.as_deref(), Some("openpid-1"));
    assert_eq!(message.ret.as_deref(), Some("0"));
    assert_eq!(message.first.as_deref(), Some("1"));
    assert_eq!(message.second.as_deref(), Some("2"));
    assert_eq!(message.reason.as_deref(), Some("驳回"));
    assert_eq!(message.we_app_reason.as_deref(), Some("审核驳回"));
    assert_eq!(message.nickname.as_deref(), Some("小程序昵称"));
    // context 仅在解密路径设置
    assert_eq!(message.context, None);
    assert_eq!(message.transaction_id.as_deref(), Some("TXN_1"));
    assert_eq!(message.merchant_id.as_deref(), Some("MCH_1"));
    assert_eq!(message.sub_merchant_id.as_deref(), Some("SUB_MCH_1"));
    assert_eq!(message.merchant_trade_no.as_deref(), Some("NO_1"));
    assert_eq!(message.pay_time, Some(1700000001));
    assert_eq!(message.msg.as_deref(), Some("发货消息"));
    assert_eq!(message.shipped_time, Some(1700000002));
    assert_eq!(message.estimated_settlement_time, Some(1700000003));
    assert_eq!(message.confirm_receive_method, Some(1));
    assert_eq!(message.confirm_receive_time, Some(1700000004));
    assert_eq!(message.settlement_time, Some(1700000005));
    assert_eq!(message.wx_refund_id.as_deref(), Some("WR_1"));
    assert_eq!(message.mch_refund_id.as_deref(), Some("MR_1"));
    assert_eq!(message.wx_order_id.as_deref(), Some("WO_1"));
    assert_eq!(message.mch_order_id.as_deref(), Some("MO_1"));
    assert_eq!(message.refund_fee, Some(100));
    assert_eq!(message.ret_code, Some(0));
    assert_eq!(message.ret_msg.as_deref(), Some("退款成功"));
    assert_eq!(message.refund_start_timestamp, Some(1700000006));
    assert_eq!(message.refund_succ_timestamp, Some(1700000007));
    assert_eq!(
        message.wxpay_refund_transaction_id.as_deref(),
        Some("WRT_1")
    );
    assert_eq!(message.retry_times, Some(1));
    assert_eq!(
        message.team_info,
        Some(WxMaXPayTeamInfo {
            activity_id: "ACT_1".to_string(),
            team_id: "TEAM_1".to_string(),
            team_type: 1,
            team_action: 2,
        })
    );
    assert_eq!(
        message.complaint_transaction_id.as_deref(),
        Some("COMPLAIN_TXN_1")
    );
    assert_eq!(message.complaint_id.as_deref(), Some("COMPLAINT_1"));
    assert_eq!(message.complaint_detail.as_deref(), Some("投诉详情"));
    assert_eq!(message.complaint_time, Some(1700000008));
    assert_eq!(message.request_id.as_deref(), Some("REQ_1"));
    assert_eq!(message.refund_time.as_deref(), Some("2024-01-01"));
    assert_eq!(message.order_time.as_deref(), Some("2024-01-02"));
    assert_eq!(message.channel_bill.as_deref(), Some("BILL_1"));
    assert_eq!(message.bundleid.as_deref(), Some("com.example.app"));
    assert_eq!(message.xpay_product_id.as_deref(), Some("PROD_1"));
    assert_eq!(message.p_count.as_deref(), Some("10"));
    assert_eq!(message.refund_request_reason.as_deref(), Some("误购买"));
    assert_eq!(message.provide_status.as_deref(), Some("1"));
    // 全量树保留（对应 Java allFieldsMap）
    let all = message.all_fields_map.as_ref().expect("全量 map 存在");
    assert_eq!(
        all.get("ToUserName").and_then(XmlValue::as_scalar),
        Some("gh_123")
    );
    assert!(all.contains_key("TeamInfo"));
}

/// 对应 Java: WxMaMessage.fromXml —— 订阅消息三类事件（List 重复元素合并数组）。
#[test]
fn from_xml_parses_subscribe_msg_events() {
    let xml = "<xml>\
               <ToUserName><![CDATA[gh_1]]></ToUserName>\
               <SubscribeMsgPopupEvent>\
               <List><TemplateId><![CDATA[T1]]></TemplateId>\
               <SubscribeStatusString><![CDATA[accept]]></SubscribeStatusString>\
               <PopupScene><![CDATA[0]]></PopupScene></List>\
               <List><TemplateId><![CDATA[T2]]></TemplateId>\
               <SubscribeStatusString><![CDATA[reject]]></SubscribeStatusString>\
               <PopupScene><![CDATA[1]]></PopupScene></List>\
               </SubscribeMsgPopupEvent>\
               <SubscribeMsgChangeEvent>\
               <List><TemplateId><![CDATA[T3]]></TemplateId>\
               <SubscribeStatusString><![CDATA[accept]]></SubscribeStatusString></List>\
               </SubscribeMsgChangeEvent>\
               <SubscribeMsgSentEvent>\
               <List><TemplateId><![CDATA[T4]]></TemplateId>\
               <MsgID><![CDATA[M_1]]></MsgID>\
               <ErrorCode><![CDATA[0]]></ErrorCode>\
               <ErrorStatus><![CDATA[success]]></ErrorStatus></List>\
               </SubscribeMsgSentEvent>\
               </xml>";
    let message = WxMaMessage::from_xml(xml).expect("订阅事件 XML 解析成功");
    let popup = message
        .subscribe_msg_popup_event
        .as_ref()
        .expect("弹窗事件存在");
    assert_eq!(
        popup.list,
        vec![
            PopupEvent {
                template_id: "T1".to_string(),
                subscribe_status_string: "accept".to_string(),
                popup_scene: "0".to_string(),
            },
            PopupEvent {
                template_id: "T2".to_string(),
                subscribe_status_string: "reject".to_string(),
                popup_scene: "1".to_string(),
            },
        ]
    );
    let change = message
        .subscribe_msg_change_event
        .as_ref()
        .expect("变更事件存在");
    assert_eq!(
        change.list,
        vec![ChangeEvent {
            template_id: "T3".to_string(),
            subscribe_status_string: "accept".to_string(),
        }]
    );
    let sent = message
        .subscribe_msg_sent_event
        .as_ref()
        .expect("发送事件存在");
    assert_eq!(
        sent.list,
        Some(SentEvent {
            template_id: "T4".to_string(),
            msg_id: "M_1".to_string(),
            error_code: "0".to_string(),
            error_status: "success".to_string(),
        })
    );
}

/// 对应 Java: XStream 语义细节 —— 空元素/混合文本/纯文本根/顶层标量。
#[test]
fn from_xml_handles_element_shapes() {
    // 空元素 → 空 Node（对应 Java dom4j 空节点）
    let message = WxMaMessage::from_xml("<xml><Content/></xml>").expect("空元素解析成功");
    // 空元素是 Node 非 Scalar，str_field 返回 None
    assert_eq!(message.content, None);

    // 混合文本与子元素：以元素为准（文本丢弃）
    let message = WxMaMessage::from_xml("<xml>mixed<Content><![CDATA[有效内容]]></Content></xml>")
        .expect("混合内容解析成功");
    assert_eq!(message.content.as_deref(), Some("有效内容"));

    // 纯文本根（无子元素）→ 根为标量 → 报错（Java dom4j 文档根必为节点）
    let err = WxMaMessage::from_xml("<xml>纯文本</xml>").expect_err("标量根应报错");
    assert!(err.contains("XML 根元素应为节点"), "实际错误: {err}");

    // 缺字段时数值解析返回 None（非法数字）
    let message = WxMaMessage::from_xml("<xml><CreateTime>abc</CreateTime></xml>")
        .expect("非法数字不阻塞解析");
    assert_eq!(message.create_time, None);
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: XML 解析错误路径
// ══════════════════════════════════════════════════════════════════════════════

/// 空文档缺少根元素 / 元素未闭合。
#[test]
fn from_xml_error_paths() {
    let err = WxMaMessage::from_xml("").expect_err("空字符串应报错");
    assert!(err.contains("缺少根元素"), "实际错误: {err}");

    let err = WxMaMessage::from_xml("<xml><Content>未闭合").expect_err("未闭合应报错");
    assert!(err.contains("元素未闭合"), "实际错误: {err}");

    let err = WxMaMessage::from_xml("plain text only").expect_err("无根元素文本应报错");
    assert!(err.contains("缺少根元素"), "实际错误: {err}");
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: to_xml 线格式（镜像 Java XStream 序列化）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMessage 的 XStream toXML —— CDATA/裸数值/null 省略。
#[test]
fn to_xml_round_trips_full_message() {
    let original = WxMaMessage::from_xml(&full_xml()).expect("解析成功");
    let xml = original.to_xml();
    assert!(xml.starts_with("<xml>"));
    assert!(xml.contains("<Encrypt><![CDATA[enc-1]]></Encrypt>"));
    assert!(xml.contains("<CreateTime>1700000000</CreateTime>"));
    assert!(xml.contains("<MsgId>8234567890123456</MsgId>"));
    assert!(xml.contains("<result><suggest><![CDATA[pass]]></suggest>"));
    assert!(xml.contains("<detail><strategy><![CDATA[content_model]]></strategy>"));
    assert!(xml.contains("<prob>90</prob></detail>"));
    assert!(xml.contains("<TeamInfo><ActivityId><![CDATA[ACT_1]]></ActivityId>"));
    assert!(xml.contains("<TeamAction>2</TeamAction></TeamInfo>"));
    assert!(xml.ends_with("</xml>"));

    // 往返：to_xml → from_xml 字段一致（context 由解密路径独占，不参与往返）
    let reparsed = WxMaMessage::from_xml(&xml).expect("再解析成功");
    assert_eq!(reparsed.to_user, original.to_user);
    assert_eq!(reparsed.content, original.content);
    assert_eq!(reparsed.result, original.result);
    assert_eq!(reparsed.detail, original.detail);
    assert_eq!(reparsed.team_info, original.team_info);
    assert_eq!(reparsed.pay_time, original.pay_time);
    assert_eq!(reparsed.retry_times, original.retry_times);
    assert_eq!(reparsed.xpay_product_id, original.xpay_product_id);
}

/// 对应 Java: XStream 序列化订阅事件 + 空消息只输出外壳。
#[test]
fn to_xml_outputs_subscribe_events_and_empty_shell() {
    let mut message = WxMaMessage::default();
    message.subscribe_msg_popup_event = Some(SubscribeMsgPopupEvent {
        list: vec![PopupEvent {
            template_id: "T1".to_string(),
            subscribe_status_string: "accept".to_string(),
            popup_scene: "0".to_string(),
        }],
    });
    message.subscribe_msg_change_event = Some(SubscribeMsgChangeEvent {
        list: vec![ChangeEvent {
            template_id: "T3".to_string(),
            subscribe_status_string: "accept".to_string(),
        }],
    });
    message.subscribe_msg_sent_event = Some(SubscribeMsgSentEvent {
        list: Some(SentEvent {
            template_id: "T4".to_string(),
            msg_id: "M_1".to_string(),
            error_code: "0".to_string(),
            error_status: "success".to_string(),
        }),
    });
    let xml = message.to_xml();
    assert!(xml.contains("<SubscribeMsgPopupEvent><List>"));
    assert!(xml.contains("<PopupScene><![CDATA[0]]></PopupScene>"));
    assert!(xml.contains("<SubscribeMsgChangeEvent><List>"));
    // 变更事件项无 PopupScene 字段
    let change_seg = xml
        .split("<SubscribeMsgChangeEvent><List>")
        .nth(1)
        .expect("变更事件段存在");
    assert!(!change_seg.contains("PopupScene"));
    assert!(xml.contains("<SubscribeMsgSentEvent><List>"));
    assert!(xml.contains("<ErrorStatus><![CDATA[success]]></ErrorStatus>"));

    // 空消息：null 字段全部省略，只剩外壳
    let empty = WxMaMessage::default().to_xml();
    assert_eq!(empty, "<xml></xml>");

    // sent 事件 list 为 None：容器输出但内部字段省略
    let mut none_sent = WxMaMessage::default();
    none_sent.subscribe_msg_sent_event = Some(SubscribeMsgSentEvent { list: None });
    assert_eq!(
        none_sent.to_xml(),
        "<xml><SubscribeMsgSentEvent><List></List></SubscribeMsgSentEvent></xml>"
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: from_json / to_json（镜像 Java Gson @SerializedName）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMessage.fromJson —— `List` 为数组时按 event 归集弹窗事件。
#[test]
fn from_json_routes_popup_event_list() {
    let json = r#"{
        "ToUserName": "gh_1",
        "Event": "subscribe_msg_popup_event",
        "List": [
            {"TemplateId": "T1", "SubscribeStatusString": "accept", "PopupScene": "0"},
            {"TemplateId": "T2", "SubscribeStatusString": "reject", "PopupScene": "1"}
        ]
    }"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    let popup = message
        .subscribe_msg_popup_event
        .as_ref()
        .expect("弹窗事件归集");
    assert_eq!(popup.list.len(), 2);
    assert_eq!(popup.list[0].template_id, "T1");
    assert_eq!(popup.list[1].popup_scene, "1");
    // 归集后原始 List 清空（对应 Java uselessMsg）
    assert_eq!(message.subscribe_msg_list, None);
    // 全量 map 保留
    assert!(message.all_fields_map.is_some());
}

/// 对应 Java: WxMaMessage.fromJson —— `List` 为对象时的三类事件归集。
#[test]
fn from_json_routes_change_and_sent_event_object_list() {
    // 变更事件：List 对象（无 PopupScene/MsgID → Change）
    let json = r#"{
        "Event": "subscribe_msg_change_event",
        "List": {"TemplateId": "T3", "SubscribeStatusString": "accept"}
    }"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    let change = message
        .subscribe_msg_change_event
        .as_ref()
        .expect("变更事件归集");
    assert_eq!(change.list.len(), 1);
    assert_eq!(change.list[0].template_id, "T3");

    // 发送事件：List 数组含 MsgID
    let json = r#"{
        "Event": "subscribe_msg_sent_event",
        "List": [{"TemplateId": "T4", "MsgID": "M_1", "ErrorCode": "0", "ErrorStatus": "success"}]
    }"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    let sent = message
        .subscribe_msg_sent_event
        .as_ref()
        .expect("发送事件归集");
    assert_eq!(sent.list.as_ref().expect("sent 项").msg_id, "M_1");

    // 其它 event 携带 List：不归集（`_ => {}` 分支）
    let json = r#"{"Event": "other_event", "List": {"TemplateId": "T9"}}"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    assert_eq!(message.subscribe_msg_popup_event, None);
    assert_eq!(message.subscribe_msg_change_event, None);
    assert_eq!(message.subscribe_msg_sent_event, None);
}

/// 对应 Java: Gson `@SerializedName` + 别名（OpenID/OpenId 双键兼容）。
#[test]
fn from_json_to_json_round_trip() {
    let json = r#"{
        "ToUserName": "gh_1",
        "FromUserName": "oABC",
        "CreateTime": 1700000000,
        "MsgType": "text",
        "Content": "你好",
        "MsgId": 123456,
        "OpenId": "openid-lower",
        "version": 2,
        "result": {"suggest": "pass", "label": "100"},
        "detail": [{"strategy": "s1", "errcode": 0, "suggest": "pass", "label": "100", "prob": 90}]
    }"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    assert_eq!(message.open_id.as_deref(), Some("openid-lower"));
    assert_eq!(message.msg_id, Some(123456));

    // to_json：@SerializedName 键名 + null 字段省略
    let out = message.to_json().expect("序列化成功");
    let value: serde_json::Value = serde_json::from_str(&out).expect("输出为 JSON");
    assert_eq!(value["ToUserName"], "gh_1");
    assert_eq!(value["CreateTime"], 1700000000);
    assert_eq!(value["OpenID"], "openid-lower");
    assert_eq!(value["version"], 2);
    assert_eq!(value["result"]["suggest"], "pass");
    assert_eq!(value["detail"][0]["prob"], 90);
    assert!(value.get("all_fields_map").is_none(), "全量 map 不上线");

    // 往返
    let reparsed = WxMaMessage::from_json(&out).expect("再解析成功");
    assert_eq!(reparsed.content, message.content);
    assert_eq!(reparsed.result, message.result);
    assert_eq!(reparsed.detail, message.detail);
}

/// 对应 Java: fromJson 的 OpenPID/大写 OpenID 键。
#[test]
fn from_json_accepts_alternate_keys() {
    let json = r#"{"OpenID": "openid-upper", "OpenPID": "openpid-1"}"#;
    let message = WxMaMessage::from_json(json).expect("JSON 解析成功");
    assert_eq!(message.open_id.as_deref(), Some("openid-upper"));
    assert_eq!(message.open_pid.as_deref(), Some("openpid-1"));
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: from_json 错误路径
// ══════════════════════════════════════════════════════════════════════════════

/// 非法 JSON / `List` 非对象数组。
#[test]
fn from_json_error_paths() {
    let err = WxMaMessage::from_json("{invalid").expect_err("非法 JSON 应报错");
    assert!(err.contains("WxMaMessage 解析失败"), "实际错误: {err}");

    let err = WxMaMessage::from_json(r#"{"Event": "e", "List": "纯字符串"}"#)
        .expect_err("List 非对象/数组应报错");
    assert!(err.contains("List 应为对象或数组"), "实际错误: {err}");
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: 加密消息（镜像 Java fromEncryptedXml / decryptField / fromEncryptedJson）
// ══════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMessage.fromEncryptedXml —— 验签解密后重解析，context 记录明文。
#[test]
fn from_encrypted_xml_round_trip() {
    let config = crypto_config();
    let plain_xml = "<xml><ToUserName><![CDATA[gh_1]]></ToUserName>\
                     <Content><![CDATA[加密内容]]></Content></xml>";
    let ctx = WxMaCryptUtils::new(&config)
        .and_then(|c| c.encrypt_context(plain_xml))
        .expect("加密成功");
    let encrypted_xml = format!(
        "<xml><Encrypt><![CDATA[{}]]></Encrypt></xml>",
        ctx.encrypted_xml
    );

    let message = WxMaMessage::from_encrypted_xml(
        &encrypted_xml,
        &config,
        &ctx.timestamp,
        &ctx.nonce,
        &ctx.signature,
    )
    .expect("解密解析成功");
    assert_eq!(message.to_user.as_deref(), Some("gh_1"));
    assert_eq!(message.content.as_deref(), Some("加密内容"));
    // context = 解密后的明文（Java 字段）
    assert_eq!(message.context.as_deref(), Some(plain_xml));
}

/// 对应 Java: WxMaMessage.decryptField —— Encrypt 字段解密重解析。
#[test]
fn decrypt_field_round_trip() {
    let config = crypto_config();
    let plain_xml = "<xml><MsgType><![CDATA[text]]></MsgType>\
                     <Content><![CDATA[字段解密]]></Content></xml>";
    let ctx = WxMaCryptUtils::new(&config)
        .and_then(|c| c.encrypt_context(plain_xml))
        .expect("加密成功");
    let message = WxMaMessage::from_xml(&format!(
        "<xml><Encrypt><![CDATA[{}]]></Encrypt></xml>",
        ctx.encrypted_xml
    ))
    .expect("解析成功");

    let decrypted = message
        .decrypt_field(&config, &ctx.timestamp, &ctx.nonce, &ctx.signature)
        .expect("字段解密成功");
    assert_eq!(decrypted.content.as_deref(), Some("字段解密"));
    // 原消息的 Encrypt 字段保留
    assert_eq!(message.encrypt.as_deref(), Some(ctx.encrypted_xml.as_str()));
}

/// 对应 Java: WxMaMessage.fromEncryptedJson —— AES 直解（不验签）后重解析 JSON。
#[test]
fn from_encrypted_json_round_trip() {
    let config = crypto_config();
    let plain_json = r#"{"ToUserName":"gh_1","MsgType":"text","Content":"json加密"}"#;
    let ctx = WxMaCryptUtils::new(&config)
        .and_then(|c| c.encrypt_context(plain_json))
        .expect("加密成功");
    let encrypted_json = format!(r#"{{"Encrypt":"{}"}}"#, ctx.encrypted_xml);

    let message = WxMaMessage::from_encrypted_json(&encrypted_json, &config).expect("解密解析成功");
    assert_eq!(message.to_user.as_deref(), Some("gh_1"));
    assert_eq!(message.content.as_deref(), Some("json加密"));
}

// ══════════════════════════════════════════════════════════════════════════════
// RUST_OBLIGATION: 加密路径错误
// ══════════════════════════════════════════════════════════════════════════════

/// 验签失败 / 缺 Encrypt 字段。
#[test]
fn encrypted_error_paths() {
    let config = crypto_config();
    let plain_xml = "<xml><Content><![CDATA[x]]></Content></xml>";
    let ctx = WxMaCryptUtils::new(&config)
        .and_then(|c| c.encrypt_context(plain_xml))
        .expect("加密成功");
    let encrypted_xml = format!(
        "<xml><Encrypt><![CDATA[{}]]></Encrypt></xml>",
        ctx.encrypted_xml
    );

    // 签名错误
    let err = WxMaMessage::from_encrypted_xml(
        &encrypted_xml,
        &config,
        &ctx.timestamp,
        &ctx.nonce,
        "bad-signature",
    )
    .expect_err("签名错误应报错");
    assert_eq!(err, "签名验证错误");

    // decrypt_field：消息无 Encrypt 字段
    let no_encrypt =
        WxMaMessage::from_xml("<xml><Content><![CDATA[x]]></Content></xml>").expect("解析成功");
    let err = no_encrypt
        .decrypt_field(&config, &ctx.timestamp, &ctx.nonce, &ctx.signature)
        .expect_err("缺 Encrypt 应报错");
    assert_eq!(err, "消息中不存在 Encrypt 字段");

    // from_encrypted_json：无 Encrypt 字段
    let err = WxMaMessage::from_encrypted_json(r#"{"Content":"x"}"#, &config)
        .expect_err("缺 Encrypt 应报错");
    assert_eq!(err, "加密 JSON 中不存在 Encrypt 字段");

    // aesKey 无效 → 构建工具失败
    let mut bad = WxMaDefaultConfig::new("wxappid", "secret");
    bad.set_token("tokentoken");
    bad.set_aes_key("short");
    let err = WxMaMessage::from_encrypted_xml(
        &encrypted_xml,
        &bad,
        &ctx.timestamp,
        &ctx.nonce,
        &ctx.signature,
    )
    .expect_err("非法 aesKey 应报错");
    assert!(err.contains("aesKey"), "实际错误: {err}");
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: getter（对齐 Java getMsgType/getFromUser/...）
// ══════════════════════════════════════════════════════════════════════════════

/// 全部 getter 的有值/无值分支。
#[test]
fn getters_expose_fields() {
    let message = WxMaMessage::from_xml(&full_xml()).expect("解析成功");
    assert_eq!(message.get_msg_type(), Some("text"));
    assert_eq!(message.get_from_user(), Some("oABC"));
    assert_eq!(message.get_event(), Some("user_enter_tempsession"));
    assert_eq!(message.get_msg_id(), Some(8234567890123456));
    assert_eq!(message.get_create_time(), Some(1700000000));
    assert_eq!(message.get_content(), Some("你好"));
    assert_eq!(message.get_title(), Some("标题"));
    assert_eq!(message.get_to_user(), Some("gh_123"));
    assert_eq!(message.get_trace_id(), Some("trace-1"));

    let empty = WxMaMessage::default();
    assert_eq!(empty.get_msg_type(), None);
    assert_eq!(empty.get_from_user(), None);
    assert_eq!(empty.get_event(), None);
    assert_eq!(empty.get_msg_id(), None);
    assert_eq!(empty.get_create_time(), None);
    assert_eq!(empty.get_content(), None);
    assert_eq!(empty.get_title(), None);
    assert_eq!(empty.get_to_user(), None);
    assert_eq!(empty.get_trace_id(), None);
}

// ══════════════════════════════════════════════════════════════════════════════
// SOURCE_PARITY: XmlValue 树值（镜像 Java XmlUtils.xml2Map 嵌套 Map/List）
// ══════════════════════════════════════════════════════════════════════════════

/// as_scalar/as_node/as_array 三态 + JSON → 树值（对象/数组/字符串/null/数值）。
#[test]
fn xml_value_semantics() {
    let scalar = XmlValue::Scalar("s".to_string());
    let mut node_map: HashMap<String, XmlValue> = HashMap::new();
    node_map.insert("k".to_string(), XmlValue::Scalar("v".to_string()));
    let node = XmlValue::Node(node_map);
    let array = XmlValue::Array(vec![XmlValue::Scalar("a".to_string())]);

    assert_eq!(scalar.as_scalar(), Some("s"));
    assert_eq!(scalar.as_node(), None);
    assert_eq!(scalar.as_array(), None);
    assert_eq!(node.as_scalar(), None);
    assert!(node.as_node().is_some());
    assert_eq!(node.as_array(), None);
    assert_eq!(array.as_scalar(), None);
    assert_eq!(array.as_node(), None);
    assert_eq!(array.as_array().map(|v| v.len()), Some(1));

    // Deserialize：JSON 对象/数组/字符串/null/数值 → 树值
    let value: XmlValue =
        serde_json::from_str(r#"{"a": "str", "b": [1, true, null], "c": {"d": "deep"}, "e": 2.5}"#)
            .expect("JSON → XmlValue");
    let map = value.as_node().expect("根为节点");
    assert_eq!(map.get("a").and_then(XmlValue::as_scalar), Some("str"));
    let b = map.get("b").and_then(XmlValue::as_array).expect("b 为数组");
    assert_eq!(b.len(), 3);
    assert_eq!(b[0].as_scalar(), Some("1"));
    assert_eq!(b[1].as_scalar(), Some("true"));
    assert_eq!(b[2].as_scalar(), Some(""));
    assert!(map.get("c").and_then(XmlValue::as_node).is_some());
    assert_eq!(map.get("e").and_then(XmlValue::as_scalar), Some("2.5"));
}

/// 顶层 JSON 数组 → XmlValue::Array。
#[test]
fn xml_value_from_top_level_array() {
    let value: XmlValue = serde_json::from_str(r#"[{"a": "1"}, "x"]"#).expect("顶层数组解析");
    let items = value.as_array().expect("顶层数组");
    assert_eq!(items.len(), 2);
    assert!(items[0].as_node().is_some());
    assert_eq!(items[1].as_scalar(), Some("x"));
}
