//! wx-rust-open Bean 综合测试。

use wx_rust_open::bean::WxOpenCreateResult;
use wx_rust_open::bean::result::WxOpenResult;
use wx_rust_open::bean::result::WxOpenMaDomainResult;
use wx_rust_open::bean::result::WxOpenMaTesterListResult;
use wx_rust_open::bean::result::WxOpenMaVisitStatusResult;
use wx_rust_open::bean::result::WxOpenMaGetOrderPathResult;
use wx_rust_open::bean::result::WxOpenMaEmbeddedListResult;
use wx_rust_open::bean::result::WxDownlooadQrcodeJumpResult;

#[test]
fn test_open_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.errmsg, "ok");
}

#[test]
fn test_open_result_from_json() {
    let json = r#"{"errcode":"0","errmsg":"success"}"#;
    let result = WxOpenResult::from_json(json).unwrap();
    assert_eq!(result.errcode, "0");
}

#[test]
fn test_open_ma_domain_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","requestdomain":["https://a.com"],"wsrequestdomain":[],"uploaddomain":[],"downloaddomain":[],"invalidrequestdomain":[],"invalidwsrequestdomain":[],"invaliduploaddomain":[],"invaliddownloaddomain":[]}"#;
    let result: WxOpenMaDomainResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.request_domain.len(), 1);
}

#[test]
fn test_open_ma_tester_list_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","members":[{"userstr":"user1"},{"userstr":"user2"}]}"#;
    let result: WxOpenMaTesterListResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.members_list.len(), 2);
    // members_list is renamed to "members" in JSON
}

#[test]
fn test_open_ma_visit_status_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","status":1}"#;
    let result: WxOpenMaVisitStatusResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.status, 1);
}

#[test]
fn test_open_create_result_serde() {
    let json = r#"{"open_appid":"open-wx1234","errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.open_appid, "open-wx1234");
}

#[test]
fn test_open_create_result_from_json() {
    let json = r#"{"open_appid":"open-wx5678","errcode":"0","errmsg":"ok"}"#;
    let result = WxOpenCreateResult::from_json(json).unwrap();
    assert_eq!(result.open_appid, "open-wx5678");
}

#[test]
fn test_ma_get_order_path_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","msg":{"path":"pages/order/detail?id=1","img_list":[],"video":""}}"#;
    let result: WxOpenMaGetOrderPathResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
    assert_eq!(result.msg.path, "pages/order/detail?id=1");
}

#[test]
fn test_ma_embedded_list_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenMaEmbeddedListResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

#[test]
fn test_download_qrcode_jump_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxDownlooadQrcodeJumpResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, "0");
}

#[test]
fn test_open_result_empty() {
    let result: WxOpenResult = serde_json::from_str("{}").unwrap();
    assert_eq!(result.errcode, "");
    assert_eq!(result.errmsg, "");
}

#[test]
fn test_ma_domain_result_empty_domains() {
    let json = r#"{"errcode":"0","errmsg":"ok"}"#;
    let result: WxOpenMaDomainResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.request_domain.len(), 0);
}
