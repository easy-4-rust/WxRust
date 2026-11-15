//! Real WxJava 4.8.4.B a49d6e1 golden: tools/compat/fixtures.jsonl.
//! The three primitive-coercion cases failed in the first real dual-runtime run.
use wx_rust_common::error::WxError;

#[test]
fn wxjava_string_errcode_is_coerced_to_i32() {
    let json = r#"{"errcode":"40001","errmsg":"invalid credential"}"#;
    let result = WxError::from_json(json);
    assert_eq!(result.error_code, 40001);
    assert_eq!(result.error_msg.as_deref(), Some("invalid credential"));
    assert_eq!(result.error_msg_en, None);
    assert_eq!(result.json.as_deref(), Some(json));
    let direct: WxError = serde_json::from_str(json).expect("serde format parity");
    assert_eq!(direct.error_code, 40001);
}

#[test]
fn wxjava_null_errcode_defaults_to_zero() {
    let json = r#"{"errcode":null,"errmsg":"empty code"}"#;
    let result = WxError::from_json(json);
    assert_eq!(result.error_code, 0);
    assert_eq!(result.error_msg.as_deref(), Some("empty code"));
    let direct: WxError = serde_json::from_str(json).expect("serde format parity");
    assert_eq!(direct.error_code, 0);
}

#[test]
fn wxjava_numeric_errmsg_is_coerced_to_string() {
    let json = r#"{"errcode":40001,"errmsg":123}"#;
    let result = WxError::from_json(json);
    assert_eq!(result.error_code, 40001);
    assert_eq!(result.error_msg.as_deref(), Some("123"));
    let direct: WxError = serde_json::from_str(json).expect("serde format parity");
    assert_eq!(direct.error_msg.as_deref(), Some("123"));
}

#[test]
fn missing_and_null_message_remain_none() {
    let missing = WxError::from_json(r#"{"errcode":0}"#);
    let null = WxError::from_json(r#"{"errcode":0,"errmsg":null}"#);
    assert_eq!(missing.error_msg, None);
    assert_eq!(null.error_msg, None);
}

#[test]
fn nested_object_is_not_coerced_into_a_message() {
    let json = r#"{"errcode":40001,"errmsg":{"nested":"bad"}}"#;
    assert!(serde_json::from_str::<WxError>(json).is_err());
}
