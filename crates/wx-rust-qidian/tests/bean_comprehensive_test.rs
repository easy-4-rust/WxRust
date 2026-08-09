//! wx-rust-qidian Bean 综合测试（SOURCE_PARITY + VALUE_ADD）。

use wx_rust_qidian::bean::*;
use wx_rust_qidian::bean::dial::*;
use wx_rust_qidian::bean::call::*;
use wx_rust_qidian::bean::common::*;

// ═══ Host Config ═══

#[test]
fn test_host_config_default() {
    let config = WxQidianHostConfig::new();
    assert_eq!(config.api_host, None);
    assert_eq!(config.open_host, None);
    assert_eq!(config.qidian_host, None);
}

#[test]
fn test_host_config_constants() {
    assert_eq!(API_DEFAULT_HOST_URL, "https://api.weixin.qq.com");
    assert_eq!(OPEN_DEFAULT_HOST_URL, "https://open.weixin.qq.com");
    assert_eq!(QIDIAN_DEFAULT_HOST_URL, "https://api.qidian.qq.com");
}

// ═══ QidianResponse ═══

#[test]
fn test_qidian_response_serde() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"success","errmsg_chinese":"成功"}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.code, 0);
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "success");
}

#[test]
fn test_qidian_response_error() {
    let json = r#"{"code":40001,"msg":"invalid","errcode":40001,"errmsg":"invalid token"}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.code, 40001);
    assert_eq!(resp.errcode, 40001);
}

// ═══ Dial Beans ═══

#[test]
fn test_ivr_serde() {
    let json = r#"{"ivr_id":"ivr-001","ivr_name":"欢迎语"}"#;
    let ivr: Ivr = serde_json::from_str(json).unwrap();
    assert_eq!(ivr.ivr_id, Some("ivr-001".to_string()));
    assert_eq!(ivr.ivr_name, Some("欢迎语".to_string()));
}

#[test]
fn test_ivr_dial_request_serde() {
    let json = r#"{"phone_number":"13800138000","ivr_id":"ivr-001","corp_phone_list":["020-1234"],"loc_pref_on":0,"skip_restrict":false}"#;
    let req: IVRDialRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.phone_number, Some("13800138000".to_string()));
    assert_eq!(req.ivr_id, Some("ivr-001".to_string()));
    assert_eq!(req.corp_phone_list.as_ref().unwrap().len(), 1);
}

#[test]
fn test_ivr_dial_request_to_json() {
    let req = IVRDialRequest {
        phone_number: Some("13900139000".to_string()),
        ivr_id: Some("ivr-002".to_string()),
        corp_phone_list: None,
        loc_pref_on: None,
        backup_corp_phone_list: None,
        skip_restrict: None,
    };
    let json = req.to_json();
    assert!(json.contains("13900139000"));
    assert!(json.contains("ivr-002"));
}

#[test]
fn test_ivr_dial_response_serde() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"success","callid":"call-001"}"#;
    let resp: IVRDialResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.base.code, 0);
    assert_eq!(resp.callid, Some("call-001".to_string()));
}

#[test]
fn test_ivr_list_response_serde() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"success","node":[{"ivr_id":"ivr-001","ivr_name":"IVR1"},{"ivr_id":"ivr-002","ivr_name":"IVR2"}]}"#;
    let resp: IVRListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.base.code, 0);
    let nodes = resp.node.unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].ivr_name, Some("IVR1".to_string()));
}

// ═══ Call Beans ═══

#[test]
fn test_switch_board_serde() {
    let json = r#"{"switchboard":"sb-001","create_time":"2024-01-01","callin_status":true,"callout_status":false,"sp_name":"主线路","city_name":"深圳"}"#;
    let sb: SwitchBoard = serde_json::from_str(json).unwrap();
    assert_eq!(sb.switchboard, Some("sb-001".to_string()));
    assert_eq!(sb.callin_status, Some(true));
    assert_eq!(sb.sp_name, Some("主线路".to_string()));
}

#[test]
fn test_switch_board_list_serde() {
    let json = r#"{"records":[{"switchboard":"sb-001","sp_name":"线路1"},{"switchboard":"sb-002","sp_name":"线路2"}]}"#;
    let list: SwitchBoardList = serde_json::from_str(json).unwrap();
    let records = list.records.unwrap();
    assert_eq!(records.len(), 2);
}

#[test]
fn test_switch_board_list_switch_boards() {
    let json = r#"{"records":[{"switchboard":"sb-001"},{"switchboard":"sb-002"}]}"#;
    let list: SwitchBoardList = serde_json::from_str(json).unwrap();
    let boards = list.switch_boards();
    assert_eq!(boards.len(), 2);
    assert_eq!(boards[0], "sb-001");
}

#[test]
fn test_get_switch_board_list_response_serde() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"success","data":{"records":[{"switchboard":"sb-001"}]}}"#;
    let resp: GetSwitchBoardListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.base.code, 0);
    let data = resp.data.unwrap();
    assert_eq!(data.records.unwrap().len(), 1);
}

#[test]
fn test_get_switch_board_list_response_from_json() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"success","data":{"records":[]}}"#;
    let resp = GetSwitchBoardListResponse::from_json(json).unwrap();
    assert_eq!(resp.base.errcode, 0);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_qidian_response_default() {
    let json = r#"{"code":0,"errcode":0,"errmsg":""}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.code, 0);
    assert_eq!(resp.errcode, 0);
}

#[test]
fn test_switch_board_list_empty() {
    let json = r#"{}"#;
    let list: SwitchBoardList = serde_json::from_str(json).unwrap();
    assert_eq!(list.records, None);
    assert_eq!(list.switch_boards().len(), 0);
}

#[test]
fn test_ivr_dial_request_empty() {
    let json = r#"{}"#;
    let req: IVRDialRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.phone_number, None);
}
