//! 微信错误对象。
//!
//! 对应 Java `me.chanjar.weixin.common.error.WxError`。

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
    #[serde(rename = "errcode", default)]
    pub error_code: i32,

    /// 错误信息（如果可以翻译为中文，就为中文）
    #[serde(rename = "errmsg")]
    pub error_msg: Option<String>,

    /// 微信接口返回的错误原始信息（英文）
    pub error_msg_en: Option<String>,

    /// 微信接口原始响应报文（仅本地使用，不参与序列化）
    #[serde(skip)]
    pub json: Option<String>,
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

        if err.error_code == 0 || wx_type.is_none() {
            return err;
        }
        if let Some(msg) = &err.error_msg
            && !msg.is_empty()
        {
            err.error_msg_en = Some(msg.clone());
        }

        if let Some(t) = wx_type {
            let translated = super::translate_error_msg(t, err.error_code);
            if let Some(msg) = translated {
                err.error_msg = Some(msg.to_string());
            }
        }
        err
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
