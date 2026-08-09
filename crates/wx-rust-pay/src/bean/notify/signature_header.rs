//! 微信通知接口头部信息。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.notify.SignatureHeader`
//! （v3 通知的 `Wechatpay-Timestamp`/`Wechatpay-Nonce`/`Wechatpay-Signature`/
//! `Wechatpay-Serial` 请求头）。Wave 0 已定型（生成器 HAND_WRITTEN 保护，不覆盖）。

/// 微信通知接口头部信息，需要做签名验证。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SignatureHeader {
    /// 时间戳（对应 Java `timeStamp`）
    pub time_stamp: Option<String>,
    /// 随机串（对应 Java `nonce`）
    pub nonce: Option<String>,
    /// 已签名字符串（对应 Java `signature`）
    pub signature: Option<String>,
    /// 证书序列号（对应 Java `serial`）
    pub serial: Option<String>,
}

impl SignatureHeader {
    /// 构建头部信息。
    pub fn new(
        time_stamp: Option<String>,
        nonce: Option<String>,
        signature: Option<String>,
        serial: Option<String>,
    ) -> Self {
        Self {
            time_stamp,
            nonce,
            signature,
            serial,
        }
    }
}
