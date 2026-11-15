//! Golden values obtained by executing pinned WxJava 4.8.4.B (a49d6e1).
//! Fixtures: tools/compat/fixtures.jsonl, sign-utf16-key-order{,-hmac}.
use std::collections::HashMap;
use wx_rust_pay::util::sign_utils::SignUtils;

fn unicode_params() -> HashMap<String, String> {
    [
        ("\u{e000}".to_owned(), "BMP".to_owned()),
        ("\u{10000}".to_owned(), "SUPPLEMENTARY".to_owned()),
        ("a".to_owned(), "ASCII".to_owned()),
    ]
    .into_iter()
    .collect()
}

#[test]
fn md5_matches_java_utf16_key_order() {
    let actual = SignUtils::create_sign(&unicode_params(), Some("MD5"), "fixture-key", &[])
        .expect("synthetic MD5 fixture");
    assert_eq!(actual, "13E7095E3558B0A585579764C14841DB");
}

#[test]
fn hmac_matches_java_utf16_key_order() {
    let actual = SignUtils::create_sign(&unicode_params(), Some("HMAC-SHA256"), "fixture-key", &[])
        .expect("synthetic HMAC fixture");
    assert_eq!(
        actual,
        "7EC82A2FF7914B831D977B8B17D8ABA8DC001668665CFB4473EEB6BB103CEB41"
    );
}
