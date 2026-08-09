//! 小程序推送消息（xml/json 双格式）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaMessage`。Java 用 XStream
//! 反射 + dom4j 全量树解析 XML，Gson（`@SerializedName`）解析 JSON；
//! Rust 以 quick-xml 解析为嵌套树后按元素名提取字段（同一线格式语义），
//! JSON 经 serde 派生（`List` 对象/数组歧义由 `from_json` 后处理解决）。

use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::Event;
use serde::{Deserialize, Serialize};

use wx_rust_common::util::crypto::WxCryptUtil;

use crate::bean::{DetailBean, ResultBean, WxMaXPayTeamInfo};
use crate::config::WxMaConfig;
use crate::message::{
    ChangeEvent, PopupEvent, SentEvent, SubscribeMsgChangeEvent, SubscribeMsgPopupEvent,
    SubscribeMsgSentEvent, WxMaSubscribeMsgEventJson,
};
use crate::util::crypto::WxMaCryptUtils;

/// XML 树节点值（对应 Java `XmlUtils.xml2Map` 的嵌套 Map/List 结构）。
#[derive(Debug, Clone, PartialEq)]
pub enum XmlValue {
    /// 叶子文本（含 CDATA 原文）。
    Scalar(String),
    /// 元素节点（子元素名 → 值）。
    Node(HashMap<String, XmlValue>),
    /// 同名重复元素的数组。
    Array(Vec<XmlValue>),
}

impl XmlValue {
    /// 取标量文本。
    pub fn as_scalar(&self) -> Option<&str> {
        match self {
            XmlValue::Scalar(s) => Some(s),
            _ => None,
        }
    }

    /// 取节点 map。
    pub fn as_node(&self) -> Option<&HashMap<String, XmlValue>> {
        match self {
            XmlValue::Node(m) => Some(m),
            _ => None,
        }
    }

    /// 取数组。
    pub fn as_array(&self) -> Option<&[XmlValue]> {
        match self {
            XmlValue::Array(v) => Some(v),
            _ => None,
        }
    }
}

/// 将 JSON 值转成树值（用于 `from_json` 的全量 map，对应 Java
/// `GsonBuilder.fromJson(json, Map.class)` 的嵌套 Map/List 语义）。
fn xml_value_from_json(v: serde_json::Value) -> XmlValue {
    match v {
        serde_json::Value::Object(m) => XmlValue::Node(
            m.into_iter()
                .map(|(k, val)| (k, xml_value_from_json(val)))
                .collect(),
        ),
        serde_json::Value::Array(a) => {
            XmlValue::Array(a.into_iter().map(xml_value_from_json).collect())
        }
        serde_json::Value::String(s) => XmlValue::Scalar(s),
        serde_json::Value::Null => XmlValue::Scalar(String::new()),
        other => XmlValue::Scalar(other.to_string()),
    }
}

impl<'de> Deserialize<'de> for XmlValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        Ok(xml_value_from_json(v))
    }
}

/// 解析 XML 文档为嵌套树（根为 `Node`）。
fn parse_tree(xml: &str) -> Result<XmlValue, String> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => break,
            Ok(Event::Eof) => return Err("XML 解析失败: 缺少根元素".to_string()),
            Ok(Event::Text(_)) | Ok(Event::CData(_)) => {}
            Err(e) => return Err(format!("XML 解析失败: {e}")),
            _ => {}
        }
        buf.clear();
    }
    parse_element_body(&mut reader)
}

/// 递归解析当前元素的子内容（当前元素的 Start 已消费），
/// 直到匹配的 End 标签，返回该元素的值。
fn parse_element_body(reader: &mut Reader<&[u8]>) -> Result<XmlValue, String> {
    let mut text = String::new();
    let mut fields: HashMap<String, XmlValue> = HashMap::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let child = parse_element_body(reader)?;
                insert_field(&mut fields, name, child);
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                insert_field(&mut fields, name, XmlValue::Node(HashMap::new()));
            }
            Ok(Event::Text(t)) => {
                let s = t.decode().map_err(|e| e.to_string())?;
                text.push_str(&s);
            }
            Ok(Event::CData(t)) => {
                let s = t.decode().map_err(|e| e.to_string())?;
                text.push_str(&s);
            }
            Ok(Event::End(_)) => break,
            Ok(Event::Eof) => return Err("XML 解析失败: 元素未闭合".to_string()),
            Err(e) => return Err(format!("XML 解析失败: {e}")),
            _ => {}
        }
        buf.clear();
    }

    if fields.is_empty() {
        Ok(XmlValue::Scalar(text))
    } else {
        // 混合文本与子元素：Java dom4j 以元素为准，文本丢弃
        Ok(XmlValue::Node(fields))
    }
}

/// 同名重复元素合并为数组（对应 Java xml2Map 的 List 语义）。
fn insert_field(fields: &mut HashMap<String, XmlValue>, name: String, value: XmlValue) {
    match fields.remove(&name) {
        None => {
            fields.insert(name, value);
        }
        Some(existing) => {
            let mut arr = match existing {
                XmlValue::Array(v) => v,
                other => vec![other],
            };
            arr.push(value);
            fields.insert(name, XmlValue::Array(arr));
        }
    }
}

/// 从节点 map 取标量字段。
fn str_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<String> {
    map.get(name)
        .and_then(XmlValue::as_scalar)
        .map(str::to_string)
}

/// 从节点 map 取 Long 字段。
fn long_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<i64> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取 Integer 字段。
fn int_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<i32> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取嵌套节点。
fn node_field<'a>(
    map: &'a HashMap<String, XmlValue>,
    name: &str,
) -> Option<&'a HashMap<String, XmlValue>> {
    map.get(name).and_then(XmlValue::as_node)
}

/// 从节点 map 取同名节点数组（单元素包装为单元素数组）。
fn node_array<'a>(map: &'a HashMap<String, XmlValue>, name: &str) -> Vec<&'a XmlValue> {
    match map.get(name) {
        Some(XmlValue::Array(v)) => v.iter().collect(),
        Some(node @ XmlValue::Node(_)) => vec![node],
        _ => Vec::new(),
    }
}

/// 解析订阅消息事件列表（`List` 节点下同名元素数组）。
fn parse_subscribe_list(map: &HashMap<String, XmlValue>) -> Vec<HashMap<String, XmlValue>> {
    node_array(map, "List")
        .into_iter()
        .filter_map(XmlValue::as_node)
        .cloned()
        .collect()
}

/// 解析异步校验结果节点（`result`）。
fn parse_result_bean(map: &HashMap<String, XmlValue>) -> ResultBean {
    ResultBean {
        suggest: str_field(map, "suggest").unwrap_or_default(),
        label: str_field(map, "label").unwrap_or_default(),
    }
}

/// 解析异步校验详细结果节点列表（`detail`）。
fn parse_detail_beans(root: &HashMap<String, XmlValue>) -> Vec<DetailBean> {
    node_array(root, "detail")
        .into_iter()
        .filter_map(XmlValue::as_node)
        .map(|item| DetailBean {
            strategy: str_field(item, "strategy").unwrap_or_default(),
            errcode: int_field(item, "errcode").unwrap_or_default(),
            suggest: str_field(item, "suggest").unwrap_or_default(),
            label: str_field(item, "label").unwrap_or_default(),
            prob: int_field(item, "prob").unwrap_or_default(),
        })
        .collect()
}

/// 解析拼团信息节点（`TeamInfo`）。
fn parse_team_info(map: &HashMap<String, XmlValue>) -> WxMaXPayTeamInfo {
    WxMaXPayTeamInfo {
        activity_id: str_field(map, "ActivityId").unwrap_or_default(),
        team_id: str_field(map, "TeamId").unwrap_or_default(),
        team_type: int_field(map, "TeamType").unwrap_or_default(),
        team_action: int_field(map, "TeamAction").unwrap_or_default(),
    }
}

/// 微信推送过来的消息（对应 Java `WxMaMessage`）。
///
/// serde 派生复现 Gson `@SerializedName` 线格式（null 字段省略）；
/// `all_fields_map` 与 Java 一致保留全量树，但 Rust 侧不参与 JSON 序列化
/// （Java Gson 会序列化该重复字段，属反射副作用，此处有意省略）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaMessage {
    /// 使用 dom4j 解析的存放所有 xml 或 json 属性和值的 map。
    #[serde(skip)]
    pub all_fields_map: Option<HashMap<String, XmlValue>>,
    /// 加密消息。
    #[serde(rename = "Encrypt", skip_serializing_if = "Option::is_none")]
    pub encrypt: Option<String>,
    /// 开发者微信号。
    #[serde(rename = "ToUserName", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 发送方帐号（一个 OpenID）。
    #[serde(rename = "FromUserName", skip_serializing_if = "Option::is_none")]
    pub from_user: Option<String>,
    /// 消息创建时间（整型）。
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    /// 消息类型。
    #[serde(rename = "MsgType", skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 消息数据格式（JSON/XML）。
    #[serde(rename = "MsgDataFormat", skip_serializing_if = "Option::is_none")]
    pub msg_data_format: Option<String>,
    /// 文本消息内容。
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 消息 id，64 位整型。
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<i64>,
    /// 图片链接。
    #[serde(rename = "PicUrl", skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
    /// 图片消息媒体 id。
    #[serde(rename = "MediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 事件类型。
    #[serde(rename = "Event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// 消息标题。
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 小程序 appid。
    #[serde(rename = "AppId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 小程序页面路径。
    #[serde(rename = "PagePath", skip_serializing_if = "Option::is_none")]
    pub page_path: Option<String>,
    /// 缩略图链接。
    #[serde(rename = "ThumbUrl", skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// 缩略图媒体 id。
    #[serde(rename = "ThumbMediaId", skip_serializing_if = "Option::is_none")]
    pub thumb_media_id: Option<String>,
    /// 会话来源。
    #[serde(rename = "SessionFrom", skip_serializing_if = "Option::is_none")]
    pub session_from: Option<String>,
    /// 异步校验是否违法违规（isrisky）。
    #[serde(rename = "isrisky", skip_serializing_if = "Option::is_none")]
    pub is_risky: Option<String>,
    /// 异步校验附加信息（extra_info_json）。
    #[serde(rename = "extra_info_json", skip_serializing_if = "Option::is_none")]
    pub extra_info_json: Option<String>,
    /// 小程序 appid（小写键 appid）。
    #[serde(rename = "appid", skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 追踪 id。
    #[serde(rename = "trace_id", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 状态码。
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// 异步校验接口版本。
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// 异步校验综合结果。
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<ResultBean>,
    /// 异步校验详细检测结果。
    #[serde(rename = "detail", default)]
    pub detail: Vec<DetailBean>,
    /// 场景值。
    #[serde(rename = "Scene", skip_serializing_if = "Option::is_none")]
    pub scene: Option<i32>,
    /// 查询参数。
    #[serde(rename = "Query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 小程序 appid（大写键 AppID）。
    #[serde(rename = "AppID", skip_serializing_if = "Option::is_none")]
    pub app_i_d: Option<String>,
    /// 授权用户资料变更。
    #[serde(rename = "RevokeInfo", skip_serializing_if = "Option::is_none")]
    pub revoke_info: Option<String>,
    /// 微信客户端生成的 session id（OpenID/OpenId 双键兼容）。
    #[serde(
        rename = "OpenID",
        alias = "OpenId",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_id: Option<String>,
    /// 插件 id。
    #[serde(rename = "PluginID", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    /// 开放平台 id。
    #[serde(rename = "OpenPID", skip_serializing_if = "Option::is_none")]
    pub open_pid: Option<String>,
    /// 订阅消息弹窗事件。
    #[serde(
        rename = "SubscribeMsgPopupEvent",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribe_msg_popup_event: Option<SubscribeMsgPopupEvent>,
    /// 订阅消息变更事件。
    #[serde(
        rename = "SubscribeMsgChangeEvent",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribe_msg_change_event: Option<SubscribeMsgChangeEvent>,
    /// 订阅消息发送事件。
    #[serde(
        rename = "SubscribeMsgSentEvent",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribe_msg_sent_event: Option<SubscribeMsgSentEvent>,
    /// 返回值（小程序基本信息通知）。
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub ret: Option<String>,
    /// 一级类目 id。
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    /// 二级类目 id。
    #[serde(rename = "second", skip_serializing_if = "Option::is_none")]
    pub second: Option<String>,
    /// 驳回原因。
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 小程序代码审核驳回原因（Reason）。
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub we_app_reason: Option<String>,
    /// 昵称。
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 原始通知内容（解密后的明文）。
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// 微信支付订单号。
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 商户号。
    #[serde(rename = "merchant_id", skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// 子商户号。
    #[serde(rename = "sub_merchant_id", skip_serializing_if = "Option::is_none")]
    pub sub_merchant_id: Option<String>,
    /// 商户订单号。
    #[serde(rename = "merchant_trade_no", skip_serializing_if = "Option::is_none")]
    pub merchant_trade_no: Option<String>,
    /// 支付成功时间，秒级时间戳。
    #[serde(rename = "pay_time", skip_serializing_if = "Option::is_none")]
    pub pay_time: Option<i64>,
    /// 消息文本内容。
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// 发货时间，秒级时间戳。
    #[serde(rename = "shipped_time", skip_serializing_if = "Option::is_none")]
    pub shipped_time: Option<i64>,
    /// 预计结算时间，秒级时间戳。
    #[serde(
        rename = "estimated_settlement_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_settlement_time: Option<i64>,
    /// 确认收货方式：1 手动确认收货；2 自动确认收货。
    #[serde(
        rename = "confirm_receive_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub confirm_receive_method: Option<i32>,
    /// 确认收货时间，秒级时间戳。
    #[serde(
        rename = "confirm_receive_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub confirm_receive_time: Option<i64>,
    /// 订单结算时间，秒级时间戳。
    #[serde(rename = "settlement_time", skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    /// 微信退款单号。
    #[serde(rename = "WxRefundId", skip_serializing_if = "Option::is_none")]
    pub wx_refund_id: Option<String>,
    /// 商户退款单号。
    #[serde(rename = "MchRefundId", skip_serializing_if = "Option::is_none")]
    pub mch_refund_id: Option<String>,
    /// 退款单对应支付单的微信单号。
    #[serde(rename = "WxOrderId", skip_serializing_if = "Option::is_none")]
    pub wx_order_id: Option<String>,
    /// 退款单对应支付单的商户单号。
    #[serde(rename = "MchOrderId", skip_serializing_if = "Option::is_none")]
    pub mch_order_id: Option<String>,
    /// 退款金额，单位分。
    #[serde(rename = "RefundFee", skip_serializing_if = "Option::is_none")]
    pub refund_fee: Option<i32>,
    /// 退款结果，0 为成功，非 0 为失败。
    #[serde(rename = "RetCode", skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    /// 退款结果详情。
    #[serde(rename = "RetMsg", skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
    /// 开始退款时间，秒级时间戳。
    #[serde(
        rename = "RefundStartTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_start_timestamp: Option<i64>,
    /// 结束退款时间，秒级时间戳。
    #[serde(
        rename = "RefundSuccTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_succ_timestamp: Option<i64>,
    /// 退款单的微信支付单号。
    #[serde(
        rename = "WxpayRefundTransactionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub wxpay_refund_transaction_id: Option<String>,
    /// 重试次数，从 0 开始。
    #[serde(rename = "RetryTimes", skip_serializing_if = "Option::is_none")]
    pub retry_times: Option<i32>,
    /// 拼团信息。
    #[serde(rename = "TeamInfo", skip_serializing_if = "Option::is_none")]
    pub team_info: Option<WxMaXPayTeamInfo>,
    /// 微信支付交易单号（投诉）。
    #[serde(rename = "TransactionId", skip_serializing_if = "Option::is_none")]
    pub complaint_transaction_id: Option<String>,
    /// 投诉单号。
    #[serde(rename = "ComplaintId", skip_serializing_if = "Option::is_none")]
    pub complaint_id: Option<String>,
    /// 投诉详情。
    #[serde(rename = "ComplaintDetail", skip_serializing_if = "Option::is_none")]
    pub complaint_detail: Option<String>,
    /// 投诉时间，秒级时间戳。
    #[serde(rename = "ComplaintTime", skip_serializing_if = "Option::is_none")]
    pub complaint_time: Option<i64>,
    /// 请求编号。
    #[serde(rename = "RequestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 问询时间（iOS 退款查询）。
    #[serde(rename = "refund_time", skip_serializing_if = "Option::is_none")]
    pub refund_time: Option<String>,
    /// 订单时间（iOS 退款查询）。
    #[serde(rename = "order_time", skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,
    /// Apple 支付票据号。
    #[serde(rename = "channel_bill", skip_serializing_if = "Option::is_none")]
    pub channel_bill: Option<String>,
    /// 应用的 Apple bundleid。
    #[serde(rename = "bundleid", skip_serializing_if = "Option::is_none")]
    pub bundleid: Option<String>,
    /// 道具 id。
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub xpay_product_id: Option<String>,
    /// 道具/代币数量。
    #[serde(rename = "p_count", skip_serializing_if = "Option::is_none")]
    pub p_count: Option<String>,
    /// 用户请求退款的原因。
    #[serde(
        rename = "refund_request_reason",
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_request_reason: Option<String>,
    /// 发货状态，0 未发货 1 已发货 2 发货中。
    #[serde(rename = "provide_status", skip_serializing_if = "Option::is_none")]
    pub provide_status: Option<String>,
    /// 订阅消息事件原始 `List`（对象或数组，仅 JSON 线格式存在；
    /// `from_json` 按 event 归集到订阅事件字段后清空，对应 Java `uselessMsg`）。
    #[serde(rename = "List", skip_serializing_if = "Option::is_none")]
    pub subscribe_msg_list: Option<serde_json::Value>,
}

impl WxMaMessage {
    /// 从 xml 字符串解析消息（对应 Java `fromXml(String)`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let tree = parse_tree(xml)?;
        let root = match tree {
            XmlValue::Node(m) => m,
            other => {
                return Err(format!(
                    "XML 根元素应为节点，实际为: {}",
                    match other {
                        XmlValue::Scalar(s) => format!("标量 {s}"),
                        _ => "数组".to_string(),
                    }
                ));
            }
        };

        let subscribe_msg_popup_event =
            node_field(&root, "SubscribeMsgPopupEvent").map(|m| SubscribeMsgPopupEvent {
                list: parse_subscribe_list(m)
                    .into_iter()
                    .map(|item| PopupEvent {
                        template_id: str_field(&item, "TemplateId").unwrap_or_default(),
                        subscribe_status_string: str_field(&item, "SubscribeStatusString")
                            .unwrap_or_default(),
                        popup_scene: str_field(&item, "PopupScene").unwrap_or_default(),
                    })
                    .collect(),
            });
        let subscribe_msg_change_event =
            node_field(&root, "SubscribeMsgChangeEvent").map(|m| SubscribeMsgChangeEvent {
                list: parse_subscribe_list(m)
                    .into_iter()
                    .map(|item| ChangeEvent {
                        template_id: str_field(&item, "TemplateId").unwrap_or_default(),
                        subscribe_status_string: str_field(&item, "SubscribeStatusString")
                            .unwrap_or_default(),
                    })
                    .collect(),
            });
        let subscribe_msg_sent_event = node_field(&root, "SubscribeMsgSentEvent").and_then(|m| {
            node_field(m, "List").map(|item| SubscribeMsgSentEvent {
                list: Some(SentEvent {
                    template_id: str_field(item, "TemplateId").unwrap_or_default(),
                    msg_id: str_field(item, "MsgID").unwrap_or_default(),
                    error_code: str_field(item, "ErrorCode").unwrap_or_default(),
                    error_status: str_field(item, "ErrorStatus").unwrap_or_default(),
                }),
            })
        });

        Ok(Self {
            all_fields_map: Some(root.clone()),
            encrypt: str_field(&root, "Encrypt"),
            to_user: str_field(&root, "ToUserName"),
            from_user: str_field(&root, "FromUserName"),
            create_time: int_field(&root, "CreateTime"),
            msg_type: str_field(&root, "MsgType"),
            msg_data_format: str_field(&root, "MsgDataFormat"),
            content: str_field(&root, "Content"),
            msg_id: long_field(&root, "MsgId"),
            pic_url: str_field(&root, "PicUrl"),
            media_id: str_field(&root, "MediaId"),
            event: str_field(&root, "Event"),
            title: str_field(&root, "Title"),
            app_id: str_field(&root, "AppId"),
            page_path: str_field(&root, "PagePath"),
            thumb_url: str_field(&root, "ThumbUrl"),
            thumb_media_id: str_field(&root, "ThumbMediaId"),
            session_from: str_field(&root, "SessionFrom"),
            is_risky: str_field(&root, "isrisky"),
            extra_info_json: str_field(&root, "extra_info_json"),
            appid: str_field(&root, "appid"),
            trace_id: str_field(&root, "trace_id"),
            status_code: str_field(&root, "status_code"),
            version: int_field(&root, "version"),
            result: node_field(&root, "result").map(parse_result_bean),
            detail: parse_detail_beans(&root),
            scene: int_field(&root, "Scene"),
            query: str_field(&root, "Query"),
            app_i_d: str_field(&root, "AppID"),
            revoke_info: str_field(&root, "RevokeInfo"),
            open_id: str_field(&root, "OpenID"),
            plugin_id: str_field(&root, "PluginID"),
            open_pid: str_field(&root, "OpenPID"),
            subscribe_msg_popup_event,
            subscribe_msg_change_event,
            subscribe_msg_sent_event,
            ret: str_field(&root, "ret"),
            first: str_field(&root, "first"),
            second: str_field(&root, "second"),
            reason: str_field(&root, "reason"),
            we_app_reason: str_field(&root, "Reason"),
            nickname: str_field(&root, "nickname"),
            context: None,
            transaction_id: str_field(&root, "transaction_id"),
            merchant_id: str_field(&root, "merchant_id"),
            sub_merchant_id: str_field(&root, "sub_merchant_id"),
            merchant_trade_no: str_field(&root, "merchant_trade_no"),
            pay_time: long_field(&root, "pay_time"),
            msg: str_field(&root, "msg"),
            shipped_time: long_field(&root, "shipped_time"),
            estimated_settlement_time: long_field(&root, "estimated_settlement_time"),
            confirm_receive_method: int_field(&root, "confirm_receive_method"),
            confirm_receive_time: long_field(&root, "confirm_receive_time"),
            settlement_time: long_field(&root, "settlement_time"),
            wx_refund_id: str_field(&root, "WxRefundId"),
            mch_refund_id: str_field(&root, "MchRefundId"),
            wx_order_id: str_field(&root, "WxOrderId"),
            mch_order_id: str_field(&root, "MchOrderId"),
            refund_fee: int_field(&root, "RefundFee"),
            ret_code: int_field(&root, "RetCode"),
            ret_msg: str_field(&root, "RetMsg"),
            refund_start_timestamp: long_field(&root, "RefundStartTimestamp"),
            refund_succ_timestamp: long_field(&root, "RefundSuccTimestamp"),
            wxpay_refund_transaction_id: str_field(&root, "WxpayRefundTransactionId"),
            retry_times: int_field(&root, "RetryTimes"),
            team_info: node_field(&root, "TeamInfo").map(parse_team_info),
            complaint_transaction_id: str_field(&root, "TransactionId"),
            complaint_id: str_field(&root, "ComplaintId"),
            complaint_detail: str_field(&root, "ComplaintDetail"),
            complaint_time: long_field(&root, "ComplaintTime"),
            request_id: str_field(&root, "RequestId"),
            refund_time: str_field(&root, "refund_time"),
            order_time: str_field(&root, "order_time"),
            channel_bill: str_field(&root, "channel_bill"),
            bundleid: str_field(&root, "bundleid"),
            xpay_product_id: str_field(&root, "product_id"),
            p_count: str_field(&root, "p_count"),
            refund_request_reason: str_field(&root, "refund_request_reason"),
            provide_status: str_field(&root, "provide_status"),
            subscribe_msg_list: None,
        })
    }

    /// 从加密字符串转换（对应 Java `fromEncryptedXml`）。
    pub fn from_encrypted_xml(
        encrypted_xml: &str,
        config: &dyn WxMaConfig,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxMaCryptUtils::new(config)?;
        let plain_text = crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)?;
        let mut message = Self::from_xml(&plain_text)?;
        message.context = Some(plain_text);
        Ok(message)
    }

    /// 解密 `Encrypt` 字段后重新解析消息（对应 Java `decryptField`）。
    pub fn decrypt_field(
        &self,
        config: &dyn WxMaConfig,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxMaCryptUtils::new(config)?;
        let cipher = self
            .encrypt
            .as_deref()
            .ok_or_else(|| "消息中不存在 Encrypt 字段".to_string())?;
        let plain_text = crypt_util.decrypt_content(msg_signature, timestamp, nonce, cipher)?;
        Self::from_xml(&plain_text)
    }

    /// 从 JSON 构建（对应 Java `fromJson`：`List` 对象/数组按 event 归集到
    /// 订阅事件字段，并保留全量 map）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        let mut message: WxMaMessage =
            serde_json::from_str(json).map_err(|e| format!("WxMaMessage 解析失败: {e}"))?;
        if let Some(list) = message.subscribe_msg_list.take() {
            let aggregate = WxMaSubscribeMsgEventJson::from_list_value(&list)?;
            match message.event.as_deref() {
                Some("subscribe_msg_popup_event") => {
                    message.subscribe_msg_popup_event = aggregate.popup_events;
                }
                Some("subscribe_msg_change_event") => {
                    message.subscribe_msg_change_event = aggregate.change_events;
                }
                Some("subscribe_msg_sent_event") => {
                    message.subscribe_msg_sent_event = aggregate.sent_event;
                }
                _ => {}
            }
        }
        let all: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("WxMaMessage 全量 map 解析失败: {e}"))?;
        if let Ok(map) = serde_json::from_value::<HashMap<String, XmlValue>>(all) {
            message.all_fields_map = Some(map);
        }
        Ok(message)
    }

    /// 从加密 JSON 转换（对应 Java `fromEncryptedJson`：解密 Encrypt 字段后重解析）。
    pub fn from_encrypted_json(
        encrypted_json: &str,
        config: &dyn WxMaConfig,
    ) -> Result<Self, String> {
        let encrypted_message = Self::from_json(encrypted_json)?;
        let cipher = encrypted_message
            .encrypt
            .as_deref()
            .ok_or_else(|| "加密 JSON 中不存在 Encrypt 字段".to_string())?;
        // Java `WxCryptUtils.decrypt`：直接 AES 解密（不校验签名）
        let aes_key = config.aes_key().unwrap_or_default().replace(' ', "");
        let crypt_util =
            WxCryptUtil::new(config.token().unwrap_or_default(), aes_key, config.app_id())?;
        let plain_text = crypt_util.decrypt(cipher)?;
        Self::from_json(&plain_text)
    }

    /// 序列化为 JSON（对应 Java `toJson`，`@SerializedName` 键名、null 省略）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaMessage 序列化失败: {e}"))
    }

    /// 消息类型 getter（对齐 Java `getMsgType()`）。
    pub fn get_msg_type(&self) -> Option<&str> {
        self.msg_type.as_deref()
    }

    /// 发送方帐号 getter（对齐 Java `getFromUser()`）。
    pub fn get_from_user(&self) -> Option<&str> {
        self.from_user.as_deref()
    }

    /// 事件类型 getter（对齐 Java `getEvent()`）。
    pub fn get_event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    /// 消息 id getter（对齐 Java `getMsgId()`）。
    pub fn get_msg_id(&self) -> Option<i64> {
        self.msg_id
    }

    /// 消息创建时间 getter（对齐 Java `getCreateTime()`）。
    pub fn get_create_time(&self) -> Option<i32> {
        self.create_time
    }

    /// 消息内容 getter（对齐 Java `getContent()`）。
    pub fn get_content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// 消息标题 getter（对齐 Java `getTitle()`）。
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// 开发者微信号 getter（对齐 Java `getToUser()`）。
    pub fn get_to_user(&self) -> Option<&str> {
        self.to_user.as_deref()
    }

    /// 追踪 id getter（对齐 Java `getTraceId()`）。
    pub fn get_trace_id(&self) -> Option<&str> {
        self.trace_id.as_deref()
    }

    /// 转换成 xml 格式（XStream 线格式：String 包 CDATA、数值裸值、null 省略、
    /// 字段按 Java 声明序；`all_fields_map` 不输出）。
    pub fn to_xml(&self) -> String {
        let mut s = String::from("<xml>");
        push_cdata(&mut s, "Encrypt", self.encrypt.as_deref());
        push_cdata(&mut s, "ToUserName", self.to_user.as_deref());
        push_cdata(&mut s, "FromUserName", self.from_user.as_deref());
        push_num(&mut s, "CreateTime", self.create_time);
        push_cdata(&mut s, "MsgType", self.msg_type.as_deref());
        push_cdata(&mut s, "MsgDataFormat", self.msg_data_format.as_deref());
        push_cdata(&mut s, "Content", self.content.as_deref());
        push_num(&mut s, "MsgId", self.msg_id);
        push_cdata(&mut s, "PicUrl", self.pic_url.as_deref());
        push_cdata(&mut s, "MediaId", self.media_id.as_deref());
        push_cdata(&mut s, "Event", self.event.as_deref());
        push_cdata(&mut s, "Title", self.title.as_deref());
        push_cdata(&mut s, "AppId", self.app_id.as_deref());
        push_cdata(&mut s, "PagePath", self.page_path.as_deref());
        push_cdata(&mut s, "ThumbUrl", self.thumb_url.as_deref());
        push_cdata(&mut s, "ThumbMediaId", self.thumb_media_id.as_deref());
        push_cdata(&mut s, "SessionFrom", self.session_from.as_deref());
        push_cdata(&mut s, "isrisky", self.is_risky.as_deref());
        push_cdata(&mut s, "extra_info_json", self.extra_info_json.as_deref());
        push_cdata(&mut s, "appid", self.appid.as_deref());
        push_cdata(&mut s, "trace_id", self.trace_id.as_deref());
        push_cdata(&mut s, "status_code", self.status_code.as_deref());
        push_num(&mut s, "version", self.version);
        if let Some(result) = &self.result {
            s.push_str("<result>");
            push_cdata(&mut s, "suggest", Some(&result.suggest));
            push_cdata(&mut s, "label", Some(&result.label));
            s.push_str("</result>");
        }
        for detail in &self.detail {
            s.push_str("<detail>");
            push_cdata(&mut s, "strategy", Some(&detail.strategy));
            push_num(&mut s, "errcode", Some(detail.errcode));
            push_cdata(&mut s, "suggest", Some(&detail.suggest));
            push_cdata(&mut s, "label", Some(&detail.label));
            push_num(&mut s, "prob", Some(detail.prob));
            s.push_str("</detail>");
        }
        push_num(&mut s, "Scene", self.scene);
        push_cdata(&mut s, "Query", self.query.as_deref());
        push_cdata(&mut s, "AppID", self.app_i_d.as_deref());
        push_cdata(&mut s, "RevokeInfo", self.revoke_info.as_deref());
        push_cdata(&mut s, "OpenID", self.open_id.as_deref());
        push_cdata(&mut s, "PluginID", self.plugin_id.as_deref());
        push_cdata(&mut s, "OpenPID", self.open_pid.as_deref());
        if let Some(ev) = &self.subscribe_msg_popup_event {
            s.push_str("<SubscribeMsgPopupEvent>");
            for item in &ev.list {
                s.push_str("<List>");
                push_cdata(&mut s, "TemplateId", Some(&item.template_id));
                push_cdata(
                    &mut s,
                    "SubscribeStatusString",
                    Some(&item.subscribe_status_string),
                );
                push_cdata(&mut s, "PopupScene", Some(&item.popup_scene));
                s.push_str("</List>");
            }
            s.push_str("</SubscribeMsgPopupEvent>");
        }
        if let Some(ev) = &self.subscribe_msg_change_event {
            s.push_str("<SubscribeMsgChangeEvent>");
            for item in &ev.list {
                s.push_str("<List>");
                push_cdata(&mut s, "TemplateId", Some(&item.template_id));
                push_cdata(
                    &mut s,
                    "SubscribeStatusString",
                    Some(&item.subscribe_status_string),
                );
                s.push_str("</List>");
            }
            s.push_str("</SubscribeMsgChangeEvent>");
        }
        if let Some(ev) = &self.subscribe_msg_sent_event {
            s.push_str("<SubscribeMsgSentEvent><List>");
            if let Some(item) = &ev.list {
                push_cdata(&mut s, "TemplateId", Some(&item.template_id));
                push_cdata(&mut s, "MsgID", Some(&item.msg_id));
                push_cdata(&mut s, "ErrorCode", Some(&item.error_code));
                push_cdata(&mut s, "ErrorStatus", Some(&item.error_status));
            }
            s.push_str("</List></SubscribeMsgSentEvent>");
        }
        push_cdata(&mut s, "ret", self.ret.as_deref());
        push_cdata(&mut s, "first", self.first.as_deref());
        push_cdata(&mut s, "second", self.second.as_deref());
        push_cdata(&mut s, "reason", self.reason.as_deref());
        push_cdata(&mut s, "Reason", self.we_app_reason.as_deref());
        push_cdata(&mut s, "nickname", self.nickname.as_deref());
        push_cdata(&mut s, "context", self.context.as_deref());
        push_cdata(&mut s, "transaction_id", self.transaction_id.as_deref());
        push_cdata(&mut s, "merchant_id", self.merchant_id.as_deref());
        push_cdata(&mut s, "sub_merchant_id", self.sub_merchant_id.as_deref());
        push_cdata(
            &mut s,
            "merchant_trade_no",
            self.merchant_trade_no.as_deref(),
        );
        push_num(&mut s, "pay_time", self.pay_time);
        push_cdata(&mut s, "msg", self.msg.as_deref());
        push_num(&mut s, "shipped_time", self.shipped_time);
        push_num(
            &mut s,
            "estimated_settlement_time",
            self.estimated_settlement_time,
        );
        push_num(
            &mut s,
            "confirm_receive_method",
            self.confirm_receive_method,
        );
        push_num(&mut s, "confirm_receive_time", self.confirm_receive_time);
        push_num(&mut s, "settlement_time", self.settlement_time);
        push_cdata(&mut s, "WxRefundId", self.wx_refund_id.as_deref());
        push_cdata(&mut s, "MchRefundId", self.mch_refund_id.as_deref());
        push_cdata(&mut s, "WxOrderId", self.wx_order_id.as_deref());
        push_cdata(&mut s, "MchOrderId", self.mch_order_id.as_deref());
        push_num(&mut s, "RefundFee", self.refund_fee);
        push_num(&mut s, "RetCode", self.ret_code);
        push_cdata(&mut s, "RetMsg", self.ret_msg.as_deref());
        push_num(&mut s, "RefundStartTimestamp", self.refund_start_timestamp);
        push_num(&mut s, "RefundSuccTimestamp", self.refund_succ_timestamp);
        push_cdata(
            &mut s,
            "WxpayRefundTransactionId",
            self.wxpay_refund_transaction_id.as_deref(),
        );
        push_num(&mut s, "RetryTimes", self.retry_times);
        if let Some(team) = &self.team_info {
            s.push_str("<TeamInfo>");
            push_cdata(&mut s, "ActivityId", Some(&team.activity_id));
            push_cdata(&mut s, "TeamId", Some(&team.team_id));
            push_num(&mut s, "TeamType", Some(team.team_type));
            push_num(&mut s, "TeamAction", Some(team.team_action));
            s.push_str("</TeamInfo>");
        }
        push_cdata(
            &mut s,
            "TransactionId",
            self.complaint_transaction_id.as_deref(),
        );
        push_cdata(&mut s, "ComplaintId", self.complaint_id.as_deref());
        push_cdata(&mut s, "ComplaintDetail", self.complaint_detail.as_deref());
        push_num(&mut s, "ComplaintTime", self.complaint_time);
        push_cdata(&mut s, "RequestId", self.request_id.as_deref());
        push_cdata(&mut s, "refund_time", self.refund_time.as_deref());
        push_cdata(&mut s, "order_time", self.order_time.as_deref());
        push_cdata(&mut s, "channel_bill", self.channel_bill.as_deref());
        push_cdata(&mut s, "bundleid", self.bundleid.as_deref());
        push_cdata(&mut s, "product_id", self.xpay_product_id.as_deref());
        push_cdata(&mut s, "p_count", self.p_count.as_deref());
        push_cdata(
            &mut s,
            "refund_request_reason",
            self.refund_request_reason.as_deref(),
        );
        push_cdata(&mut s, "provide_status", self.provide_status.as_deref());
        s.push_str("</xml>");
        s
    }
}

/// 输出 String 字段为 CDATA 元素（null 省略）。
pub(crate) fn push_cdata(s: &mut String, name: &str, value: Option<&str>) {
    if let Some(v) = value {
        s.push_str(&format!("<{name}><![CDATA[{v}]]></{name}>"));
    }
}

/// 输出数值字段（null 省略）。
fn push_num<T: std::fmt::Display>(s: &mut String, name: &str, value: Option<T>) {
    if let Some(v) = value {
        s.push_str(&format!("<{name}>{v}</{name}>"));
    }
}
