//! 微信支付异常。
//!
//! 对应 Java `com.github.binarywang.wxpay.exception.WxPayException`：
//! 携带 v2 XML 报文的五元组（returnCode/returnMsg/resultCode/errCode/
//! errCodeDes）+ 原始报文 xmlString + 自定义文案 customErrorMsg；
//! `Builder.buildErrorMsg()` 以「，」拼接各非空片段。
//!
//! ADAPTED：Java 为受检异常（`extends Exception`）；Rust 以结构体承载
//! 同一字段集与文案拼装，经 [`From`] 转为 [`wx_rust_common::error::
//! WxErrorException::Runtime`] 进入服务既有错误通道（与 `api/impl` 各处
//! "对应 Java：throw new WxPayException(...)" 的映射一致）。

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

/// 微信支付异常（对应 Java `WxPayException`）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WxPayException {
    /// 自定义错误信息（对应 Java 字段 `customErrorMsg`）。
    custom_error_msg: Option<String>,
    /// 返回代码（对应 Java 字段 `returnCode`，v2 XML `return_code`）。
    return_code: Option<String>,
    /// 返回信息（对应 Java 字段 `returnMsg`，v2 XML `return_msg`）。
    return_msg: Option<String>,
    /// 结果代码（对应 Java 字段 `resultCode`，v2 XML `result_code`）。
    result_code: Option<String>,
    /// 错误代码（对应 Java 字段 `errCode`，v2 XML `err_code`）。
    err_code: Option<String>,
    /// 错误详情（对应 Java 字段 `errCodeDes`，v2 XML `err_code_des`）。
    err_code_des: Option<String>,
    /// 微信返回的原始 XML 报文（对应 Java 字段 `xmlString`）。
    xml_string: Option<String>,
}

impl WxPayException {
    /// 以自定义文案构造（对应 Java `WxPayException(String customErrorMsg)`）。
    pub fn new(custom_error_msg: impl Into<String>) -> Self {
        Self {
            custom_error_msg: Some(custom_error_msg.into()),
            ..Self::default()
        }
    }

    /// 由 BaseWxPayResult 形态的字段构造（对应 Java 静态方法
    /// `WxPayException.from(BaseWxPayResult)`；v2 结果 bean 的公共五元组
    /// 由生成器平铺进各 bean，此处以值形态承载同一拼装语义，
    /// `error_code`/`error_message` 非 None 时覆盖 errCode/errCodeDes）。
    pub fn from_base_result_fields(
        return_code: Option<&str>,
        return_msg: Option<&str>,
        result_code: Option<&str>,
        err_code: Option<&str>,
        err_code_des: Option<&str>,
        xml_string: Option<&str>,
    ) -> Self {
        Self {
            custom_error_msg: None,
            return_code: return_code.map(str::to_string),
            return_msg: return_msg.map(str::to_string),
            result_code: result_code.map(str::to_string),
            err_code: err_code.map(str::to_string),
            err_code_des: err_code_des.map(str::to_string),
            xml_string: xml_string.map(str::to_string),
        }
    }

    /// 新建 Builder（对应 Java 静态方法 `newBuilder()`）。
    pub fn new_builder() -> WxPayExceptionBuilder {
        WxPayExceptionBuilder::default()
    }

    /// 自定义错误信息（对应 Java `getCustomErrorMsg()`）。
    pub fn custom_error_msg(&self) -> Option<&str> {
        self.custom_error_msg.as_deref()
    }

    /// 返回代码（对应 Java `getReturnCode()`）。
    pub fn return_code(&self) -> Option<&str> {
        self.return_code.as_deref()
    }

    /// 设置返回代码（对应 Java `setReturnCode`）。
    pub fn set_return_code(&mut self, return_code: Option<String>) {
        self.return_code = return_code;
    }

    /// 返回信息（对应 Java `getReturnMsg()`）。
    pub fn return_msg(&self) -> Option<&str> {
        self.return_msg.as_deref()
    }

    /// 结果代码（对应 Java `getResultCode()`）。
    pub fn result_code(&self) -> Option<&str> {
        self.result_code.as_deref()
    }

    /// 错误代码（对应 Java `getErrCode()`）。
    pub fn err_code(&self) -> Option<&str> {
        self.err_code.as_deref()
    }

    /// 设置错误代码（对应 Java `setErrCode`，`from(BaseWxPayResult)` 的
    /// `errorCode != null` 覆盖分支）。
    pub fn set_err_code(&mut self, err_code: Option<String>) {
        self.err_code = err_code;
    }

    /// 错误详情（对应 Java `getErrCodeDes()`）。
    pub fn err_code_des(&self) -> Option<&str> {
        self.err_code_des.as_deref()
    }

    /// 设置错误详情（对应 Java `setErrCodeDes`）。
    pub fn set_err_code_des(&mut self, err_code_des: Option<String>) {
        self.err_code_des = err_code_des;
    }

    /// 原始 XML 报文（对应 Java `getXmlString()`）。
    pub fn xml_string(&self) -> Option<&str> {
        self.xml_string.as_deref()
    }

    /// 拼装错误信息（对应 Java `Builder.buildErrorMsg()` + 构造器
    /// `super(builder.buildErrorMsg())`：以「，」拼接非空片段，
    /// 自定义文案存在时优先以其为消息）。
    pub fn build_error_msg(&self) -> String {
        if let Some(custom) = &self.custom_error_msg {
            return custom.clone();
        }
        let mut parts: Vec<String> = Vec::new();
        if let Some(v) = &self.return_code {
            parts.push(format!("返回代码：[{v}]"));
        }
        if let Some(v) = &self.return_msg {
            parts.push(format!("返回信息：[{v}]"));
        }
        if let Some(v) = &self.result_code {
            parts.push(format!("结果代码：[{v}]"));
        }
        if let Some(v) = &self.err_code {
            parts.push(format!("错误代码：[{v}]"));
        }
        if let Some(v) = &self.err_code_des {
            parts.push(format!("错误详情：[{v}]"));
        }
        if let Some(v) = &self.xml_string {
            parts.push(format!("微信返回的原始报文：\n{v}"));
        }
        parts.join("，")
    }
}

impl std::fmt::Display for WxPayException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.build_error_msg())
    }
}

impl std::error::Error for WxPayException {}

impl From<WxPayException> for WxErrorException {
    fn from(e: WxPayException) -> Self {
        WxErrorException::Runtime(WxRuntimeError::new(e.build_error_msg()))
    }
}

/// [`WxPayException`] 构建器（对应 Java 内部类
/// `WxPayException.Builder`）。
#[derive(Debug, Clone, Default)]
pub struct WxPayExceptionBuilder {
    return_code: Option<String>,
    return_msg: Option<String>,
    result_code: Option<String>,
    err_code: Option<String>,
    err_code_des: Option<String>,
    xml_string: Option<String>,
}

impl WxPayExceptionBuilder {
    /// 设置返回代码（对应 Java `Builder.returnCode`）。
    pub fn return_code(mut self, v: impl Into<String>) -> Self {
        self.return_code = Some(v.into());
        self
    }

    /// 设置返回信息（对应 Java `Builder.returnMsg`）。
    pub fn return_msg(mut self, v: impl Into<String>) -> Self {
        self.return_msg = Some(v.into());
        self
    }

    /// 设置结果代码（对应 Java `Builder.resultCode`）。
    pub fn result_code(mut self, v: impl Into<String>) -> Self {
        self.result_code = Some(v.into());
        self
    }

    /// 设置错误代码（对应 Java `Builder.errCode`）。
    pub fn err_code(mut self, v: impl Into<String>) -> Self {
        self.err_code = Some(v.into());
        self
    }

    /// 设置错误详情（对应 Java `Builder.errCodeDes`）。
    pub fn err_code_des(mut self, v: impl Into<String>) -> Self {
        self.err_code_des = Some(v.into());
        self
    }

    /// 设置原始报文（对应 Java `Builder.xmlString`）。
    pub fn xml_string(mut self, v: impl Into<String>) -> Self {
        self.xml_string = Some(v.into());
        self
    }

    /// 构建异常（对应 Java `Builder.build()`）。
    pub fn build(self) -> WxPayException {
        WxPayException {
            custom_error_msg: None,
            return_code: self.return_code,
            return_msg: self.return_msg,
            result_code: self.result_code,
            err_code: self.err_code,
            err_code_des: self.err_code_des,
            xml_string: self.xml_string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 错误文案拼装与 Java `buildErrorMsg` 逐片段一致（「，」连接、
    /// skipNulls、原始报文换行前缀）。
    #[test]
    fn build_error_msg_joins_non_null_parts() {
        let e = WxPayException::new_builder()
            .return_code("FAIL")
            .return_msg("签名错误")
            .err_code("SIGN_ERROR")
            .xml_string("<xml><return_code>FAIL</return_code></xml>")
            .build();
        assert_eq!(
            e.build_error_msg(),
            "返回代码：[FAIL]，返回信息：[签名错误]，错误代码：[SIGN_ERROR]，\
微信返回的原始报文：\n<xml><return_code>FAIL</return_code></xml>"
        );
    }

    /// 自定义文案优先（对应 Java `WxPayException(String customErrorMsg)`
    /// 的 super 消息）。
    #[test]
    fn custom_error_msg_takes_precedence() {
        let e = WxPayException::new("无响应结果");
        assert_eq!(e.build_error_msg(), "无响应结果");
    }

    /// from(BaseWxPayResult) 形态：errorCode/errorMessage 覆盖分支。
    #[test]
    fn from_base_result_fields_overrides_err_code() {
        let mut e = WxPayException::from_base_result_fields(
            Some("FAIL"),
            Some("ok"),
            Some("FAIL"),
            Some("ORDERNOTEXIST"),
            None,
            Some("<xml/>"),
        );
        assert_eq!(e.err_code(), Some("ORDERNOTEXIST"));
        e.set_err_code(Some("OVERRIDE".into()));
        e.set_err_code_des(Some("详情".into()));
        assert_eq!(e.err_code(), Some("OVERRIDE"));
        assert_eq!(e.err_code_des(), Some("详情"));

        // 转 WxErrorException 走 Runtime 通道
        let wx_err: WxErrorException = e.into();
        assert!(wx_err.to_string().contains("返回代码：[FAIL]"));
    }
}
