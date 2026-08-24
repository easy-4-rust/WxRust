//! 微信支付异常（对应 Java `com.github.binarywang.wxpay.exception` 包）。

pub mod wx_pay_exception;
pub mod wx_sign_test_exception;

pub use wx_pay_exception::WxPayException;
pub use wx_sign_test_exception::WxSignTestException;
