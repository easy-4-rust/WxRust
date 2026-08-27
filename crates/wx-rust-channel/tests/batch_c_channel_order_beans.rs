#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——Channel 订单 bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - OrderInfoParamTest（订单查询参数验证）
//! - OrderCouponInfoTest（订单优惠券信息解析）
//! - DecodeSensitiveInfoResponseTest（敏感信息解密响应解析）
//! - OrderAddressInfoTest（订单地址信息解析）
//! - OrderCustomInfoTest（订单自定义信息解析）

use wx_rust_channel::bean::order::*;

// ═══════════════════════════════════════════════════════════════
// #1 OrderInfoParamTest —— 订单查询参数验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: OrderInfoParamTest（订单查询参数 JSON 构建）
#[test]
fn test_order_info_param_serde() {
    let param = OrderInfoParam {
        order_id: "ORDER_001".to_string(),
        encode_sensitive_info: true,
    };
    let json_str = serde_json::to_string(&param).expect("序列化");
    let param2: OrderInfoParam = serde_json::from_str(&json_str).expect("反序列化");
    assert_eq!(param, param2);
    assert_eq!(param2.order_id, "ORDER_001");
    assert!(param2.encode_sensitive_info);
}

/// 对应 Java: OrderInfoParamTest（订单查询参数 JSON 反序列化）
#[test]
fn test_order_info_param_from_json() {
    let json_str = r#"{
        "order_id": "ORDER_002",
        "encode_sensitive_info": false
    }"#;
    let param: OrderInfoParam = serde_json::from_str(json_str).expect("解析订单查询参数");
    assert_eq!(param.order_id, "ORDER_002");
    assert!(!param.encode_sensitive_info);
}

/// 对应 Java: OrderInfoParamTest（订单查询参数默认值验证）
#[test]
fn test_order_info_param_default() {
    let json_str = r#"{}"#;
    let param: OrderInfoParam = serde_json::from_str(json_str).expect("解析空 JSON");
    assert_eq!(param.order_id, "");
    assert!(!param.encode_sensitive_info);
}

// ═══════════════════════════════════════════════════════════════
// #2 OrderCouponInfoTest —— 订单优惠券信息解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: OrderCouponInfoTest（优惠券信息 JSON 解析）
#[test]
fn test_order_coupon_info_from_json() {
    let json_str = r#"{
        "user_coupon_id": "COUPON_001",
        "coupon_type": 1,
        "discounted_price": 500,
        "coupon_id": "COUPON_ID_001"
    }"#;
    let info: OrderCouponInfo = serde_json::from_str(json_str).expect("解析优惠券信息");
    assert_eq!(info.user_coupon_id, "COUPON_001");
    assert_eq!(info.coupon_type, 1);
    assert_eq!(info.discounted_price, 500);
    assert_eq!(info.coupon_id, "COUPON_ID_001");
}

/// 对应 Java: OrderCouponInfoTest（优惠券信息序列化往返验证）
#[test]
fn test_order_coupon_info_roundtrip() {
    let json_str = r#"{
        "user_coupon_id": "COUPON_002",
        "coupon_type": 2,
        "discounted_price": 1000,
        "coupon_id": "COUPON_ID_002"
    }"#;
    let info: OrderCouponInfo = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&info).expect("序列化");
    let info2: OrderCouponInfo = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(info, info2);
}

/// 对应 Java: OrderCouponInfoTest（优惠券信息默认值验证）
#[test]
fn test_order_coupon_info_default() {
    let json_str = r#"{}"#;
    let info: OrderCouponInfo = serde_json::from_str(json_str).expect("解析空 JSON");
    assert_eq!(info.user_coupon_id, "");
    assert_eq!(info.coupon_type, 0);
    assert_eq!(info.discounted_price, 0);
    assert_eq!(info.coupon_id, "");
}

// ═══════════════════════════════════════════════════════════════
// #3 DecodeSensitiveInfoResponseTest —— 敏感信息解密响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: DecodeSensitiveInfoResponseTest（敏感信息解密响应 JSON 解析）
#[test]
fn test_decode_sensitive_info_response_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "address_info": {
            "postal_code": "518000",
            "province_name": "广东省",
            "city_name": "深圳市",
            "county_name": "南山区",
            "virtual_order_tel_number": "13800000000"
        },
        "virtual_number_info": {
            "virtual_number": "13800000000",
            "extension": "1234",
            "expiration": 1627886400
        }
    }"#;
    let resp: DecodeSensitiveInfoResponse =
        serde_json::from_str(json_str).expect("解析敏感信息解密响应");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "ok");
    assert_eq!(resp.address_info.province_name, "广东省");
    assert_eq!(resp.address_info.city_name, "深圳市");
    assert_eq!(resp.virtual_number_info.virtual_number, "13800000000");
}

/// 对应 Java: DecodeSensitiveInfoResponseTest（敏感信息解密响应错误码解析）
#[test]
fn test_decode_sensitive_info_response_error() {
    let json_str = r#"{
        "errcode": 40001,
        "errmsg": "invalid credential",
        "address_info": {},
        "virtual_number_info": {}
    }"#;
    let resp: DecodeSensitiveInfoResponse = serde_json::from_str(json_str).expect("解析错误响应");
    assert_eq!(resp.err_code, 40001);
    assert_eq!(resp.err_msg, "invalid credential");
}

/// 对应 Java: DecodeSensitiveInfoResponseTest（敏感信息解密响应序列化往返验证）
#[test]
fn test_decode_sensitive_info_response_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "address_info": {
            "postal_code": "100000",
            "province_name": "北京市",
            "city_name": "北京市",
            "county_name": "朝阳区",
            "virtual_order_tel_number": ""
        },
        "virtual_number_info": {
            "virtual_number": "13900000000",
            "extension": "",
            "expiration": 0
        }
    }"#;
    let resp: DecodeSensitiveInfoResponse = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&resp).expect("序列化");
    let resp2: DecodeSensitiveInfoResponse = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(resp.err_code, resp2.err_code);
    assert_eq!(
        resp.address_info.province_name,
        resp2.address_info.province_name
    );
}

// ═══════════════════════════════════════════════════════════════
// #4 OrderAddressInfoTest —— 订单地址信息解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: OrderAddressInfoTest（订单地址信息 JSON 解析）
#[test]
fn test_order_address_info_from_json() {
    let json_str = r#"{
        "postal_code": "100000",
        "province_name": "北京市",
        "city_name": "北京市",
        "county_name": "朝阳区",
        "virtual_order_tel_number": "13800000000",
        "tel_number_ext_info": {},
        "use_tel_number": 1,
        "hash_code": "abc123"
    }"#;
    let info: OrderAddressInfo = serde_json::from_str(json_str).expect("解析订单地址信息");
    assert_eq!(info.postal_code, "100000");
    assert_eq!(info.province_name, "北京市");
    assert_eq!(info.city_name, "北京市");
    assert_eq!(info.county_name, "朝阳区");
    assert_eq!(info.virtual_order_tel_number, "13800000000");
    assert_eq!(info.use_tel_number, 1);
    assert_eq!(info.hash_code, "abc123");
}

/// 对应 Java: OrderAddressInfoTest（订单地址信息序列化往返验证）
#[test]
fn test_order_address_info_roundtrip() {
    let json_str = r#"{
        "postal_code": "200000",
        "province_name": "上海市",
        "city_name": "上海市",
        "county_name": "浦东新区",
        "virtual_order_tel_number": "",
        "tel_number_ext_info": {},
        "use_tel_number": 0,
        "hash_code": ""
    }"#;
    let info: OrderAddressInfo = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&info).expect("序列化");
    let info2: OrderAddressInfo = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(info, info2);
}

// ═══════════════════════════════════════════════════════════════
// #5 OrderCustomInfoTest —— 订单自定义信息解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: OrderCustomInfoTest（订单自定义信息 JSON 解析）
#[test]
fn test_order_custom_info_from_json() {
    let json_str = r#"{
        "custom_img_url": "https://example.com/custom.png",
        "custom_word": "自定义备注信息",
        "custom_type": 1,
        "custom_preview_img_url": "https://example.com/preview.png"
    }"#;
    let info: OrderCustomInfo = serde_json::from_str(json_str).expect("解析订单自定义信息");
    assert_eq!(info.custom_img_url, "https://example.com/custom.png");
    assert_eq!(info.custom_word, "自定义备注信息");
    assert_eq!(info.custom_type, 1);
    assert_eq!(
        info.custom_preview_img_url,
        "https://example.com/preview.png"
    );
}

/// 对应 Java: OrderCustomInfoTest（订单自定义信息序列化往返验证）
#[test]
fn test_order_custom_info_roundtrip() {
    let json_str = r#"{
        "custom_img_url": "",
        "custom_word": "测试备注",
        "custom_type": 0,
        "custom_preview_img_url": ""
    }"#;
    let info: OrderCustomInfo = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&info).expect("序列化");
    let info2: OrderCustomInfo = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(info, info2);
}

/// 对应 Java: OrderCustomInfoTest（订单自定义信息默认值验证）
#[test]
fn test_order_custom_info_default() {
    let json_str = r#"{}"#;
    let info: OrderCustomInfo = serde_json::from_str(json_str).expect("解析空 JSON");
    assert_eq!(info.custom_img_url, "");
    assert_eq!(info.custom_word, "");
    assert_eq!(info.custom_type, 0);
    assert_eq!(info.custom_preview_img_url, "");
}
