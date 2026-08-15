//! 企业微信服务商推送消息（xml 格式）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpTpXmlMessage`。Java 用
//! XStream 反射映射 + dom4j 全量树；Rust 复用 `wx_cp_xml_message` 的
//! quick-xml 嵌套树解析（同一线格式语义）。
//!
//! 加密推送（服务商回调以 `Encrypt` 字段包裹密文）对应 Java
//! `fromEncryptedXml`（`WxCpTpCryptUtil`）：Rust 以 `WxCpTpCryptUtil`
//! 表达（从 `WxCpTpConfigStorage` 取 token/encodingAESKey/corpId，
//! Wave 5 C5 从 `WxCpConfigStorage` 升级为服务商配置存储）。

use crate::bean::message::wx_cp_xml_message::{
    double_field, int_field, long_field, node_array, node_field, parse_scan_code_info,
    parse_send_location_info, parse_send_pics_info, parse_tree, str_field,
};
use crate::bean::message::{WxCpXmlApprovalInfo, XmlValue};
use crate::config::WxCpTpConfigStorage;
use crate::util::crypto::WxCpTpCryptUtil;

/// 服务商推送过来的消息（xml 格式）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpTpXmlMessage {
    /// 存放所有 xml 属性和值的 map（对应 Java `allFieldsMap`）。
    pub all_fields_map: Option<std::collections::HashMap<String, XmlValue>>,
    /// 第三方应用的 suiteid。
    pub suite_id: Option<String>,
    /// 消息类型（suite_ticket/change_auth/...）。
    pub info_type: Option<String>,
    /// 时间戳。
    pub time_stamp: Option<String>,
    /// suite_ticket 内容。
    pub suite_ticket: Option<String>,
    /// 授权码。
    pub auth_code: Option<String>,
    /// 授权方企业 corpid。
    pub auth_corp_id: Option<String>,
    /// 变更类型。
    pub change_type: Option<String>,
    /// 变更信息的成员 UserID。
    pub user_id: Option<String>,
    /// 成员部门列表（`Department` 元素数组）。
    pub departments: Vec<i32>,
    /// 主部门。
    pub main_department: Option<i32>,
    /// 所在部门是否为上级（顺序与 Department 逐一对应）。
    pub is_leader_in_dept: Vec<i32>,
    /// 手机号码。
    pub mobile: Option<String>,
    /// 职位信息。
    pub position: Option<String>,
    /// 性别（1 男 2 女）。
    pub gender: Option<i32>,
    /// 邮箱。
    pub email: Option<String>,
    /// 激活状态（1 已激活 2 已禁用 4 未激活）。
    pub status: Option<String>,
    /// 头像 url。
    pub avatar: Option<String>,
    /// 别名。
    pub alias: Option<String>,
    /// 座机。
    pub telephone: Option<String>,
    /// 部门 Id。
    pub id: Option<String>,
    /// 部门名称（或成员名称）。
    pub name: Option<String>,
    /// 父部门 id。
    pub parent_id: Option<String>,
    /// 部门排序。
    pub order: Option<i32>,
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
    /// 服务商企业 corpid。
    pub service_corp_id: Option<String>,
    /// 注册码。
    pub register_code: Option<String>,
    /// 通讯录同步信息。
    pub contact_sync: ContactSync,
    /// 模板 id。
    pub template_id: Option<String>,
    /// 消息创建时间。
    pub create_time: Option<i64>,
    /// 开发者微信号。
    pub to_user_name: Option<String>,
    /// 发送方帐号。
    pub from_user_name: Option<String>,
    /// 消息类型。
    pub msg_type: Option<String>,
    /// 事件类型。
    pub event: Option<String>,
    /// 异步任务信息。
    pub batch_job: BatchJob,
    /// 变更信息的外部联系人 userid。
    pub external_user_id: Option<String>,
    /// 「联系我」方式配置的 state 参数。
    pub state: Option<String>,
    /// 来源。
    pub source: Option<String>,
    /// 客户接替失败的原因。
    pub fail_reason: Option<String>,
    /// 群 ID。
    pub chat_id: Option<String>,
    /// 变更详情。
    pub update_detail: Option<String>,
    /// 加入场景。
    pub join_scene: Option<String>,
    /// 退出场景。
    pub quit_scene: Option<String>,
    /// 成员变更数量。
    pub mem_change_cnt: Option<String>,
    /// 标签类型。
    pub tag_type: Option<String>,
    /// 欢迎语 code。
    pub welcome_code: Option<String>,
    /// 发送方帐号（FromUser）。
    pub from_user: Option<String>,
    /// 文本消息内容。
    pub content: Option<String>,
    /// 消息 id。
    pub msg_id: Option<String>,
    /// 企业应用的 id。
    pub agent_id: Option<String>,
    /// 图片链接。
    pub pic_url: Option<String>,
    /// 媒体 id。
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
    /// 事件 KEY 值。
    pub event_key: Option<String>,
    /// 地理位置纬度（事件）。
    pub latitude: Option<f64>,
    /// 地理位置经度（事件）。
    pub longitude: Option<f64>,
    /// 地理位置精度。
    pub precision: Option<f64>,
    /// 应用类型。
    pub app_type: Option<String>,
    /// 扫码信息。
    pub scan_code_info: crate::bean::message::ScanCodeInfo,
    /// 发送图片信息。
    pub send_pics_info: crate::bean::message::SendPicsInfo,
    /// 发送位置信息。
    pub send_location_info: crate::bean::message::SendLocationInfo,
    /// 审批消息。
    pub approval_info: WxCpXmlApprovalInfo,
    /// 任务 id。
    pub task_id: Option<String>,
    /// 已支付企业的 corpid。
    pub paid_corp_id: Option<String>,
    /// 订单 id。
    pub order_id: Option<String>,
    /// 操作者 userid。
    pub operator_id: Option<String>,
    /// 原订单 id。
    pub old_order_id: Option<String>,
    /// 新订单 id。
    pub new_order_id: Option<String>,
    /// 授权方用户信息。
    pub auth_user_info: AuthUserInfo,
}

impl WxCpTpXmlMessage {
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
        let contact_sync = node_field(&root, "ContactSync")
            .map(|m| ContactSync {
                access_token: str_field(m, "AccessToken"),
                expires_in: int_field(m, "ExpiresIn"),
            })
            .unwrap_or_default();
        let batch_job = node_field(&root, "BatchJob")
            .map(|m| BatchJob {
                job_id: str_field(m, "JobId"),
                job_type: str_field(m, "JobType"),
                err_code: int_field(m, "ErrCode"),
                err_msg: str_field(m, "ErrMsg"),
            })
            .unwrap_or_default();
        let auth_user_info = node_field(&root, "AuthUserInfo")
            .map(|m| AuthUserInfo {
                user_id: str_field(m, "UserId"),
            })
            .unwrap_or_default();
        let approval_info = node_field(&root, "ApprovalInfo")
            .map(WxCpXmlApprovalInfo::from_tree)
            .unwrap_or_default();

        Ok(Self {
            all_fields_map: Some(root.clone()),
            suite_id: str_field(&root, "SuiteId"),
            info_type: str_field(&root, "InfoType"),
            time_stamp: str_field(&root, "TimeStamp"),
            suite_ticket: str_field(&root, "SuiteTicket"),
            auth_code: str_field(&root, "AuthCode"),
            auth_corp_id: str_field(&root, "AuthCorpId"),
            change_type: str_field(&root, "ChangeType"),
            user_id: str_field(&root, "UserID"),
            departments: node_array(&root, "Department")
                .into_iter()
                .filter_map(XmlValue::as_scalar)
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            main_department: int_field(&root, "MainDepartment"),
            is_leader_in_dept: node_array(&root, "IsLeaderInDept")
                .into_iter()
                .filter_map(XmlValue::as_scalar)
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            mobile: str_field(&root, "Mobile"),
            position: str_field(&root, "Position"),
            gender: int_field(&root, "Gender"),
            email: str_field(&root, "Email"),
            status: str_field(&root, "Status"),
            avatar: str_field(&root, "Avatar"),
            alias: str_field(&root, "Alias"),
            telephone: str_field(&root, "Telephone"),
            id: str_field(&root, "Id"),
            name: str_field(&root, "Name"),
            parent_id: str_field(&root, "ParentId"),
            order: int_field(&root, "Order"),
            tag_id: str_field(&root, "TagId"),
            add_user_items: str_field(&root, "AddUserItems"),
            del_user_items: str_field(&root, "DelUserItems"),
            add_party_items: str_field(&root, "AddPartyItems"),
            del_party_items: str_field(&root, "DelPartyItems"),
            service_corp_id: str_field(&root, "ServiceCorpId"),
            register_code: str_field(&root, "RegisterCode"),
            contact_sync,
            template_id: str_field(&root, "TemplateId"),
            create_time: long_field(&root, "CreateTime"),
            to_user_name: str_field(&root, "ToUserName"),
            from_user_name: str_field(&root, "FromUserName"),
            msg_type: str_field(&root, "MsgType"),
            event: str_field(&root, "Event"),
            batch_job,
            external_user_id: str_field(&root, "ExternalUserID"),
            state: str_field(&root, "State"),
            source: str_field(&root, "Source"),
            fail_reason: str_field(&root, "FailReason"),
            chat_id: str_field(&root, "ChatId"),
            update_detail: str_field(&root, "UpdateDetail"),
            join_scene: str_field(&root, "JoinScene"),
            quit_scene: str_field(&root, "QuitScene"),
            mem_change_cnt: str_field(&root, "MemChangeCnt"),
            tag_type: str_field(&root, "TagType"),
            welcome_code: str_field(&root, "WelcomeCode"),
            from_user: str_field(&root, "FromUser"),
            content: str_field(&root, "Content"),
            msg_id: str_field(&root, "MsgId"),
            agent_id: str_field(&root, "AgentID"),
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
            event_key: str_field(&root, "EventKey"),
            latitude: double_field(&root, "Latitude"),
            longitude: double_field(&root, "Longitude"),
            precision: double_field(&root, "Precision"),
            app_type: str_field(&root, "AppType"),
            scan_code_info: node_field(&root, "ScanCodeInfo")
                .map(parse_scan_code_info)
                .unwrap_or_default(),
            send_pics_info: node_field(&root, "SendPicsInfo")
                .map(parse_send_pics_info)
                .unwrap_or_default(),
            send_location_info: node_field(&root, "SendLocationInfo")
                .map(parse_send_location_info)
                .unwrap_or_default(),
            approval_info,
            task_id: str_field(&root, "TaskId"),
            paid_corp_id: str_field(&root, "PaidCorpId"),
            order_id: str_field(&root, "OrderId"),
            operator_id: str_field(&root, "OperatorId"),
            old_order_id: str_field(&root, "OldOrderId"),
            new_order_id: str_field(&root, "NewOrderId"),
            auth_user_info,
        })
    }

    /// 从加密字符串转换（对应 Java `fromEncryptedXml`：以
    /// `WxCpTpCryptUtil`（服务商配置存储 token/encodingAESKey/corpId）
    /// 验证签名后解密）。
    pub fn from_encrypted_xml(
        encrypted_xml: &str,
        config: &dyn WxCpTpConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxCpTpCryptUtil::new(config)?;
        let plain_text = crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)?;
        Self::from_xml(&plain_text)
    }
}

/// 通讯录同步信息（对应 Java `WxCpTpXmlMessage.ContactSync`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContactSync {
    /// 通讯录同步 access_token。
    pub access_token: Option<String>,
    /// 过期时间（秒）。
    pub expires_in: Option<i32>,
}

/// 授权方用户信息（对应 Java `WxCpTpXmlMessage.AuthUserInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AuthUserInfo {
    /// 授权方成员 userid。
    pub user_id: Option<String>,
}

/// 异步任务信息（对应 Java `WxCpTpXmlMessage.BatchJob`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchJob {
    /// 异步任务 id。
    pub job_id: Option<String>,
    /// 任务类型。
    pub job_type: Option<String>,
    /// 返回码。
    pub err_code: Option<i32>,
    /// 错误信息。
    pub err_msg: Option<String>,
}
