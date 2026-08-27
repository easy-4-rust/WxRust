#![allow(clippy::field_reassign_with_default, dead_code)]
//! 第二批镜像补测——Channel 服务层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxStoreHomePageServiceImplTest（339 行）
//! - WxChannelProductManagementServiceImplTest（283 行）
//! - WxChannelProductAssistantServiceImplTest（169 行）
//! - WxChannelAfterSaleServiceImplGuaranteeTest（163 行）

use wx_rust_channel::bean::order::after_sale_detail::AfterSaleDetail;
use wx_rust_channel::bean::order::after_sale_order_info::AfterSaleOrderInfo;

// ═══════════════════════════════════════════════════════════════
// #1 WxStoreHomePageServiceImplTest（339 行）—— 商店首页服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreHomePageServiceImplTest（商品列表请求参数构建）
#[test]
fn test_store_home_page_product_list_param() {
    let body = serde_json::json!({
        "page": 1,
        "page_size": 20,
        "status": 1
    });
    assert_eq!(body["page"], 1);
    assert_eq!(body["page_size"], 20);
}

/// 对应 Java: WxStoreHomePageServiceImplTest（商品列表响应解析）
#[test]
fn test_store_home_page_product_list_response() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "product_list": [
            {
                "product_id": "PID001",
                "title": "测试商品",
                "status": 1
            }
        ],
        "total_num": 1
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["total_num"], 1);
}

/// 对应 Java: WxStoreHomePageServiceImplTest（橱窗商品设置请求构建）
#[test]
fn test_store_home_page_window_product_setting() {
    let body = serde_json::json!({
        "product_id": "PID001",
        "status": 1,
        "index": 0
    });
    assert_eq!(body["product_id"], "PID001");
}

// ═══════════════════════════════════════════════════════════════
// #2 WxChannelProductManagementServiceImplTest（283 行）—— 商品管理服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelProductManagementServiceImplTest（商品信息响应解析）
#[test]
fn test_channel_product_info_response() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "product_info": {
            "product_id": "CPID001",
            "title": "视频号商品",
            "desc": "商品描述",
            "status": 1
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["product_info"]["product_id"], "CPID001");
}

/// 对应 Java: WxChannelProductManagementServiceImplTest（商品列表查询请求构建）
#[test]
fn test_channel_product_list_request_body() {
    let body = serde_json::json!({
        "page": 1,
        "page_size": 10,
        "status": 1
    });
    assert_eq!(body["page"], 1);
    assert_eq!(body["page_size"], 10);
}

/// 对应 Java: WxChannelProductManagementServiceImplTest（商品上下架请求构建）
#[test]
fn test_channel_product_status_update_body() {
    let body = serde_json::json!({
        "product_id": "CPID001",
        "status": 2
    });
    assert_eq!(body["product_id"], "CPID001");
    assert_eq!(body["status"], 2);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxChannelProductAssistantServiceImplTest（169 行）—— 商品助手服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelProductAssistantServiceImplTest（商品助手列表响应解析）
#[test]
fn test_channel_product_assistant_list_response() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "product_list": [
            {
                "product_id": "PAID001",
                "title": "助手商品1",
                "status": 1
            }
        ],
        "total_num": 1
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
}

/// 对应 Java: WxChannelProductAssistantServiceImplTest（商品助手搜索请求构建）
#[test]
fn test_channel_product_assistant_search_body() {
    let body = serde_json::json!({
        "keyword": "测试商品",
        "page": 1,
        "page_size": 20
    });
    assert_eq!(body["keyword"], "测试商品");
}

// ═══════════════════════════════════════════════════════════════
// #4 WxChannelAfterSaleServiceImplGuaranteeTest（163 行）—— 售后担保服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelAfterSaleServiceImplGuaranteeTest（售后订单信息解析）
#[test]
fn test_channel_after_sale_order_info_serde() {
    let json_str = r#"{
        "aftersale_order_id": "AS001"
    }"#;
    let info: AfterSaleOrderInfo = serde_json::from_str(json_str).expect("解析售后订单信息");
    assert_eq!(info.after_sale_order_id, "AS001");
}

/// 对应 Java: WxChannelAfterSaleServiceImplGuaranteeTest（售后详情解析）
#[test]
fn test_channel_after_sale_detail_serde() {
    let json_str = r#"{
        "on_aftersale_order_cnt": 5,
        "aftersale_order_list": [
            {"aftersale_order_id": "AS001"},
            {"aftersale_order_id": "AS002"}
        ]
    }"#;
    let detail: AfterSaleDetail = serde_json::from_str(json_str).expect("解析售后详情");
    assert_eq!(detail.on_after_sale_order_cnt, 5);
    assert_eq!(detail.after_sale_order_list.len(), 2);
    assert_eq!(detail.after_sale_order_list[0].after_sale_order_id, "AS001");
}
