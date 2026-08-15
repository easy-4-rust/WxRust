//! wx-rust-cp Bean 综合测试。

use wx_rust_cp::bean::WxCpMsgTemplateAddResult;
use wx_rust_cp::bean::WxCpTpTagAddOrRemoveUsersResult;
use wx_rust_cp::bean::external::WxCpGetMomentSendResult;
use wx_rust_cp::bean::external::WxCpUserTransferCustomerResp;
use wx_rust_cp::bean::external::contact::WxCpExternalContactBatchInfo;
use wx_rust_cp::bean::external::contact::WxCpExternalContactListInfo;
use wx_rust_cp::bean::external::contact::WxCpGroupMsgResult;
use wx_rust_cp::bean::external::contact::WxCpGroupMsgSendResult;

#[test]
fn test_cp_tp_tag_result_serde() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","invalidlist":"user1,user2","invalidparty":["dept1"]}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.invalid_users, "user1,user2");
}

#[test]
fn test_external_contact_list_info_serde() {
    let json = r#"{"errcode":"0","errmsg":"ok","next_cursor":"cursor1","info_list":[{"is_customer":true,"tmp_openid":"ox1"}]}"#;
    let info: WxCpExternalContactListInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.errcode, "0");
    assert_eq!(info.info_list.len(), 1);
}

#[test]
fn test_external_contact_batch_info_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","external_contact_list":[]}"#;
    let info: WxCpExternalContactBatchInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.errcode, 0);
}

#[test]
fn test_group_msg_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","msg_id":"msg-001"}"#;
    let result: WxCpGroupMsgResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

#[test]
fn test_group_msg_send_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let result: WxCpGroupMsgSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

#[test]
fn test_msg_template_add_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","msgid":"tpl-001"}"#;
    let result: WxCpMsgTemplateAddResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.msg_id, "tpl-001");
}

#[test]
fn test_moment_send_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","jobid":"job-001"}"#;
    let result: WxCpGetMomentSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

#[test]
fn test_user_transfer_customer_resp_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxCpUserTransferCustomerResp = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 0);
}

#[test]
fn test_external_contact_list_roundtrip() {
    let json = r#"{"errcode":"0","errmsg":"ok","next_cursor":"","info_list":[]}"#;
    let info: WxCpExternalContactListInfo = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&info).unwrap();
    let deserialized: WxCpExternalContactListInfo = serde_json::from_str(&serialized).unwrap();
    assert_eq!(info, deserialized);
}

#[test]
fn test_empty_defaults() {
    let json = r#"{"errcode":0,"errmsg":""}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.invalid_users, "");
}
