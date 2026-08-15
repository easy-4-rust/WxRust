//! Phase 2 Batch 2.3: miniapp 子域测试
//!
//! 镜像 Java WxMaAnalysisServiceImplTest / WxMaSchemeServiceImplTest /
//! WxMaSecurityServiceImplTest / WxMaSettingServiceImplTest

use wx_rust_miniapp::bean::analysis::*;
use wx_rust_miniapp::bean::scheme::*;
use wx_rust_miniapp::bean::*;

// ═══ Analysis Beans ═══

#[test]
fn test_visit_trend_serde() {
    let json = r#"{"ref_date":"2024-01-01","session_cnt":1000,"visit_pv":5000,"visit_uv":800,"visit_uv_new":200,"stay_time_uv":120.5,"stay_time_session":45.2}"#;
    let trend: WxMaVisitTrend = serde_json::from_str(json).unwrap();
    assert_eq!(trend.ref_date, "2024-01-01");
    assert_eq!(trend.session_cnt, 1000);
    assert_eq!(trend.visit_pv, 5000);
    assert_eq!(trend.visit_uv_new, 200);
}

#[test]
fn test_retain_info_from_json() {
    let json =
        r#"{"ref_date":"2024-01-01","visit_uv_new":{"0":100,"1":80},"visit_uv":{"0":500,"1":400}}"#;
    let info = WxMaRetainInfo::from_json(json).unwrap();
    assert_eq!(info.ref_date, "2024-01-01");
    assert!(info.visit_uv_new.contains_key(&0));
}

#[test]
fn test_visit_distribution_serde() {
    let json = r#"{"ref_date":"2024-01-01","list":{"1":{"1":100,"2":200},"2":{"1":300}}}"#;
    let dist = WxMaVisitDistribution::from_json(json).unwrap();
    assert_eq!(dist.ref_date, "2024-01-01");
}

#[test]
fn test_visit_page_serde() {
    let json = r#"{"page_path":"pages/index/index","page_visit_pv":100,"page_visit_uv":80,"page_stay_time_pv":60.5,"entry_page_pv":50,"exit_page_pv":30,"page_share_pv":10,"page_share_uv":8}"#;
    let page: WxMaVisitPage = serde_json::from_str(json).unwrap();
    assert_eq!(page.page_path, "pages/index/index");
    assert_eq!(page.page_visit_pv, 100);
}

// ═══ Scheme Beans ═══

#[test]
fn test_generate_scheme_request_serde() {
    let json = r#"{"jump_wxa":{"path":"pages/index/index"},"is_expire":true,"expire_time":1700000100,"expire_type":1,"expire_interval":3600}"#;
    let req: WxMaGenerateSchemeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.jump_wxa.path, "pages/index/index");
    assert!(req.is_expire);
    assert_eq!(req.expire_type, 1);
}

#[test]
fn test_generate_nfc_scheme_request_serde() {
    let json = r#"{"jump_wxa":{"path":"pages/index/index"},"model_id":"model-001","sn":"SN123"}"#;
    let req: WxMaGenerateNfcSchemeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.model_id, "model-001");
    assert_eq!(req.sn, "SN123");
}

// ═══ Updatable Message ═══

#[test]
fn test_updatable_msg_serde() {
    let json = r#"{"activity_id":"act-001","target_state":0,"template_info":{"parameter_list":[{"name":"path","value":"pages/index"},{"name":"version_type","value":"develop"}]}}"#;
    let msg: WxMaUpdatableMsg = serde_json::from_str(json).unwrap();
    assert_eq!(msg.activity_id, "act-001");
    assert_eq!(msg.target_state, 0);
    assert_eq!(msg.template_info.parameter_list.len(), 2);
    assert_eq!(msg.template_info.parameter_list[0].name, "path");
    assert_eq!(msg.template_info.parameter_list[0].value, "pages/index");
}

// ═══ Phone Number ═══

#[test]
fn test_phone_number_info_serde() {
    let json = r#"{"phoneNumber":"+8613800138000","purePhoneNumber":"13800138000","countryCode":"86","watermark":{"timestamp":1700000000,"appid":"wx1234"}}"#;
    let info: WxMaPhoneNumberInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.phone_number, "+8613800138000");
    assert_eq!(info.country_code, "86");
}

// ═══ Order Management ═══

#[test]
fn test_order_management_result_serde() {
    let json = r#"{"err_code":0,"errmsg":"ok"}"#;
    let result: WxMaOrderManagementResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
}

#[test]
fn test_order_detail_path_serde() {
    let json = r#"{"err_code":0,"errmsg":"ok","path":"pages/order/detail?id=123"}"#;
    let path: WxMaOrderManagementGetOrderDetailPath = serde_json::from_str(json).unwrap();
    assert_eq!(path.path, "pages/order/detail?id=123");
}

// ═══ Base Response ═══

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

// ═══ Qrcode ═══

#[test]
fn test_qrcode_serde() {
    let json = r#"{"path":"pages/index/index","width":430}"#;
    let qrcode: WxMaQrcode = serde_json::from_str(json).unwrap();
    assert_eq!(qrcode.path, "pages/index/index");
    assert_eq!(qrcode.width, 430);
}

// ═══ Run Step Info ═══

#[test]
fn test_run_step_info_serde() {
    let json = r#"{"timestamp":1700000000,"step":8888}"#;
    let info: WxMaRunStepInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.timestamp, 1700000000);
    assert_eq!(info.step, 8888);
}

// ═══ Code Line Color ═══

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

// ═══ Plugin List ═══

#[test]
fn test_plugin_list_result_serde() {
    let json = r#"{"plugin_list":[{"appId":"plugin-app-001","status":"1","nickName":"Plugin1","headImgUrl":"http://img.example.com"}]}"#;
    let result: WxMaPluginListResult = serde_json::from_str(json).unwrap();

    assert_eq!(result.plugin_list.len(), 1);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_empty_json_defaults() {
    let resp: WxMaBaseResponse = serde_json::from_str("{}").unwrap();
    assert_eq!(resp.errcode, 0);
    let qrcode: WxMaQrcode = serde_json::from_str("{}").unwrap();
    assert_eq!(qrcode.path, "");
    assert_eq!(qrcode.width, 0);
}

#[test]
fn test_visit_trend_roundtrip() {
    let json = r#"{"ref_date":"2024-01-01","session_cnt":100,"visit_pv":500,"visit_uv":80,"visit_uv_new":20,"stay_time_uv":60.0,"stay_time_session":30.0}"#;
    let trend: WxMaVisitTrend = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&trend).unwrap();
    let deserialized: WxMaVisitTrend = serde_json::from_str(&serialized).unwrap();
    assert_eq!(trend.ref_date, deserialized.ref_date);
    assert_eq!(trend.session_cnt, deserialized.session_cnt);
}
