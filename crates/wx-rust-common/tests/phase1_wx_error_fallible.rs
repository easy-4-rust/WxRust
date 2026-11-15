//! Additive fallible error parsing for Java WxError.fromJson exception semantics.
//! No breaking change to legacy WxError::from_json -> Self.
use wx_rust_common::error::{WxError, WxErrorParseError};

#[test]
fn malformed_json_is_a_typed_syntax_error() {
    let error = WxError::try_from_json("{not-json").expect_err("Java throws JsonSyntaxException");
    assert!(matches!(error, WxErrorParseError::MalformedJson(_)));
    assert_eq!(error.category(), "syntax_error");
}

#[test]
fn null_root_is_a_typed_null_root_error() {
    let error = WxError::try_from_json("null").expect_err("Java dereferences null");
    assert!(matches!(error, WxErrorParseError::NullRoot));
    assert_eq!(error.category(), "null_root");
}

#[test]
fn array_root_is_a_typed_syntax_error() {
    let error = WxError::try_from_json("[]").expect_err("Java throws JsonSyntaxException");
    assert!(matches!(
        error,
        WxErrorParseError::UnexpectedRootType("array")
    ));
    assert_eq!(error.category(), "syntax_error");
}

#[test]
fn object_field_parse_errors_are_separate_from_syntax_errors() {
    let error = WxError::try_from_json(r#"{"errcode":"not-a-number"}"#).expect_err("invalid field");
    assert!(matches!(error, WxErrorParseError::InvalidField(_)));
    assert_eq!(error.category(), "field_error");
}

#[test]
fn valid_json_keeps_public_type_and_original_json() {
    let raw = r#"{"errcode":"40001","errmsg":123}"#;
    let error = WxError::try_from_json(raw).expect("Gson primitive coercion");
    assert_eq!(error.error_code, 40001);
    assert_eq!(error.error_msg.as_deref(), Some("123"));
    assert_eq!(error.json.as_deref(), Some(raw));
}

#[test]
fn legacy_non_fallible_api_preserves_existing_fallback() {
    for raw in ["{not-json", "null", "[]"] {
        let error = WxError::from_json(raw);
        assert_eq!(error.error_code, -99);
        assert!(
            error
                .error_msg
                .as_deref()
                .unwrap()
                .contains("JSON 解析失败")
        );
    }
}
