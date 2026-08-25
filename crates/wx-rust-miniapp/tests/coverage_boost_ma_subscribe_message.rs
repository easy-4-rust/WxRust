//! Coverage boost: `wx_ma_subscribe_message.rs` (98 missed, 44.3%).
//!
//! Exercises `WxMaSubscribeMessage` serialization, `add_data`, `reset_value`
//! for all field types (thing/number/letter/symbol/character_string/
//! phone_number/car_number/name/phrase).

use wx_rust_miniapp::bean::wx_ma_subscribe_message::{MsgData, WxMaSubscribeMessage};

#[test]
fn default_values() {
    let msg = WxMaSubscribeMessage::new();
    assert!(msg.to_user.is_none());
    assert!(msg.template_id.is_none());
    assert!(msg.page.is_none());
    assert!(msg.data.is_empty());
    assert_eq!(msg.miniprogram_state.as_deref(), Some("formal"));
    assert_eq!(msg.lang.as_deref(), Some("zh_CN"));
}

#[test]
fn to_json_basic() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.to_user = Some("user_1".into());
    msg.template_id = Some("tpl_1".into());
    msg.page = Some("/pages/index".into());
    let json = msg.to_json().unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["touser"], "user_1");
    assert_eq!(v["template_id"], "tpl_1");
    assert_eq!(v["page"], "/pages/index");
    assert_eq!(v["miniprogram_state"], "formal");
    assert_eq!(v["lang"], "zh_CN");
}

#[test]
fn to_json_with_data() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.to_user = Some("user_1".into());
    msg.template_id = Some("tpl_1".into());
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "hello".into(),
    });
    let json = msg.to_json().unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["data"]["thing1"]["value"], "hello");
}

#[test]
fn add_data_chaining() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "a".into(),
    })
    .add_data(MsgData {
        name: "number1".into(),
        value: "123".into(),
    });
    assert_eq!(msg.data.len(), 2);
}

// ========================================================================
// reset_value: thing (20 chars max, truncate with "...")
// ========================================================================

#[test]
fn reset_value_thing_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "hello".into(),
    });
    assert_eq!(msg.data[0].value, "hello");
}

#[test]
fn reset_value_thing_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "a".repeat(25),
    });
    assert!(msg.data[0].value.ends_with("..."));
    assert_eq!(msg.data[0].value.chars().count(), 20); // 17 + "..."
}

#[test]
fn reset_value_empty_to_dash() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "".into(),
    });
    assert_eq!(msg.data[0].value, "-");
}

#[test]
fn reset_value_whitespace_to_dash() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "thing1".into(),
        value: "   ".into(),
    });
    assert_eq!(msg.data[0].value, "-");
}

// ========================================================================
// reset_value: number (32 digits, only digits/dot/dash)
// ========================================================================

#[test]
fn reset_value_number_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: "123.45".into(),
    });
    assert_eq!(msg.data[0].value, "123.45");
}

#[test]
fn reset_value_number_strip_letters() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: "12abc34".into(),
    });
    assert_eq!(msg.data[0].value, "1234");
}

#[test]
fn reset_value_number_invalid_to_zero() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: "abc".into(),
    });
    assert_eq!(msg.data[0].value, "0");
}

#[test]
fn reset_value_number_negative() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: "-42.5".into(),
    });
    assert_eq!(msg.data[0].value, "-42.5");
}

#[test]
fn reset_value_number_dot_only() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: ".5".into(),
    });
    assert_eq!(msg.data[0].value, ".5");
}

#[test]
fn reset_value_number_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "number1".into(),
        value: "1".repeat(40),
    });
    assert_eq!(msg.data[0].value.chars().count(), 32);
}

// ========================================================================
// reset_value: letter (32 chars, only alphabetic)
// ========================================================================

#[test]
fn reset_value_letter_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "letter1".into(),
        value: "abcXYZ".into(),
    });
    assert_eq!(msg.data[0].value, "abcXYZ");
}

#[test]
fn reset_value_letter_strip_non_alpha() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "letter1".into(),
        value: "ab12cd".into(),
    });
    assert_eq!(msg.data[0].value, "abcd");
}

#[test]
fn reset_value_letter_empty_to_a() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "letter1".into(),
        value: "123".into(),
    });
    assert_eq!(msg.data[0].value, "A");
}

// ========================================================================
// reset_value: symbol (5 chars, non-alphanumeric non-CJK)
// ========================================================================

#[test]
fn reset_value_symbol_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "symbol1".into(),
        value: "!@#".into(),
    });
    assert_eq!(msg.data[0].value, "!@#");
}

#[test]
fn reset_value_symbol_strip_alphanumeric() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "symbol1".into(),
        value: "a1!".into(),
    });
    assert_eq!(msg.data[0].value, "!");
}

#[test]
fn reset_value_symbol_empty_to_dash() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "symbol1".into(),
        value: "abc".into(),
    });
    assert_eq!(msg.data[0].value, "-");
}

#[test]
fn reset_value_symbol_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "symbol1".into(),
        value: "!@#$%^&*".into(),
    });
    assert_eq!(msg.data[0].value.chars().count(), 5);
}

// ========================================================================
// reset_value: character_string (32 chars, no CJK)
// ========================================================================

#[test]
fn reset_value_character_string_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "character_string1".into(),
        value: "abc123".into(),
    });
    assert_eq!(msg.data[0].value, "abc123");
}

#[test]
fn reset_value_character_string_strip_cjk() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "character_string1".into(),
        value: "abc你好def".into(),
    });
    assert_eq!(msg.data[0].value, "abcdef");
}

#[test]
fn reset_value_character_string_empty_to_zero() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "character_string1".into(),
        value: "你好".into(),
    });
    assert_eq!(msg.data[0].value, "0");
}

// ========================================================================
// reset_value: phone_number (17 chars, digits/+/-)
// ========================================================================

#[test]
fn reset_value_phone_number_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phone_number1".into(),
        value: "13800138000".into(),
    });
    assert_eq!(msg.data[0].value, "13800138000");
}

#[test]
fn reset_value_phone_number_with_plus() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phone_number1".into(),
        value: "+86-13800138000".into(),
    });
    assert_eq!(msg.data[0].value, "+86-13800138000");
}

#[test]
fn reset_value_phone_number_multiple_plus() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phone_number1".into(),
        value: "+86+138".into(),
    });
    assert_eq!(msg.data[0].value, "+86138");
}

#[test]
fn reset_value_phone_number_strip_letters() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phone_number1".into(),
        value: "abc".into(),
    });
    assert_eq!(msg.data[0].value, "0");
}

#[test]
fn reset_value_phone_number_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phone_number1".into(),
        value: "1".repeat(20),
    });
    assert_eq!(msg.data[0].value.chars().count(), 17);
}

// ========================================================================
// reset_value: car_number (8 chars max)
// ========================================================================

#[test]
fn reset_value_car_number_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "car_number1".into(),
        value: "京A12345".into(),
    });
    assert_eq!(msg.data[0].value, "京A12345");
}

#[test]
fn reset_value_car_number_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "car_number1".into(),
        value: "京A12345678".into(),
    });
    assert_eq!(msg.data[0].value.chars().count(), 8);
}

// ========================================================================
// reset_value: name (10 CJK or 20 alpha)
// ========================================================================

#[test]
fn reset_value_name_chinese() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "张三".into(),
    });
    assert_eq!(msg.data[0].value, "张三");
}

#[test]
fn reset_value_name_chinese_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "一二三四五六七八九十一二三".into(),
    });
    assert!(msg.data[0].value.ends_with("..."));
    assert_eq!(msg.data[0].value.chars().count(), 10); // 7 + "..."
}

#[test]
fn reset_value_name_english() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "John Doe".into(),
    });
    assert_eq!(msg.data[0].value, "John Doe");
}

#[test]
fn reset_value_name_english_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "a".repeat(25),
    });
    assert!(msg.data[0].value.ends_with("..."));
    assert_eq!(msg.data[0].value.chars().count(), 20); // 17 + "..."
}

#[test]
fn reset_value_name_strip_digits() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "John123".into(),
    });
    assert_eq!(msg.data[0].value, "John");
}

#[test]
fn reset_value_name_empty_to_dash() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "name1".into(),
        value: "123".into(),
    });
    assert_eq!(msg.data[0].value, "-");
}

// ========================================================================
// reset_value: phrase (5 CJK chars max)
// ========================================================================

#[test]
fn reset_value_phrase_normal() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phrase1".into(),
        value: "你好世界".into(),
    });
    assert_eq!(msg.data[0].value, "你好世界");
}

#[test]
fn reset_value_phrase_strip_non_cjk() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phrase1".into(),
        value: "你好abc".into(),
    });
    assert_eq!(msg.data[0].value, "你好");
}

#[test]
fn reset_value_phrase_empty_to_hao() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phrase1".into(),
        value: "abc".into(),
    });
    assert_eq!(msg.data[0].value, "好");
}

#[test]
fn reset_value_phrase_truncate() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "phrase1".into(),
        value: "一二三四五六七八".into(),
    });
    assert_eq!(msg.data[0].value.chars().count(), 5);
}

// ========================================================================
// Unknown field type (no reset)
// ========================================================================

#[test]
fn reset_value_unknown_type() {
    let mut msg = WxMaSubscribeMessage::new();
    msg.add_data(MsgData {
        name: "unknown1".into(),
        value: "any value".into(),
    });
    assert_eq!(msg.data[0].value, "any value");
}

// ========================================================================
// MsgData serde
// ========================================================================

#[test]
fn msg_data_serde_roundtrip() {
    let d = MsgData {
        name: "thing1".into(),
        value: "hello".into(),
    };
    let json = serde_json::to_string(&d).unwrap();
    let back: MsgData = serde_json::from_str(&json).unwrap();
    assert_eq!(d, back);
}

#[test]
fn msg_data_default() {
    let d = MsgData::default();
    assert!(d.name.is_empty());
    assert!(d.value.is_empty());
}
