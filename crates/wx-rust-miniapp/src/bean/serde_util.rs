//! bean 层序列化工具。
//!
//! 对应 Java Gson 的宽容类型强转语义：
//! - Java `BigDecimal` 字段序列化为 JSON 数字；Rust 以 `String` 承载值时需
//!   [`ser_decimal`] 在序列化时转回数字（整数保持整数，小数保持小数）。
//! - Java `String` 字段被 Gson 强转接受 JSON 数字；Rust 需 [`de_num_or_str`]
//!   宽容反序列化。

use serde::{Deserializer, Serializer};

/// `String` 值序列化为 JSON 数字（对应 Java `BigDecimal` → JSON number）。
///
/// 整数串 → 整数（`"5"` → `5`）；小数串 → 浮点（`"121.281379"` → `121.281379`）；
/// 无法解析为数字时回退为字符串原样输出。
pub fn ser_decimal<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Ok(i) = value.parse::<i64>() {
        return serializer.serialize_i64(i);
    }
    if let Ok(f) = value.parse::<f64>() {
        return serializer.serialize_f64(f);
    }
    serializer.serialize_str(value)
}

/// 数字或字符串 → 字符串的宽容反序列化（对应 Java Gson 数字→String 强转）。
///
/// 微信接口部分字段（如 `watermark.timestamp`、msg_sec_check `label`、
/// delivery/live 的 `BigDecimal` 费用字段）返回 JSON 数字，而 Java bean 声明为
/// `String`；Gson 数字→String 强转可解析，serde 需自定义适配。
pub fn de_num_or_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct NumOrStrVisitor;
    impl serde::de::Visitor<'_> for NumOrStrVisitor {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("字符串或数字")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }

        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }
    }
    deserializer.deserialize_any(NumOrStrVisitor)
}
