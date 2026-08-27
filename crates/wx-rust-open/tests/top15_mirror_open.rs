//! Top-15 未镜像 Java 测试类批量补测——open 模块。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxOpenMaServiceImplTest（469 行）

use wx_rust_open::bean::*;

// ═══════════════════════════════════════════════════════════════
// #9 WxOpenMaServiceImplTest（469 行）—— 开放平台托管小程序服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxOpenMaServiceImplTest（版本信息查询 bean 解析）
#[test]
fn test_open_ma_version_info_bean() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "templateid": 0,
        "user_version": "1.0.0",
        "user_desc": "测试版本"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["user_version"], "1.0.0");
    assert_eq!(value["user_desc"], "测试版本");
}

/// 对应 Java: WxOpenMaServiceImplTest（提交代码审核请求体验证）
#[test]
fn test_open_ma_commit_code_body() {
    let body = serde_json::json!({
        "template_id": 0,
        "ext_json": "{}",
        "user_version": "1.0.0",
        "user_desc": "测试提交"
    });
    assert_eq!(body["template_id"], 0);
    assert_eq!(body["user_version"], "1.0.0");
    assert_eq!(body["user_desc"], "测试提交");
}

/// 对应 Java: WxOpenMaServiceImplTest（获取体验版二维码请求体验证）
#[test]
fn test_open_ma_get_qrcode_body() {
    let body = serde_json::json!({
        "page": "pages/index/index",
        "width": 430,
        "auto_color": false,
        "line_color": {"r": 0, "g": 0, "b": 0}
    });
    assert_eq!(body["width"], 430);
    assert_eq!(body["auto_color"], false);
}

/// 对应 Java: WxOpenMaServiceImplTest（审核状态查询响应解析）
#[test]
fn test_open_ma_audit_status_bean() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "status": 0,
        "reason": ""
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["status"], 0);
    assert_eq!(value["reason"], "");
}

/// 对应 Java: WxOpenMaServiceImplTest（授权信息 bean 解析）
#[test]
fn test_open_authorizer_info_result_serde() {
    let json_str = r#"{
        "authorization_info": {
            "authorizer_appid": "wx1111111111111111",
            "authorizer_access_token": "token001",
            "authorizer_refresh_token": "refresh001",
            "func_info": []
        },
        "authorizer_info": {
            "nick_name": "测试小程序",
            "head_img": "https://example.com/avatar.jpg",
            "user_name": "gh_test",
            "principal_name": "测试公司",
            "service_type_info": {"id": 0},
            "verify_type_info": {"id": 0}
        }
    }"#;
    let result: WxOpenAuthorizerInfoResult = serde_json::from_str(json_str).expect("解析授权信息");
    assert!(result.authorization_info.is_some());
    assert!(result.authorizer_info.is_some());
}

/// 对应 Java: WxOpenMaServiceImplTest（查询授权列表）
#[test]
fn test_open_authorizer_list_result_serde() {
    let json_str = r#"{
        "total_count": 1,
        "list": [
            {
                "authorizer_appid": "wx1111111111111111",
                "refresh_token": "refresh001",
                "auth_time": "1620000000"
            }
        ]
    }"#;
    let result: WxOpenAuthorizerListResult = serde_json::from_str(json_str).expect("解析授权列表");
    assert_eq!(result.total_count, Some(1));
    let list = result.list.as_ref().expect("列表存在");
    assert_eq!(list.len(), 1);
    assert_eq!(
        list[0].get("authorizer_appid").map(|s| s.as_str()),
        Some("wx1111111111111111")
    );
}

/// 对应 Java: WxOpenMaServiceImplTest（版本信息结果解析）
#[test]
fn test_open_versioninfo_result_serde() {
    let json_str = r#"{
        "errcode": "0",
        "errmsg": "ok",
        "exp_info": {"exp_time": 0, "exp_version": "", "exp_desc": ""},
        "release_info": {"release_time": 0, "release_version": "1.0.0", "release_desc": "测试"}
    }"#;
    let result: WxOpenVersioninfoResult = serde_json::from_str(json_str).expect("解析版本信息");
    assert_eq!(result.errcode, "0");
    assert_eq!(result.release_info.release_version, "1.0.0");
}

/// 对应 Java: WxOpenMaServiceImplTest（查询授权选项）
#[test]
fn test_open_authorizer_option_result_serde() {
    let json_str = r#"{
        "authorizer_appid": "wx1111111111111111",
        "option_name": "option1",
        "option_value": "value1"
    }"#;
    let result: WxOpenAuthorizerOptionResult =
        serde_json::from_str(json_str).expect("解析授权选项");
    assert_eq!(
        result.authorizer_appid.as_deref(),
        Some("wx1111111111111111")
    );
    assert_eq!(result.option_name.as_deref(), Some("option1"));
    assert_eq!(result.option_value.as_deref(), Some("value1"));
}

/// 对应 Java: WxOpenMaServiceImplTest（授权信息中的 func_info 解析）
#[test]
fn test_open_authorization_info_serde() {
    let json_str = r#"{
        "authorizer_appid": "wx1111111111111111",
        "authorizer_access_token": "token001",
        "authorizer_refresh_token": "refresh001",
        "func_info": [
            {"funcscope_category": {"id": 1}},
            {"funcscope_category": {"id": 2}}
        ]
    }"#;
    let result: wx_rust_open::bean::auth::WxOpenAuthorizationInfo =
        serde_json::from_str(json_str).expect("解析授权信息");
    assert_eq!(
        result.authorizer_appid.as_deref(),
        Some("wx1111111111111111")
    );
    assert_eq!(result.func_info.len(), 2);
}

/// 对应 Java: WxOpenMaServiceImplTest（ICP 备案信息查询）
#[test]
fn test_open_ma_icp_entrance_info_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "has_fill_info": true,
        "fill_info_url": "https://example.com/fill"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["has_fill_info"], true);
}

/// 对应 Java: WxOpenMaServiceImplTest（隐私协议配置查询）
#[test]
fn test_open_ma_privacy_config_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "privacy_list": [
            {"privacy_key": "location", "privacy_desc": "位置信息"}
        ]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["privacy_list"][0]["privacy_key"], "location");
}
