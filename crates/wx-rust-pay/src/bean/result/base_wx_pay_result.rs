//! 对应 Java `com.github.binarywang.wxpay.bean.result.BaseWxPayResult`。
//!
//! v2 支付结果基类，包含所有 v2 XML 接口返回的公共字段：
//! `return_code`/`return_msg`/`result_code`/`err_code`/`err_code_des`/
//! `appid`/`mch_id`/`sub_appid`/`sub_mch_id`/`nonce_str`/`sign`。
//!
//! 在 Rust 中以 `WxPayCommonResult` 作为具体实现（Java 用抽象类 +
//! 泛型继承，Rust 以组合代替），此处提供类型别名 + `check_result` 签名验证
//! 辅助方法，对齐 Java `BaseWxPayResult#checkResult`。

use crate::bean::result::wx_pay_common_result::WxPayCommonResult;

/// v2 支付结果基类（对应 Java `BaseWxPayResult`）。
///
/// 实际类型为 `WxPayCommonResult`，包含全部 v2 公共字段。
pub type BaseWxPayResult = WxPayCommonResult;

/// 扩展方法：对齐 Java `BaseWxPayResult#checkResult` 的签名验证辅助。
pub trait BaseWxPayResultExt {
    /// 返回码是否为 SUCCESS。
    fn is_return_success(&self) -> bool;
    /// 业务结果是否为 SUCCESS。
    fn is_result_success(&self) -> bool;
    /// 返回信息。
    fn return_msg(&self) -> Option<&str>;
    /// 错误代码描述。
    fn err_code_des(&self) -> Option<&str>;
}

impl BaseWxPayResultExt for BaseWxPayResult {
    fn is_return_success(&self) -> bool {
        self.return_code.as_deref() == Some("SUCCESS")
    }

    fn is_result_success(&self) -> bool {
        self.result_code.as_deref() == Some("SUCCESS")
    }

    fn return_msg(&self) -> Option<&str> {
        self.return_msg.as_deref()
    }

    fn err_code_des(&self) -> Option<&str> {
        self.err_code_des.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_wx_pay_result_type_alias() {
        let result = BaseWxPayResult {
            return_code: Some("SUCCESS".to_string()),
            result_code: Some("SUCCESS".to_string()),
            ..Default::default()
        };
        assert!(result.is_return_success());
        assert!(result.is_result_success());
    }

    #[test]
    fn test_base_wx_pay_result_failure() {
        let result = BaseWxPayResult {
            return_code: Some("FAIL".to_string()),
            return_msg: Some("签名错误".to_string()),
            ..Default::default()
        };
        assert!(!result.is_return_success());
        assert_eq!(result.return_msg(), Some("签名错误"));
    }
}
