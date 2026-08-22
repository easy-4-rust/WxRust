#![allow(clippy::field_reassign_with_default)]
//! Phase 3 P2 扩展: pay 投诉（Complaint）+ 银行（Bank）+ 子商户进件（Applyment）Bean 测试。
//!
//! 镜像 Java:
//! - `ComplaintServiceImplTest`（投诉查询/详情/协商历史/回复/完结）
//! - `BankServiceImplTest`（银行列表/支行列表/城市列表）
//! - `Applyment4SubServiceImplTest`（子商户进件/查询/修改结算）
//! - `MerchantLimitationServiceImplTest`（商户限额查询）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde skip_serializing_if、Option 语义
//! - VALUE_ADD: 空值/边界/默认值路径

use wx_rust_pay::bean::applyment::*;
use wx_rust_pay::bean::bank::*;
use wx_rust_pay::bean::complaint::*;
use wx_rust_pay::bean::merchantlimitation::*;

// ═══════════════════════════════════════════════════════════════
// 1. 投诉详情（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testQueryComplaint）
// ═══════════════════════════════════════════════════════════════

/// 投诉详情结果 serde（对应 Java `ComplaintDetailResult`：
/// `complaint_id`/`complaint_time`/`complaint_detail`/`complainted_mchid`/
/// `complaint_state`/`payer_phone`/`payer_openid`/`complaint_media_list`）。
/// 对应 Java: ComplaintServiceImplTest.testQueryComplaint
#[test]
fn test_complaint_detail_result_serde() {
    let json = r#"{
        "complaint_id":"CP-001",
        "complaint_time":"2024-01-01T12:00:00+08:00",
        "complaint_detail":"商品质量问题",
        "complainted_mchid":"mch123",
        "complaint_state":"PENDING",
        "payer_phone":"13800138000",
        "payer_openid":"ox123",
        "complaint_media_list":[
            {"media_type":"IMAGE","media_url":["https://example.com/img1.jpg"],"complaint_order_info":[]}
        ]
    }"#;
    let result: ComplaintDetailResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.complaint_id.as_deref(), Some("CP-001"));
    assert_eq!(result.complaint_state.as_deref(), Some("PENDING"));
    assert_eq!(result.complaint_detail.as_deref(), Some("商品质量问题"));
    assert_eq!(result.payer_openid.as_deref(), Some("ox123"));
    assert_eq!(result.complaint_media_list.len(), 1);
    assert_eq!(
        result.complaint_media_list[0].media_type.as_deref(),
        Some("IMAGE")
    );
}

/// 投诉详情含订单信息（对应 Java `ComplaintOrder`：`transaction_id`/
/// `out_trade_no`/`amount`/`service_order_info`）。
/// 对应 Java: ComplaintServiceImplTest (order info)
#[test]
fn test_complaint_detail_with_orders() {
    let json = r#"{
        "complaint_id":"CP-002",
        "complaint_state":"PROCESSING",
        "complaint_media_list":[
            {
                "media_type":"IMAGE",
                "media_url":["https://example.com/img1.jpg"],
                "complaint_order_info":[
                    {
                        "transaction_id":"4200001234",
                        "out_trade_no":"ORDER-001",
                        "amount":1000,
                        "service_order_info":[
                            {"order_id":"SO-001","out_order_no":"OUT-SO-001","state":"DOING"}
                        ]
                    }
                ]
            }
        ]
    }"#;
    let result: ComplaintDetailResult = serde_json::from_str(json).unwrap();
    let media = &result.complaint_media_list[0];
    let order = &media.complaint_order_info[0];
    assert_eq!(order.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(order.out_trade_no.as_deref(), Some("ORDER-001"));
    assert_eq!(order.amount, Some(1000));
    assert_eq!(order.service_order_info.len(), 1);
    assert_eq!(
        order.service_order_info[0].order_id.as_deref(),
        Some("SO-001")
    );
}

/// 投诉详情含附加信息（对应 Java `AdditionalInfo`/`SharePowerInfo`/
/// `ReturnAddressInfo` 嵌套在 `ServiceOrder` 中）。
/// 对应 Java: ComplaintServiceImplTest (additional info)
#[test]
fn test_complaint_detail_with_additional_info() {
    let json = r#"{
        "complaint_id":"CP-003",
        "complaint_state":"PENDING",
        "complaint_media_list":[
            {
                "media_type":"IMAGE",
                "media_url":["https://example.com/img1.jpg"],
                "complaint_order_info":[
                    {
                        "transaction_id":"4200001234",
                        "service_order_info":[
                            {
                                "order_id":"SO-001",
                                "state":"DOING",
                                "problem_description":"商品质量问题",
                                "apply_refund_amount":500,
                                "additional_info":{
                                    "type":"RETURN",
                                    "share_power_info":{"return_time":"2024-01-05"}
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;
    let result: ComplaintDetailResult = serde_json::from_str(json).unwrap();
    let service = &result.complaint_media_list[0].complaint_order_info[0].service_order_info[0];
    assert_eq!(service.problem_description.as_deref(), Some("商品质量问题"));
    assert_eq!(service.apply_refund_amount, Some(500));
    assert!(service.additional_info.is_some());
}

// ═══════════════════════════════════════════════════════════════
// 2. 投诉列表（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testQueryComplaintList）
// ═══════════════════════════════════════════════════════════════

/// 投诉列表请求 serde（对应 Java `ComplaintRequest`：`begin_date`/
/// `end_date`/`offset`/`limit`/`complainted_mchid`）。
/// 对应 Java: ComplaintServiceImplTest.testQueryComplaintList
#[test]
fn test_complaint_request_serde() {
    let json = r#"{
        "begin_date":"2024-01-01",
        "end_date":"2024-01-31",
        "offset":0,
        "limit":20,
        "complainted_mchid":"mch123"
    }"#;
    let request: ComplaintRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.begin_date.as_deref(), Some("2024-01-01"));
    assert_eq!(request.end_date.as_deref(), Some("2024-01-31"));
}

/// 投诉列表结果 serde。
#[test]
fn test_complaint_result_serde() {
    let json = r#"{
        "total_count":10,
        "data":[
            {"complaint_id":"CP-001","complaint_time":"2024-01-01","complaint_state":"PENDING"}
        ]
    }"#;
    let result: ComplaintResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(10));
}

// ═══════════════════════════════════════════════════════════════
// 3. 协商历史（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testQueryNegotiationHistory）
// ═══════════════════════════════════════════════════════════════

/// 协商历史请求 serde。
/// 对应 Java: ComplaintServiceImplTest.testQueryNegotiationHistory
#[test]
fn test_negotiation_history_request_serde() {
    let json = r#"{"complaint_id":"CP-001"}"#;
    let request: NegotiationHistoryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.complaint_id.as_deref(), Some("CP-001"));
}

/// 协商历史结果 serde（对应 Java `NegotiationHistoryResult`：
/// `data` 列表，每项含 `complaint_media_list`，内嵌 `ComplaintMedia`：
/// `operate_type`/`operate_details`/`operate_time`/`operator`/`image_list`）。
/// 对应 Java: ComplaintServiceImplTest.testQueryNegotiationHistory
#[test]
fn test_negotiation_history_result_serde() {
    let json = r#"{
        "total_count":1,
        "data":[
            {
                "complaint_media_list":{
                    "media_type":"TEXT",
                    "operate_type":"MERCHANT_REPLY",
                    "operate_details":"已安排退款",
                    "operate_time":"2024-01-02T10:00:00+08:00",
                    "operator":"客服",
                    "image_list":["https://example.com/img1.jpg"]
                }
            }
        ]
    }"#;
    let result: NegotiationHistoryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(1));
    assert_eq!(result.data.len(), 1);
    let media = result.data[0].complaint_media_list.as_ref().unwrap();
    assert_eq!(media.operate_type.as_deref(), Some("MERCHANT_REPLY"));
    assert_eq!(media.operator.as_deref(), Some("客服"));
    assert_eq!(media.image_list.len(), 1);
}

// ═══════════════════════════════════════════════════════════════
// 4. 投诉回复（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testResponseComplaint）
// ═══════════════════════════════════════════════════════════════

/// 投诉回复请求 serde（对应 Java `ResponseRequest`：
/// `complaint_id`/`response_content`/`jump_weapp`）。
/// 对应 Java: ComplaintServiceImplTest.testResponseComplaint
#[test]
fn test_response_request_serde() {
    let json = r#"{
        "complaint_id":"CP-001",
        "response_content":"已收到投诉，正在处理"
    }"#;
    let request: ResponseRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.complaint_id.as_deref(), Some("CP-001"));
    assert_eq!(
        request.response_content.as_deref(),
        Some("已收到投诉，正在处理")
    );
}

/// 投诉回复含小程序跳转信息（对应 Java `MiniProgramJumpInfo`：
/// `appid`/`path`/`text`）。
/// 对应 Java: ComplaintServiceImplTest (mini program jump)
#[test]
fn test_response_request_with_mini_program_jump() {
    let json = r#"{
        "complaint_id":"CP-001",
        "response_content":"请查看退款进度",
        "mini_program_jump_info":{"appid":"wx1234","path":"/pages/refund/detail","text":"查看退款"}
    }"#;
    let request: ResponseRequest = serde_json::from_str(json).unwrap();
    assert!(request.mini_program_jump_info.is_some());
    let jump = request.mini_program_jump_info.as_ref().unwrap();
    assert_eq!(jump.app_id.as_deref(), Some("wx1234"));
    assert_eq!(jump.path.as_deref(), Some("/pages/refund/detail"));
}

// ═══════════════════════════════════════════════════════════════
// 5. 投诉完结（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testCompleteComplaint）
// ═══════════════════════════════════════════════════════════════

/// 投诉完结请求 serde（对应 Java `CompleteRequest`：`complaint_id`）。
/// 对应 Java: ComplaintServiceImplTest.testCompleteComplaint
#[test]
fn test_complete_request_serde() {
    let json = r#"{"complaint_id":"CP-001"}"#;
    let request: CompleteRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.complaint_id.as_deref(), Some("CP-001"));
}

// ═══════════════════════════════════════════════════════════════
// 6. 投诉通知URL（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testComplaintNotifyUrl）
// ═══════════════════════════════════════════════════════════════

/// 投诉通知URL设置请求 serde。
/// 对应 Java: ComplaintServiceImplTest.testSetComplaintNotifyUrl
#[test]
fn test_complaint_notify_url_request_serde() {
    let json = r#"{"url":"https://example.com/complaint/notify"}"#;
    let request: ComplaintNotifyUrlRequest = serde_json::from_str(json).unwrap();
    assert_eq!(
        request.url.as_deref(),
        Some("https://example.com/complaint/notify")
    );
}

/// 投诉通知URL设置结果 serde。
/// 对应 Java: ComplaintServiceImplTest (notify url result)
#[test]
fn test_complaint_notify_url_result_serde() {
    let json = r#"{"mchid":"mch123","url":"https://example.com/complaint/notify"}"#;
    let result: ComplaintNotifyUrlResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.mchid.as_deref(), Some("mch123"));
    assert_eq!(
        result.url.as_deref(),
        Some("https://example.com/complaint/notify")
    );
}

// ═══════════════════════════════════════════════════════════════
// 7. 投诉更新退款进度（SOURCE_PARITY:
//    Java ComplaintServiceImplTest.testUpdateRefundProgress）
// ═══════════════════════════════════════════════════════════════

/// 投诉更新退款进度请求 serde（对应 Java `UpdateRefundProgressRequest`：
/// `complaint_id`/`action`/`launch_refund_day`/`reject_reason`/`remark`）。
/// 对应 Java: ComplaintServiceImplTest.testUpdateRefundProgress
#[test]
fn test_update_refund_progress_request_serde() {
    let json = r#"{
        "complaint_id":"CP-001",
        "action":"REFUND",
        "launch_refund_day":7,
        "remark":"已安排退款"
    }"#;
    let request: UpdateRefundProgressRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.complaint_id.as_deref(), Some("CP-001"));
    assert_eq!(request.action.as_deref(), Some("REFUND"));
    assert_eq!(request.launch_refund_day, Some(7));
    assert_eq!(request.remark.as_deref(), Some("已安排退款"));
}

// ═══════════════════════════════════════════════════════════════
// 8. 银行信息（SOURCE_PARITY:
//    Java BankServiceImplTest.testQueryBank）
// ═══════════════════════════════════════════════════════════════

/// 银行信息 serde（对应 Java `BankInfo`：`bank_alias`/`bank_alias_code`/
/// `account_bank`/`account_bank_code`/`need_bank_branch`）。
/// 对应 Java: BankServiceImplTest.testQueryBank
#[test]
fn test_bank_info_serde() {
    let json = r#"{
        "bank_alias":"工商银行",
        "bank_alias_code":"ICBC",
        "account_bank":"工商银行",
        "account_bank_code":1001,
        "need_bank_branch":true
    }"#;
    let info: BankInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.bank_alias.as_deref(), Some("工商银行"));
    assert_eq!(info.bank_alias_code.as_deref(), Some("ICBC"));
    assert_eq!(info.account_bank_code, Some(1001));
    assert_eq!(info.need_bank_branch, Some(true));
}

/// 银行账户结果 serde（对应 Java `BankAccountResult`：
/// `total_count`/`data` 列表，每项含 `BankInfo`）。
/// 对应 Java: BankServiceImplTest.testQueryBankAccount
#[test]
fn test_bank_account_result_serde() {
    let json = r#"{
        "total_count":2,
        "data":[
            {"bank_alias":"工商银行","bank_alias_code":"ICBC","account_bank":"工商银行"},
            {"bank_alias":"建设银行","bank_alias_code":"CCB","account_bank":"建设银行"}
        ]
    }"#;
    let result: BankAccountResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(2));
    assert_eq!(result.data.len(), 2);
    assert_eq!(result.data[0].bank_alias.as_deref(), Some("工商银行"));
}

/// 银行信息列表结果 serde（对应 Java `BankingResult`）。
#[test]
fn test_banking_result_serde() {
    let json = r#"{
        "data":[
            {"bank_alias":"工商银行","bank_alias_code":"ICBC"},
            {"bank_alias":"建设银行","bank_alias_code":"CCB"}
        ],
        "offset":0,
        "limit":20,
        "total_count":2
    }"#;
    let result: BankingResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(2));
}

/// 城市列表结果 serde（对应 Java `CitiesResult`：`total_count`/`data` 列表，
/// 每项 `CityInfo` 含 `city_code`(i32)/`city_name`）。
/// 对应 Java: BankServiceImplTest.testQueryCities
#[test]
fn test_cities_result_serde() {
    let json = r#"{
        "data":[
            {"city_code":110000,"city_name":"北京市"}
        ],
        "total_count":1
    }"#;
    let result: CitiesResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(1));
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].city_code, Some(110000));
    assert_eq!(result.data[0].city_name.as_deref(), Some("北京市"));
}

/// 支行列表结果 serde（对应 Java `BankBranchesResult`）。
#[test]
fn test_bank_branches_result_serde() {
    let json = r#"{
        "data":[
            {"bank_branch_id":"1001","bank_name":"工商银行浦东支行","bank_code":"ICBC","address_code":"310115"}
        ],
        "offset":0,
        "limit":20,
        "total_count":1
    }"#;
    let result: BankBranchesResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(1));
}

// ═══════════════════════════════════════════════════════════════
// 9. 子商户进件（SOURCE_PARITY:
//    Java Applyment4SubServiceImplTest.testCreateApplyment）
// ═══════════════════════════════════════════════════════════════

/// 子商户进件请求 serde（对应 Java `WxPayApplyment4SubCreateRequest`：
/// `business_code`/`contact_info`/`settlement_info`/`bank_account_info`）。
/// 对应 Java: Applyment4SubServiceImplTest.testCreateApplyment
#[test]
fn test_applyment4_sub_create_request_serde() {
    let json = r#"{
        "business_code":"123456",
        "contact_info":{"contact_type":"LEGAL","contact_name":"张三","contact_id_doc_type":"IDENTIFICATION_CARD","contact_id_number":"310101199001010001"},
        "settlement_info":{"settlement_id":"715","qualification_type":"餐饮"},
        "bank_account_info":{"bank_account_type":"BANK_ACCOUNT_CORPORATE","account_name":"测试商户","account_number":"1234567890","bank_name":"工商银行"}
    }"#;
    let request: WxPayApplyment4SubCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.business_code.as_deref(), Some("123456"));
    let contact = request.contact_info.as_ref().unwrap();
    assert_eq!(contact.contact_name.as_deref(), Some("张三"));
    let settlement = request.settlement_info.as_ref().unwrap();
    assert_eq!(
        request
            .bank_account_info
            .as_ref()
            .unwrap()
            .account_name
            .as_deref(),
        Some("测试商户")
    );
    assert_eq!(settlement.settlement_id.as_deref(), Some("715"));
}

// ═══════════════════════════════════════════════════════════════
// 10. 子商户进件查询（SOURCE_PARITY:
//     Java Applyment4SubServiceImplTest.testQueryApplyment）
// ═══════════════════════════════════════════════════════════════

/// 子商户进件查询结果 serde（对应 Java `ApplymentStateQueryResult`：
/// `applyment_id`/`applyment_state`/`applyment_state_msg`/`sub_mchid`/
/// `business_code`/`audit_detail`）。
/// 对应 Java: Applyment4SubServiceImplTest.testQueryApplyment
#[test]
fn test_applyment_state_query_result_serde() {
    let json = r#"{
        "applyment_id":"AP-001",
        "business_code":"123456",
        "sub_mchid":"sub123",
        "sign_url":"https://example.com/sign",
        "applyment_state":"AUDITING",
        "applyment_state_msg":"审核中",
        "audit_detail":[{"field":"id_card","field_name":"身份证","reject_reason":"照片不清晰"}]
    }"#;
    let result: ApplymentStateQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.applyment_id.as_deref(), Some("AP-001"));
    assert_eq!(result.applyment_state, "AUDITING");
    assert_eq!(result.sub_mchid.as_deref(), Some("sub123"));
    assert_eq!(result.audit_detail.len(), 1);
    assert_eq!(result.audit_detail[0].field.as_deref(), Some("id_card"));
}

/// 子商户进件创建结果 serde。
/// 对应 Java: Applyment4SubServiceImplTest (create result)
#[test]
fn test_applyment_create_result_serde() {
    let json = r#"{"applyment_id":"AP-001"}"#;
    let result: WxPayApplymentCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.applyment_id.as_deref(), Some("AP-001"));
}

// ═══════════════════════════════════════════════════════════════
// 11. 修改结算账户（SOURCE_PARITY:
//     Java Applyment4SubServiceImplTest.testModifySettlement）
// ═══════════════════════════════════════════════════════════════

/// 修改结算账户请求 serde（对应 Java `ModifySettlementRequest`：
/// `account_type`/`account_bank`/`bank_branch_id`/
/// `account_number`/`account_name`/`bank_address_code`/`bank_name`）。
/// 对应 Java: Applyment4SubServiceImplTest.testModifySettlement
#[test]
fn test_modify_settlement_request_serde() {
    let json = r#"{
        "account_type":"BANK_ACCOUNT_CORPORATE",
        "account_name":"测试商户",
        "account_bank":"工商银行",
        "bank_address_code":"110000",
        "bank_name":"中国工商银行",
        "bank_branch_id":"1001",
        "account_number":"1234567890"
    }"#;
    let request: ModifySettlementRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.account_type, "BANK_ACCOUNT_CORPORATE");
    assert_eq!(request.account_name.as_deref(), Some("测试商户"));
    assert_eq!(request.account_bank.as_deref(), Some("工商银行"));
}

/// 结算信息查询结果 serde。
/// 对应 Java: Applyment4SubServiceImplTest.testQuerySettlement
#[test]
fn test_settlement_info_result_serde() {
    let json = r#"{
        "account_type":"BANK_ACCOUNT_CORPORATE",
        "account_bank":"工商银行",
        "bank_name":"中国工商银行",
        "bank_branch_id":"1001",
        "account_number":"1234****7890"
    }"#;
    let result: SettlementInfoResult = serde_json::from_str(json).unwrap();
    assert_eq!(
        result.account_type.as_deref(),
        Some("BANK_ACCOUNT_CORPORATE")
    );
    assert_eq!(result.account_bank.as_deref(), Some("工商银行"));
}

/// 修改结算账户状态查询结果 serde（对应 Java `SettlementModifyStateQueryResult`：
/// `account_type`/`verify_result`/`verify_finish_time`/`account_name`）。
/// 对应 Java: Applyment4SubServiceImplTest.testQueryModifySettlement
#[test]
fn test_settlement_modify_state_query_result_serde() {
    let json = r#"{
        "account_name":"测试商户",
        "account_type":"BANK_ACCOUNT_CORPORATE",
        "account_bank":"工商银行",
        "bank_name":"中国工商银行",
        "account_number":"1234****7890",
        "verify_result":"SUCCESS",
        "verify_finish_time":"2024-01-02T00:00:00+08:00"
    }"#;
    let result: SettlementModifyStateQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.account_type, "BANK_ACCOUNT_CORPORATE");
    assert_eq!(result.verify_result, "SUCCESS");
    assert_eq!(result.account_name.as_deref(), Some("测试商户"));
}

/// 结算申请结果 serde（对应 Java `SettlementApplicationResult`：
/// `account_type`/`verify_result`/`account_name`）。
/// 对应 Java: Applyment4SubServiceImplTest (settlement application)
#[test]
fn test_settlement_application_result_serde() {
    let json = r#"{
        "account_name":"测试商户",
        "account_type":"BANK_ACCOUNT_CORPORATE",
        "account_bank":"工商银行",
        "account_number":"1234****7890",
        "verify_result":"VERIFYING"
    }"#;
    let result: SettlementApplicationResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.account_name.as_deref(), Some("测试商户"));
    assert_eq!(result.verify_result.as_deref(), Some("VERIFYING"));
}

// ═══════════════════════════════════════════════════════════════
// 12. 商户限额（SOURCE_PARITY:
//     Java MerchantLimitationServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// 商户限额查询结果 serde（对应 Java `MerchantLimitationResult`：
/// `mchid`/`limited_functions`/`recovery_specifications`）。
/// 对应 Java: MerchantLimitationServiceImplTest.testQuery
#[test]
fn test_merchant_limitation_result_serde() {
    let json = r#"{
        "mchid":"mch123",
        "limited_functions":["PAYMENT","TRANSFER"],
        "other_limited_functions":"",
        "recovery_specifications":[
            {
                "limitation_case_id":"LC-001",
                "limitation_reason_type":"RISK",
                "limitation_reason":"风控限制",
                "recover_way":"AUTO",
                "limitation_start_date":"2024-01-01",
                "limitation_date":"2024-02-01"
            }
        ]
    }"#;
    let result: MerchantLimitationResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.mch_id.as_deref(), Some("mch123"));
    assert_eq!(result.limited_functions.len(), 2);
    assert_eq!(result.recovery_specifications.len(), 1);
    assert_eq!(
        result.recovery_specifications[0]
            .limitation_case_id
            .as_deref(),
        Some("LC-001")
    );
    assert_eq!(
        result.recovery_specifications[0].recover_way.as_deref(),
        Some("AUTO")
    );
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 投诉详情默认值。
#[test]
fn test_complaint_detail_result_default() {
    let result: ComplaintDetailResult = serde_json::from_str("{}").unwrap();
    assert!(result.complaint_id.is_none());
    assert!(result.complaint_media_list.is_empty());
}

/// 投诉回复请求默认值。
#[test]
fn test_response_request_default() {
    let request = ResponseRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("complaint_id"));
    assert!(!json.contains("response_content"));
}

/// 银行信息默认值。
#[test]
fn test_bank_info_default() {
    let info = BankInfo::default();
    assert!(info.bank_alias.is_none());
    assert_eq!(info.account_bank_code, None);
    assert_eq!(info.need_bank_branch, None);
}

/// 子商户进件请求默认值。
#[test]
fn test_applyment4_sub_create_request_default() {
    let request = WxPayApplyment4SubCreateRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("business_code"));
    assert!(!json.contains("contact_info"));
}

/// 投诉协商历史结果默认值。
#[test]
fn test_negotiation_history_result_default() {
    let result: NegotiationHistoryResult = serde_json::from_str("{}").unwrap();
    assert!(result.data.is_empty());
}

/// 投诉列表结果默认值。
#[test]
fn test_complaint_result_default() {
    let result: ComplaintResult = serde_json::from_str("{}").unwrap();
    assert!(result.total_count.is_none());
}

/// 子商户进件查询结果默认值。
#[test]
fn test_applyment_state_query_result_default() {
    let result: ApplymentStateQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.applyment_id.is_none());
    assert!(result.sub_mchid.is_none());
    assert!(result.audit_detail.is_empty());
}

/// 修改结算账户请求默认值。
#[test]
fn test_modify_settlement_request_default() {
    let request = ModifySettlementRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"account_type\":\"\""));
    assert!(!json.contains("account_name"));
}

/// 商户限额结果默认值。
#[test]
fn test_merchant_limitation_result_default() {
    let result: MerchantLimitationResult = serde_json::from_str("{}").unwrap();
    assert!(result.mch_id.is_none());
    assert!(result.limited_functions.is_empty());
    assert!(result.recovery_specifications.is_empty());
}
