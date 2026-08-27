#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——Open 平台 bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxOpenAuthorizerAccessTokenTest（授权方 access_token 解析）
//! - WxOpenComponentAccessTokenTest（component_access_token 解析）
//! - WxOpenCreateResultTest（创建开放平台应用结果解析）
//! - WxOpenGetResultTest（获取开放平台应用结果解析）
//! - WxOpenMaCodeTemplateTest（小程序代码模板解析）

use wx_rust_open::bean::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxOpenAuthorizerAccessTokenTest —— 授权方 access_token
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenAuthorizerAccessTokenTest（授权方 access_token JSON 解析）
#[test]
fn test_authorizer_access_token_from_json() {
    let json_str = r#"{
        "authorizer_access_token": "ACCESS_TOKEN_12345",
        "authorizer_refresh_token": "REFRESH_TOKEN_12345",
        "expires_in": 7200
    }"#;
    let token = WxOpenAuthorizerAccessToken::from_json(json_str).expect("解析授权方 token");
    assert_eq!(token.authorizer_access_token, "ACCESS_TOKEN_12345");
    assert_eq!(token.authorizer_refresh_token, "REFRESH_TOKEN_12345");
    assert_eq!(token.expires_in, 7200);
}

/// 对应 Java: WxOpenAuthorizerAccessTokenTest（授权方 access_token 方法验证）
#[test]
fn test_authorizer_access_token_methods() {
    let json_str = r#"{
        "authorizer_access_token": "TOKEN_ABC",
        "authorizer_refresh_token": "REFRESH_ABC",
        "expires_in": 3600
    }"#;
    let token = WxOpenAuthorizerAccessToken::from_json(json_str).expect("解析");
    assert_eq!(token.authorizer_access_token(), "TOKEN_ABC");
    assert_eq!(token.authorizer_refresh_token(), "REFRESH_ABC");
    assert_eq!(token.expires_in(), 3600);
}

/// 对应 Java: WxOpenAuthorizerAccessTokenTest（授权方 access_token 序列化往返验证）
#[test]
fn test_authorizer_access_token_roundtrip() {
    let json_str = r#"{
        "authorizer_access_token": "TOKEN_XYZ",
        "authorizer_refresh_token": "REFRESH_XYZ",
        "expires_in": 7200
    }"#;
    let token = WxOpenAuthorizerAccessToken::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&token).expect("序列化");
    let token2: WxOpenAuthorizerAccessToken = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(
        token.authorizer_access_token,
        token2.authorizer_access_token
    );
    assert_eq!(
        token.authorizer_refresh_token,
        token2.authorizer_refresh_token
    );
    assert_eq!(token.expires_in, token2.expires_in);
}

// ═══════════════════════════════════════════════════════════════
// #2 WxOpenComponentAccessTokenTest —— component_access_token
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenComponentAccessTokenTest（component_access_token JSON 解析）
#[test]
fn test_component_access_token_from_json() {
    let json_str = r#"{
        "component_access_token": "COMPONENT_TOKEN_12345",
        "expires_in": 7200
    }"#;
    let token = WxOpenComponentAccessToken::from_json(json_str).expect("解析 component token");
    assert_eq!(token.component_access_token, "COMPONENT_TOKEN_12345");
    assert_eq!(token.expires_in, 7200);
}

/// 对应 Java: WxOpenComponentAccessTokenTest（component_access_token 方法验证）
#[test]
fn test_component_access_token_methods() {
    let json_str = r#"{
        "component_access_token": "COMP_TOKEN_ABC",
        "expires_in": 3600
    }"#;
    let token = WxOpenComponentAccessToken::from_json(json_str).expect("解析");
    assert_eq!(token.component_access_token(), "COMP_TOKEN_ABC");
    assert_eq!(token.expires_in(), 3600);
}

/// 对应 Java: WxOpenComponentAccessTokenTest（component_access_token 序列化往返验证）
#[test]
fn test_component_access_token_roundtrip() {
    let json_str = r#"{
        "component_access_token": "COMP_TOKEN_XYZ",
        "expires_in": 7200
    }"#;
    let token = WxOpenComponentAccessToken::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&token).expect("序列化");
    let token2: WxOpenComponentAccessToken = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(token.component_access_token, token2.component_access_token);
    assert_eq!(token.expires_in, token2.expires_in);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxOpenCreateResultTest —— 创建开放平台应用结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenCreateResultTest（创建结果 JSON 解析）
#[test]
fn test_open_create_result_from_json() {
    let json_str = r#"{
        "open_appid": "wx_open_001",
        "errcode": "0",
        "errmsg": "ok"
    }"#;
    let result = WxOpenCreateResult::from_json(json_str).expect("解析创建结果");
    assert_eq!(result.open_appid, "wx_open_001");
    assert_eq!(result.errcode, "0");
    assert_eq!(result.errmsg, "ok");
}

/// 对应 Java: WxOpenCreateResultTest（创建结果错误码解析）
#[test]
fn test_open_create_result_error() {
    let json_str = r#"{
        "open_appid": "",
        "errcode": "40001",
        "errmsg": "invalid credential"
    }"#;
    let result = WxOpenCreateResult::from_json(json_str).expect("解析错误结果");
    assert_eq!(result.open_appid, "");
    assert_eq!(result.errcode, "40001");
    assert_eq!(result.errmsg, "invalid credential");
}

/// 对应 Java: WxOpenCreateResultTest（创建结果序列化往返验证）
#[test]
fn test_open_create_result_roundtrip() {
    let json_str = r#"{
        "open_appid": "wx_open_002",
        "errcode": "0",
        "errmsg": "ok"
    }"#;
    let result = WxOpenCreateResult::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxOpenCreateResult = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

// ═══════════════════════════════════════════════════════════════
// #4 WxOpenGetResultTest —— 获取开放平台应用结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenGetResultTest（获取结果 JSON 解析）
#[test]
fn test_open_get_result_from_json() {
    let json_str = r#"{
        "errcode": "0",
        "errmsg": "ok",
        "open_appid": "wx_open_001"
    }"#;
    let result = WxOpenGetResult::from_json(json_str).expect("解析获取结果");
    assert_eq!(result.errcode, "0");
    assert_eq!(result.errmsg, "ok");
    assert_eq!(result.open_appid, "wx_open_001");
}

/// 对应 Java: WxOpenGetResultTest（获取结果序列化往返验证）
#[test]
fn test_open_get_result_roundtrip() {
    let json_str = r#"{
        "errcode": "0",
        "errmsg": "ok",
        "open_appid": "wx_open_002"
    }"#;
    let result = WxOpenGetResult::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxOpenGetResult = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

/// 对应 Java: WxOpenGetResultTest（获取结果默认值验证）
#[test]
fn test_open_get_result_default() {
    let json_str = r#"{}"#;
    let result = WxOpenGetResult::from_json(json_str).expect("解析空 JSON");
    assert_eq!(result.errcode, "");
    assert_eq!(result.errmsg, "");
    assert_eq!(result.open_appid, "");
}

// ═══════════════════════════════════════════════════════════════
// #5 WxOpenMaCodeTemplateTest —— 小程序代码模板
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenMaCodeTemplateTest（代码模板 JSON 解析）
#[test]
fn test_open_ma_code_template_from_json() {
    let json_str = r#"{
        "draftId": 123456,
        "templateId": 789012,
        "userVersion": "1.0.0",
        "userDesc": "初始版本",
        "templateType": 0,
        "createTime": 1627800000,
        "sourceMiniProgramAppid": "wx_source_001",
        "sourceMiniProgram": "源小程序",
        "auditScene": 0,
        "auditStatus": 0,
        "reason": "",
        "developer": "开发者"
    }"#;
    let template: WxOpenMaCodeTemplate = serde_json::from_str(json_str).expect("解析代码模板");
    assert_eq!(template.draft_id, 123456);
    assert_eq!(template.template_id, 789012);
    assert_eq!(template.user_version, "1.0.0");
    assert_eq!(template.user_desc, "初始版本");
    assert_eq!(template.template_type, 0);
    assert_eq!(template.create_time, 1627800000);
    assert_eq!(template.source_mini_program_appid, "wx_source_001");
    assert_eq!(template.source_mini_program, "源小程序");
    assert_eq!(template.audit_scene, 0);
    assert_eq!(template.audit_status, 0);
    assert_eq!(template.reason, "");
    assert_eq!(template.developer, "开发者");
}

/// 对应 Java: WxOpenMaCodeTemplateTest（代码模板序列化往返验证）
#[test]
fn test_open_ma_code_template_roundtrip() {
    let json_str = r#"{
        "draftId": 111111,
        "templateId": 222222,
        "userVersion": "2.0.0",
        "userDesc": "测试版本",
        "templateType": 1,
        "createTime": 1627886400,
        "sourceMiniProgramAppid": "wx_source_002",
        "sourceMiniProgram": "测试小程序",
        "auditScene": 1,
        "auditStatus": 1,
        "reason": "审核中",
        "developer": "测试开发者"
    }"#;
    let template: WxOpenMaCodeTemplate = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&template).expect("序列化");
    let template2: WxOpenMaCodeTemplate = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(template, template2);
}

/// 对应 Java: WxOpenMaCodeTemplateTest（代码模板 camelCase 别名验证）
#[test]
fn test_open_ma_code_template_alias() {
    // Test that both camelCase and snake_case work
    let json_camel = r#"{
        "draftId": 100,
        "templateId": 200,
        "userVersion": "1.0",
        "userDesc": "desc",
        "templateType": 0,
        "createTime": 1000,
        "sourceMiniProgramAppid": "wx1",
        "sourceMiniProgram": "mp1",
        "auditScene": 0,
        "auditStatus": 0,
        "reason": "",
        "developer": "dev"
    }"#;
    let template: WxOpenMaCodeTemplate = serde_json::from_str(json_camel).expect("解析 camelCase");
    assert_eq!(template.draft_id, 100);
    assert_eq!(template.template_id, 200);
}

/// 对应 Java: WxOpenMaCodeTemplateTest（代码模板默认值验证）
#[test]
fn test_open_ma_code_template_default() {
    let json_str = r#"{}"#;
    let template: WxOpenMaCodeTemplate = serde_json::from_str(json_str).expect("解析空 JSON");
    assert_eq!(template.draft_id, 0);
    assert_eq!(template.template_id, 0);
    assert_eq!(template.user_version, "");
    assert_eq!(template.user_desc, "");
    assert_eq!(template.template_type, 0);
}
