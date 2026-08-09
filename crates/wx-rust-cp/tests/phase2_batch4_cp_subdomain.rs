//! Phase 2 Batch 2.4: cp 子域测试
//!
//! 镜像 Java WxCpChatServiceImplTest / WxCpMenuServiceImplTest /
//! WxCpTaskCardServiceImplTest / WxCpOaServiceImplTest

use wx_rust_cp::bean::*;
use wx_rust_cp::bean::external::*;
use wx_rust_cp::bean::external::contact::*;

// ═══ External Contact ═══

#[test]
fn test_external_contact_list_info_full() {
    let json = r#"{"errcode":"0","errmsg":"ok","next_cursor":"cursor1","info_list":[{"is_customer":true,"tmp_openid":"ox1","follow_user":[{"userid":"user1","remark":"remark1","create_time":1700000000}]},{"is_customer":false,"tmp_openid":"ox2"}]}"#;
    let info: WxCpExternalContactListInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.errcode, "0");
    assert_eq!(info.info_list.len(), 2);
    assert!(info.info_list[0].is_customer);
    assert!(!info.info_list[1].is_customer);
}

#[test]
fn test_external_contact_batch_info_full() {
    let json = r#"{"errcode":0,"errmsg":"ok","external_contact_list":[{"external_contact":{"external_userid":"ext1","name":"test","type":1},"follow_info":{"userid":"user1","remark":"remark"}}]}"#;
    let info: WxCpExternalContactBatchInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.errcode, 0);
}

// ═══ Group Message ═══

#[test]
fn test_group_msg_list_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","msg_list":[{"msgid":"msg-001","creator_userid":"user1","create_time":1700000000}]}"#;
    let list: WxCpGroupMsgListResult = serde_json::from_str(json).unwrap();
    assert_eq!(list.errcode, 0);
}

#[test]
fn test_group_msg_send_result_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let result: WxCpGroupMsgSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

// ═══ Template Card Message ═══

#[test]
fn test_template_card_message_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","msgid":"card-001"}"#;
    let result: WxCpMsgTemplateAddResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.msg_id, "card-001");
}

// ═══ Tag Result ═══

#[test]
fn test_tp_tag_result_full() {
    let json = r#"{"errcode":0,"errmsg":"ok","invalidlist":"user1,user2","invalidparty":["dept1","dept2"]}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.err_code, 0);
    assert_eq!(result.invalid_party.len(), 2);
}

// ═══ Moment Result ═══

#[test]
fn test_moment_send_result_full() {
    let json = r#"{"errcode":0,"errmsg":"ok","jobid":"job-001"}"#;
    let result: WxCpGetMomentSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

// ═══ Transfer Customer ═══

#[test]
fn test_user_transfer_customer_full() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxCpUserTransferCustomerResp = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 0);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_external_contact_empty_list() {
    let json = r#"{"errcode":"0","errmsg":"ok","next_cursor":"","info_list":[]}"#;
    let info: WxCpExternalContactListInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.info_list.len(), 0);
}

#[test]
fn test_group_msg_empty() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let result: WxCpGroupMsgSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.errcode, 0);
}

#[test]
fn test_tag_result_empty_invalids() {
    let json = r#"{"errcode":0,"errmsg":"ok","invalidlist":"","invalidparty":[]}"#;
    let result: WxCpTpTagAddOrRemoveUsersResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.invalid_users, "");
    assert_eq!(result.invalid_party.len(), 0);
}
