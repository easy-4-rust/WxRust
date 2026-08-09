//! 企业微信推送消息（xml 格式）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlMessage`。Java 用
//! XStream 反射映射 + dom4j 全量树；Rust 以 quick-xml 解析为嵌套树后
//! 按元素名提取字段（同一线格式语义，参考 mp 的 `WxMpXmlMessage`）。
//!
//! 解析流程（对应 Java `fromXml`）：
//! 1. 修正微信变态的消息内容格式（`</PicList><PicList>` 相邻闭合/开启合并）；
//! 2. dom4j 树解析（`XmlValue` 嵌套树，同名元素合并为数组）；
//! 3. 按 XStream 字段声明序从根节点提取字段（String 包 CDATA、数值裸值）。
//!
//! 加密推送（企业微信以 `Encrypt` 字段包裹密文）：`from_encrypted_xml`
//! 经 `util::crypto::WxCpCryptUtils`（对应 Java `WxCpCryptUtil`）验签解密
//! 后重新解析，与 Java `fromEncryptedXml` 语义一致。

use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::bean::message::WxCpXmlApprovalInfo;
use crate::config::WxCpConfigStorage;
use crate::util::crypto::WxCpCryptUtils;

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
pub(crate) fn parse_tree(xml: &str) -> Result<XmlValue, String> {
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
pub(crate) fn parse_element_body(reader: &mut Reader<&[u8]>) -> Result<XmlValue, String> {
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
pub(crate) fn insert_field(fields: &mut HashMap<String, XmlValue>, name: String, value: XmlValue) {
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
pub(crate) fn str_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<String> {
    map.get(name)
        .and_then(XmlValue::as_scalar)
        .map(str::to_string)
}

/// 从节点 map 取 Long 字段。
pub(crate) fn long_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<i64> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取 Integer 字段。
pub(crate) fn int_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<i32> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取 Double 字段。
pub(crate) fn double_field(map: &HashMap<String, XmlValue>, name: &str) -> Option<f64> {
    str_field(map, name).and_then(|s| s.trim().parse().ok())
}

/// 从节点 map 取嵌套节点。
pub(crate) fn node_field<'a>(
    map: &'a HashMap<String, XmlValue>,
    name: &str,
) -> Option<&'a HashMap<String, XmlValue>> {
    map.get(name).and_then(XmlValue::as_node)
}

/// 从节点 map 取同名节点数组（单元素包装为单元素数组，对应 Java List 语义）。
pub(crate) fn node_array<'a>(map: &'a HashMap<String, XmlValue>, name: &str) -> Vec<&'a XmlValue> {
    match map.get(name) {
        Some(XmlValue::Array(v)) => v.iter().collect(),
        Some(node @ XmlValue::Node(_)) => vec![node],
        _ => Vec::new(),
    }
}

/// 集合字段取元素：兼容两种线格式形态。
///
/// - 直接重复形态：`<item>` 元素重复出现（微信官方文档形态）；
/// - 包装形态：`<wrapper><item>…</item>…</wrapper>`（XStream 对 List 字段的
///   wrapper + 类别名 item 语义，如 `SelectedItems`/`ApprovalNodes`）。
pub(crate) fn list_items<'a>(
    map: &'a HashMap<String, XmlValue>,
    wrappers: &[&str],
    items: &[&str],
) -> Vec<&'a XmlValue> {
    // 直接重复形态
    for it in items {
        if let Some(XmlValue::Array(arr)) = map.get(*it) {
            return arr.iter().collect();
        }
    }
    // 包装形态
    for w in wrappers {
        if let Some(node) = map.get(*w).and_then(XmlValue::as_node) {
            let mut out = Vec::new();
            for it in items {
                if let Some(v) = node.get(*it) {
                    match v {
                        XmlValue::Array(arr) => out.extend(arr.iter()),
                        other => out.push(other),
                    }
                }
            }
            if !out.is_empty() {
                return out;
            }
        }
    }
    Vec::new()
}

/// 解析扫码信息节点。
pub(crate) fn parse_scan_code_info(map: &HashMap<String, XmlValue>) -> ScanCodeInfo {
    ScanCodeInfo {
        scan_type: str_field(map, "ScanType"),
        scan_result: str_field(map, "ScanResult"),
    }
}

/// 解析发送图片信息节点。
///
/// 微信推送存在 `<PicList>` 相邻重复（Java 先以
/// `xml.replace("</PicList><PicList>", "")` 合并，XStream 亦接受多个
/// PicList 元素），此处两种形态都支持。
pub(crate) fn parse_send_pics_info(map: &HashMap<String, XmlValue>) -> SendPicsInfo {
    let pic_list = match map.get("PicList") {
        Some(XmlValue::Array(arr)) => arr
            .iter()
            .filter_map(XmlValue::as_node)
            .flat_map(|pl| node_array(pl, "item"))
            .filter_map(XmlValue::as_node)
            .map(|item| crate::bean::message::SendPicsItem {
                pic_md5_sum: str_field(item, "PicMd5Sum"),
            })
            .collect(),
        Some(node @ XmlValue::Node(_)) => node_array(node.as_node().unwrap(), "item")
            .into_iter()
            .filter_map(XmlValue::as_node)
            .map(|item| crate::bean::message::SendPicsItem {
                pic_md5_sum: str_field(item, "PicMd5Sum"),
            })
            .collect(),
        _ => Vec::new(),
    };
    SendPicsInfo {
        count: long_field(map, "Count"),
        pic_list,
    }
}

/// 解析发送位置信息节点。
pub(crate) fn parse_send_location_info(map: &HashMap<String, XmlValue>) -> SendLocationInfo {
    SendLocationInfo {
        location_x: str_field(map, "Location_X"),
        location_y: str_field(map, "Location_Y"),
        scale: str_field(map, "Scale"),
        label: str_field(map, "Label"),
        poi_name: str_field(map, "Poiname"),
    }
}

/// 解析选中项节点（`SelectedItems` 包装 + `SelectedItem` 元素，对应 Java
/// `@XStreamAlias("SelectedItems") List<SelectedItem>`）。
pub(crate) fn parse_selected_items(map: &HashMap<String, XmlValue>) -> Vec<SelectedItem> {
    list_items(map, &["SelectedItems", "selectedItems"], &["SelectedItem"])
        .into_iter()
        .filter_map(XmlValue::as_node)
        .map(|item| SelectedItem {
            question_key: str_field(item, "QuestionKey"),
            // OptionIds 包装 + OptionId 元素（List<String> 的 XStream 形态）
            option_ids: list_items(item, &["OptionIds", "optionIds"], &["OptionId"])
                .into_iter()
                .filter_map(XmlValue::as_scalar)
                .map(str::to_string)
                .collect(),
        })
        .collect()
}

/// 微信推送过来的消息（xml 格式）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlMessage {
    /// 存放所有 xml 属性和值的 map（对应 Java `allFieldsMap`）。
    pub all_fields_map: Option<HashMap<String, XmlValue>>,
    /// 企业应用的 id（对应 Java `AgentID`）。
    pub agent_id: Option<String>,
    /// 开发者微信号（对应 Java `ToUserName`）。
    pub to_user_name: Option<String>,
    /// 发送方帐号（对应 Java `FromUserName`）。
    pub from_user_name: Option<String>,
    /// 消息创建时间（整型）。
    pub create_time: Option<i64>,
    /// 消息类型（text/image/voice/video/location/link/event）。
    pub msg_type: Option<String>,
    /// 文本消息内容。
    pub content: Option<String>,
    /// 消息 id。
    pub msg_id: Option<String>,
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
    /// 变更详情。
    pub update_detail: Option<String>,
    /// 加入场景。
    pub join_scene: Option<String>,
    /// 退出场景。
    pub quit_scene: Option<String>,
    /// 成员变更数量。
    pub mem_change_cnt: Option<String>,
    /// 成员变更列表。
    pub mem_change_list: Option<String>,
    /// 最新版本。
    pub last_mem_ver: Option<String>,
    /// 当前版本。
    pub cur_mem_ver: Option<String>,
    /// 来源。
    pub source: Option<String>,
    /// 客户群策略 id。
    pub strategy_id: Option<String>,
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
    /// 任务 id。
    pub task_id: Option<String>,
    /// 卡券类型。
    pub card_type: Option<String>,
    /// 卡券响应码。
    pub response_code: Option<String>,
    /// 选中项列表。
    pub selected_items: Vec<SelectedItem>,
    /// 异步任务 id。
    pub job_id: Option<String>,
    /// 微信客服拉取消息的校验 token。
    pub token: Option<String>,
    /// 有新消息的客服账号。
    pub open_kf_id: Option<String>,
    /// 新增授权的客服账号列表。
    pub auth_add_open_kf_id: Option<String>,
    /// 取消授权的客服账号列表。
    pub auth_del_open_kf_id: Option<String>,
    /// 失效的获客链接 ID。
    pub link_id: Option<String>,
    /// 智能机器人 ID。
    pub robot_id: Option<String>,
    /// 智能机器人会话 ID。
    pub session_id: Option<String>,
    /// 通讯录变更类型。
    pub change_type: Option<String>,
    /// 变更信息的成员 UserID。
    pub user_id: Option<String>,
    /// 变更信息的外部联系人 userid。
    pub external_user_id: Option<String>,
    /// 「联系我」方式配置的 state 参数。
    pub state: Option<String>,
    /// 欢迎语 code。
    pub welcome_code: Option<String>,
    /// 新的 UserID。
    pub new_user_id: Option<String>,
    /// 成员名称（或部门名称）。
    pub name: Option<String>,
    /// 成员部门列表（`Department` 元素数组）。
    pub departments: Vec<i64>,
    /// 主部门。
    pub main_department: Option<i64>,
    /// 手机号码。
    pub mobile: Option<String>,
    /// 职位信息。
    pub position: Option<String>,
    /// 群 ID。
    pub chat_id: Option<String>,
    /// 性别（1 男 2 女）。
    pub gender: Option<i32>,
    /// 邮箱。
    pub email: Option<String>,
    /// 企业邮箱。
    pub biz_mail: Option<String>,
    /// 头像 url。
    pub avatar: Option<String>,
    /// 英文名。
    pub english_name: Option<String>,
    /// 是否上级（0 普通 1 上级）。
    pub is_leader: Option<i32>,
    /// 所在部门是否为上级（顺序与 Department 逐一对应）。
    pub is_leader_in_dept: Vec<i32>,
    /// 座机。
    pub telephone: Option<String>,
    /// 地址。
    pub address: Option<String>,
    /// 日程 ID。
    pub schedule_id: Option<String>,
    /// 日历 ID。
    pub cal_id: Option<String>,
    /// 会议室 ID。
    pub meeting_room_id: Option<String>,
    /// 会议室预定 id。
    pub booking_id: Option<String>,
    /// 扩展属性。
    pub ext_attrs: ExtAttr,
    /// 部门 Id（或客户联系回调标签/标签组 id）。
    pub id: Option<String>,
    /// 父部门 id。
    pub parent_id: Option<String>,
    /// 部门排序。
    pub order: Option<String>,
    /// 标签 Id。
    pub tag_id: Option<String>,
    /// 标签中新增的成员 userid 列表（逗号分隔）。
    pub add_user_items: Option<String>,
    /// 标签中删除的成员 userid 列表（逗号分隔）。
    pub del_user_items: Option<String>,
    /// 标签中新增的部门 id 列表（逗号分隔）。
    pub add_party_items: Option<String>,
    /// 标签中删除的部门 id 列表（逗号分隔）。
    pub del_party_items: Option<String>,
    /// 客户接替失败的原因。
    pub fail_reason: Option<String>,
    /// 标签类型。
    pub tag_type: Option<String>,
    /// 群发的结果 / 激活状态 / 直播状态（多场景共用字段）。
    pub status: Option<String>,
    /// 直播 ID。
    pub living_id: Option<String>,
    /// 粉丝数。
    pub total_count: Option<i32>,
    /// 过滤后准备发送的粉丝数。
    pub filter_count: Option<i32>,
    /// 发送成功的粉丝数。
    pub sent_count: Option<i32>,
    /// 发送失败的粉丝数。
    pub error_count: Option<i32>,
    /// 扫码信息。
    pub scan_code_info: ScanCodeInfo,
    /// 发送图片信息。
    pub send_pics_info: SendPicsInfo,
    /// 发送位置信息。
    pub send_location_info: SendLocationInfo,
    /// 审批消息。
    pub approval_info: WxCpXmlApprovalInfo,
}

impl WxCpXmlMessage {
    /// 从 xml 字符串解析消息（对应 Java `fromXml(String)`）。
    ///
    /// 先修正微信变态的消息内容格式（`</PicList><PicList>` 相邻闭合/开启），
    /// 再解析字段与全量树。
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

        Ok(Self {
            all_fields_map: Some(root.clone()),
            agent_id: str_field(&root, "AgentID"),
            to_user_name: str_field(&root, "ToUserName"),
            from_user_name: str_field(&root, "FromUserName"),
            create_time: long_field(&root, "CreateTime"),
            msg_type: str_field(&root, "MsgType"),
            content: str_field(&root, "Content"),
            msg_id: str_field(&root, "MsgId"),
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
            update_detail: str_field(&root, "UpdateDetail"),
            join_scene: str_field(&root, "JoinScene"),
            quit_scene: str_field(&root, "QuitScene"),
            mem_change_cnt: str_field(&root, "MemChangeCnt"),
            mem_change_list: str_field(&root, "MemChangeList"),
            last_mem_ver: str_field(&root, "LastMemVer"),
            cur_mem_ver: str_field(&root, "CurMemVer"),
            source: str_field(&root, "Source"),
            strategy_id: str_field(&root, "StrategyId"),
            event_key: str_field(&root, "EventKey"),
            ticket: str_field(&root, "Ticket"),
            latitude: double_field(&root, "Latitude"),
            longitude: double_field(&root, "Longitude"),
            precision: double_field(&root, "Precision"),
            recognition: str_field(&root, "Recognition"),
            task_id: str_field(&root, "TaskId"),
            card_type: str_field(&root, "CardType"),
            response_code: str_field(&root, "ResponseCode"),
            selected_items: parse_selected_items(&root),
            job_id: str_field(&root, "JobId"),
            token: str_field(&root, "Token"),
            open_kf_id: str_field(&root, "OpenKfId"),
            auth_add_open_kf_id: str_field(&root, "AuthAddOpenKfId"),
            auth_del_open_kf_id: str_field(&root, "AuthDelOpenKfId"),
            link_id: str_field(&root, "LinkId"),
            robot_id: str_field(&root, "RobotId"),
            session_id: str_field(&root, "SessionId"),
            change_type: str_field(&root, "ChangeType"),
            user_id: str_field(&root, "UserID"),
            external_user_id: str_field(&root, "ExternalUserID"),
            state: str_field(&root, "State"),
            welcome_code: str_field(&root, "WelcomeCode"),
            new_user_id: str_field(&root, "NewUserID"),
            name: str_field(&root, "Name"),
            departments: node_array(&root, "Department")
                .into_iter()
                .filter_map(XmlValue::as_scalar)
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            main_department: long_field(&root, "MainDepartment"),
            mobile: str_field(&root, "Mobile"),
            position: str_field(&root, "Position"),
            chat_id: str_field(&root, "ChatId"),
            gender: int_field(&root, "Gender"),
            email: str_field(&root, "Email"),
            biz_mail: str_field(&root, "BizMail"),
            avatar: str_field(&root, "Avatar"),
            english_name: str_field(&root, "EnglishName"),
            is_leader: int_field(&root, "IsLeader"),
            is_leader_in_dept: node_array(&root, "IsLeaderInDept")
                .into_iter()
                .filter_map(XmlValue::as_scalar)
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            telephone: str_field(&root, "Telephone"),
            address: str_field(&root, "Address"),
            schedule_id: str_field(&root, "ScheduleId"),
            cal_id: str_field(&root, "CalId"),
            meeting_room_id: str_field(&root, "MeetingRoomId"),
            booking_id: str_field(&root, "BookingId"),
            ext_attrs: parse_ext_attrs(&root),
            id: str_field(&root, "Id"),
            parent_id: str_field(&root, "ParentId"),
            order: str_field(&root, "Order"),
            tag_id: str_field(&root, "TagId"),
            add_user_items: str_field(&root, "AddUserItems"),
            del_user_items: str_field(&root, "DelUserItems"),
            add_party_items: str_field(&root, "AddPartyItems"),
            del_party_items: str_field(&root, "DelPartyItems"),
            fail_reason: str_field(&root, "FailReason"),
            tag_type: str_field(&root, "TagType"),
            status: str_field(&root, "Status"),
            living_id: str_field(&root, "LivingId"),
            total_count: int_field(&root, "TotalCount"),
            filter_count: int_field(&root, "FilterCount"),
            sent_count: int_field(&root, "SentCount"),
            error_count: int_field(&root, "ErrorCount"),
            scan_code_info: node_field(&root, "ScanCodeInfo")
                .map(parse_scan_code_info)
                .unwrap_or_default(),
            send_pics_info: node_field(&root, "SendPicsInfo")
                .map(parse_send_pics_info)
                .unwrap_or_default(),
            send_location_info: node_field(&root, "SendLocationInfo")
                .map(parse_send_location_info)
                .unwrap_or_default(),
            approval_info: node_field(&root, "ApprovalInfo")
                .map(WxCpXmlApprovalInfo::from_tree)
                .unwrap_or_default(),
        })
    }

    /// 从 xml 字符串解析消息并携带 agentId（对应 Java
    /// `fromXml(String, String)`）。
    pub fn from_xml_with_agent_id(xml: &str, agent_id: &str) -> Result<Self, String> {
        // 修改微信变态的消息内容格式，方便解析（与 Java 逐字一致）
        let xml = xml.replace("</PicList><PicList>", "");
        let mut message = Self::from_xml(&xml)?;
        message.agent_id = Some(agent_id.to_string());
        Ok(message)
    }

    /// 从加密字符串转换（对应 Java `fromEncryptedXml(String,
    /// WxCpConfigStorage, String, String, String)`）。
    ///
    /// 流程：先解析外层 XML 取出 `AgentID`（Java 语义），再经
    /// `WxCpCryptUtils` 验签解密得到明文，最后按明文重新解析。
    pub fn from_encrypted_xml(
        encrypted_xml: &str,
        config: &dyn WxCpConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxCpCryptUtils::new(config)?;
        let outer = Self::from_xml(encrypted_xml)?;
        let agent_id = outer.agent_id.clone();
        let plain_text = crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)?;
        match agent_id {
            Some(agent_id) => Self::from_xml_with_agent_id(&plain_text, &agent_id),
            None => Self::from_xml(&plain_text),
        }
    }
}

/// 解析扩展属性节点（`ExtAttr` 下重复的 `Item` 元素，对应 Java
/// `XStreamImplicit(itemFieldName = "Item")`）。
fn parse_ext_attrs(map: &HashMap<String, XmlValue>) -> ExtAttr {
    let items = match map.get("ExtAttr") {
        Some(node @ XmlValue::Node(_)) => node_array(node.as_node().unwrap(), "Item")
            .into_iter()
            .filter_map(XmlValue::as_node)
            .map(|item| ExtAttrItem {
                name: str_field(item, "Name"),
                value: str_field(item, "Value"),
            })
            .collect(),
        Some(XmlValue::Array(arr)) => arr
            .iter()
            .filter_map(XmlValue::as_node)
            .flat_map(|n| node_array(n, "Item"))
            .filter_map(XmlValue::as_node)
            .map(|item| ExtAttrItem {
                name: str_field(item, "Name"),
                value: str_field(item, "Value"),
            })
            .collect(),
        _ => Vec::new(),
    };
    ExtAttr { items }
}

/// 扫码信息（对应 Java `WxCpXmlMessage.ScanCodeInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScanCodeInfo {
    /// 扫描类型，一般是 qrcode。
    pub scan_type: Option<String>,
    /// 扫描结果，即二维码对应的字符串信息。
    pub scan_result: Option<String>,
}

/// 扩展属性（对应 Java `WxCpXmlMessage.ExtAttr`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExtAttr {
    /// 属性项列表（`Item` 元素数组）。
    pub items: Vec<ExtAttrItem>,
}

/// 扩展属性项（对应 Java `WxCpXmlMessage.ExtAttr.Item`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExtAttrItem {
    /// 属性名。
    pub name: Option<String>,
    /// 属性值。
    pub value: Option<String>,
}

/// 发送图片信息（对应 Java `WxCpXmlMessage.SendPicsInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendPicsInfo {
    /// 图片数量。
    pub count: Option<i64>,
    /// 图片列表。
    pub pic_list: Vec<SendPicsItem>,
}

/// 发送图片项（对应 Java `WxCpXmlMessage.SendPicsInfo.Item`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendPicsItem {
    /// 图片的 MD5 值。
    pub pic_md5_sum: Option<String>,
}

/// 发送位置信息（对应 Java `WxCpXmlMessage.SendLocationInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendLocationInfo {
    /// 地理位置纬度。
    pub location_x: Option<String>,
    /// 地理位置经度。
    pub location_y: Option<String>,
    /// 地图缩放大小。
    pub scale: Option<String>,
    /// 地理位置信息。
    pub label: Option<String>,
    /// 朋友圈 POI 的名字。
    pub poi_name: Option<String>,
}

/// 选中项（对应 Java `WxCpXmlMessage.SelectedItem`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectedItem {
    /// 问题 key。
    pub question_key: Option<String>,
    /// 选项 id 列表。
    pub option_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `WxCpXmlMessageTest.testFromXml` 线格式 golden（元素/CDATA/数值原样）。
    #[test]
    fn from_xml_golden() {
        let xml = concat!(
            "<xml>",
            "<ToUserName><![CDATA[toUser]]></ToUserName>",
            "<FromUserName><![CDATA[fromUser]]></FromUserName> ",
            "<CreateTime>1348831860</CreateTime>",
            "<MsgType><![CDATA[text]]></MsgType>",
            "<Content><![CDATA[this is a test]]></Content>",
            "<MsgId>1234567890123456</MsgId>",
            "<PicUrl><![CDATA[this is a url]]></PicUrl>",
            "<MediaId><![CDATA[media_id]]></MediaId>",
            "<Format><![CDATA[Format]]></Format>",
            "<ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>",
            "<Location_X>23.134521</Location_X>",
            "<Location_Y>113.358803</Location_Y>",
            "<Scale>20</Scale>",
            "<Label><![CDATA[位置信息]]></Label>",
            "<Description><![CDATA[公众平台官网链接]]></Description>",
            "<Url><![CDATA[url]]></Url>",
            "<Title><![CDATA[公众平台官网链接]]></Title>",
            "<Event><![CDATA[subscribe]]></Event>",
            "<EventKey><![CDATA[qrscene_123123]]></EventKey>",
            "<Ticket><![CDATA[TICKET]]></Ticket>",
            "<Latitude>23.137466</Latitude>",
            "<Longitude>113.352425</Longitude>",
            "<Precision>119.385040</Precision>",
            "<ScanCodeInfo>",
            " <ScanType><![CDATA[qrcode]]></ScanType>",
            " <ScanResult><![CDATA[1]]></ScanResult>",
            "</ScanCodeInfo>",
            "<SendPicsInfo>",
            " <Count>1</Count>\n",
            " <PicList>",
            "  <item>",
            "   <PicMd5Sum><![CDATA[1b5f7c23b5bf75682a53e7b6d163e185]]></PicMd5Sum>",
            "  </item>",
            " </PicList>",
            "</SendPicsInfo>",
            "<SendLocationInfo>",
            "  <Location_X><![CDATA[23]]></Location_X>\n",
            "  <Location_Y><![CDATA[113]]></Location_Y>\n",
            "  <Scale><![CDATA[15]]></Scale>\n",
            "  <Label><![CDATA[ 广州市海珠区客村艺苑路 106号]]></Label>\n",
            "  <Poiname><![CDATA[wo de poi]]></Poiname>\n",
            "</SendLocationInfo>",
            "</xml>"
        );
        let m = WxCpXmlMessage::from_xml(xml).unwrap();
        assert_eq!(m.to_user_name.as_deref(), Some("toUser"));
        assert_eq!(m.from_user_name.as_deref(), Some("fromUser"));
        assert_eq!(m.create_time, Some(1348831860));
        assert_eq!(m.msg_type.as_deref(), Some("text"));
        assert_eq!(m.content.as_deref(), Some("this is a test"));
        assert_eq!(m.msg_id.as_deref(), Some("1234567890123456"));
        assert_eq!(m.media_id.as_deref(), Some("media_id"));
        assert_eq!(m.location_x, Some(23.134521));
        assert_eq!(m.scan_code_info.scan_type.as_deref(), Some("qrcode"));
        assert_eq!(m.send_pics_info.count, Some(1));
        assert_eq!(
            m.send_pics_info.pic_list[0].pic_md5_sum.as_deref(),
            Some("1b5f7c23b5bf75682a53e7b6d163e185")
        );
        assert_eq!(m.send_location_info.poi_name.as_deref(), Some("wo de poi"));
    }

    /// Java `WxCpXmlMessageTest.testSendPicsInfo`：`</PicList><PicList>` 合并修复。
    #[test]
    fn from_xml_pic_list_merge_golden() {
        let xml = concat!(
            "<xml>",
            "<ToUserName><![CDATA[wx45a0972125658be9]]></ToUserName>",
            "<FromUserName><![CDATA[xiaohe]]></FromUserName>",
            "<CreateTime>1502012364</CreateTime>",
            "<MsgType><![CDATA[event]]></MsgType>",
            "<AgentID>1000004</AgentID>",
            "<Event><![CDATA[pic_weixin]]></Event>",
            "<EventKey><![CDATA[faceSimilarity]]></EventKey>",
            "<SendPicsInfo>",
            "<PicList><item><PicMd5Sum><![CDATA[aef52ae501537e552725c5d7f99c1741]]></PicMd5Sum></item></PicList>",
            "<PicList><item><PicMd5Sum><![CDATA[c4564632a4fab91378c39bea6aad6f9e]]></PicMd5Sum></item></PicList>",
            "<Count>2</Count>",
            "</SendPicsInfo>",
            "</xml>"
        );
        let m = WxCpXmlMessage::from_xml(xml).unwrap();
        assert_eq!(m.agent_id.as_deref(), Some("1000004"));
        assert_eq!(m.send_pics_info.count, Some(2));
        assert_eq!(m.send_pics_info.pic_list.len(), 2);
    }

    /// Java `WxCpXmlMessageTest.testOpenApprovalChange`：审批信息解析。
    #[test]
    fn from_xml_approval_golden() {
        let xml = concat!(
            "<xml>\n",
            " <ToUserName><![CDATA[wwddddccc7775555aaa]]></ToUserName>\n",
            "  <FromUserName><![CDATA[sys]]></FromUserName>\n",
            "  <CreateTime>1527838022</CreateTime>\n",
            "  <MsgType><![CDATA[event]]></MsgType>\n",
            "  <Event><![CDATA[open_approval_change]]></Event>\n",
            "  <AgentID>1</AgentID>\n",
            "  <ApprovalInfo>\n",
            "    <ThirdNo><![CDATA[201806010001]]></ThirdNo>\n",
            "    <OpenSpName><![CDATA[付款]]></OpenSpName>\n",
            "    <OpenTemplateId><![CDATA[1234567890]]></OpenTemplateId>\n",
            "    <OpenSpStatus>1</OpenSpStatus>\n",
            "    <ApplyTime>1527837645</ApplyTime>\n",
            "    <ApplyUserName><![CDATA[xiaoming]]></ApplyUserName>\n",
            "    <ApplyUserId><![CDATA[1]]></ApplyUserId>\n",
            "    <ApplyUserParty><![CDATA[产品部]]></ApplyUserParty>\n",
            "    <ApplyUserImage><![CDATA[http://www.qq.com/xxx.png]]></ApplyUserImage>\n",
            "    <ApprovalNodes>\n",
            "      <ApprovalNode>\n",
            "        <NodeStatus>1</NodeStatus>\n",
            "        <NodeAttr>1</NodeAttr>\n",
            "        <NodeType>1</NodeType>\n",
            "        <Items>\n",
            "          <Item>\n",
            "            <ItemName><![CDATA[xiaohong]]></ItemName>\n",
            "            <ItemUserId><![CDATA[2]]></ItemUserId>\n",
            "            <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>\n",
            "            <ItemStatus>1</ItemStatus>\n",
            "            <ItemSpeech><![CDATA[]]></ItemSpeech>\n",
            "            <ItemOpTime>0</ItemOpTime>\n",
            "          </Item>\n",
            "        </Items>\n",
            "      </ApprovalNode>\n",
            "      <ApprovalNode>\n",
            "        <NodeStatus>1</NodeStatus>\n",
            "        <NodeAttr>1</NodeAttr>\n",
            "        <NodeType>1</NodeType>\n",
            "        <Items>\n",
            "          <Item>\n",
            "            <ItemName><![CDATA[xiaohong]]></ItemName>\n",
            "            <ItemUserId><![CDATA[2]]></ItemUserId>\n",
            "            <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>\n",
            "            <ItemStatus>1</ItemStatus>\n",
            "            <ItemSpeech><![CDATA[]]></ItemSpeech>\n",
            "            <ItemOpTime>0</ItemOpTime>\n",
            "          </Item>\n",
            "          <Item>\n",
            "            <ItemName><![CDATA[xiaohong]]></ItemName>\n",
            "            <ItemUserId><![CDATA[2]]></ItemUserId>\n",
            "            <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>\n",
            "            <ItemStatus>1</ItemStatus>\n",
            "            <ItemSpeech><![CDATA[]]></ItemSpeech>\n",
            "            <ItemOpTime>0</ItemOpTime>\n",
            "          </Item>\n",
            "        </Items>\n",
            "      </ApprovalNode>\n",
            "    </ApprovalNodes>\n",
            "    <NotifyNodes>\n",
            "      <NotifyNode>\n",
            "        <ItemName><![CDATA[xiaogang]]></ItemName>\n",
            "        <ItemUserId><![CDATA[3]]></ItemUserId>\n",
            "        <ItemImage><![CDATA[http://www.qq.com/xxx.png]]></ItemImage>\n",
            "      </NotifyNode>\n",
            "    </NotifyNodes>\n",
            "    <approverstep>0</approverstep>\n",
            "  </ApprovalInfo>\n",
            "</xml>\n"
        );
        let m = WxCpXmlMessage::from_xml(xml).unwrap();
        let approval = &m.approval_info;
        assert_eq!(approval.third_no.as_deref(), Some("201806010001"));
        assert_eq!(approval.open_sp_name.as_deref(), Some("付款"));
        assert_eq!(approval.open_sp_status, Some(1));
        assert_eq!(approval.approval_nodes.len(), 2);
        assert_eq!(approval.approval_nodes[0].items.len(), 1);
        assert_eq!(
            approval.approval_nodes[0].items[0].item_name.as_deref(),
            Some("xiaohong")
        );
        assert_eq!(approval.notify_nodes.len(), 1);
        assert_eq!(approval.notify_nodes[0].item_user_id.as_deref(), Some("3"));
    }
}
