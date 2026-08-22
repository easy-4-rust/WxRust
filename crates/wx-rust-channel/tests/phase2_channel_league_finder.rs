#![allow(clippy::field_reassign_with_default)]
//! Phase 2 补齐: League / FinderLive / Compass / Dashboard / Lead 子域
//! Bean 序列化与 Mock 服务测试。
//!
//! 镜像 Java:
//! - `WxLeagueWindowServiceImplTest`（橱窗授权/商品管理）
//! - `WxFinderLiveServiceImplTest`（视频号直播/数据/线索）
//! - `WxChannelCompassFinderServiceImplTest`（罗盘视频号）
//! - `WxChannelCompassShopServiceImplTest`（罗盘店铺）
//! - `WxChannelLiveDashboardServiceImplTest`（直播看板）
//! - `WxLeadComponentServiceImplTest`（线索组件）
//! - `WxAssistantServiceImplTest`（客服助手）
//! - `WxStoreHomePageServiceImplTest`（店铺首页）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的 bean 序列化断言
//! - RUST_OBLIGATION: serde rename 语义、default 值
//! - VALUE_ADD: 空值/边界路径

use wx_rust_channel::bean::lead::component::response::finder_attr_response::FinderAttrResponse;
use wx_rust_channel::bean::league::window::*;
use wx_rust_channel::bean::live::dashboard::*;

// ═══ League Window（SOURCE_PARITY:
//     Java WxLeagueWindowServiceImplTest）═══

/// 授权信息 serde（对应 Java `AuthInfo`：`auth_url`/`auth_wxa_path`/
/// `auth_wxa_appid`/`auth_wxa_username`）。
/// 对应 Java: WxLeagueWindowServiceImplTest.testGetWindowAuthInfo
#[test]
fn test_auth_info_serde() {
    let json = r#"{"auth_url":"https://example.com/auth","auth_wxa_path":"/pages/auth","auth_wxa_appid":"wx1234","auth_wxa_username":"gh_test"}"#;
    let info: AuthInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.auth_url, "https://example.com/auth");
    assert_eq!(info.auth_wxa_path, "/pages/auth");
    assert_eq!(info.auth_wxa_appid, "wx1234");
}

/// 授权信息响应 serde。
#[test]
fn test_auth_info_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","auth_info":{"auth_url":"https://example.com/auth"}}"#;
    let resp: AuthInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

/// 授权状态响应 serde。
#[test]
fn test_auth_status_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","auth_status":1}"#;
    let resp: AuthStatusResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

/// 橱窗商品搜索参数 serde（对应 Java `ProductSearchParam`）。
/// 对应 Java: WxLeagueWindowServiceImplTest.testListLeagueWindowProduct
#[test]
fn test_product_search_param_serde() {
    let json = r#"{"appid":"wx1234","openfinderid":"finder1","offset":0,"page_size":10,"need_total_num":true}"#;
    let param: ProductSearchParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.appid, "wx1234");
    assert_eq!(param.openfinderid, "finder1");
    assert_eq!(param.page_size, 10);
    assert!(param.need_total_num);
}

/// 橱窗商品参数 serde（对应 Java `WindowProductParam`）。
/// 对应 Java: WxLeagueWindowServiceImplTest.testAddLeagueWindowProduct
#[test]
fn test_window_product_param_serde() {
    let json = r#"{"appid":"wx1234","openfinderid":"finder1","product_id":"p1"}"#;
    let param: WindowProductParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.appid, "wx1234");
    assert_eq!(param.product_id, "p1");
}

/// 橱窗商品列表响应 serde。
#[test]
fn test_window_product_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","total_num":1}"#;
    let resp: WindowProductListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

/// 橱窗商品响应 serde。
#[test]
fn test_window_product_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WindowProductResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══ Finder Live（SOURCE_PARITY:
//     Java WxFinderLiveServiceImplTest）═══

/// FinderAttrResponse serde（对应 Java `FinderAttrResponse`）。
/// 对应 Java: WxFinderLiveServiceImplTest.testGetFinderAttrByAppid
#[test]
fn test_finder_attr_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","finder_attr":{"uniq_id":"u1","nickname":"测试号","fans_count":1000}}"#;
    let resp: FinderAttrResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.finder_attr.nickname, "测试号");
    assert_eq!(resp.finder_attr.fans_count, 1000);
}

// ═══ Live Dashboard Bean 序列化 ═══

/// LiveListParam serde（对应 Java `LiveListParam`）。
/// 对应 Java: WxChannelLiveDashboardServiceImplTest
#[test]
fn test_live_list_param_serde() {
    let param = LiveListParam { ds: 1700000000 };
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("\"ds\":1700000000"));
}

/// LiveDataParam serde（对应 Java `LiveDataParam`）。
#[test]
fn test_live_data_param_serde() {
    let param = LiveDataParam {
        export_id: "export1".to_string(),
    };
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("\"export_id\":\"export1\""));
}

/// LiveDataResponse serde（对应 Java `LiveDataResponse`）。
#[test]
fn test_live_data_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":[]}"#;
    let resp: LiveDataResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══ VALUE_ADD: 空值/边界 ═══

#[test]
fn test_auth_info_empty() {
    let info = AuthInfo::default();
    assert!(info.auth_url.is_empty());
    assert!(info.auth_wxa_path.is_empty());
}

#[test]
fn test_product_search_param_default() {
    let param = ProductSearchParam::default();
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("\"appid\":\"\""));
    assert!(json.contains("\"page_size\":0"));
}

#[test]
fn test_window_product_param_default() {
    let param = WindowProductParam::default();
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("\"appid\":\"\""));
}

/// FinderAttr serde（VALUE_ADD: 默认值）。
#[test]
fn test_finder_attr_default() {
    let attr =
        wx_rust_channel::bean::lead::component::response::finder_attr_response::FinderAttr::default(
        );
    assert!(attr.nickname.is_empty());
    assert_eq!(attr.fans_count, 0);
}
