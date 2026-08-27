//! 微信推送消息（xml 格式）。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.WxMpXmlMessage`。Java 用
//! XStream 反射映射 + dom4j 全量树；Rust 以 quick-xml 解析为嵌套树后
//! 按元素名提取字段（同一线格式语义）。

use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::events::Event;

use crate::bean::message::{
    ArticleUrlResult, ArticleUrlResultItem, HardWare, PicItem, ScanCodeInfo, SendLocationInfo,
    SendPicsInfo, WxMpSubscribeMsgChangeEvent, WxMpSubscribeMsgPopupEvent,
    WxMpSubscribeMsgSentEvent,
};
use crate::config::WxMpConfigStorage;
use crate::util::crypto::WxMpCryptUtil;

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

/// 解析 XML 文档为嵌套树（根为 `Node`）。
fn parse_tree(xml: &str) -> Result<XmlValue, String> {
    let mut reader = Reader::from_str(xml);
    // Java dom4j 保留文本原样（含 CDATA 与前后空白），不 trim
    // 消费根起始标签（跳过前导文本）
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
                let name = e.name().as_ref().to_string();
                let child = parse_element_body(reader)?;
                insert_field(&mut fields, name, child);
            }
            Ok(Event::Empty(e)) => {
                let name = e.name().as_ref().to_string();
                insert_field(&mut fields, name, XmlValue::Node(HashMap::new()));
            }
            Ok(Event::Text(t)) => {
                let s = t.xml_content(XmlVersion::Implicit1_0);
                text.push_str(&s);
            }
            Ok(Event::CData(t)) => {
                let s = t.xml_content(XmlVersion::Implicit1_0);
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

/// 从节点 map 取 Double 字段。
fn double_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<f64> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取嵌套节点。
fn node_field<'a>(
    map: &'a HashMap<String, XmlValue>,
    name: &str,
) -> Option<&'a HashMap<String, XmlValue>> {
    map.get(name).and_then(XmlValue::as_node)
}

/// 从节点 map 取同名节点数组。
///
/// Java XStream 的 List 字段在元素唯一时仍是 List；树中单元素存为
/// `Node`，此处包装为单元素数组（对应 Java `getList().get(0)` 语义）。
fn node_array<'a>(map: &'a HashMap<String, XmlValue>, name: &str) -> Vec<&'a XmlValue> {
    match map.get(name) {
        Some(XmlValue::Array(v)) => v.iter().collect(),
        Some(node @ XmlValue::Node(_)) => vec![node],
        _ => Vec::new(),
    }
}

/// 解析扫码信息节点。
fn parse_scan_code_info(map: &HashMap<String, XmlValue>) -> ScanCodeInfo {
    ScanCodeInfo {
        scan_type: str_field(map, "ScanType"),
        scan_result: str_field(map, "ScanResult"),
    }
}

/// 解析发送图片信息节点。
fn parse_send_pics_info(map: &HashMap<String, XmlValue>) -> SendPicsInfo {
    let pic_list = node_field(map, "PicList")
        .map(|pic_list| {
            node_array(pic_list, "item")
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(|item| PicItem {
                    pic_md5_sum: str_field(item, "PicMd5Sum"),
                })
                .collect()
        })
        .unwrap_or_default();
    SendPicsInfo {
        count: long_field(map, "Count"),
        pic_list,
    }
}

/// 解析发送位置信息节点。
fn parse_send_location_info(map: &HashMap<String, XmlValue>) -> SendLocationInfo {
    SendLocationInfo {
        location_x: str_field(map, "Location_X"),
        location_y: str_field(map, "Location_Y"),
        scale: str_field(map, "Scale"),
        label: str_field(map, "Label"),
        poi_name: str_field(map, "Poiname"),
    }
}

/// 解析图文 url 结果节点。
fn parse_article_url_result(map: &HashMap<String, XmlValue>) -> ArticleUrlResult {
    let result_list = node_field(map, "ResultList")
        .map(|result_list| {
            node_array(result_list, "item")
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(|item| ArticleUrlResultItem {
                    article_idx: str_field(item, "ArticleIdx"),
                    article_url: str_field(item, "ArticleUrl"),
                })
                .collect()
        })
        .unwrap_or_default();
    ArticleUrlResult {
        count: long_field(map, "Count"),
        result_list,
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

/// 微信推送过来的消息（xml 格式）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxMpXmlMessage {
    /// 存放所有 xml 属性和值的 map（对应 Java `allFieldsMap`）。
    pub all_fields_map: Option<HashMap<String, XmlValue>>,
    /// 开发者微信号。
    pub to_user: Option<String>,
    /// 发送方帐号（一个 OpenID）。
    pub from_user: Option<String>,
    /// 消息创建时间（整型）。
    pub create_time: Option<i64>,
    /// 消息类型。
    pub msg_type: Option<String>,
    /// 文本消息内容。
    pub content: Option<String>,
    /// 菜单 id。
    pub menu_id: Option<i64>,
    /// 消息 id，64 位整型。
    pub msg_id: Option<i64>,
    /// 图片链接。
    pub pic_url: Option<String>,
    /// 图片消息媒体 id。
    pub media_id: Option<String>,
    /// 语音格式。
    pub format: Option<String>,
    /// 视频消息缩略图的媒体 id。
    pub thumb_media_id: Option<String>,
    /// 地理位置纬度。
    pub location_x: Option<f64>,
    /// 地理位置经度。
    pub location_y: Option<f64>,
    /// 地图缩放大小。
    pub scale: Option<f64>,
    /// 地理位置信息。
    pub label: Option<String>,
    /// 消息标题。
    pub title: Option<String>,
    /// 消息描述。
    pub description: Option<String>,
    /// 消息链接。
    pub url: Option<String>,
    /// 事件类型。
    pub event: Option<String>,
    /// 事件 KEY 值。
    pub event_key: Option<String>,
    /// 二维码 ticket。
    pub ticket: Option<String>,
    /// 地理位置纬度。
    pub latitude: Option<f64>,
    /// 地理位置经度。
    pub longitude: Option<f64>,
    /// 地理位置精度。
    pub precision: Option<f64>,
    /// 语音识别结果。
    pub recognition: Option<String>,
    /// 用户 union id。
    pub union_id: Option<String>,
    /// 回包参数（企业微信兼容）。
    pub ret: Option<i32>,
    /// 昵称。
    pub nickname: Option<String>,
    /// 首字段（企业微信兼容）。
    pub first: Option<String>,
    /// 次字段（企业微信兼容）。
    pub second: Option<String>,
    /// 群发的消息 ID。
    pub mass_msg_id: Option<i64>,
    /// 群发的结果。
    pub status: Option<String>,
    /// group_id 下粉丝数。
    pub total_count: Option<i32>,
    /// 过滤后准备发送的粉丝数。
    pub filter_count: Option<i32>,
    /// 发送成功的粉丝数。
    pub sent_count: Option<i32>,
    /// 发送失败的粉丝数。
    pub error_count: Option<i32>,
    /// 客服帐号。
    pub kf_account: Option<String>,
    /// 转入客服帐号。
    pub to_kf_account: Option<String>,
    /// 转出客服帐号。
    pub from_kf_account: Option<String>,
    /// 卡券 id。
    pub card_id: Option<String>,
    /// 转赠用户的用户名。
    pub friend_user_name: Option<String>,
    /// 是否为转赠，1 是 0 否。
    pub is_give_by_friend: Option<i32>,
    /// 用户卡券码。
    pub user_card_code: Option<String>,
    /// 转赠前卡券码。
    pub old_user_card_code: Option<String>,
    /// 外部 id。
    pub outer_id: Option<i32>,
    /// 用户删除会员卡后可找回标记。
    pub is_restore_member_card: Option<String>,
    /// 领取场景值。
    pub outer_str: Option<String>,
    /// 是否转赠退回。
    pub is_return_back: Option<String>,
    /// 是否是群转赠。
    pub is_chat_room: Option<String>,
    /// 核销来源。
    pub consume_source: Option<String>,
    /// 门店名称。
    pub location_name: Option<String>,
    /// 核销员的 openid。
    pub staff_open_id: Option<String>,
    /// 自助核销时用户输入的验证码。
    pub verify_code: Option<String>,
    /// 自助核销时用户输入的备注金额。
    pub remark_amount: Option<String>,
    /// 报警详细信息。
    pub detail: Option<String>,
    /// 变动的积分值。
    pub modify_bonus: Option<String>,
    /// 变动的余额值。
    pub modify_balance: Option<String>,
    /// 微信支付交易订单号。
    pub trans_id: Option<String>,
    /// 门店 ID。
    pub location_id: Option<String>,
    /// 实付金额（分）。
    pub fee: Option<String>,
    /// 应付金额（分）。
    pub original_fee: Option<String>,
    /// 扫码信息。
    pub scan_code_info: Option<ScanCodeInfo>,
    /// 发送图片信息。
    pub send_pics_info: Option<SendPicsInfo>,
    /// 发送位置信息。
    pub send_location_info: Option<SendLocationInfo>,
    /// 图文 url 结果。
    pub article_url_result: Option<ArticleUrlResult>,
    /// 审核不通过原因。
    pub refuse_reason: Option<String>,
    /// 是否为朋友推荐。
    pub is_recommend_by_friend: Option<String>,
    /// 购买券点时实际支付成功的时间。
    pub pay_finish_time: Option<String>,
    /// 支付二维码的生成时间。
    pub create_order_time: Option<String>,
    /// 描述。
    pub desc: Option<String>,
    /// 剩余免费券点数量。
    pub free_coin_count: Option<String>,
    /// 剩余付费券点数量。
    pub pay_coin_count: Option<String>,
    /// 本次变动的免费券点数量。
    pub refund_free_coin_count: Option<String>,
    /// 本次变动的付费券点数量。
    pub refund_pay_coin_count: Option<String>,
    /// 订单类型。
    pub order_type: Option<String>,
    /// 系统备注。
    pub memo: Option<String>,
    /// 所开发票的详情。
    pub receipt_info: Option<String>,
    /// 商户自己内部 ID（sid）。
    pub store_uniq_id: Option<String>,
    /// 微信的门店 ID。
    pub poi_id: Option<String>,
    /// 审核结果。
    pub result: Option<String>,
    /// 通知信息或驳回理由。
    pub msg: Option<String>,
    /// 认证有效期。
    pub expired_time: Option<i64>,
    /// 失败发生时间。
    pub fail_time: Option<i64>,
    /// 认证失败的原因。
    pub fail_reason: Option<String>,
    /// 重新填写时间戳。
    pub refill_time: Option<i64>,
    /// 重新填写原因。
    pub refill_reason: Option<String>,
    /// 订单 ID。
    pub order_id: Option<String>,
    /// 订单状态。
    pub order_status: Option<String>,
    /// 商品 ID。
    pub product_id: Option<String>,
    /// 商品 SKU 信息。
    pub sku_info: Option<String>,
    /// 设备类型。
    pub device_type: Option<String>,
    /// 设备 ID。
    pub device_id: Option<String>,
    /// 微信客户端生成的 session id。
    pub session_id: Option<String>,
    /// 微信用户账号的 OpenID。
    pub open_id: Option<String>,
    /// 硬件信息。
    pub hard_ware: Option<HardWare>,
    /// 请求类型。
    pub op_type: Option<i32>,
    /// 设备状态。
    pub device_status: Option<i32>,
    /// 审核成功时的时间。
    pub success_time: Option<i64>,
    /// 审核失败的原因。
    pub reason: Option<String>,
    /// 审核延后时的时间。
    pub delay_time: Option<i64>,
    /// 审核不通过的截图示例。
    pub screen_shot: Option<String>,
    /// 商品编码标准。
    pub key_standard: Option<String>,
    /// 商品编码内容。
    pub key_str: Option<String>,
    /// 国家。
    pub country: Option<String>,
    /// 省份。
    pub province: Option<String>,
    /// 城市。
    pub city: Option<String>,
    /// 性别。
    pub sex: Option<String>,
    /// 打开商品主页的场景。
    pub scene: Option<String>,
    /// 标识参数。
    pub ext_info: Option<String>,
    /// 实时地理位置信息。
    pub region_code: Option<String>,
    /// 审核未通过的原因。
    pub reason_msg: Option<String>,
    /// 菜单消息类型客服消息被点击的菜单 ID。
    pub biz_msg_menu_id: Option<String>,
    /// 授权成功的订单号。
    pub succ_order_id: Option<String>,
    /// 授权失败的订单号。
    pub fail_order_id: Option<String>,
    /// 获取授权页链接的 AppId。
    pub authorize_app_id: Option<String>,
    /// 授权来源。
    pub source: Option<String>,
    /// 发票请求流水号。
    pub fpqqlsh: Option<String>,
    /// 纳税人识别码。
    pub nsrsbh: Option<String>,
    /// 授权用户资料变更。
    pub revoke_info: Option<String>,
    /// 加密消息。
    pub encrypt: Option<String>,
    /// 订阅消息弹窗事件。
    pub subscribe_msg_popup_event: Option<WxMpSubscribeMsgPopupEvent>,
    /// 订阅消息变更事件。
    pub subscribe_msg_change_event: Option<WxMpSubscribeMsgChangeEvent>,
    /// 订阅消息发送事件。
    pub subscribe_msg_sent_event: Option<WxMpSubscribeMsgSentEvent>,
}

impl WxMpXmlMessage {
    /// 从 xml 字符串解析消息。
    ///
    /// 对应 Java `fromXml(String)`：先修正微信变态的消息内容格式
    /// （`</PicList><PicList>` 相邻闭合/开启），再解析字段与全量树。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        // 修改微信变态的消息内容格式，方便解析（与 Java 逐字一致）
        let xml = xml.replace("</PicList><PicList>", "");
        let tree = parse_tree(&xml)?;
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

        let scan_code_info = node_field(&root, "ScanCodeInfo").map(parse_scan_code_info);
        let send_pics_info = node_field(&root, "SendPicsInfo").map(parse_send_pics_info);
        let send_location_info =
            node_field(&root, "SendLocationInfo").map(parse_send_location_info);
        let article_url_result =
            node_field(&root, "ArticleUrlResult").map(parse_article_url_result);
        let hard_ware = node_field(&root, "HardWare").map(|m| HardWare {
            message_view: str_field(m, "messageView"),
            message_action: str_field(m, "messageAction"),
        });

        let subscribe_msg_popup_event =
            node_field(&root, "SubscribeMsgPopupEvent").map(|m| WxMpSubscribeMsgPopupEvent {
                list: parse_subscribe_list(m)
                    .into_iter()
                    .map(|item| crate::bean::message::PopupEvent {
                        template_id: str_field(&item, "TemplateId"),
                        subscribe_status_string: str_field(&item, "SubscribeStatusString"),
                        popup_scene: str_field(&item, "PopupScene"),
                    })
                    .collect(),
            });
        let subscribe_msg_change_event =
            node_field(&root, "SubscribeMsgChangeEvent").map(|m| WxMpSubscribeMsgChangeEvent {
                list: parse_subscribe_list(m)
                    .into_iter()
                    .map(|item| crate::bean::message::ChangeEvent {
                        template_id: str_field(&item, "TemplateId"),
                        subscribe_status_string: str_field(&item, "SubscribeStatusString"),
                    })
                    .collect(),
            });
        let subscribe_msg_sent_event =
            node_field(&root, "SubscribeMsgSentEvent").map(|m| WxMpSubscribeMsgSentEvent {
                list: parse_subscribe_list(m)
                    .into_iter()
                    .map(|item| crate::bean::message::SentEvent {
                        template_id: str_field(&item, "TemplateId"),
                        msg_id: str_field(&item, "MsgID"),
                        error_code: str_field(&item, "ErrorCode"),
                        error_status: str_field(&item, "ErrorStatus"),
                    })
                    .collect(),
            });

        Ok(Self {
            all_fields_map: Some(root.clone()),
            to_user: str_field(&root, "ToUserName"),
            from_user: str_field(&root, "FromUserName"),
            create_time: long_field(&root, "CreateTime"),
            msg_type: str_field(&root, "MsgType"),
            content: str_field(&root, "Content"),
            menu_id: long_field(&root, "MenuId"),
            msg_id: long_field(&root, "MsgId"),
            pic_url: str_field(&root, "PicUrl"),
            media_id: str_field(&root, "MediaId"),
            format: str_field(&root, "Format"),
            thumb_media_id: str_field(&root, "ThumbMediaId"),
            location_x: double_field(&root, "Location_X"),
            location_y: double_field(&root, "Location_Y"),
            scale: double_field(&root, "Scale"),
            label: str_field(&root, "Label"),
            title: str_field(&root, "Title"),
            description: str_field(&root, "Description"),
            url: str_field(&root, "Url"),
            event: str_field(&root, "Event"),
            event_key: str_field(&root, "EventKey"),
            ticket: str_field(&root, "Ticket"),
            latitude: double_field(&root, "Latitude"),
            longitude: double_field(&root, "Longitude"),
            precision: double_field(&root, "Precision"),
            recognition: str_field(&root, "Recognition"),
            union_id: str_field(&root, "UnionId"),
            ret: int_field(&root, "ret"),
            nickname: str_field(&root, "nickname"),
            first: str_field(&root, "first"),
            second: str_field(&root, "second"),
            mass_msg_id: long_field(&root, "MsgID"),
            status: str_field(&root, "Status"),
            total_count: int_field(&root, "TotalCount"),
            filter_count: int_field(&root, "FilterCount"),
            sent_count: int_field(&root, "SentCount"),
            error_count: int_field(&root, "ErrorCount"),
            kf_account: str_field(&root, "KfAccount"),
            to_kf_account: str_field(&root, "ToKfAccount"),
            from_kf_account: str_field(&root, "FromKfAccount"),
            card_id: str_field(&root, "CardId"),
            friend_user_name: str_field(&root, "FriendUserName"),
            is_give_by_friend: int_field(&root, "IsGiveByFriend"),
            user_card_code: str_field(&root, "UserCardCode"),
            old_user_card_code: str_field(&root, "OldUserCardCode"),
            outer_id: int_field(&root, "OuterId"),
            is_restore_member_card: str_field(&root, "IsRestoreMemberCard"),
            outer_str: str_field(&root, "OuterStr"),
            is_return_back: str_field(&root, "IsReturnBack"),
            is_chat_room: str_field(&root, "IsChatRoom"),
            consume_source: str_field(&root, "ConsumeSource"),
            location_name: str_field(&root, "LocationName"),
            staff_open_id: str_field(&root, "StaffOpenId"),
            verify_code: str_field(&root, "VerifyCode"),
            remark_amount: str_field(&root, "RemarkAmount"),
            detail: str_field(&root, "Detail"),
            modify_bonus: str_field(&root, "ModifyBonus"),
            modify_balance: str_field(&root, "ModifyBalance"),
            trans_id: str_field(&root, "TransId"),
            location_id: str_field(&root, "LocationId"),
            fee: str_field(&root, "Fee"),
            original_fee: str_field(&root, "OriginalFee"),
            scan_code_info,
            send_pics_info,
            send_location_info,
            article_url_result,
            refuse_reason: str_field(&root, "RefuseReason"),
            is_recommend_by_friend: str_field(&root, "IsRecommendByFriend"),
            pay_finish_time: str_field(&root, "PayFinishTime"),
            create_order_time: str_field(&root, "CreateOrderTime"),
            desc: str_field(&root, "Desc"),
            free_coin_count: str_field(&root, "FreeCoinCount"),
            pay_coin_count: str_field(&root, "PayCoinCount"),
            refund_free_coin_count: str_field(&root, "RefundFreeCoinCount"),
            refund_pay_coin_count: str_field(&root, "RefundPayCoinCount"),
            order_type: str_field(&root, "OrderType"),
            memo: str_field(&root, "Memo"),
            receipt_info: str_field(&root, "ReceiptInfo"),
            store_uniq_id: str_field(&root, "UniqId"),
            poi_id: str_field(&root, "PoiId"),
            result: str_field(&root, "Result"),
            msg: str_field(&root, "msg"),
            expired_time: long_field(&root, "ExpiredTime"),
            fail_time: long_field(&root, "FailTime"),
            fail_reason: str_field(&root, "FailReason"),
            refill_time: long_field(&root, "RefillTime"),
            refill_reason: str_field(&root, "RefillReason"),
            order_id: str_field(&root, "OrderId"),
            order_status: str_field(&root, "OrderStatus"),
            product_id: str_field(&root, "ProductId"),
            sku_info: str_field(&root, "SkuInfo"),
            device_type: str_field(&root, "DeviceType"),
            device_id: str_field(&root, "DeviceID"),
            session_id: str_field(&root, "SessionID"),
            open_id: str_field(&root, "OpenID"),
            hard_ware,
            op_type: int_field(&root, "OpType"),
            device_status: int_field(&root, "DeviceStatus"),
            success_time: long_field(&root, "SuccTime"),
            reason: str_field(&root, "Reason"),
            delay_time: long_field(&root, "DelayTime"),
            screen_shot: str_field(&root, "ScreenShot"),
            key_standard: str_field(&root, "KeyStandard"),
            key_str: str_field(&root, "KeyStr"),
            country: str_field(&root, "Country"),
            province: str_field(&root, "Province"),
            city: str_field(&root, "City"),
            sex: str_field(&root, "Sex"),
            scene: str_field(&root, "Scene"),
            ext_info: str_field(&root, "ExtInfo"),
            region_code: str_field(&root, "RegionCode"),
            reason_msg: str_field(&root, "ReasonMsg"),
            biz_msg_menu_id: str_field(&root, "bizmsgmenuid"),
            succ_order_id: str_field(&root, "SuccOrderId"),
            fail_order_id: str_field(&root, "FailOrderId"),
            authorize_app_id: str_field(&root, "AuthorizeAppId"),
            source: str_field(&root, "source"),
            fpqqlsh: str_field(&root, "fpqqlsh"),
            nsrsbh: str_field(&root, "nsrsbh"),
            revoke_info: str_field(&root, "RevokeInfo"),
            encrypt: str_field(&root, "Encrypt"),
            subscribe_msg_popup_event,
            subscribe_msg_change_event,
            subscribe_msg_sent_event,
        })
    }

    /// 从加密字符串转换。
    ///
    /// 对应 Java `fromEncryptedXml(String, WxMpConfigStorage, String, String, String)`。
    pub fn from_encrypted_xml(
        encrypted_xml: &str,
        config: &dyn WxMpConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxMpCryptUtil::new(config)?;
        let plain_text = crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)?;
        Self::from_xml(&plain_text)
    }

    /// 解密 `Encrypt` 字段后重新解析消息。
    ///
    /// 对应 Java `decryptField(WxMpConfigStorage, String, String, String)`。
    pub fn decrypt_field(
        &self,
        config: &dyn WxMpConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxMpCryptUtil::new(config)?;
        let cipher = self
            .encrypt
            .as_deref()
            .ok_or_else(|| "消息中不存在 Encrypt 字段".to_string())?;
        let plain_text = crypt_util.decrypt_content(msg_signature, timestamp, nonce, cipher)?;
        Self::from_xml(&plain_text)
    }

    /// 消息类型 getter（对齐 Java `getMsgType()`）。
    pub fn get_msg_type(&self) -> Option<&str> {
        self.msg_type.as_deref()
    }
}
