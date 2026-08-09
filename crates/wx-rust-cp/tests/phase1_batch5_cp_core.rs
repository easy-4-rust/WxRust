//! Phase 1 Batch 1.5: wx-rust-cp 核心测试
//!
//! 镜像 Java WxCpUserServiceImplTest / WxCpDepartmentServiceImplTest /
//! WxCpTagServiceImplTest / WxCpMessageServiceImplTest

use wx_rust_cp::bean::*;
use wx_rust_cp::bean::external::*;
use wx_rust_cp::bean::external::contact::*;

// ═══ Tag Result ═══

#[test]
fn test_tp_tag_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","invalidlist":"user1,user2","invalidparty":["dept1"]}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.invalid_users, "user1,user2");
    assert_eq!(result.invalid_party, vec!["dept1".to_string()]);
}

// ═══ External Contact ═══

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

// ═══ Group Message ═══

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

// ═══ Template Result ═══

#[test]
fn test_msg_template_add_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","msgid":"tpl-001"}"#;
    let result: WxCpMsgTemplateAddResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.msg_id, "tpl-001");
}

// ═══ Moment Result ═══

#[test]
fn test_moment_send_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","jobid":"job-001"}"#;
    let result: WxCpGetMomentSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

// ═══ Transfer Customer ═══

#[test]
fn test_user_transfer_customer_resp_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxCpUserTransferCustomerResp = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 0);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_empty_defaults() {
    let json = r#"{"errcode":0,"errmsg":""}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.invalid_users, "");
    assert_eq!(result.invalid_party.len(), 0);
}
