#![allow(clippy::field_reassign_with_default, dead_code)]
//! 第二批镜像补测——CP 会话存档 + OA 服务。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxCpMsgAuditTest（877 行）
//! - WxCpOaServiceImplTest（713 行）
//! - WxCpOaWeDriveServiceTest（287 行）
//! - WxCpOaAgentTest（218 行）

use wx_rust_cp::bean::message::*;
use wx_rust_cp::bean::msgaudit::*;
use wx_rust_cp::bean::oa::WxCpCheckinData;
use wx_rust_cp::bean::oa::WxCpCheckinDayData;
use wx_rust_cp::bean::oa::WxCpCheckinMonthData;
use wx_rust_cp::bean::oa::WxCpCheckinOption;
use wx_rust_cp::bean::oa::WxCpOaApprovalTemplateResult;
use wx_rust_cp::bean::oa::WxCpSetCheckinSchedule;
use wx_rust_cp::bean::oa::wedrive::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxCpMsgAuditTest（877 行）—— 会话内容存档
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpMsgAuditTest（msg_audit_approved 事件 XML 解析）
#[test]
fn test_msg_audit_approved_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[toUser]]></ToUserName>",
        "<FromUserName><![CDATA[sys]]></FromUserName>",
        "<CreateTime>1403610513</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[change_external_contact]]></Event>",
        "<ChangeType><![CDATA[msg_audit_approved]]></ChangeType>",
        "<UserID><![CDATA[zhangsan]]></UserID>",
        "<ExternalUserID><![CDATA[woAJ2GCAAABiuyujaWJHDDGi0mACHAAA]]></ExternalUserID>",
        "<WelcomeCode><![CDATA[WELCOMECODE]]></WelcomeCode>",
        "</xml>"
    );
    let msg = WxCpXmlMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user_name.as_deref(), Some("toUser"));
    assert_eq!(msg.from_user_name.as_deref(), Some("sys"));
    assert_eq!(msg.create_time, Some(1403610513));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("change_external_contact"));
    assert_eq!(msg.change_type.as_deref(), Some("msg_audit_approved"));
    assert_eq!(msg.user_id.as_deref(), Some("zhangsan"));
}

/// 对应 Java: WxCpMsgAuditTest（msgaudit_notify 事件 XML 解析）
#[test]
fn test_msg_audit_notify_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[CorpID]]></ToUserName>",
        "<FromUserName><![CDATA[sys]]></FromUserName>",
        "<CreateTime>1629101687</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<AgentID>2000004</AgentID>",
        "<Event><![CDATA[msgaudit_notify]]></Event>",
        "</xml>"
    );
    let msg = WxCpXmlMessage::from_xml(xml).expect("解析 XML 成功");
    assert_eq!(msg.to_user_name.as_deref(), Some("CorpID"));
    assert_eq!(msg.from_user_name.as_deref(), Some("sys"));
    assert_eq!(msg.create_time, Some(1629101687));
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.agent_id.as_deref(), Some("2000004"));
    assert_eq!(msg.event.as_deref(), Some("msgaudit_notify"));
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 文本消息解析）
#[test]
fn test_chat_model_text_json() {
    let json_str = r#"{
        "msgtype": "text",
        "text": {
            "content": "测试消息内容"
        },
        "from": "zhangsan"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析聊天模型");
    assert_eq!(model.msg_type, "text");
    assert_eq!(model.text.content, "测试消息内容");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 图片消息解析）
#[test]
fn test_chat_model_image_json() {
    let json_str = r#"{
        "msgtype": "image",
        "image": {
            "md5sum": "abc123",
            "sdkfileid": "file001"
        },
        "from": "lisi"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析图片聊天模型");
    assert_eq!(model.msg_type, "image");
    assert_eq!(model.image.md5_sum, "abc123");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 撤回消息解析）
#[test]
fn test_chat_model_revoke_json() {
    let json_str = r#"{
        "msgtype": "revoke",
        "revoke": {
            "pre_msgid": "MSG001"
        },
        "from": "wangwu"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析撤回聊天模型");
    assert_eq!(model.msg_type, "revoke");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 同意消息解析）
#[test]
fn test_chat_model_agree_json() {
    let json_str = r#"{
        "msgtype": "agree",
        "agree": {
            "userid": "zhangsan",
            "agree_time": 1620000000
        },
        "from": "zhangsan"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析同意聊天模型");
    assert_eq!(model.msg_type, "agree");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 文件消息解析）
#[test]
fn test_chat_model_file_json() {
    let json_str = r#"{
        "msgtype": "file",
        "file": {
            "md5sum": "def456",
            "filename": "report.pdf",
            "filesize": 2048
        },
        "from": "zhangsan"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析文件聊天模型");
    assert_eq!(model.msg_type, "file");
    assert_eq!(model.file.md5_sum, "def456");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 链接消息解析）
#[test]
fn test_chat_model_link_json() {
    let json_str = r#"{
        "msgtype": "link",
        "link": {
            "title": "测试链接",
            "link_url": "https://example.com",
            "description": "链接描述"
        },
        "from": "zhangsan"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析链接聊天模型");
    assert_eq!(model.msg_type, "link");
    assert_eq!(model.link.title, "测试链接");
}

/// 对应 Java: WxCpMsgAuditTest（WxCpChatModel.fromJson 小程序消息解析）
#[test]
fn test_chat_model_weapp_json() {
    let json_str = r#"{
        "msgtype": "weapp",
        "weapp": {
            "title": "测试小程序",
            "des": "描述"
        },
        "from": "zhangsan"
    }"#;
    let model: WxCpChatModel = serde_json::from_str(json_str).expect("解析小程序聊天模型");
    assert_eq!(model.msg_type, "weapp");
}

// ═══════════════════════════════════════════════════════════════
// #2 WxCpOaServiceImplTest（713 行）—— OA 数据接口
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpOaServiceImplTest.testGetCheckinData（打卡数据 bean 验证）
#[test]
fn test_checkin_data_bean_serde() {
    let json_str = r#"{
        "userid": "binary",
        "groupname": "打卡组",
        "checkin_type": "ClockIn",
        "exception_type": "None",
        "checkin_time": 1556611200,
        "location_title": "公司",
        "location_detail": "地址",
        "notes": "备注",
        "wifiname": "WiFi",
        "wifimac": "AA:BB:CC:DD:EE:FF"
    }"#;
    let data: WxCpCheckinData = serde_json::from_str(json_str).expect("解析打卡数据");
    assert_eq!(data.user_id, "binary");
    assert_eq!(data.checkin_time, 1556611200);
}

/// 对应 Java: WxCpOaServiceImplTest.testGetCheckinDayData（打卡日报 bean 验证）
#[test]
fn test_checkin_day_data_bean_serde() {
    let json_str = r#"{
        "base_info": {
            "date": 20210701,
            "record_type": 1,
            "name": "张三"
        },
        "summary_info": {
            "checkin_count": 2,
            "regular_work_sec": 28800,
            "standard_work_sec": 28800
        }
    }"#;
    let data: WxCpCheckinDayData = serde_json::from_str(json_str).expect("解析打卡日报");
    assert_eq!(data.base_info.record_type, 1);
}

/// 对应 Java: WxCpOaServiceImplTest.testGetCheckinMonthData（打卡月报 bean 验证）
#[test]
fn test_checkin_month_data_bean_serde() {
    let json_str = r#"{
        "base_info": {
            "record_type": 1,
            "name": "张三",
            "days": 22,
            "work_days": 22
        },
        "summary_info": {
            "checkin_count": 44,
            "regular_work_sec": 576000,
            "standard_work_sec": 576000
        }
    }"#;
    let data: WxCpCheckinMonthData = serde_json::from_str(json_str).expect("解析打卡月报");
    assert_eq!(data.base_info.record_type, 1);
}

/// 对应 Java: WxCpOaServiceImplTest.testGetCheckinOption（打卡选项 bean 验证）
#[test]
fn test_checkin_option_bean_serde() {
    let json_str = r#"{
        "userid": "binary",
        "group_checkin_option": {
            "groupid": 1,
            "groupname": "默认组",
            "checkindates": [],
            "spe_workdaytime": [],
            "spe_offdaytime": []
        }
    }"#;
    let option: WxCpCheckinOption = serde_json::from_str(json_str).expect("解析打卡选项");
    assert_eq!(option.user_id, "binary");
}

/// 对应 Java: WxCpOaServiceImplTest.testSetCheckinScheduleList（排班设置请求体构建）
#[test]
fn test_set_checkin_schedule_body() {
    let mut schedule = WxCpSetCheckinSchedule::default();
    schedule.group_id = 3;
    schedule.yearmonth = 202108;
    let item = wx_rust_cp::bean::oa::wx_cp_set_checkin_schedule::Item {
        schedule_id: 0,
        day: 20,
        userid: "12003648".to_string(),
    };
    schedule.items = vec![item];
    assert_eq!(schedule.group_id, 3);
    assert_eq!(schedule.yearmonth, 202108);
    assert_eq!(schedule.items.len(), 1);
    assert_eq!(schedule.items[0].day, 20);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxCpOaWeDriveServiceTest（287 行）—— 微文档云盘服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpOaWeDriveServiceTest（创建空间请求体构建）
#[test]
fn test_wedrive_space_create_request() {
    let json_str = r#"{
        "userid": "USERID",
        "space_name": "SPACE_NAME",
        "auth_info": [
            {"type": 1, "userid": "USERID", "auth": 2},
            {"type": 2, "departmentid": 2, "auth": 1}
        ]
    }"#;
    let request: WxCpSpaceCreateRequest = serde_json::from_str(json_str).expect("解析创建空间请求");
    assert_eq!(request.user_id, "USERID");
    assert_eq!(request.space_name, "SPACE_NAME");
    assert_eq!(request.auth_info.len(), 2);
}

/// 对应 Java: WxCpOaWeDriveServiceTest（文件信息响应解析）
#[test]
fn test_wedrive_file_info_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "file_info": {
            "fileid": "FILE001",
            "file_name": "测试文件.docx",
            "file_size": 1024,
            "file_type": 1
        }
    }"#;
    let info: WxCpFileInfo = serde_json::from_str(json_str).expect("解析文件信息");
    assert_eq!(info.errcode, 0);
}

/// 对应 Java: WxCpOaWeDriveServiceTest（文件列表响应解析）
#[test]
fn test_wedrive_file_list_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "has_more": false,
        "next_start": 0,
        "file_list": {
            "item": [
                {"fileid": "F1", "file_name": "文件1.docx"},
                {"fileid": "F2", "file_name": "文件2.xlsx"}
            ]
        }
    }"#;
    let list: WxCpFileList = serde_json::from_str(json_str).expect("解析文件列表");
    assert_eq!(list.errcode, 0);
}

// ═══════════════════════════════════════════════════════════════
// #4 WxCpOaAgentTest（218 行）—— OA 代理测试
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpOaAgentTest（审批模板详情 bean 结构验证）
#[test]
fn test_oa_approval_template_bean() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "template_names": [
            {"text": "请假", "lang": "zh_CN"}
        ],
        "template_content": {
            "controls": [
                {
                    "property": {
                        "control": "Textarea",
                        "id": "Textarea-1",
                        "title": [{"text": "请假事由", "lang": "zh_CN"}]
                    }
                }
            ]
        }
    }"#;
    let result: WxCpOaApprovalTemplateResult =
        serde_json::from_str(json_str).expect("解析审批模板");
    assert_eq!(result.err_code, 0);
}

/// 对应 Java: WxCpOaAgentTest（审批详情 bean 结构验证）
#[test]
fn test_oa_approval_detail_bean() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "info": {
            "sp_no": "SP202107010001",
            "sp_name": "请假申请",
            "sp_status": 1,
            "apply_time": 1625068800,
            "applyer": {
                "userid": "zhangsan",
                "department_id": 1
            }
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["info"]["sp_no"], "SP202107010001");
    assert_eq!(value["info"]["sp_status"], 1);
}
