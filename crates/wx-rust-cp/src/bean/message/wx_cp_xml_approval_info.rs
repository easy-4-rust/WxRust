//! 审批消息。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpXmlApprovalInfo`。Java
//! 以 XStream 反射映射 + `@XStreamAlias` 声明；Rust 复用
//! `wx_cp_xml_message` 的 XML 树解析（同一线格式语义）。

use std::collections::HashMap;

use crate::bean::message::XmlValue;
use crate::bean::message::wx_cp_xml_message::{
    int_field, list_items, long_field, node_field, str_field,
};

/// 审批消息（对应 Java `WxCpXmlApprovalInfo`，XStreamAlias `ApprovalInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpXmlApprovalInfo {
    /// 审批单号。
    pub third_no: Option<String>,
    /// 审批模板名称。
    pub open_sp_name: Option<String>,
    /// 审批模板 id。
    pub open_template_id: Option<String>,
    /// 审批状态（1 审批中 2 已通过 3 已驳回 4 已撤销 10 已支付）。
    pub open_sp_status: Option<i32>,
    /// 提交人姓名。
    pub apply_user_name: Option<String>,
    /// 提交人 userid。
    pub apply_user_id: Option<String>,
    /// 提交人所在部门。
    pub apply_user_party: Option<String>,
    /// 提交人头像。
    pub apply_user_image: Option<String>,
    /// 审批流程节点状态（1 审批中 2 已批准 3 已驳回 4 已转审）。
    pub approver_step: Option<i32>,
    /// 审批流程信息（`ApprovalNodes` 下重复 `ApprovalNode`）。
    pub approval_nodes: Vec<ApprovalNode>,
    /// 抄送人列表（`NotifyNodes` 下重复 `NotifyNode`）。
    pub notify_nodes: Vec<NotifyNode>,
    /// 审批单号（状态通知事件）。
    pub sp_no: Option<String>,
    /// 审批单名称。
    pub sp_name: Option<String>,
    /// 审批单状态。
    pub sp_status: Option<i32>,
    /// 审批模板 id。
    pub template_id: Option<String>,
    /// 提交申请时间（unix 时间戳）。
    pub apply_time: Option<i64>,
    /// 申请人信息。
    pub applier: Applier,
    /// 审批流程信息。
    pub sp_records: Vec<SpRecord>,
    /// 抄送人列表。
    pub notifier: Vec<Notifier>,
    /// 评论列表。
    pub comments: Vec<Comment>,
    /// 状态变更事件（0 审批申请 1 审批通过 2 审批驳回 ...）。
    pub status_change_event: Option<i32>,
}

impl WxCpXmlApprovalInfo {
    /// 从 XML 树节点解析（对应 Java XStream `fromXml` 语义）。
    pub(crate) fn from_tree(map: &HashMap<String, XmlValue>) -> Self {
        Self {
            third_no: str_field(map, "ThirdNo"),
            open_sp_name: str_field(map, "OpenSpName"),
            open_template_id: str_field(map, "OpenTemplateId"),
            open_sp_status: int_field(map, "OpenSpStatus"),
            apply_user_name: str_field(map, "ApplyUserName"),
            apply_user_id: str_field(map, "ApplyUserId"),
            apply_user_party: str_field(map, "ApplyUserParty"),
            apply_user_image: str_field(map, "ApplyUserImage"),
            approver_step: int_field(map, "ApproverStep"),
            approval_nodes: list_items(map, &["ApprovalNodes", "approvalNodes"], &["ApprovalNode"])
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(parse_approval_node)
                .collect(),
            notify_nodes: list_items(map, &["NotifyNodes", "notifyNodes"], &["NotifyNode"])
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(parse_notify_node)
                .collect(),
            sp_no: str_field(map, "SpNo"),
            sp_name: str_field(map, "SpName"),
            sp_status: int_field(map, "SpStatus"),
            template_id: str_field(map, "TemplateId"),
            apply_time: long_field(map, "ApplyTime"),
            applier: node_field(map, "Applyer")
                .map(|m| Applier {
                    user_id: str_field(m, "UserId"),
                    party: str_field(m, "Party"),
                })
                .unwrap_or_default(),
            sp_records: list_items(map, &["SpRecords", "spRecords"], &["SpRecord"])
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(parse_sp_record)
                .collect(),
            notifier: list_items(map, &["Notifyer", "notifier"], &["Notifyer"])
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(|m| Notifier {
                    user_id: str_field(m, "UserId"),
                })
                .collect(),
            comments: list_items(map, &["Comments", "comments"], &["Comments"])
                .into_iter()
                .filter_map(XmlValue::as_node)
                .map(parse_comment)
                .collect(),
            status_change_event: int_field(map, "StatuChangeEvent"),
        }
    }
}

/// 审批节点（对应 Java `WxCpXmlApprovalInfo.ApprovalNode`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApprovalNode {
    /// 节点审批状态（1 审批中 2 已批准 3 已驳回 4 已转审）。
    pub node_status: Option<i32>,
    /// 审批节点属性（1 或签 2 会签）。
    pub node_attr: Option<i32>,
    /// 审批节点类型（1 固定审批人 2 发起人自选 3 连续多级审批）。
    pub node_type: Option<i32>,
    /// 审批人列表（`Items` 下重复 `Item`）。
    pub items: Vec<Item>,
}

/// 审批节点明细（对应 Java `WxCpXmlApprovalInfo.Item`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Item {
    /// 分支审批人姓名。
    pub item_name: Option<String>,
    /// 分支审批人 userid。
    pub item_user_id: Option<String>,
    /// 分支审批人所在部门。
    pub item_party: Option<String>,
    /// 分支审批人头像。
    pub item_image: Option<String>,
    /// 分支审批意见。
    pub item_speech: Option<String>,
    /// 分支审批状态（1 审批中 2 已批准 3 已驳回 4 已转审）。
    pub item_status: Option<i32>,
    /// 分支审批操作时间。
    pub item_op_time: Option<i64>,
}

/// 抄送人（对应 Java `WxCpXmlApprovalInfo.NotifyNode`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NotifyNode {
    /// 抄送人姓名。
    pub item_name: Option<String>,
    /// 抄送人 userid。
    pub item_user_id: Option<String>,
    /// 抄送人所在部门。
    pub item_party: Option<String>,
    /// 抄送人头像。
    pub item_image: Option<String>,
}

/// 申请人（对应 Java `WxCpXmlApprovalInfo.Applier`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Applier {
    /// 申请人 userid。
    pub user_id: Option<String>,
    /// 申请人所在部门。
    pub party: Option<String>,
}

/// 审批流程记录（对应 Java `WxCpXmlApprovalInfo.SpRecord`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpRecord {
    /// 审批节点状态（1 审批中 2 已通过 3 已驳回 4 已转审）。
    pub sp_status: Option<String>,
    /// 审批方式（1 或签 2 会签）。
    pub approver_attr: Option<String>,
    /// 审批人详情列表（`Details` 下重复 `Detail`）。
    pub details: Vec<Detail>,
}

/// 审批人详情（对应 Java `WxCpXmlApprovalInfo.Detail`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Detail {
    /// 审批人信息。
    pub approver: Approver,
    /// 审批意见。
    pub speech: Option<String>,
    /// 审批节点状态（1 审批中 2 已批准 3 已驳回 4 已转审）。
    pub sp_status: Option<String>,
    /// 审批操作时间。
    pub sp_time: Option<i64>,
    /// 附件列表。
    pub attach: Vec<String>,
}

/// 审批人（对应 Java `WxCpXmlApprovalInfo.Approver`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Approver {
    /// 审批人 userid。
    pub user_id: Option<String>,
}

/// 抄送人（对应 Java `WxCpXmlApprovalInfo.Notifier`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Notifier {
    /// 抄送人 userid。
    pub user_id: Option<String>,
}

/// 评论（对应 Java `WxCpXmlApprovalInfo.Comment`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Comment {
    /// 评论人信息。
    pub comment_user_info: CommentUserInfo,
    /// 评论时间。
    pub comment_time: Option<String>,
    /// 评论内容。
    pub comment_content: Option<String>,
    /// 评论 id。
    pub comment_id: Option<String>,
    /// 附件列表。
    pub attach: Vec<String>,
}

/// 评论人信息（对应 Java `WxCpXmlApprovalInfo.CommentUserInfo`）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommentUserInfo {
    /// 评论人 userid。
    pub user_id: Option<String>,
}

fn parse_approval_node(map: &HashMap<String, XmlValue>) -> ApprovalNode {
    ApprovalNode {
        node_status: int_field(map, "NodeStatus"),
        node_attr: int_field(map, "NodeAttr"),
        node_type: int_field(map, "NodeType"),
        items: list_items(map, &["Items", "items"], &["Item"])
            .into_iter()
            .filter_map(XmlValue::as_node)
            .map(|m| Item {
                item_name: str_field(m, "ItemName"),
                item_user_id: str_field(m, "ItemUserId"),
                item_party: str_field(m, "ItemParty"),
                item_image: str_field(m, "ItemImage"),
                item_speech: str_field(m, "ItemSpeech"),
                item_status: int_field(m, "ItemStatus"),
                item_op_time: long_field(m, "ItemOpTime"),
            })
            .collect(),
    }
}

fn parse_notify_node(map: &HashMap<String, XmlValue>) -> NotifyNode {
    NotifyNode {
        item_name: str_field(map, "ItemName"),
        item_user_id: str_field(map, "ItemUserId"),
        item_party: str_field(map, "ItemParty"),
        item_image: str_field(map, "ItemImage"),
    }
}

fn parse_sp_record(map: &HashMap<String, XmlValue>) -> SpRecord {
    SpRecord {
        sp_status: str_field(map, "SpStatus"),
        approver_attr: str_field(map, "ApproverAttr"),
        details: list_items(map, &["Details", "details"], &["Detail"])
            .into_iter()
            .filter_map(XmlValue::as_node)
            .map(|m| Detail {
                approver: Approver {
                    user_id: node_field(m, "Approver").and_then(|a| str_field(a, "UserId")),
                },
                speech: str_field(m, "Speech"),
                sp_status: str_field(m, "SpStatus"),
                sp_time: long_field(m, "SpTime"),
                attach: list_items(m, &["Attach", "attach"], &["Attach"])
                    .into_iter()
                    .filter_map(XmlValue::as_scalar)
                    .map(str::to_string)
                    .collect(),
            })
            .collect(),
    }
}

fn parse_comment(map: &HashMap<String, XmlValue>) -> Comment {
    Comment {
        comment_user_info: CommentUserInfo {
            user_id: node_field(map, "CommentUserInfo").and_then(|m| str_field(m, "UserId")),
        },
        comment_time: str_field(map, "CommentTime"),
        comment_content: str_field(map, "CommentContent"),
        comment_id: str_field(map, "CommentId"),
        attach: list_items(map, &["Attach", "attach"], &["Attach"])
            .into_iter()
            .filter_map(XmlValue::as_scalar)
            .map(str::to_string)
            .collect(),
    }
}
