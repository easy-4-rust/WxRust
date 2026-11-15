//! 微信错误对象。
//!
//! 对应 Java `me.chanjar.weixin.common.error.WxError`。

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::enums::WxType;

/// 微信错误码对象，承载微信接口返回的错误信息。
///
/// 当微信接口返回 `errcode` 非 0 时，通用执行器会将其封装为 [`WxError`]。
///
/// # 字段
/// - `error_code`：微信错误代码
/// - `error_msg`：错误信息（可翻译为中文）
/// - `error_msg_en`：微信接口返回的错误原始信息（英文）
/// - `json`：微信接口原始响应报文（仅本地使用，不参与序列化）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WxError {
    /// 微信错误代码
    #[serde(
        rename = "errcode",
        default,
        deserialize_with = "deserialize_gson_errcode"
    )]
    pub error_code: i32,

    /// 错误信息（如果可以翻译为中文，就为中文）
    #[serde(
        rename = "errmsg",
        default,
        deserialize_with = "deserialize_gson_errmsg"
    )]
    pub error_msg: Option<String>,

    /// 微信接口返回的错误原始信息（英文）
    pub error_msg_en: Option<String>,

    /// 微信接口原始响应报文（仅本地使用，不参与序列化）
    #[serde(skip)]
    pub json: Option<String>,
}

/// Emulates WxJava 4.8.4.B WxErrorAdapter and GsonHelper.getAsPrimitiveInt
/// when reading the errcode member.
///
/// Gson accepts numeric strings and JSON numbers; missing or JSON-null values
/// leave the primitive Java int at zero. Numeric strings use Java
/// Integer.parseInt semantics, while JSON numeric primitives use
/// Number.intValue semantics (including truncation of floating values).
fn deserialize_gson_errcode<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<serde_json::Value>::deserialize(deserializer)?;
    match raw {
        None => Ok(0),
        Some(serde_json::Value::String(text)) => text.parse::<i32>().map_err(de::Error::custom),
        Some(serde_json::Value::Number(number)) => {
            if let Some(value) = number.as_i64() {
                Ok(value as i32)
            } else if let Some(value) = number.as_u64() {
                Ok(value as i32)
            } else if let Some(value) = number.as_f64() {
                Ok(value as i32)
            } else {
                Err(de::Error::custom("invalid numeric errcode"))
            }
        }
        Some(_) => Err(de::Error::custom(
            "errcode must be a numeric JSON primitive",
        )),
    }
}

/// Emulates GsonHelper.getAsString on the errmsg member.
///
/// Gson converts JSON numbers and booleans to strings; missing and JSON null
/// remain absent. Arrays/objects are not silently converted to formatted JSON.
fn deserialize_gson_errmsg<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<serde_json::Value>::deserialize(deserializer)?;
    match raw {
        None => Ok(None),
        Some(serde_json::Value::String(text)) => Ok(Some(text)),
        Some(serde_json::Value::Number(number)) => Ok(Some(number.to_string())),
        Some(serde_json::Value::Bool(value)) => Ok(Some(value.to_string())),
        Some(_) => Err(de::Error::custom("errmsg must be a JSON primitive")),
    }
}

/// Structured parsing failures for callers needing WxJava-style failure propagation.
///
/// The legacy WxError::from_json continues to return a -99 fallback for
/// malformed input, preserving compatibility with existing Rust callers.
/// This additive API distinguishes malformed JSON, null roots, wrong
/// root types and invalid fields without throwing or panicking.
#[derive(Debug, thiserror::Error)]
pub enum WxErrorParseError {
    /// Invalid JSON syntax (Java Gson typically throws JsonSyntaxException).
    #[error("invalid JSON syntax: {0}")]
    MalformedJson(#[source] serde_json::Error),
    /// JSON root was null (Java WxError.fromJson dereferences null).
    #[error("JSON root is null, expected WxError object")]
    NullRoot,
    /// JSON root was an array or another non-object value.
    #[error("expected WxError JSON object, got {0}")]
    UnexpectedRootType(&'static str),
    /// Parsed JSON object contains an invalid field.
    #[error("invalid WxError JSON field: {0}")]
    InvalidField(#[source] serde_json::Error),
}

impl WxErrorParseError {
    /// Cross-runtime failure category, not a claim of identical exception types.
    pub const fn category(&self) -> &'static str {
        match self {
            Self::MalformedJson(_) | Self::UnexpectedRootType(_) => "syntax_error",
            Self::NullRoot => "null_root",
            Self::InvalidField(_) => "field_error",
        }
    }
}

impl WxError {
    /// 构建一个只有错误码与错误信息的 `WxError`。
    ///
    /// # 参数
    /// - `error_code`：微信错误代码
    /// - `error_msg`：错误信息
    pub fn new(error_code: i32, error_msg: impl Into<String>) -> Self {
        Self {
            error_code,
            error_msg: Some(error_msg.into()),
            error_msg_en: None,
            json: None,
        }
    }

    /// Parse a WxJava-compatible error object without hiding malformed input.
    ///
    /// This is additive: existing from_json callers still receive the
    /// historical -99 fallback rather than a Result.
    pub fn try_from_json(json: &str) -> Result<Self, WxErrorParseError> {
        Self::try_from_json_with_type(json, None)
    }

    /// Fallible variant of WxJava WxError.fromJson(json, wxType).
    ///
    /// A valid object follows the same platform translation as from_json;
    /// malformed input returns a structured Rust error instead of a -99 object.
    pub fn try_from_json_with_type(
        json: &str,
        wx_type: Option<WxType>,
    ) -> Result<Self, WxErrorParseError> {
        let root: serde_json::Value =
            serde_json::from_str(json).map_err(WxErrorParseError::MalformedJson)?;
        match &root {
            serde_json::Value::Null => return Err(WxErrorParseError::NullRoot),
            serde_json::Value::Object(_) => {}
            serde_json::Value::Array(_) => {
                return Err(WxErrorParseError::UnexpectedRootType("array"));
            }
            serde_json::Value::String(_) => {
                return Err(WxErrorParseError::UnexpectedRootType("string"));
            }
            serde_json::Value::Number(_) => {
                return Err(WxErrorParseError::UnexpectedRootType("number"));
            }
            serde_json::Value::Bool(_) => {
                return Err(WxErrorParseError::UnexpectedRootType("boolean"));
            }
        }
        let mut parsed: WxError =
            serde_json::from_value(root).map_err(WxErrorParseError::InvalidField)?;
        parsed.json = Some(json.to_string());
        Ok(Self::translate_parsed(parsed, wx_type))
    }

    fn translate_parsed(mut err: Self, wx_type: Option<WxType>) -> Self {
        if err.error_code == 0 || wx_type.is_none() {
            return err;
        }
        if let Some(msg) = &err.error_msg
            && !msg.is_empty()
        {
            err.error_msg_en = Some(msg.clone());
        }

        if let Some(t) = wx_type
            && let Some(translated) = super::translate_error_msg(t, err.error_code)
        {
            err.error_msg = Some(translated.to_string());
        }
        err
    }

    /// 从微信接口返回的 JSON 报文解析 `WxError`。
    ///
    /// # 参数
    /// - `json`：微信接口返回的 JSON 字符串
    ///
    /// # 返回
    /// 解析出的 `WxError`；若 JSON 无法解析则返回默认错误（错误码 -99）。
    pub fn from_json(json: &str) -> Self {
        Self::from_json_with_type(json, None)
    }

    /// 从微信接口返回的 JSON 报文解析 `WxError`，并按平台翻译错误信息。
    ///
    /// # 参数
    /// - `json`：微信接口返回的 JSON 字符串
    /// - `wx_type`：微信平台类型，用于选择对应的错误码翻译表
    ///
    /// # 返回
    /// 解析后的 `WxError`；错误码为 0 或未指定平台时不做翻译。
    pub fn from_json_with_type(json: &str, wx_type: Option<WxType>) -> Self {
        let mut err = match serde_json::from_str::<WxError>(json) {
            Ok(e) => e,
            Err(_) => WxError {
                error_code: -99,
                error_msg: Some(format!("JSON 解析失败，原始报文：{json}")),
                error_msg_en: None,
                json: Some(json.to_string()),
            },
        };
        err.json = Some(json.to_string());

        Self::translate_parsed(err, wx_type)
    }
}

impl std::fmt::Display for WxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(json) = &self.json {
            write!(
                f,
                "错误代码：{}, 错误信息：{}，微信原始报文：{}",
                self.error_code,
                self.error_msg.as_deref().unwrap_or(""),
                json
            )
        } else {
            write!(
                f,
                "错误代码：{}, 错误信息：{}",
                self.error_code,
                self.error_msg.as_deref().unwrap_or("")
            )
        }
    }
}
