//! 试用小程序快速认证消息（仅供第三方开发者代小程序调用）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.message.WxOpenMaVerifyBetaWeappMessage`
//! （`@SerializedName` 线格式），引用的 `WxMaVerifyBetaWeappVerifyInfo`
//! 为 open 包内 `ma` 子包类型，serde 派生表达同一线格式。

use crate::bean::ma::WxMaVerifyBetaWeappVerifyInfo;

/// 试用小程序快速认证消息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaVerifyBetaWeappMessage {
    /// 企业法人认证需要的信息。
    #[serde(rename = "verify_info", default)]
    pub verify_info: Option<WxMaVerifyBetaWeappVerifyInfo>,
}
