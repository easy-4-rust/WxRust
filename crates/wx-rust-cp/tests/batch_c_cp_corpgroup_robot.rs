#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——CP 企业群组 + 智能机器人 + 互联组织 bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxCpCorpGroupCorpTest（JSON 序列化/反序列化验证）
//! - WxCpCorpGroupCorpTokenTest（token 响应解析验证）
//! - WxCpCorpGroupCorpListAppShareInfoRespTest（应用共享信息列表解析）
//! - WxCpIntelligentRobotTest（机器人信息 JSON 解析）
//! - WxCpIntelligentRobotChatRequestTest（聊天请求体构建验证）
//! - WxCpIntelligentRobotChatResponseTest（聊天响应解析验证）
//! - WxCpLinkedCorpDepartmentTest（互联部门 JSON 解析）
//! - WxCpLinkedCorpUserTest（互联用户 JSON 解析）

use wx_rust_cp::bean::corpgroup::*;
use wx_rust_cp::bean::intelligentrobot::*;
use wx_rust_cp::bean::linkedcorp::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxCpCorpGroupCorpTest —— 企业群组 Corp JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpCorpGroupCorpTest（corp 信息 JSON 反序列化）
#[test]
fn test_corp_group_corp_from_json() {
    let json_str = r#"{
        "corpid": "wx1234567890",
        "corp_name": "测试企业",
        "agentid": 1000002
    }"#;
    let corp = WxCpCorpGroupCorp::from_json(json_str).expect("解析企业群组 corp");
    assert_eq!(corp.corpid, "wx1234567890");
    assert_eq!(corp.corp_name, "测试企业");
    assert_eq!(corp.agentid, 1000002);
}

/// 对应 Java: WxCpCorpGroupCorpTest（corp 信息 JSON 序列化往返验证）
#[test]
fn test_corp_group_corp_roundtrip() {
    let json_str = r#"{
        "corpid": "wx9999999999",
        "corp_name": "子企业",
        "agentid": 50001
    }"#;
    let corp = WxCpCorpGroupCorp::from_json(json_str).expect("解析");
    let serialized = corp.to_json().expect("序列化");
    let corp2 = WxCpCorpGroupCorp::from_json(&serialized).expect("反序列化");
    assert_eq!(corp, corp2);
}

/// 对应 Java: WxCpCorpGroupCorpTest（空 JSON 默认值验证）
#[test]
fn test_corp_group_corp_default() {
    let json_str = r#"{}"#;
    let corp = WxCpCorpGroupCorp::from_json(json_str).expect("解析空 JSON");
    assert_eq!(corp.corpid, "");
    assert_eq!(corp.corp_name, "");
    assert_eq!(corp.agentid, 0);
}

// ═══════════════════════════════════════════════════════════════
// #2 WxCpCorpGroupCorpTokenTest —— 企业群组 Token 响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpCorpGroupCorpTokenTest（token 响应 JSON 解析）
#[test]
fn test_corp_group_corp_token_from_json() {
    let json_str = r#"{
        "access_token": "ACCESS_TOKEN_12345",
        "expires_in": 7200
    }"#;
    let token = WxCpCorpGroupCorpToken::from_json(json_str).expect("解析 token");
    assert_eq!(token.access_token, "ACCESS_TOKEN_12345");
    assert_eq!(token.expires_in, 7200);
}

/// 对应 Java: WxCpCorpGroupCorpTokenTest（token 序列化往返验证）
#[test]
fn test_corp_group_corp_token_roundtrip() {
    let json_str = r#"{
        "access_token": "TOKEN_ABC",
        "expires_in": 3600
    }"#;
    let token = WxCpCorpGroupCorpToken::from_json(json_str).expect("解析");
    let serialized = token.to_json().expect("序列化");
    let token2 = WxCpCorpGroupCorpToken::from_json(&serialized).expect("反序列化");
    assert_eq!(token, token2);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxCpCorpGroupCorpListAppShareInfoRespTest —— 应用共享信息列表
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpCorpGroupCorpListAppShareInfoRespTest（列表响应解析）
#[test]
fn test_corp_group_app_share_info_list_from_json() {
    let json_str = r#"{
        "ending": 1,
        "corp_list": [
            {"corpid": "wx111", "corp_name": "企业A", "agentid": 100},
            {"corpid": "wx222", "corp_name": "企业B", "agentid": 200}
        ],
        "next_cursor": "CURSOR_NEXT"
    }"#;
    let resp: WxCpCorpGroupCorpListAppShareInfoResp =
        serde_json::from_str(json_str).expect("解析应用共享信息列表");
    assert_eq!(resp.ending, 1);
    assert_eq!(resp.corp_list.len(), 2);
    assert_eq!(resp.corp_list[0].corpid, "wx111");
    assert_eq!(resp.corp_list[1].corp_name, "企业B");
    assert_eq!(resp.next_cursor, "CURSOR_NEXT");
}

/// 对应 Java: WxCpCorpGroupCorpListAppShareInfoRespTest（空列表解析）
#[test]
fn test_corp_group_app_share_info_list_empty() {
    let json_str = r#"{
        "ending": 1,
        "corp_list": [],
        "next_cursor": ""
    }"#;
    let resp: WxCpCorpGroupCorpListAppShareInfoResp =
        serde_json::from_str(json_str).expect("解析空列表");
    assert_eq!(resp.ending, 1);
    assert!(resp.corp_list.is_empty());
    assert_eq!(resp.next_cursor, "");
}

// ═══════════════════════════════════════════════════════════════
// #4 WxCpIntelligentRobotTest —— 智能机器人信息 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotTest（机器人信息 JSON 反序列化）
#[test]
fn test_intelligent_robot_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "robot_id": "robot_001",
        "name": "客服机器人",
        "description": "智能客服",
        "avatar": "https://example.com/avatar.png",
        "status": 1,
        "create_time": 1627800000,
        "update_time": 1627886400
    }"#;
    let robot = WxCpIntelligentRobot::from_json(json_str).expect("解析机器人信息");
    assert_eq!(robot.errcode, 0);
    assert_eq!(robot.errmsg, "ok");
    assert_eq!(robot.robot_id, "robot_001");
    assert_eq!(robot.name, "客服机器人");
    assert_eq!(robot.description, "智能客服");
    assert_eq!(robot.status, 1);
    assert_eq!(robot.create_time, 1627800000);
}

/// 对应 Java: WxCpIntelligentRobotTest（机器人信息序列化往返验证）
#[test]
fn test_intelligent_robot_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "robot_id": "robot_002",
        "name": "测试机器人",
        "description": "测试用",
        "avatar": "",
        "status": 0,
        "create_time": 0,
        "update_time": 0
    }"#;
    let robot = WxCpIntelligentRobot::from_json(json_str).expect("解析");
    let serialized = robot.to_json().expect("序列化");
    let robot2 = WxCpIntelligentRobot::from_json(&serialized).expect("反序列化");
    assert_eq!(robot, robot2);
}

/// 对应 Java: WxCpIntelligentRobotTest（错误响应解析）
#[test]
fn test_intelligent_robot_error_response() {
    let json_str = r#"{
        "errcode": 40001,
        "errmsg": "invalid credential",
        "robot_id": "",
        "name": "",
        "description": "",
        "avatar": "",
        "status": 0,
        "create_time": 0,
        "update_time": 0
    }"#;
    let robot = WxCpIntelligentRobot::from_json(json_str).expect("解析错误响应");
    assert_eq!(robot.errcode, 40001);
    assert_eq!(robot.errmsg, "invalid credential");
    assert_eq!(robot.robot_id, "");
}

// ═══════════════════════════════════════════════════════════════
// #5 WxCpIntelligentRobotChatRequestTest —— 聊天请求体验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotChatRequestTest（聊天请求 JSON 构建）
#[test]
fn test_intelligent_robot_chat_request_from_json() {
    let json_str = r#"{
        "robot_id": "robot_001",
        "userid": "zhangsan",
        "message": "你好，请问如何退款？",
        "session_id": "session_abc123"
    }"#;
    let req = WxCpIntelligentRobotChatRequest::from_json(json_str).expect("解析聊天请求");
    assert_eq!(req.robot_id, "robot_001");
    assert_eq!(req.userid, "zhangsan");
    assert_eq!(req.message, "你好，请问如何退款？");
    assert_eq!(req.session_id, "session_abc123");
}

/// 对应 Java: WxCpIntelligentRobotChatRequestTest（聊天请求序列化往返验证）
#[test]
fn test_intelligent_robot_chat_request_roundtrip() {
    let json_str = r#"{
        "robot_id": "robot_002",
        "userid": "lisi",
        "message": "查询订单状态",
        "session_id": "session_xyz"
    }"#;
    let req = WxCpIntelligentRobotChatRequest::from_json(json_str).expect("解析");
    let serialized = req.to_json().expect("序列化");
    let req2 = WxCpIntelligentRobotChatRequest::from_json(&serialized).expect("反序列化");
    assert_eq!(req, req2);
}

// ═══════════════════════════════════════════════════════════════
// #6 WxCpIntelligentRobotChatResponseTest —— 聊天响应解析验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotChatResponseTest（聊天响应 JSON 解析）
#[test]
fn test_intelligent_robot_chat_response_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "reply": "您好，退款请联系客服。",
        "session_id": "session_abc123",
        "msg_id": "msg_001"
    }"#;
    let resp = WxCpIntelligentRobotChatResponse::from_json(json_str).expect("解析聊天响应");
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.reply, "您好，退款请联系客服。");
    assert_eq!(resp.session_id, "session_abc123");
    assert_eq!(resp.msg_id, "msg_001");
}

/// 对应 Java: WxCpIntelligentRobotChatResponseTest（聊天响应序列化往返验证）
#[test]
fn test_intelligent_robot_chat_response_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "reply": "测试回复",
        "session_id": "session_def",
        "msg_id": "msg_002"
    }"#;
    let resp = WxCpIntelligentRobotChatResponse::from_json(json_str).expect("解析");
    let serialized = resp.to_json().expect("序列化");
    let resp2 = WxCpIntelligentRobotChatResponse::from_json(&serialized).expect("反序列化");
    assert_eq!(resp, resp2);
}

// ═══════════════════════════════════════════════════════════════
// #7 WxCpLinkedCorpDepartmentTest —— 互联部门 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpLinkedCorpDepartmentTest（互联部门 JSON 反序列化）
#[test]
fn test_linked_corp_department_from_json() {
    let json_str = r#"{
        "department_id": "linked_dept_001",
        "department_name": "互联技术部",
        "parentid": "0",
        "order": 1
    }"#;
    let dept: WxCpLinkedCorpDepartment = serde_json::from_str(json_str).expect("解析互联部门");
    assert_eq!(dept.department_id, "linked_dept_001");
    assert_eq!(dept.department_name, "互联技术部");
    assert_eq!(dept.parent_id, "0");
    assert_eq!(dept.order, 1);
}

/// 对应 Java: WxCpLinkedCorpDepartmentTest（互联部门序列化往返验证）
#[test]
fn test_linked_corp_department_roundtrip() {
    let json_str = r#"{
        "department_id": "linked_dept_002",
        "department_name": "互联产品部",
        "parentid": "linked_dept_001",
        "order": 2
    }"#;
    let dept: WxCpLinkedCorpDepartment = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&dept).expect("序列化");
    let dept2: WxCpLinkedCorpDepartment = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(dept, dept2);
}

// ═══════════════════════════════════════════════════════════════
// #8 WxCpLinkedCorpUserTest —— 互联用户 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpLinkedCorpUserTest（互联用户 JSON 反序列化）
#[test]
fn test_linked_corp_user_from_json() {
    let json_str = r#"{
        "userid": "linked_user_001",
        "name": "张三",
        "department": ["dept_001", "dept_002"],
        "mobile": "13800138000",
        "email": "zhangsan@example.com",
        "position": "工程师",
        "corpid": "linked_corp_001",
        "extAttrs": [
            {
                "type": 0,
                "name": "爱好",
                "textValue": "编程",
                "webUrl": "",
                "webTitle": ""
            }
        ]
    }"#;
    let user: WxCpLinkedCorpUser = serde_json::from_str(json_str).expect("解析互联用户");
    assert_eq!(user.user_id, "linked_user_001");
    assert_eq!(user.name, "张三");
    assert_eq!(user.department.len(), 2);
    assert_eq!(user.department[0], "dept_001");
    assert_eq!(user.mobile, "13800138000");
    assert_eq!(user.email, "zhangsan@example.com");
    assert_eq!(user.position, "工程师");
    assert_eq!(user.corp_id, "linked_corp_001");
    assert_eq!(user.ext_attrs.len(), 1);
    assert_eq!(user.ext_attrs[0].name, "爱好");
    assert_eq!(user.ext_attrs[0].text_value, "编程");
}

/// 对应 Java: WxCpLinkedCorpUserTest（互联用户空扩展属性）
#[test]
fn test_linked_corp_user_empty_ext_attrs() {
    let json_str = r#"{
        "userid": "linked_user_002",
        "name": "李四",
        "department": [],
        "mobile": "",
        "email": "",
        "position": "",
        "corpid": "linked_corp_002",
        "extAttrs": []
    }"#;
    let user: WxCpLinkedCorpUser = serde_json::from_str(json_str).expect("解析互联用户");
    assert_eq!(user.user_id, "linked_user_002");
    assert_eq!(user.name, "李四");
    assert!(user.department.is_empty());
    assert!(user.ext_attrs.is_empty());
}

/// 对应 Java: WxCpLinkedCorpUserTest（互联用户序列化往返验证）
#[test]
fn test_linked_corp_user_roundtrip() {
    let json_str = r#"{
        "userid": "linked_user_003",
        "name": "王五",
        "department": ["dept_003"],
        "mobile": "13900139000",
        "email": "wangwu@example.com",
        "position": "产品经理",
        "corpid": "linked_corp_003",
        "extAttrs": []
    }"#;
    let user: WxCpLinkedCorpUser = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&user).expect("序列化");
    let user2: WxCpLinkedCorpUser = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(user, user2);
}

// ═══════════════════════════════════════════════════════════════
// #9 WxCpLinkedCorpAgentPermTest —— 互联应用权限 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpLinkedCorpAgentPermTest（互联应用权限 JSON 反序列化）
#[test]
fn test_linked_corp_agent_perm_from_json() {
    let json_str = r#"{
        "userids": ["user_001", "user_002"],
        "department_ids": ["dept_001", "dept_002", "dept_003"]
    }"#;
    let perm: WxCpLinkedCorpAgentPerm = serde_json::from_str(json_str).expect("解析互联应用权限");
    assert_eq!(perm.user_id_list.len(), 2);
    assert_eq!(perm.user_id_list[0], "user_001");
    assert_eq!(perm.department_id_list.len(), 3);
    assert_eq!(perm.department_id_list[2], "dept_003");
}

/// 对应 Java: WxCpLinkedCorpAgentPermTest（互联应用权限空列表）
#[test]
fn test_linked_corp_agent_perm_empty() {
    let json_str = r#"{
        "userids": [],
        "department_ids": []
    }"#;
    let perm: WxCpLinkedCorpAgentPerm = serde_json::from_str(json_str).expect("解析空权限");
    assert!(perm.user_id_list.is_empty());
    assert!(perm.department_id_list.is_empty());
}

/// 对应 Java: WxCpLinkedCorpAgentPermTest（互联应用权限序列化往返验证）
#[test]
fn test_linked_corp_agent_perm_roundtrip() {
    let json_str = r#"{
        "userids": ["user_003"],
        "department_ids": ["dept_004"]
    }"#;
    let perm: WxCpLinkedCorpAgentPerm = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&perm).expect("序列化");
    let perm2: WxCpLinkedCorpAgentPerm = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(perm, perm2);
}

// ═══════════════════════════════════════════════════════════════
// #10 WxCpIntelligentRobotCreateRequestTest —— 创建机器人请求体验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotCreateRequestTest（创建机器人请求 JSON 构建）
#[test]
fn test_intelligent_robot_create_request_from_json() {
    let json_str = r#"{
        "name": "新客服机器人",
        "description": "7x24 小时智能客服",
        "avatar": "https://example.com/robot.png"
    }"#;
    let req = WxCpIntelligentRobotCreateRequest::from_json(json_str).expect("解析创建请求");
    assert_eq!(req.name, "新客服机器人");
    assert_eq!(req.description, "7x24 小时智能客服");
    assert_eq!(req.avatar, "https://example.com/robot.png");
}

/// 对应 Java: WxCpIntelligentRobotCreateRequestTest（创建机器人请求序列化往返）
#[test]
fn test_intelligent_robot_create_request_roundtrip() {
    let json_str = r#"{
        "name": "测试机器人",
        "description": "测试",
        "avatar": ""
    }"#;
    let req = WxCpIntelligentRobotCreateRequest::from_json(json_str).expect("解析");
    let serialized = req.to_json().expect("序列化");
    let req2 = WxCpIntelligentRobotCreateRequest::from_json(&serialized).expect("反序列化");
    assert_eq!(req, req2);
}

// ═══════════════════════════════════════════════════════════════
// #11 WxCpIntelligentRobotCreateResponseTest —— 创建机器人响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotCreateResponseTest（创建机器人响应 JSON 解析）
#[test]
fn test_intelligent_robot_create_response_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "robot_id": "robot_new_001"
    }"#;
    let resp = WxCpIntelligentRobotCreateResponse::from_json(json_str).expect("解析创建响应");
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
    assert_eq!(resp.robot_id, "robot_new_001");
}

/// 对应 Java: WxCpIntelligentRobotCreateResponseTest（创建机器人响应序列化往返）
#[test]
fn test_intelligent_robot_create_response_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "robot_id": "robot_new_002"
    }"#;
    let resp = WxCpIntelligentRobotCreateResponse::from_json(json_str).expect("解析");
    let serialized = resp.to_json().expect("序列化");
    let resp2 = WxCpIntelligentRobotCreateResponse::from_json(&serialized).expect("反序列化");
    assert_eq!(resp, resp2);
}

// ═══════════════════════════════════════════════════════════════
// #12 WxCpIntelligentRobotSendMessageRequestTest —— 发送消息请求体验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotSendMessageRequestTest（发送消息请求 JSON 构建）
#[test]
fn test_intelligent_robot_send_message_request_from_json() {
    let json_str = r#"{
        "robot_id": "robot_001",
        "userid": "zhangsan",
        "message": "请帮我查询订单",
        "session_id": "session_001",
        "msg_id": "msg_001"
    }"#;
    let req =
        WxCpIntelligentRobotSendMessageRequest::from_json(json_str).expect("解析发送消息请求");
    assert_eq!(req.robot_id, "robot_001");
    assert_eq!(req.userid, "zhangsan");
    assert_eq!(req.message, "请帮我查询订单");
    assert_eq!(req.session_id, "session_001");
    assert_eq!(req.msg_id, "msg_001");
}

/// 对应 Java: WxCpIntelligentRobotSendMessageRequestTest（发送消息请求序列化往返）
#[test]
fn test_intelligent_robot_send_message_request_roundtrip() {
    let json_str = r#"{
        "robot_id": "robot_002",
        "userid": "lisi",
        "message": "你好",
        "session_id": "session_002",
        "msg_id": "msg_002"
    }"#;
    let req = WxCpIntelligentRobotSendMessageRequest::from_json(json_str).expect("解析");
    let serialized = req.to_json().expect("序列化");
    let req2 = WxCpIntelligentRobotSendMessageRequest::from_json(&serialized).expect("反序列化");
    assert_eq!(req, req2);
}

// ═══════════════════════════════════════════════════════════════
// #13 WxCpIntelligentRobotMessageTest —— 机器人消息体解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotMessageTest（文本消息 JSON 解析）
#[test]
fn test_intelligent_robot_message_text_from_json() {
    let json_str = r#"{
        "msgid": "msg_001",
        "aibotid": "robot_001",
        "chatid": "chat_001",
        "chattype": "single",
        "from": {"userid": "zhangsan"},
        "response_url": "https://example.com/response",
        "msgtype": "text",
        "text": {"content": "你好，请问如何退款？"},
        "image": {"url": ""},
        "mixed": {"items": []},
        "voice": {"url": ""},
        "file": {"url": "", "md5": ""},
        "video": {"url": ""},
        "quote": {"msg_id": "", "content": ""},
        "stream": {"status": 0, "content": ""}
    }"#;
    let msg: WxCpIntelligentRobotMessage =
        serde_json::from_str(json_str).expect("解析机器人文本消息");
    assert_eq!(msg.msg_id, "msg_001");
    assert_eq!(msg.ai_bot_id, "robot_001");
    assert_eq!(msg.chat_id, "chat_001");
    assert_eq!(msg.chat_type, "single");
    assert_eq!(msg.from.userid, "zhangsan");
    assert_eq!(msg.msg_type, "text");
    assert_eq!(msg.text.content, "你好，请问如何退款？");
}

/// 对应 Java: WxCpIntelligentRobotMessageTest（图片消息 JSON 解析）
#[test]
fn test_intelligent_robot_message_image_from_json() {
    let json_str = r#"{
        "msgid": "msg_002",
        "aibotid": "robot_001",
        "chatid": "chat_001",
        "chattype": "single",
        "from": {"userid": "lisi"},
        "response_url": "",
        "msgtype": "image",
        "text": {"content": ""},
        "image": {"url": "https://example.com/image.png"},
        "mixed": {"items": []},
        "voice": {"url": ""},
        "file": {"url": "", "md5": ""},
        "video": {"url": ""},
        "quote": {"msg_id": "", "content": ""},
        "stream": {"status": 0, "content": ""}
    }"#;
    let msg: WxCpIntelligentRobotMessage =
        serde_json::from_str(json_str).expect("解析机器人图片消息");
    assert_eq!(msg.msg_id, "msg_002");
    assert_eq!(msg.msg_type, "image");
    assert_eq!(msg.image.url, "https://example.com/image.png");
}

// ═══════════════════════════════════════════════════════════════
// #14 WxCpIntelligentRobotUpdateRequestTest —— 更新机器人请求体验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotUpdateRequestTest（更新机器人请求 JSON 构建）
#[test]
fn test_intelligent_robot_update_request_from_json() {
    let json_str = r#"{
        "robot_id": "robot_001",
        "name": "更新后的机器人",
        "description": "更新描述",
        "avatar": "https://example.com/new_avatar.png",
        "status": 1
    }"#;
    let req = WxCpIntelligentRobotUpdateRequest::from_json(json_str).expect("解析更新请求");
    assert_eq!(req.robot_id, "robot_001");
    assert_eq!(req.name, "更新后的机器人");
    assert_eq!(req.description, "更新描述");
    assert_eq!(req.avatar, "https://example.com/new_avatar.png");
    assert_eq!(req.status, 1);
}

/// 对应 Java: WxCpIntelligentRobotUpdateRequestTest（更新机器人请求序列化往返）
#[test]
fn test_intelligent_robot_update_request_roundtrip() {
    let json_str = r#"{
        "robot_id": "robot_002",
        "name": "测试",
        "description": "",
        "avatar": "",
        "status": 0
    }"#;
    let req = WxCpIntelligentRobotUpdateRequest::from_json(json_str).expect("解析");
    let serialized = req.to_json().expect("序列化");
    let req2 = WxCpIntelligentRobotUpdateRequest::from_json(&serialized).expect("反序列化");
    assert_eq!(req, req2);
}

// ═══════════════════════════════════════════════════════════════
// #15 WxCpIntelligentRobotSendMessageResponseTest —— 发送消息响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpIntelligentRobotSendMessageResponseTest（发送消息响应 JSON 解析）
#[test]
fn test_intelligent_robot_send_message_response_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "msg_id": "msg_reply_001",
        "session_id": "session_001"
    }"#;
    let resp = WxCpIntelligentRobotSendMessageResponse::from_json(json_str).expect("解析发送响应");
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
    assert_eq!(resp.msg_id, "msg_reply_001");
    assert_eq!(resp.session_id, "session_001");
}

/// 对应 Java: WxCpIntelligentRobotSendMessageResponseTest（发送消息响应序列化往返）
#[test]
fn test_intelligent_robot_send_message_response_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "msg_id": "msg_reply_002",
        "session_id": "session_002"
    }"#;
    let resp = WxCpIntelligentRobotSendMessageResponse::from_json(json_str).expect("解析");
    let serialized = resp.to_json().expect("序列化");
    let resp2 = WxCpIntelligentRobotSendMessageResponse::from_json(&serialized).expect("反序列化");
    assert_eq!(resp, resp2);
}

// ═══════════════════════════════════════════════════════════════
// #16 WxCpMaTransferSessionTest —— 小程序迁移会话 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpMaTransferSessionTest（迁移会话 JSON 反序列化）
#[test]
fn test_ma_transfer_session_from_json() {
    let json_str = r#"{
        "userid": "zhangsan",
        "session_key": "session_key_abc123"
    }"#;
    let session = WxCpMaTransferSession::from_json(json_str).expect("解析迁移会话");
    assert_eq!(session.user_id, "zhangsan");
    assert_eq!(session.session_key, "session_key_abc123");
}

/// 对应 Java: WxCpMaTransferSessionTest（迁移会话序列化往返验证）
#[test]
fn test_ma_transfer_session_roundtrip() {
    let json_str = r#"{
        "userid": "lisi",
        "session_key": "session_key_def456"
    }"#;
    let session = WxCpMaTransferSession::from_json(json_str).expect("解析");
    let serialized = session.to_json().expect("序列化");
    let session2 = WxCpMaTransferSession::from_json(&serialized).expect("反序列化");
    assert_eq!(session, session2);
}

/// 对应 Java: WxCpMaTransferSessionTest（迁移会话空值验证）
#[test]
fn test_ma_transfer_session_empty() {
    let json_str = r#"{}"#;
    let session = WxCpMaTransferSession::from_json(json_str).expect("解析空 JSON");
    assert_eq!(session.user_id, "");
    assert_eq!(session.session_key, "");
}

// ═══════════════════════════════════════════════════════════════
// #17 WxCpCorpGroupCorpGetTokenReqTest —— 获取 token 请求体验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpCorpGroupCorpGetTokenReqTest（获取 token 请求 JSON 构建）
#[test]
fn test_corp_group_corp_get_token_req_from_json() {
    let json_str = r#"{
        "corpid": "wx1234567890",
        "business_type": 1,
        "agentid": 1000002
    }"#;
    let req: WxCpCorpGroupCorpGetTokenReq =
        serde_json::from_str(json_str).expect("解析获取 token 请求");
    assert_eq!(req.corp_id, "wx1234567890");
    assert_eq!(req.business_type, 1);
    assert_eq!(req.agent_id, 1000002);
}

/// 对应 Java: WxCpCorpGroupCorpGetTokenReqTest（获取 token 请求序列化往返）
#[test]
fn test_corp_group_corp_get_token_req_roundtrip() {
    let json_str = r#"{
        "corpid": "wx9999999999",
        "business_type": 0,
        "agentid": 50001
    }"#;
    let req: WxCpCorpGroupCorpGetTokenReq = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&req).expect("序列化");
    let req2: WxCpCorpGroupCorpGetTokenReq = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(req, req2);
}

/// 对应 Java: WxCpCorpGroupCorpGetTokenReqTest（获取 token 请求默认值验证）
#[test]
fn test_corp_group_corp_get_token_req_default() {
    let json_str = r#"{}"#;
    let req: WxCpCorpGroupCorpGetTokenReq = serde_json::from_str(json_str).expect("解析空 JSON");
    assert_eq!(req.corp_id, "");
    assert_eq!(req.business_type, 0);
    assert_eq!(req.agent_id, 0);
}
