//! 消息 bean 的 serde 辅助（Rust 适配，无对应 Java 类）。
//!
//! 对应 Jackson 的宽松类型强制语义：
//! - [`AnyScalar`]：接受任意标量（字符串/数字/布尔）并保留其文本形式，
//!   对应 Java `Map<String, Object>` 中的 `Object`（unpack setter 场景）；
//! - `opt_string_or_i64` / `opt_string_or_i32`：数字字段同时接受 JSON 数字与
//!   数字字符串（对应 Jackson 自动将 `"1662480000"` 强转为 Long 的语义，
//!   见 Java 测试 `close_timestamp: "1662480000"`）。

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};

/// 任意标量值（文本形式）。
///
/// 对应 Java `Map<String, Object>` 值（unpack setter 的 `Object obj`）：
/// JSON 数字/布尔/字符串与 XML 文本统一以字符串承载，由调用方按目标字段
/// 类型解析（对应 Java `instanceof` 分支转换）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AnyScalar(pub(crate) String);

impl<'de> Deserialize<'de> for AnyScalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnyScalarVisitor;

        impl<'de> Visitor<'de> for AnyScalarVisitor {
            type Value = AnyScalar;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "任意标量（字符串/数字/布尔）")
            }

            fn visit_bool<E>(self, v: bool) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_i64<E>(self, v: i64) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_u64<E>(self, v: u64) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_f64<E>(self, v: f64) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_str<E>(self, v: &str) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v))
            }

            fn visit_char<E>(self, v: char) -> Result<AnyScalar, E> {
                Ok(AnyScalar(v.to_string()))
            }

            fn visit_unit<E>(self) -> Result<AnyScalar, E> {
                Ok(AnyScalar(String::new()))
            }

            fn visit_some<D2>(self, deserializer: D2) -> Result<AnyScalar, D2::Error>
            where
                D2: Deserializer<'de>,
            {
                AnyScalar::deserialize(deserializer)
            }

            fn visit_map<M>(self, mut map: M) -> Result<AnyScalar, M::Error>
            where
                M: MapAccess<'de>,
            {
                // quick-xml 对元素值 `deserialize_any` 返回 map（元素可能含
                // 子元素/属性）；纯文本元素时文本位于 `$text` 键，其余键跳过。
                let mut text: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "$text" {
                        text = Some(map.next_value::<String>()?);
                    } else {
                        let _ = map.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(AnyScalar(text.unwrap_or_default()))
            }
        }

        deserializer.deserialize_any(AnyScalarVisitor)
    }
}

/// `Option<i64>`：接受 JSON 数字或数字字符串（对应 Jackson Long 强转）。
pub(crate) fn opt_string_or_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<AnyScalar>::deserialize(deserializer)?
        .map(|s| {
            s.0.parse::<i64>()
                .map_err(|_| de::Error::custom(format!("无法将 `{}` 解析为 i64", s.0)))
        })
        .transpose()
}

/// `Option<i32>`：接受 JSON 数字或数字字符串（对应 Jackson Integer 强转）。
pub(crate) fn opt_string_or_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<AnyScalar>::deserialize(deserializer)?
        .map(|s| {
            s.0.parse::<i32>()
                .map_err(|_| de::Error::custom(format!("无法将 `{}` 解析为 i32", s.0)))
        })
        .transpose()
}
