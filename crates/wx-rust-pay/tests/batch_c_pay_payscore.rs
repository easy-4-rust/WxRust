#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——Pay 支付分 bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxPartnerPayScoreSignPlanResultTest（签约计划结果 JSON 解析）
//! - WxPartnerPayScoreUserSignPlanResultTest（用户签约计划结果解析）
//! - PartnerUserSignPlanEntityTest（签约计划实体 JSON 解析）
//! - PayScorePlanDetailResultTest（计划详情结果解析）
//! - PayScorePlanDetailTest（计划详情 JSON 解析）

use wx_rust_pay::bean::payscore::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxPartnerPayScoreSignPlanResultTest —— 签约计划结果 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPartnerPayScoreSignPlanResultTest（签约计划结果 JSON 反序列化）
#[test]
fn test_partner_pay_score_sign_plan_result_from_json() {
    let json_str = r#"{
        "appid": "wx1234567890",
        "mchid": "1234567890",
        "out_order_no": "ORDER_001",
        "service_id": "SERVICE_001",
        "service_introduction": "借充电宝",
        "state": "DOING",
        "state_description": "进行中",
        "need_collection": false,
        "post_payments": [],
        "post_discounts": [],
        "plan_detail_list": [],
        "payScoreSignInfo": {}
    }"#;
    let result: WxPartnerPayScoreSignPlanResult =
        serde_json::from_str(json_str).expect("解析签约计划结果");
    assert_eq!(result.appid.as_deref(), Some("wx1234567890"));
    assert_eq!(result.mchid.as_deref(), Some("1234567890"));
    assert_eq!(result.out_order_no.as_deref(), Some("ORDER_001"));
    assert_eq!(result.service_id.as_deref(), Some("SERVICE_001"));
    assert_eq!(result.state.as_deref(), Some("DOING"));
    assert!(!result.need_collection);
}

/// 对应 Java: WxPartnerPayScoreSignPlanResultTest（签约计划结果含 post_payments 解析）
#[test]
fn test_partner_pay_score_sign_plan_result_with_post_payments() {
    let json_str = r#"{
        "appid": "wx1234567890",
        "mchid": "1234567890",
        "out_order_no": "ORDER_002",
        "service_id": "SERVICE_002",
        "service_introduction": "租借服务",
        "state": "DONE",
        "state_description": "已完成",
        "need_collection": true,
        "post_payments": [
            {"name": "租金", "amount": 100, "description": "首小时租金", "count": 1}
        ],
        "post_discounts": [],
        "plan_detail_list": [],
        "payScoreSignInfo": {}
    }"#;
    let result: WxPartnerPayScoreSignPlanResult =
        serde_json::from_str(json_str).expect("解析签约计划结果");
    assert_eq!(result.state.as_deref(), Some("DONE"));
    assert!(result.need_collection);
    assert_eq!(result.post_payments.len(), 1);
}

/// 对应 Java: WxPartnerPayScoreSignPlanResultTest（签约计划结果序列化往返验证）
#[test]
fn test_partner_pay_score_sign_plan_result_roundtrip() {
    let json_str = r#"{
        "appid": "wx9999999999",
        "mchid": "9999999999",
        "out_order_no": "ORDER_003",
        "service_id": "SERVICE_003",
        "service_introduction": "测试服务",
        "state": "CREATED",
        "state_description": "已创建",
        "need_collection": false,
        "post_payments": [],
        "post_discounts": [],
        "plan_detail_list": [],
        "payScoreSignInfo": {}
    }"#;
    let result: WxPartnerPayScoreSignPlanResult = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxPartnerPayScoreSignPlanResult =
        serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result.appid, result2.appid);
    assert_eq!(result.out_order_no, result2.out_order_no);
    assert_eq!(result.state, result2.state);
}

// ═══════════════════════════════════════════════════════════════
// #2 WxPartnerPayScoreUserSignPlanResultTest —— 用户签约计划结果解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPartnerPayScoreUserSignPlanResultTest（用户签约计划结果 JSON 解析）
#[test]
fn test_partner_pay_score_user_sign_plan_result_from_json() {
    let json_str = r#"{
        "sign_plan": {
            "sign_plan_id": "PLAN_001",
            "openid": "openid_001",
            "service_id": "SERVICE_001",
            "mchid": "1234567890",
            "appid": "wx1234567890",
            "merchant_sign_plan_no": "MPLAN_001",
            "plan_id": "PLAN_ID_001",
            "sign_state": "SIGNED",
            "plan_name": "月度会员",
            "total_origin_price": 1000,
            "deduction_quantity": 1,
            "total_actual_price": 800,
            "signed_detail_list": []
        },
        "package": "package_token_001"
    }"#;
    let result: WxPartnerPayScoreUserSignPlanResult =
        serde_json::from_str(json_str).expect("解析用户签约计划结果");
    assert!(result.sign_plan.is_some());
    let plan = result.sign_plan.as_ref().unwrap();
    assert_eq!(plan.sign_plan_id.as_deref(), Some("PLAN_001"));
    assert_eq!(plan.openid.as_deref(), Some("openid_001"));
    assert_eq!(plan.sign_state.as_deref(), Some("SIGNED"));
    assert_eq!(plan.plan_name.as_deref(), Some("月度会员"));
    assert_eq!(result.pack.as_deref(), Some("package_token_001"));
}

/// 对应 Java: WxPartnerPayScoreUserSignPlanResultTest（用户签约计划结果序列化往返）
#[test]
fn test_partner_pay_score_user_sign_plan_result_roundtrip() {
    let json_str = r#"{
        "sign_plan": {
            "sign_plan_id": "PLAN_002",
            "openid": "openid_002",
            "service_id": "SERVICE_002",
            "mchid": "9876543210",
            "appid": "wx9876543210",
            "merchant_sign_plan_no": "MPLAN_002",
            "plan_id": "PLAN_ID_002",
            "sign_state": "CANCELLED",
            "plan_name": "季度会员",
            "total_origin_price": 3000,
            "deduction_quantity": 3,
            "total_actual_price": 2400,
            "signed_detail_list": []
        },
        "package": "package_token_002"
    }"#;
    let result: WxPartnerPayScoreUserSignPlanResult = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxPartnerPayScoreUserSignPlanResult =
        serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result.sign_plan, result2.sign_plan);
    assert_eq!(result.pack, result2.pack);
}

/// 对应 Java: WxPartnerPayScoreUserSignPlanResultTest（用户签约计划结果空值验证）
#[test]
fn test_partner_pay_score_user_sign_plan_result_empty() {
    let json_str = r#"{}"#;
    let result: WxPartnerPayScoreUserSignPlanResult =
        serde_json::from_str(json_str).expect("解析空 JSON");
    assert!(result.sign_plan.is_none());
    assert!(result.pack.is_none());
}

// ═══════════════════════════════════════════════════════════════
// #3 PartnerUserSignPlanEntityTest —— 签约计划实体 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: PartnerUserSignPlanEntityTest（签约计划实体 JSON 反序列化）
#[test]
fn test_partner_user_sign_plan_entity_from_json() {
    let json_str = r#"{
        "sign_plan_id": "PLAN_001",
        "openid": "openid_001",
        "sub_openid": "sub_openid_001",
        "service_id": "SERVICE_001",
        "mchid": "1234567890",
        "sub_mchid": "sub_1234567890",
        "appid": "wx1234567890",
        "sub_appid": "wx_sub_1234567890",
        "merchant_sign_plan_no": "MPLAN_001",
        "merchant_callback_url": "https://example.com/callback",
        "plan_id": "PLAN_ID_001",
        "going_detail_no": 1,
        "sign_state": "SIGNED",
        "plan_name": "月度会员",
        "plan_over_time": "2026-12-31 23:59:59",
        "total_origin_price": 1000,
        "deduction_quantity": 1,
        "total_actual_price": 800,
        "signed_detail_list": [],
        "sign_time": "2026-01-01 00:00:00"
    }"#;
    let entity: PartnerUserSignPlanEntity =
        serde_json::from_str(json_str).expect("解析签约计划实体");
    assert_eq!(entity.sign_plan_id.as_deref(), Some("PLAN_001"));
    assert_eq!(entity.openid.as_deref(), Some("openid_001"));
    assert_eq!(entity.service_id.as_deref(), Some("SERVICE_001"));
    assert_eq!(entity.sign_state.as_deref(), Some("SIGNED"));
    assert_eq!(entity.plan_name.as_deref(), Some("月度会员"));
    assert_eq!(entity.going_detail_no, Some(1));
    assert_eq!(entity.total_origin_price, Some(1000));
    assert_eq!(entity.deduction_quantity, Some(1));
    assert_eq!(entity.total_actual_price, Some(800));
}

/// 对应 Java: PartnerUserSignPlanEntityTest（签约计划实体序列化往返验证）
#[test]
fn test_partner_user_sign_plan_entity_roundtrip() {
    let json_str = r#"{
        "sign_plan_id": "PLAN_002",
        "openid": "openid_002",
        "service_id": "SERVICE_002",
        "mchid": "9876543210",
        "appid": "wx9876543210",
        "merchant_sign_plan_no": "MPLAN_002",
        "plan_id": "PLAN_ID_002",
        "sign_state": "CANCELLED",
        "plan_name": "季度会员",
        "total_origin_price": 3000,
        "deduction_quantity": 3,
        "total_actual_price": 2400,
        "signed_detail_list": []
    }"#;
    let entity: PartnerUserSignPlanEntity = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&entity).expect("序列化");
    let entity2: PartnerUserSignPlanEntity = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(entity.sign_plan_id, entity2.sign_plan_id);
    assert_eq!(entity.openid, entity2.openid);
    assert_eq!(entity.sign_state, entity2.sign_state);
    assert_eq!(entity.plan_name, entity2.plan_name);
}

/// 对应 Java: PartnerUserSignPlanEntityTest（签约计划实体取消状态解析）
#[test]
fn test_partner_user_sign_plan_entity_cancelled() {
    let json_str = r#"{
        "sign_plan_id": "PLAN_003",
        "openid": "openid_003",
        "service_id": "SERVICE_003",
        "mchid": "1111111111",
        "appid": "wx1111111111",
        "merchant_sign_plan_no": "MPLAN_003",
        "plan_id": "PLAN_ID_003",
        "sign_state": "CANCELLED",
        "cancel_sign_time": "2026-06-01 12:00:00",
        "cancel_sign_type": "USER",
        "cancel_reason": "用户主动解约",
        "plan_name": "年费会员",
        "total_origin_price": 12000,
        "deduction_quantity": 6,
        "total_actual_price": 6000,
        "signed_detail_list": []
    }"#;
    let entity: PartnerUserSignPlanEntity =
        serde_json::from_str(json_str).expect("解析取消状态实体");
    assert_eq!(entity.sign_state.as_deref(), Some("CANCELLED"));
    assert_eq!(
        entity.cancel_sign_time.as_deref(),
        Some("2026-06-01 12:00:00")
    );
    assert_eq!(entity.cancel_sign_type.as_deref(), Some("USER"));
    assert_eq!(entity.cancel_reason.as_deref(), Some("用户主动解约"));
}

// ═══════════════════════════════════════════════════════════════
// #4 PayScorePlanDetailResultTest —— 计划详情结果解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: PayScorePlanDetailResultTest（计划详情结果 JSON 解析）
#[test]
fn test_pay_score_plan_detail_result_from_json() {
    let json_str = r#"{
        "original_price": 1000,
        "plan_discount_description": "首月8折",
        "actual_price": 800,
        "plan_detail_name": "月度会员",
        "plan_detail_no": 1
    }"#;
    let detail: PayScorePlanDetailResult =
        serde_json::from_str(json_str).expect("解析计划详情结果");
    assert_eq!(detail.original_price, Some(1000));
    assert_eq!(detail.plan_discount_description.as_deref(), Some("首月8折"));
    assert_eq!(detail.actual_price, Some(800));
    assert_eq!(detail.plan_detail_name.as_deref(), Some("月度会员"));
    assert_eq!(detail.plan_detail_no, Some(1));
}

/// 对应 Java: PayScorePlanDetailResultTest（计划详情结果序列化往返验证）
#[test]
fn test_pay_score_plan_detail_result_roundtrip() {
    let json_str = r#"{
        "original_price": 2000,
        "plan_discount_description": "季度优惠",
        "actual_price": 1600,
        "plan_detail_name": "季度会员",
        "plan_detail_no": 2
    }"#;
    let detail: PayScorePlanDetailResult = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&detail).expect("序列化");
    let detail2: PayScorePlanDetailResult = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(detail, detail2);
}

// ═══════════════════════════════════════════════════════════════
// #5 PayScorePlanDetailTest —— 计划详情 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: PayScorePlanDetailTest（计划详情 JSON 解析）
#[test]
fn test_pay_score_plan_detail_from_json() {
    let json_str = r#"{
        "original_price": 500,
        "plan_discount_description": "新用户优惠",
        "actual_price": "400",
        "plan_detail_name": "体验会员",
        "plan_detail_no": 1
    }"#;
    let detail: PayScorePlanDetail = serde_json::from_str(json_str).expect("解析计划详情");
    assert_eq!(detail.original_price, Some(500));
    assert_eq!(
        detail.plan_discount_description.as_deref(),
        Some("新用户优惠")
    );
    assert_eq!(detail.actual_price.as_deref(), Some("400"));
    assert_eq!(detail.plan_detail_name.as_deref(), Some("体验会员"));
    assert_eq!(detail.plan_detail_no, Some(1));
}

/// 对应 Java: PayScorePlanDetailTest（计划详情序列化往返验证）
#[test]
fn test_pay_score_plan_detail_roundtrip() {
    let json_str = r#"{
        "original_price": 1500,
        "plan_discount_description": "年度优惠",
        "actual_price": "1200",
        "plan_detail_name": "年度会员",
        "plan_detail_no": 3
    }"#;
    let detail: PayScorePlanDetail = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&detail).expect("序列化");
    let detail2: PayScorePlanDetail = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(detail, detail2);
}

/// 对应 Java: PayScorePlanDetailTest（计划详情空值验证）
#[test]
fn test_pay_score_plan_detail_empty() {
    let json_str = r#"{}"#;
    let detail: PayScorePlanDetail = serde_json::from_str(json_str).expect("解析空 JSON");
    assert!(detail.original_price.is_none());
    assert!(detail.plan_discount_description.is_none());
    assert!(detail.actual_price.is_none());
    assert!(detail.plan_detail_name.is_none());
    assert!(detail.plan_detail_no.is_none());
}
