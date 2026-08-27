#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——MP 摇一摇 + 导购 bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxMpShakeAroundPageAddResultTest（摇一摇页面添加结果解析）
//! - WxMpShakeAroundRelationSearchResultTest（设备关系搜索结果解析）
//! - WxMpShakeAroundPageAddQueryTest（摇一摇页面添加请求体验证）
//! - WxMpShakeAroundDeviceBindPageQueryTest（设备绑定页面查询验证）
//! - WxMpShakeAroundRelationSearchQueryTest（关系搜索查询验证）
//! - WxMpDeviceIdentifierTest（设备标识符 JSON 解析）
//! - WxMpGuideBuyerRespTest（导购员响应解析）

use wx_rust_mp::bean::guide::WxMpGuideBuyerResp;
use wx_rust_mp::bean::shake::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxMpShakeAroundPageAddResultTest —— 摇一摇页面添加结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpShakeAroundPageAddResultTest（页面添加结果 JSON 解析）
#[test]
fn test_shake_around_page_add_result_from_json() {
    let json_str = r#"{
        "errorCode": 0,
        "errorMsg": "success",
        "pageId": 12345
    }"#;
    let result = WxMpShakeAroundPageAddResult::from_json(json_str).expect("解析页面添加结果");
    assert_eq!(result.error_code, 0);
    assert_eq!(result.error_msg, "success");
    assert_eq!(result.page_id, 12345);
}

/// 对应 Java: WxMpShakeAroundPageAddResultTest（页面添加结果错误码解析）
#[test]
fn test_shake_around_page_add_result_error() {
    let json_str = r#"{
        "errorCode": -1,
        "errorMsg": "system error",
        "pageId": 0
    }"#;
    let result = WxMpShakeAroundPageAddResult::from_json(json_str).expect("解析错误结果");
    assert_eq!(result.error_code, -1);
    assert_eq!(result.error_msg, "system error");
    assert_eq!(result.page_id, 0);
}

/// 对应 Java: WxMpShakeAroundPageAddResultTest（页面添加结果序列化往返验证）
#[test]
fn test_shake_around_page_add_result_roundtrip() {
    let json_str = r#"{
        "errorCode": 0,
        "errorMsg": "ok",
        "pageId": 67890
    }"#;
    let result = WxMpShakeAroundPageAddResult::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxMpShakeAroundPageAddResult =
        serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

// ═══════════════════════════════════════════════════════════════
// #2 WxMpShakeAroundRelationSearchResultTest —— 设备关系搜索结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpShakeAroundRelationSearchResultTest（关系搜索结果 JSON 解析）
#[test]
fn test_shake_around_relation_search_result_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "success",
        "data": {
            "relations": [
                {"device_id": 10001, "uuid": "UUID_001", "page_id": 1, "major": 1, "minor": 1},
                {"device_id": 10002, "uuid": "UUID_002", "page_id": 2, "major": 1, "minor": 2}
            ],
            "total_count": 2
        }
    }"#;
    let result =
        WxMpShakeAroundRelationSearchResult::from_json(json_str).expect("解析关系搜索结果");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.errmsg, "success");
    assert_eq!(result.data.total_count, 2);
    assert_eq!(result.data.relations.len(), 2);
    assert_eq!(result.data.relations[0].device_id, 10001);
    assert_eq!(result.data.relations[1].uuid, "UUID_002");
}

/// 对应 Java: WxMpShakeAroundRelationSearchResultTest（空关系搜索结果）
#[test]
fn test_shake_around_relation_search_result_empty() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "data": {
            "relations": [],
            "total_count": 0
        }
    }"#;
    let result = WxMpShakeAroundRelationSearchResult::from_json(json_str).expect("解析空搜索结果");
    assert_eq!(result.errcode, 0);
    assert!(result.data.relations.is_empty());
    assert_eq!(result.data.total_count, 0);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxMpShakeAroundPageAddQueryTest —— 摇一摇页面添加请求体
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpShakeAroundPageAddQueryTest（页面添加请求 JSON 构建）
#[test]
fn test_shake_around_page_add_query_serde() {
    let query = WxMpShakeAroundPageAddQuery {
        title: "测试页面".to_string(),
        description: "摇一摇测试页面".to_string(),
        page_url: "https://example.com/page".to_string(),
        comment: "测试备注".to_string(),
        icon_url: "https://example.com/icon.png".to_string(),
    };
    let json_str = serde_json::to_string(&query).expect("序列化");
    let query2: WxMpShakeAroundPageAddQuery = serde_json::from_str(&json_str).expect("反序列化");
    assert_eq!(query, query2);
    assert_eq!(query2.title, "测试页面");
    assert_eq!(query2.page_url, "https://example.com/page");
}

/// 对应 Java: WxMpShakeAroundPageAddQueryTest（页面添加请求 JSON 反序列化）
#[test]
fn test_shake_around_page_add_query_from_json() {
    let json_str = r#"{
        "title": "活动页面",
        "description": "摇一摇活动",
        "pageUrl": "https://example.com/activity",
        "comment": "活动备注",
        "iconUrl": "https://example.com/activity_icon.png"
    }"#;
    let query: WxMpShakeAroundPageAddQuery =
        serde_json::from_str(json_str).expect("解析页面添加请求");
    assert_eq!(query.title, "活动页面");
    assert_eq!(query.description, "摇一摇活动");
    assert_eq!(query.page_url, "https://example.com/activity");
    assert_eq!(query.comment, "活动备注");
    assert_eq!(query.icon_url, "https://example.com/activity_icon.png");
}

// ═══════════════════════════════════════════════════════════════
// #4 WxMpShakeAroundDeviceBindPageQueryTest —— 设备绑定页面查询
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpShakeAroundDeviceBindPageQueryTest（设备绑定页面查询 JSON 构建）
#[test]
fn test_shake_around_device_bind_page_query_serde() {
    let query = WxMpShakeAroundDeviceBindPageQuery {
        device_identifier: WxMpDeviceIdentifier {
            device_id: 10001,
            uuid: "UUID_001".to_string(),
            page_id: 1,
            major: 1,
            minor: 1,
        },
        page_ids: vec![1, 2, 3],
    };
    let json_str = serde_json::to_string(&query).expect("序列化");
    let query2: WxMpShakeAroundDeviceBindPageQuery =
        serde_json::from_str(&json_str).expect("反序列化");
    assert_eq!(query, query2);
    assert_eq!(query2.device_identifier.device_id, 10001);
    assert_eq!(query2.page_ids.len(), 3);
}

/// 对应 Java: WxMpShakeAroundDeviceBindPageQueryTest（设备绑定页面查询 JSON 反序列化）
#[test]
fn test_shake_around_device_bind_page_query_from_json() {
    let json_str = r#"{
        "deviceIdentifier": {
            "device_id": 20001,
            "uuid": "UUID_002",
            "page_id": 2,
            "major": 1,
            "minor": 2
        },
        "pageIds": [4, 5]
    }"#;
    let query: WxMpShakeAroundDeviceBindPageQuery =
        serde_json::from_str(json_str).expect("解析设备绑定查询");
    assert_eq!(query.device_identifier.device_id, 20001);
    assert_eq!(query.device_identifier.uuid, "UUID_002");
    assert_eq!(query.page_ids, vec![4, 5]);
}

// ═══════════════════════════════════════════════════════════════
// #5 WxMpShakeAroundRelationSearchQueryTest —— 关系搜索查询
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpShakeAroundRelationSearchQueryTest（关系搜索查询 JSON 构建）
#[test]
fn test_shake_around_relation_search_query_serde() {
    let query = WxMpShakeAroundRelationSearchQuery {
        r#type: 1,
        page_id: 100,
        begin: 0,
        count: 10,
        device_identifier: WxMpDeviceIdentifier {
            device_id: 10001,
            uuid: "UUID_001".to_string(),
            page_id: 1,
            major: 1,
            minor: 1,
        },
    };
    let json_str = serde_json::to_string(&query).expect("序列化");
    let query2: WxMpShakeAroundRelationSearchQuery =
        serde_json::from_str(&json_str).expect("反序列化");
    assert_eq!(query, query2);
}

/// 对应 Java: WxMpShakeAroundRelationSearchQueryTest（关系搜索查询 JSON 反序列化）
#[test]
fn test_shake_around_relation_search_query_from_json() {
    let json_str = r#"{
        "type": 2,
        "pageId": 200,
        "begin": 10,
        "count": 20,
        "deviceIdentifier": {
            "device_id": 30001,
            "uuid": "UUID_003",
            "page_id": 3,
            "major": 2,
            "minor": 1
        }
    }"#;
    let query: WxMpShakeAroundRelationSearchQuery =
        serde_json::from_str(json_str).expect("解析关系搜索查询");
    assert_eq!(query.r#type, 2);
    assert_eq!(query.page_id, 200);
    assert_eq!(query.begin, 10);
    assert_eq!(query.count, 20);
    assert_eq!(query.device_identifier.device_id, 30001);
}

// ═══════════════════════════════════════════════════════════════
// #6 WxMpDeviceIdentifierTest —— 设备标识符 JSON 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpDeviceIdentifierTest（设备标识符 JSON 反序列化）
#[test]
fn test_device_identifier_from_json() {
    let json_str = r#"{
        "device_id": 10001,
        "uuid": "FDA50693-A4E2-4FB1-AFCF-C6EB0764E826",
        "page_id": 1,
        "major": 1,
        "minor": 1
    }"#;
    let device: WxMpDeviceIdentifier = serde_json::from_str(json_str).expect("解析设备标识符");
    assert_eq!(device.device_id, 10001);
    assert_eq!(device.uuid, "FDA50693-A4E2-4FB1-AFCF-C6EB0764E826");
    assert_eq!(device.page_id, 1);
    assert_eq!(device.major, 1);
    assert_eq!(device.minor, 1);
}

/// 对应 Java: WxMpDeviceIdentifierTest（设备标识符序列化往返验证）
#[test]
fn test_device_identifier_roundtrip() {
    let json_str = r#"{
        "device_id": 20002,
        "uuid": "E2C56DB5-DFFB-48D2-B060-D0F5A71096E0",
        "page_id": 2,
        "major": 2,
        "minor": 3
    }"#;
    let device: WxMpDeviceIdentifier = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&device).expect("序列化");
    let device2: WxMpDeviceIdentifier = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(device, device2);
}

/// 对应 Java: WxMpDeviceIdentifierTest（设备标识符默认值验证）
#[test]
fn test_device_identifier_default() {
    let device = WxMpDeviceIdentifier::default();
    assert_eq!(device.device_id, 0);
    assert_eq!(device.uuid, "");
    assert_eq!(device.page_id, 0);
    assert_eq!(device.major, 0);
    assert_eq!(device.minor, 0);
}

// ═══════════════════════════════════════════════════════════════
// #7 WxMpGuideBuyerRespTest —— 导购员响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpGuideBuyerRespTest（导购员响应 JSON 解析）
#[test]
fn test_guide_buyer_resp_from_json() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "openid": "openid_001"
    }"#;
    let resp: WxMpGuideBuyerResp = serde_json::from_str(json_str).expect("解析导购员响应");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "ok");
    assert_eq!(resp.openid, "openid_001");
}

/// 对应 Java: WxMpGuideBuyerRespTest（导购员响应错误码解析）
#[test]
fn test_guide_buyer_resp_error() {
    let json_str = r#"{
        "errcode": 40001,
        "errmsg": "invalid credential",
        "openid": ""
    }"#;
    let resp: WxMpGuideBuyerResp = serde_json::from_str(json_str).expect("解析错误响应");
    assert_eq!(resp.err_code, 40001);
    assert_eq!(resp.err_msg, "invalid credential");
    assert_eq!(resp.openid, "");
}

/// 对应 Java: WxMpGuideBuyerRespTest（导购员响应序列化往返验证）
#[test]
fn test_guide_buyer_resp_roundtrip() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "openid": "openid_002"
    }"#;
    let resp: WxMpGuideBuyerResp = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&resp).expect("序列化");
    let resp2: WxMpGuideBuyerResp = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(resp, resp2);
}
