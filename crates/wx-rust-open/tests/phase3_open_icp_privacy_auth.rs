//! Phase 3 P2 扩展: open ICP 备案/隐私设置/授权方信息/购物订单 Bean 测试。
//!
//! 镜像 Java:
//! - `WxOpenMaIcpServiceImplTest`（ICP 备案申请/查询/验证任务）
//! - `WxOpenMaPrivacyServiceImplTest`（隐私设置查询/设置/上传）
//! - `WxOpenAuthorizerInfoTest`（授权方信息解析/小程序信息）
//! - `WxOpenShoppingOrdersServiceImplTest`（购物订单上传/确认）
//! - `WxOpenComponentAccessTokenTest`（component_access_token 解析）
//! - `WxOpenMaCodeTemplateTest`（代码模板查询）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde rename、自定义反序列化（func_info 扁平化等）
//! - VALUE_ADD: 空值/边界/默认值路径

use wx_rust_open::bean::auth::*;
use wx_rust_open::bean::icp::*;
use wx_rust_open::bean::ma::privacy::*;
use wx_rust_open::bean::result::*;
use wx_rust_open::bean::shoppingOrders::*;
use wx_rust_open::bean::wx_open_component_access_token::WxOpenComponentAccessToken;

// ═══════════════════════════════════════════════════════════════
// 1. 授权方信息（SOURCE_PARITY:
//    Java WxOpenAuthorizerInfoTest）
// ═══════════════════════════════════════════════════════════════

/// 授权方信息 serde（对应 Java `WxOpenAuthorizerInfo`：
/// `nick_name`/`head_img`/`service_type_info`(`{"id":N}` → i32)/
/// `verify_type_info`/`user_name`/`principal_name`/`business_info`/
/// `alias`/`qrcode_url`/`account_status`/`signature`/
/// `MiniProgramInfo`/`register_type`）。
/// 对应 Java: WxOpenAuthorizerInfoTest.testGetAuthorizerInfo
#[test]
fn test_authorizer_info_serde() {
    let json = r#"{
        "nick_name":"测试公众号",
        "head_img":"https://example.com/avatar.jpg",
        "service_type_info":{"id":2},
        "verify_type_info":{"id":0},
        "user_name":"gh_test",
        "principal_name":"测试主体",
        "business_info":{"open_store":1,"open_scan":0},
        "alias":"test_alias",
        "qrcode_url":"https://example.com/qr.jpg",
        "account_status":1,
        "signature":"测试签名",
        "register_type":0
    }"#;
    let info: WxOpenAuthorizerInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.nick_name.as_deref(), Some("测试公众号"));
    assert_eq!(info.service_type_info, Some(2));
    assert_eq!(info.verify_type_info, Some(0));
    assert_eq!(info.user_name.as_deref(), Some("gh_test"));
    assert_eq!(info.account_status, Some(1));
    let biz = info.business_info.as_ref().unwrap();
    assert_eq!(biz.get("open_store"), Some(&1));
}

/// 授权方信息含小程序信息（对应 Java `MiniProgramInfo`：
/// `visit_status`/`network`/`categories`）。
/// 对应 Java: WxOpenAuthorizerInfoTest (mini program info)
#[test]
fn test_authorizer_info_with_mini_program() {
    let json = r#"{
        "nick_name":"测试小程序",
        "service_type_info":{"id":0},
        "verify_type_info":{"id":0},
        "MiniProgramInfo":{
            "visit_status":1,
            "network":{"RequestDomain":["https://a.com"],"WsRequestDomain":[],"UploadDomain":[],"DownloadDomain":[]},
            "categories":[{"left":"工具","right":"生活服务"}]
        }
    }"#;
    let info: WxOpenAuthorizerInfo = serde_json::from_str(json).unwrap();
    let mp = info.mini_program_info.as_ref().unwrap();
    assert_eq!(mp.visit_status, Some(1));
}

/// 授权方信息默认值。
#[test]
fn test_authorizer_info_default() {
    let info: WxOpenAuthorizerInfo = serde_json::from_str("{}").unwrap();
    assert!(info.nick_name.is_none());
    assert!(info.mini_program_info.is_none());
    assert!(info.business_info.is_none());
}

// ═══════════════════════════════════════════════════════════════
// 2. 授权信息（SOURCE_PARITY:
//    Java WxOpenAuthorizationInfoTest）
// ═══════════════════════════════════════════════════════════════

/// 授权信息 serde（对应 Java `WxOpenAuthorizationInfo`：
/// `authorizer_appid`/`authorizer_access_token`/`expires_in`/
/// `authorizer_refresh_token`/`func_info`（`[{"funcscope_category":{"id":N}}]`
/// → `[N]` 扁平化））。
/// 对应 Java: WxOpenAuthorizationInfoTest
#[test]
fn test_authorization_info_serde() {
    let json = r#"{
        "authorizer_appid":"wx1234",
        "authorizer_access_token":"token123",
        "expires_in":7200,
        "authorizer_refresh_token":"refresh123",
        "func_info":[
            {"funcscope_category":{"id":1}},
            {"funcscope_category":{"id":2}},
            {"funcscope_category":{"id":3}}
        ]
    }"#;
    let info: WxOpenAuthorizationInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.authorizer_appid.as_deref(), Some("wx1234"));
    assert_eq!(info.expires_in, Some(7200));
    assert_eq!(info.func_info, vec![1, 2, 3]);
}

/// 授权信息空 func_info。
#[test]
fn test_authorization_info_empty_func_info() {
    let json = r#"{"authorizer_appid":"wx1234","func_info":[]}"#;
    let info: WxOpenAuthorizationInfo = serde_json::from_str(json).unwrap();
    assert!(info.func_info.is_empty());
}

/// 授权信息默认值。
#[test]
fn test_authorization_info_default() {
    let info: WxOpenAuthorizationInfo = serde_json::from_str("{}").unwrap();
    assert!(info.authorizer_appid.is_none());
    assert!(info.func_info.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 3. 查询授权结果（SOURCE_PARITY:
//    Java WxOpenQueryAuthResultTest）
// ═══════════════════════════════════════════════════════════════

/// 查询授权结果 serde（对应 Java `WxOpenQueryAuthResult`：
/// `authorization_info`）。
/// 对应 Java: WxOpenQueryAuthResultTest
#[test]
fn test_query_auth_result_serde() {
    let json = r#"{
        "authorization_info":{
            "authorizer_appid":"wx1234",
            "authorizer_access_token":"token123",
            "expires_in":7200,
            "func_info":[{"funcscope_category":{"id":1}}]
        }
    }"#;
    let result: WxOpenQueryAuthResult = serde_json::from_str(json).unwrap();
    let auth = result.authorization_info.as_ref().unwrap();
    assert_eq!(auth.authorizer_appid.as_deref(), Some("wx1234"));
    assert_eq!(auth.func_info, vec![1]);
}

// ═══════════════════════════════════════════════════════════════
// 4. component_access_token（SOURCE_PARITY:
//    Java WxOpenComponentAccessTokenTest）
// ═══════════════════════════════════════════════════════════════

/// component_access_token serde（对应 Java `WxOpenComponentAccessToken`：
/// `component_access_token`/`expires_in`）。
/// 对应 Java: WxOpenComponentAccessTokenTest
#[test]
fn test_component_access_token_serde() {
    let json = r#"{"component_access_token":"comp_token_123","expires_in":7200}"#;
    let token: WxOpenComponentAccessToken = serde_json::from_str(json).unwrap();
    assert_eq!(token.component_access_token(), "comp_token_123");
    assert_eq!(token.expires_in(), 7200);
}

/// component_access_token from_json 方法。
#[test]
fn test_component_access_token_from_json() {
    let json = r#"{"component_access_token":"comp_token_456","expires_in":3600}"#;
    let token = WxOpenComponentAccessToken::from_json(json).unwrap();
    assert_eq!(token.component_access_token(), "comp_token_456");
}

// ═══════════════════════════════════════════════════════════════
// 5. 授权方信息结果（SOURCE_PARITY:
//    Java WxOpenAuthorizerInfoResultTest）
// ═══════════════════════════════════════════════════════════════

/// 授权方信息结果 serde（对应 Java `WxOpenAuthorizerInfoResult`）。
/// 对应 Java: WxOpenAuthorizerInfoResultTest
#[test]
fn test_authorizer_info_result_serde() {
    let json = r#"{
        "authorizer_info":{
            "nick_name":"测试",
            "service_type_info":{"id":2},
            "verify_type_info":{"id":0}
        },
        "authorization_info":{
            "authorizer_appid":"wx1234",
            "func_info":[{"funcscope_category":{"id":1}}]
        }
    }"#;
    let result: WxOpenAuthorizerInfoResult = serde_json::from_str(json).unwrap();
    assert!(result.authorizer_info.is_some());
    assert!(result.authorization_info.is_some());
}

// ═══════════════════════════════════════════════════════════════
// 6. 授权方选项结果（SOURCE_PARITY:
//    Java WxOpenAuthorizerOptionResultTest）
// ═══════════════════════════════════════════════════════════════

/// 授权方选项结果 serde（对应 Java `WxOpenAuthorizerOptionResult`）。
/// 对应 Java: WxOpenAuthorizerOptionResultTest
#[test]
fn test_authorizer_option_result_serde() {
    let json = r#"{"option_name":"option1","option_value":"value1"}"#;
    let result: WxOpenAuthorizerOptionResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.option_name.as_deref(), Some("option1"));
    assert_eq!(result.option_value.as_deref(), Some("value1"));
}

// ═══════════════════════════════════════════════════════════════
// 7. 授权方列表结果（SOURCE_PARITY:
//    Java WxOpenAuthorizerListResultTest）
// ═══════════════════════════════════════════════════════════════

/// 授权方列表结果 serde（对应 Java `WxOpenAuthorizerListResult`：
/// `total_count`/`list`，每项为 `HashMap<String, String>`）。
/// 对应 Java: WxOpenAuthorizerListResultTest
#[test]
fn test_authorizer_list_result_serde() {
    let json = r#"{
        "total_count":2,
        "list":[
            {"authorizer_appid":"wx1234"},
            {"authorizer_appid":"wx5678"}
        ]
    }"#;
    let result: WxOpenAuthorizerListResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(2));
    let list = result.list.as_ref().unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(
        list[0].get("authorizer_appid").map(|s| s.as_str()),
        Some("wx1234")
    );
}

// ═══════════════════════════════════════════════════════════════
// 8. 隐私设置（SOURCE_PARITY:
//    Java WxOpenMaPrivacyServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// 隐私设置查询结果 serde（对应 Java `GetPrivacySettingResult`：
/// `errcode`/`errmsg`/`code_exist`/`privacy_list`/`setting_list`/
/// `update_time`/`owner_setting`/`privacy_desc`）。
/// 对应 Java: WxOpenMaPrivacyServiceImplTest.testGetPrivacySetting
#[test]
fn test_get_privacy_setting_result_serde() {
    let json = r#"{
        "errcode":"0",
        "errmsg":"ok",
        "code_exist":1,
        "privacy_list":["location","camera"],
        "setting_list":[
            {"privacy_key":"location","privacy_text":"位置信息","privacy_label":"用于定位"}
        ],
        "update_time":1662480000
    }"#;
    let result: GetPrivacySettingResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.code_exist, 1);
    assert_eq!(result.privacy_list.len(), 2);
    assert_eq!(result.setting_list.len(), 1);
    assert_eq!(result.setting_list[0].privacy_key, "location");
    assert_eq!(result.update_time, 1662480000);
}

/// 隐私设置查询结果默认值。
#[test]
fn test_get_privacy_setting_result_default() {
    let result: GetPrivacySettingResult = serde_json::from_str("{}").unwrap();
    assert!(result.errcode.is_empty());
    assert!(result.privacy_list.is_empty());
    assert!(result.setting_list.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 9. ICP 备案（SOURCE_PARITY:
//    Java WxOpenMaIcpServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// ICP 备案申请参数 serde（对应 Java `WxOpenApplyIcpFilingParam`：
/// `icp_subject`/`icp_applets`/`icp_materials`，SubjectBaseInfo 含 `type`/
/// `name`/`province`/`city`/`address`/`zip_code`/`contact_name` 等）。
/// 对应 Java: WxOpenMaIcpServiceImplTest.testApplyIcpFiling
#[test]
fn test_apply_icp_filing_param_serde() {
    let json = r#"{
        "icp_subject":{
            "base_info":{"type":1,"name":"测试主体","province":"上海","city":"上海","address":"浦东新区","zip_code":"200000","contact_name":"张三","contact_phone":"13800138000","contact_email":"test@example.com"}
        },
        "icp_applets":{
            "base_info":{"name":"测试小程序","app_id":"wx1234","service_content_types":[1]}
        },
        "icp_materials":{
            "base_info":{}
        }
    }"#;
    let param: WxOpenApplyIcpFilingParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.icp_subject.base_info.name, "测试主体");
    assert_eq!(param.icp_subject.base_info.province, "上海");
    assert_eq!(param.icp_applets.base_info.name, "测试小程序");
}

/// ICP 备案申请结果 serde（对应 Java `WxOpenApplyIcpFilingResult`）。
#[test]
fn test_apply_icp_filing_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenApplyIcpFilingResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

/// ICP 验证任务创建参数 serde（对应 Java `WxOpenCreateIcpVerifyTaskParam`：
/// `along_with_auth`）。
/// 对应 Java: WxOpenMaIcpServiceImplTest.testCreateIcpVerifyTask
#[test]
fn test_create_icp_verify_task_param_serde() {
    let json = r#"{"along_with_auth":true}"#;
    let param: WxOpenCreateIcpVerifyTaskParam = serde_json::from_str(json).unwrap();
    assert!(param.along_with_auth);
}

/// ICP 验证任务结果 serde（对应 Java `WxOpenIcpCreateIcpVerifyTaskResult`：
/// `errcode`/`errmsg`/`task_id`/`verify_url`）。
/// 对应 Java: WxOpenMaIcpServiceImplTest (verify task result)
#[test]
fn test_icp_verify_task_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","task_id":"T-001","verify_url":"https://example.com/verify"}"#;
    let result: WxOpenIcpCreateIcpVerifyTaskResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.task_id, "T-001");
    assert_eq!(result.verify_url, "https://example.com/verify");
}

/// ICP 入口信息结果 serde（对应 Java `WxOpenIcpEntranceInfoResult`：
/// `errcode`/`errmsg`/`info`，`Info` 含 `status`/`canceling`/`available`/
/// `sms_verify_status`/`audit_data`）。
/// 对应 Java: WxOpenMaIcpServiceImplTest (entrance info)
#[test]
fn test_icp_entrance_info_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","info":{"status":1,"canceling":false,"available":1,"sms_verify_status":0,"audit_data":[]}}"#;
    let result: WxOpenIcpEntranceInfoResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.info.status, 1);
    assert!(!result.info.canceling);
}

// ═══════════════════════════════════════════════════════════════
// 10. 购物订单（SOURCE_PARITY:
//     Java WxOpenShoppingOrdersServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// 购物订单信息 serde（对应 Java `ShoppingInfo`：
/// `order_key`/`order_list`/`payer`/`logistics_type`/`upload_time`）。
/// 对应 Java: WxOpenShoppingOrdersServiceImplTest.testUploadShoppingInfo
#[test]
fn test_shopping_info_serde() {
    let json = r#"{
        "order_key":{"order_number_type":1,"transaction_id":"4200001234"},
        "order_list":[
            {"merchant_order_no":"MO-001","order_detail_jump_link":{"url":"https://example.com","appid":"wx1234","path":"/pages/order","type":1},"item_list":[]}
        ],
        "payer":{"openid":"ox123"},
        "logistics_type":1,
        "upload_time":"2024-01-01T12:00:00+08:00"
    }"#;
    let info: ShoppingInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.order_key.transaction_id, "4200001234");
    assert_eq!(info.order_list.len(), 1);
    assert_eq!(info.order_list[0].merchant_order_no, "MO-001");
    assert_eq!(info.payer.openid, "ox123");
    assert_eq!(info.logistics_type, 1);
}

/// 购物订单信息默认值。
#[test]
fn test_shopping_info_default() {
    let info = ShoppingInfo::default();
    assert!(info.order_list.is_empty());
    assert_eq!(info.logistics_type, 0);
}

/// 购物订单确认结果 serde（对应 Java `WxOpenShoppingOrdersConfirmResult`）。
#[test]
fn test_shopping_orders_confirm_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenShoppingOrdersConfirmResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

/// 购物信息验证上传结果 serde（对应 Java `WxOpenShoppingInfoVerifyUploadResult`）。
#[test]
fn test_shopping_info_verify_upload_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenShoppingInfoVerifyUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

// ═══════════════════════════════════════════════════════════════
// 11. 小程序版本信息（SOURCE_PARITY:
//     Java WxOpenVersionInfoResultTest）
// ═══════════════════════════════════════════════════════════════

/// 小程序版本信息结果 serde（对应 Java `WxOpenVersioninfoResult`：
/// `errcode`/`errmsg`/`exp_info`/`release_info`）。
/// 对应 Java: WxOpenVersionInfoResultTest
#[test]
fn test_version_info_result_serde() {
    let json = r#"{
        "errcode":"0",
        "errmsg":"ok",
        "exp_info":{"exp_time":1662480000,"exp_version":"1.0.0","exp_desc":"体验版"},
        "release_info":{"release_time":1662480100,"release_version":"1.0.0","release_desc":"正式版"}
    }"#;
    let result: WxOpenVersioninfoResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.release_info.release_version, "1.0.0");
    assert_eq!(result.exp_info.exp_version, "1.0.0");
}

// ═══════════════════════════════════════════════════════════════
// 12. 快速注册小程序（SOURCE_PARITY:
//     Java WxOpenRegisterBetaWeappResultTest）
// ═══════════════════════════════════════════════════════════════

/// 快速注册小程序结果 serde（对应 Java `WxOpenRegisterBetaWeappResult`）。
/// 对应 Java: WxOpenRegisterBetaWeappResultTest
#[test]
fn test_register_beta_weapp_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","authorizer_appid":"wx1234"}"#;
    let result: WxOpenRegisterBetaWeappResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 授权信息 func_info 缺失字段跳过。
#[test]
fn test_authorization_info_malformed_func_info() {
    let json = r#"{"func_info":[{"no_funcscope_category":true},{"funcscope_category":{"id":5}}]}"#;
    let info: WxOpenAuthorizationInfo = serde_json::from_str(json).unwrap();
    // 第一个元素缺少 funcscope_category，被跳过；第二个正常解析
    assert_eq!(info.func_info, vec![5]);
}

/// 授权方列表结果默认值。
#[test]
fn test_authorizer_list_result_default() {
    let result: WxOpenAuthorizerListResult = serde_json::from_str("{}").unwrap();
    assert!(result.list.is_none());
}

/// component_access_token 默认值。
#[test]
fn test_component_access_token_default() {
    let token = WxOpenComponentAccessToken {
        component_access_token: String::new(),
        expires_in: -1,
    };
    assert_eq!(token.expires_in(), -1);
    assert!(token.component_access_token().is_empty());
}

/// 购物订单 OrderKeyBean serde。
#[test]
fn test_order_key_bean_serde() {
    let json = r#"{"order_number_type":1,"transaction_id":"4200001234"}"#;
    let key: OrderKeyBean = serde_json::from_str(json).unwrap();
    assert_eq!(key.order_number_type, 1);
    assert_eq!(key.transaction_id, "4200001234");
}

/// 购物订单 PayerBean serde。
#[test]
fn test_payer_bean_serde() {
    let json = r#"{"openid":"ox123"}"#;
    let payer: PayerBean = serde_json::from_str(json).unwrap();
    assert_eq!(payer.openid, "ox123");
}

/// 隐私设置 Setting serde。
#[test]
fn test_privacy_setting_serde() {
    let json = r#"{"privacy_key":"camera","privacy_text":"相机","privacy_label":"用于拍照"}"#;
    let setting: Setting = serde_json::from_str(json).unwrap();
    assert_eq!(setting.privacy_key, "camera");
    assert_eq!(setting.privacy_text, "相机");
}
