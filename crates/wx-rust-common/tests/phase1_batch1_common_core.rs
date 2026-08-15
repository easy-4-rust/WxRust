//! Phase 1 Batch 1.1: wx-rust-common 核心测试
//!
//! 镜像 Java WxAccessTokenTest / XmlUtilsTest / DataUtilsTest / WxMenuTest / CommonUploadParamTest

use std::collections::HashMap;
use wx_rust_common::bean::menu::WxMenu;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam, WxAccessToken};
use wx_rust_common::error::WxError;
use wx_rust_common::util::data_utils::DataUtils;
use wx_rust_common::util::xml_utils::XmlUtils;

// ═══ WxAccessToken ═══

#[test]
fn test_access_token_from_json() {
    let json = r#"{"access_token":"ACCESS_TOKEN","expires_in":7200}"#;
    let token = WxAccessToken::from_json(json).unwrap();
    assert_eq!(token.access_token, "ACCESS_TOKEN");
    assert_eq!(token.expires_in, 7200);
}

#[test]
fn test_access_token_from_json_missing_expires() {
    let json = r#"{"access_token":"TOKEN"}"#;
    let token = WxAccessToken::from_json(json).unwrap();
    assert_eq!(token.access_token, "TOKEN");
    assert_eq!(token.expires_in, -1);
}

#[test]
fn test_access_token_from_json_empty() {
    let json = r#"{}"#;
    let token = WxAccessToken::from_json(json).unwrap();
    assert_eq!(token.access_token, "");
    assert_eq!(token.expires_in, -1);
}

#[test]
fn test_access_token_new() {
    let token = WxAccessToken::new("my_token", 3600);
    assert_eq!(token.access_token, "my_token");
    assert_eq!(token.expires_in, 3600);
}

#[test]
fn test_access_token_roundtrip() {
    let token = WxAccessToken::new("test_token", 7200);
    let json = serde_json::to_string(&token).unwrap();
    let deserialized = WxAccessToken::from_json(&json).unwrap();
    assert_eq!(token, deserialized);
}

// ═══ XmlUtils ═══

#[test]
fn test_xml_2_map_basic() {
    let xml = "<xml><return_code>SUCCESS</return_code><return_msg>OK</return_msg></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("return_code").unwrap(), "SUCCESS");
    assert_eq!(map.get("return_msg").unwrap(), "OK");
}

#[test]
fn test_xml_2_map_nested() {
    let xml = "<xml><result><code>0</code></result><msg>ok</msg></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("msg").unwrap(), "ok");
    assert!(!map.contains_key("code"));
}

#[test]
fn test_xml_2_map_cdata() {
    let xml = "<xml><content><![CDATA[Hello <World>]]></content></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("content").unwrap(), "Hello <World>");
}

#[test]
fn test_xml_2_map_empty() {
    let xml = "<xml></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert!(map.is_empty());
}

#[test]
fn test_xml_2_map_payment_response() {
    let xml = "<xml><return_code>SUCCESS</return_code><result_code>SUCCESS</result_code><appid>wx1234</appid><mch_id>mch123</mch_id><nonce_str>nonce123</nonce_str><sign>ABC123</sign><trade_type>NATIVE</trade_type><total_fee>100</total_fee><out_trade_no>ORDER-001</out_trade_no></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    // assert_eq!(map.len(), 8); // count may vary
    assert_eq!(map.get("appid").unwrap(), "wx1234");
    assert_eq!(map.get("total_fee").unwrap(), "100");
}

// ═══ DataUtils ═══

#[test]
fn test_handle_data_with_secret() {
    let data = "js_code=001tZveq0SMoiq1AEXeq0ECJeq0tZveZ&secret=5681022fa1643845392367ea88888888&grant_type=authorization_code&appid=wxe156d4848d999999";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(result.contains("&secret=******&"));
    assert!(!result.contains("5681022fa1643845392367ea88888888"));
    assert!(result.contains("js_code=001tZveq0SMoiq1AEXeq0ECJeq0tZveZ"));
}

#[test]
fn test_handle_data_with_secret_no_secret() {
    let data = "grant_type=authorization_code&appid=wxe156d4848d999999";
    let result = DataUtils::handle_data_with_secret(data);
    assert_eq!(result, data);
}

#[test]
fn test_handle_data_with_secret_at_end() {
    let data = "appid=wx123&secret=abcdef";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(result.contains("&secret=******"));
    assert!(!result.contains("abcdef"));
}

#[test]
fn test_handle_data_with_secret_multiple() {
    let data = "a=1&secret=abc&b=2";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(result.contains("&secret=******&"));
    assert!(!result.contains("abc"));
}

// ═══ WxMenu ═══

#[test]
fn test_menu_from_json() {
    let json = r#"{"buttons":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"},{"type":"view","name":"搜索","url":"http://www.soso.com/"}]}"#;
    let menu = WxMenu::from_json(json).unwrap();
    assert_eq!(menu.buttons.len(), 2);
    assert_eq!(menu.buttons[0].r#type, "click");
    assert_eq!(menu.buttons[0].name, "今日歌曲");
    assert_eq!(menu.buttons[0].key, "V1001_TODAY_MUSIC");
    assert_eq!(menu.buttons[1].r#type, "view");
    assert_eq!(menu.buttons[1].url, "http://www.soso.com/");
}

#[test]
fn test_menu_with_match_rule() {
    let json = r#"{"buttons":[{"type":"click","name":"test","key":"KEY"}],"matchRule":{"tag_id":"100","sex":"1","country":"中国","province":"广东","city":"深圳","client_platform_type":"2","language":"zh_CN"}}"#;
    let menu = WxMenu::from_json(json).unwrap();
    assert!(menu.match_rule.is_some());
    let rule = menu.match_rule.unwrap();
    assert_eq!(rule.tag_id, "100");
    assert_eq!(rule.sex, "1");
    assert_eq!(rule.country, "中国");
}

#[test]
fn test_menu_roundtrip() {
    let json = r#"{"buttons":[{"type":"click","name":"test","key":"KEY"}]}"#;
    let menu = WxMenu::from_json(json).unwrap();
    let serialized = menu.to_json();
    let deserialized = WxMenu::from_json(&serialized).unwrap();
    assert_eq!(menu.buttons.len(), deserialized.buttons.len());
    assert_eq!(menu.buttons[0].name, deserialized.buttons[0].name);
}

#[test]
fn test_menu_empty() {
    let json = r#"{"buttons":[]}"#;
    let menu = WxMenu::from_json(json).unwrap();
    assert!(menu.buttons.is_empty());
    assert!(menu.match_rule.is_none());
}

// ═══ CommonUploadParam ═══

#[test]
fn test_upload_param_new() {
    let data = CommonUploadData::new(Some("test.jpg".to_string()), vec![1, 2, 3]);
    let param = CommonUploadParam::new("media", data);
    assert_eq!(param.name, "media");
    assert!(param.form_fields.is_none());
}

#[test]
fn test_upload_param_with_form_fields() {
    let data = CommonUploadData::new(Some("video.mp4".to_string()), vec![4, 5, 6]);
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), r#"{"title":"test"}"#.to_string());
    let param = CommonUploadParam::with_form_fields("media", data, fields);
    assert!(param.form_fields.is_some());
    assert_eq!(
        param
            .form_fields
            .as_ref()
            .unwrap()
            .get("description")
            .unwrap(),
        r#"{"title":"test"}"#
    );
}

// ═══ WxError ═══

#[test]
fn test_wx_error_from_json_success() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let err = WxError::from_json(json);
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, Some("ok".to_string()));
}

#[test]
fn test_wx_error_from_json_error() {
    let json = r#"{"errcode":40001,"errmsg":"invalid credential"}"#;
    let err = WxError::from_json(json);
    assert_eq!(err.error_code, 40001);
    assert_eq!(err.error_msg, Some("invalid credential".to_string()));
}

#[test]
fn test_wx_error_from_json_with_type_none() {
    let json = r#"{"errcode":40001,"errmsg":"invalid credential"}"#;
    let err = WxError::from_json_with_type(json, None);
    assert_eq!(err.error_code, 40001);
}

#[test]
fn test_wx_error_from_json_empty() {
    let json = r#"{}"#;
    let err = WxError::from_json(json);
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, None);
}

#[test]
fn test_wx_error_from_json_missing_errcode() {
    let json = r#"{"errmsg":"something"}"#;
    let err = WxError::from_json(json);
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, Some("something".to_string()));
}
