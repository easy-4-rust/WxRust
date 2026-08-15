//! Phase 1 Batch 1.3: wx-rust-miniapp 核心测试
//!
//! 镜像 Java WxMaServiceImplTest / WxMaCryptUtilsTest / WxMaUserServiceImplTest

use wx_rust_miniapp::bean::analysis::*;
use wx_rust_miniapp::bean::scheme::*;
use wx_rust_miniapp::bean::*;

// ═══ WxMaJscode2SessionResult ═══

#[test]
fn test_jscode2session_serde() {
    let json = r#"{"openid":"ox123","session_key":"sk456","unionid":"union1"}"#;
    let result: WxMaJscode2SessionResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.openid, "ox123");
    assert_eq!(result.session_key, "sk456");
}

// ═══ WxMaUserInfo ═══

#[test]
fn test_user_info_serde() {
    let json = r#"{"openId":"ox123","nickName":"test","gender":"1","city":"深圳","province":"广东","country":"中国","avatarUrl":"http://img.example.com","unionId":"union1"}"#;
    let info: WxMaUserInfo = serde_json::from_str(json).unwrap();

    assert_eq!(info.nick_name, "test");
    // Note: WxMaUserInfo does not have open_id field
}

// ═══ WxMaPhoneNumberInfo ═══

#[test]
fn test_phone_number_info_serde() {
    let json = r#"{"phoneNumber":"+8613800138000","purePhoneNumber":"13800138000","countryCode":"86","watermark":{"timestamp":1700000000,"appid":"wx1234"}}"#;
    let info: WxMaPhoneNumberInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.phone_number, "+8613800138000");
    assert_eq!(info.pure_phone_number, "13800138000");
    assert_eq!(info.country_code, "86");
}

// ═══ WxMaBaseResponse ═══

#[test]
fn test_base_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxMaBaseResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
}

#[test]
fn test_base_response_error() {
    let json = r#"{"errcode":40001,"errmsg":"invalid credential"}"#;
    let resp: WxMaBaseResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 40001);
}

// ═══ WxMaQrcode ═══

#[test]
fn test_qrcode_serde() {
    let json = r#"{"path":"pages/index/index","width":430}"#;
    let qrcode: WxMaQrcode = serde_json::from_str(json).unwrap();
    assert_eq!(qrcode.path, "pages/index/index");
    assert_eq!(qrcode.width, 430);
}

// ═══ WxMaCodeLineColor ═══

#[test]
fn test_code_line_color_default() {
    let color = WxMaCodeLineColor::default();
    assert_eq!(color.r, "0");
    assert_eq!(color.g, "0");
    assert_eq!(color.b, "0");
}

#[test]
fn test_code_line_color_serde() {
    let json = r#"{"r":"255","g":"128","b":"0"}"#;
    let color: WxMaCodeLineColor = serde_json::from_str(json).unwrap();
    assert_eq!(color.r, "255");
}

// ═══ Analysis ═══

#[test]
fn test_visit_trend_serde() {
    let json = r#"{"ref_date":"2024-01-01","session_cnt":1000,"visit_pv":5000,"visit_uv":800,"visit_uv_new":200,"stay_time_uv":120.5,"stay_time_session":45.2}"#;
    let trend: WxMaVisitTrend = serde_json::from_str(json).unwrap();
    assert_eq!(trend.ref_date, "2024-01-01");
    assert_eq!(trend.session_cnt, 1000);
}

// ═══ Scheme ═══

#[test]
fn test_generate_scheme_request_serde() {
    let json = r#"{"jump_wxa":{"path":"pages/index/index"},"is_expire":true,"expire_time":1700000100,"expire_type":1,"expire_interval":3600}"#;
    let req: WxMaGenerateSchemeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.jump_wxa.path, "pages/index/index");
    assert!(req.is_expire);
}

// ═══ Updatable Message ═══

#[test]
fn test_updatable_msg_serde() {
    let json = r#"{"activity_id":"act-001","target_state":0,"template_info":{"parameter_list":[{"name":"path","val":"pages/index"}]}}"#;
    let msg: WxMaUpdatableMsg = serde_json::from_str(json).unwrap();
    assert_eq!(msg.activity_id, "act-001");
    assert_eq!(msg.template_info.parameter_list.len(), 1);
}

// ═══ WxMaShareInfo ═══

#[test]
fn test_share_info_serde() {
    let json = r#"{"openGId":"Share Title"}"#;
    let info: WxMaShareInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.open_g_id, "Share Title");
}

// ═══ VALUE_ADD ═══

#[test]
fn test_empty_json_defaults() {
    let resp: WxMaBaseResponse = serde_json::from_str("{}").unwrap();
    assert_eq!(resp.errcode, 0);
    let qrcode: WxMaQrcode = serde_json::from_str("{}").unwrap();
    assert_eq!(qrcode.path, "");
}
