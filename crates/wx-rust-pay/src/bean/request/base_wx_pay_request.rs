//! 对应 Java `com.github.binarywang.wxpay.bean.request.BaseWxPayRequest`。
//!
//! v2 支付请求基类，包含所有 v2 XML 接口请求的公共字段：
//! `appid`/`mch_id`/`sub_appid`/`sub_mch_id`/`nonce_str`/`sign`/`sign_type`。
//!
//! 在 Rust 中以 `WxPayDefaultRequest` 作为具体实现（Java 用抽象类 +
//! 泛型继承，Rust 以组合代替），此处提供类型别名 + `check_and_sign`
//! 签名装配辅助方法，对齐 Java `BaseWxPayRequest#checkAndSign`。

use crate::bean::request::wx_pay_default_request::WxPayDefaultRequest;

/// v2 支付请求基类（对应 Java `BaseWxPayRequest`）。
///
/// 实际类型为 `WxPayDefaultRequest`，包含全部 v2 公共字段。
pub type BaseWxPayRequest = WxPayDefaultRequest;

/// 扩展方法：对齐 Java `BaseWxPayRequest#checkAndSign` 的签名装配辅助。
pub trait BaseWxPayRequestExt {
    /// 设置 appid。
    fn with_appid(self, appid: &str) -> Self;
    /// 设置 mch_id。
    fn with_mch_id(self, mch_id: &str) -> Self;
    /// 设置 nonce_str（自动生成随机串）。
    fn with_nonce_str(self, nonce_str: &str) -> Self;
    /// 设置 sign。
    fn with_sign(self, sign: &str) -> Self;
    /// 设置 sign_type。
    fn with_sign_type(self, sign_type: &str) -> Self;
}

impl BaseWxPayRequestExt for BaseWxPayRequest {
    fn with_appid(mut self, appid: &str) -> Self {
        self.appid = Some(appid.to_string());
        self
    }

    fn with_mch_id(mut self, mch_id: &str) -> Self {
        self.mch_id = Some(mch_id.to_string());
        self
    }

    fn with_nonce_str(mut self, nonce_str: &str) -> Self {
        self.nonce_str = Some(nonce_str.to_string());
        self
    }

    fn with_sign(mut self, sign: &str) -> Self {
        self.sign = Some(sign.to_string());
        self
    }

    fn with_sign_type(mut self, sign_type: &str) -> Self {
        self.sign_type = Some(sign_type.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_wx_pay_request_type_alias() {
        let req = BaseWxPayRequest::default();
        let req = req
            .with_appid("wx8888888888888888")
            .with_mch_id("1230000109")
            .with_nonce_str("random123")
            .with_sign("ABC123")
            .with_sign_type("MD5");
        assert_eq!(req.appid.as_deref(), Some("wx8888888888888888"));
        assert_eq!(req.mch_id.as_deref(), Some("1230000109"));
        assert_eq!(req.nonce_str.as_deref(), Some("random123"));
        assert_eq!(req.sign.as_deref(), Some("ABC123"));
        assert_eq!(req.sign_type.as_deref(), Some("MD5"));
    }

    #[test]
    fn test_xml_roundtrip() {
        let req = BaseWxPayRequest {
            appid: Some("wx8888888888888888".to_string()),
            mch_id: Some("1230000109".to_string()),
            nonce_str: Some("random123".to_string()),
            sign: Some("ABC123".to_string()),
            ..Default::default()
        };
        let xml = req.to_xml().expect("序列化 XML 成功");
        assert!(xml.contains("wx8888888888888888"));
        assert!(xml.contains("1230000109"));
    }
}
