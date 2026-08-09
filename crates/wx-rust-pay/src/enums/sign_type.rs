//! 签名类型枚举。
//!
//! 对应 Java `WxPayConstants.SignType`（`HMAC_SHA256`/`MD5`），
//! v2 报文签名算法选择器。

use crate::constant::wx_pay_constants::sign_type;

/// 微信支付 v2 签名类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignType {
    /// HMAC-SHA256（对应 Java `SignType.HMAC_SHA256`）
    HmacSha256,
    /// MD5（对应 Java `SignType.MD5`）
    Md5,
}

impl SignType {
    /// 字符串值（对应 Java 常量值）。
    pub const HMAC_SHA256: &'static str = sign_type::HMAC_SHA256;
    /// 字符串值（对应 Java 常量值）。
    pub const MD5: &'static str = sign_type::MD5;
    /// 全部支持的签名类型（对应 Java `SignType.ALL_SIGN_TYPES`）。
    pub const ALL_SIGN_TYPES: [&str; 2] = sign_type::ALL_SIGN_TYPES;

    /// 返回签名类型字符串（对应 Java 枚举/常量的字符串值）。
    pub fn as_str(&self) -> &'static str {
        match self {
            SignType::HmacSha256 => Self::HMAC_SHA256,
            SignType::Md5 => Self::MD5,
        }
    }

    /// 由字符串解析签名类型；`None` 时 Java 语义默认为 MD5。
    pub fn parse_str(value: &str) -> Option<SignType> {
        match value {
            v if v == Self::HMAC_SHA256 => Some(SignType::HmacSha256),
            v if v == Self::MD5 => Some(SignType::Md5),
            _ => None,
        }
    }
}

impl std::fmt::Display for SignType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
