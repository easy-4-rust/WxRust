//! 商户请求凭据（Authorization token 构造）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.WxPayCredentials`：
//! 持有商户号 + [`super::Signer`]（+ 可选 `signUriStripPrefix` 反向代理
//! 路径前缀），`getToken(request)` 生成
//! `mchid="..",nonce_str="..",timestamp="..",serial_no="..",signature=".."`，
//! 签名串为 `METHOD\ncanonical_url\ntimestamp\nnonce\nbody\n`。
//!
//! 签名/拼串原语复用 [`crate::util::crypto`] 的
//! `build_request_message`/`build_authorization_token`/`gen_nonce_str`/
//! `gen_timestamp`（与既有 `create_authorization_header` 流程同源）。

use std::sync::Arc;

use crate::util::crypto::{
    AUTHORIZATION_SCHEMA, WxV3CryptoError, build_authorization_token, build_request_message,
    gen_nonce_str, gen_timestamp,
};
use crate::v3::{Credentials, CredentialsRequest};

use super::Signer;

/// 商户请求凭据（对应 Java `WxPayCredentials implements Credentials`）。
pub struct WxPayCredentials {
    /// 商户号（对应 Java 字段 `merchantId`）。
    merchant_id: String,
    /// 签名器（对应 Java 字段 `signer`，如 `PrivateKeySigner`）。
    signer: Arc<dyn Signer>,
    /// 签名前从 URI path 中移除的前缀（对应 Java 字段
    /// `signUriStripPrefix`；带路径前缀的反向代理场景，如配置
    /// `/api-weixin` 时 `/api-weixin/v3/pay/...` 参与签名为 `/v3/pay/...`）。
    sign_uri_strip_prefix: Option<String>,
}

impl std::fmt::Debug for WxPayCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WxPayCredentials")
            .field("merchant_id", &self.merchant_id)
            .field("sign_uri_strip_prefix", &self.sign_uri_strip_prefix)
            .finish_non_exhaustive()
    }
}

impl WxPayCredentials {
    /// 构造（对应 Java `WxPayCredentials(String merchantId, Signer signer)`）。
    pub fn new(merchant_id: impl Into<String>, signer: Arc<dyn Signer>) -> Self {
        Self {
            merchant_id: merchant_id.into(),
            signer,
            sign_uri_strip_prefix: None,
        }
    }

    /// 构造并设置签名路径前缀（对应 Java
    /// `WxPayCredentials(merchantId, signer, signUriStripPrefix)`，规范化
    /// 规则见 [`Self::set_sign_uri_strip_prefix`]）。
    pub fn with_sign_uri_strip_prefix(
        merchant_id: impl Into<String>,
        signer: Arc<dyn Signer>,
        sign_uri_strip_prefix: Option<&str>,
    ) -> Self {
        let mut credentials = Self::new(merchant_id, signer);
        credentials.set_sign_uri_strip_prefix(sign_uri_strip_prefix);
        credentials
    }

    /// 商户号（对应 Java `getMerchantId()`）。
    pub fn merchant_id(&self) -> &str {
        &self.merchant_id
    }

    /// 签名路径前缀（规范化后，对应 Java `getSignUriStripPrefix` 语义）。
    pub fn sign_uri_strip_prefix(&self) -> Option<&str> {
        self.sign_uri_strip_prefix.as_deref()
    }

    /// 设置签名路径前缀（对应 Java `setSignUriStripPrefix`）：
    /// `None`/空白 → 清除；去除首尾空白；补齐开头 `/`；长度大于 1 时去掉
    /// 结尾 `/`。
    pub fn set_sign_uri_strip_prefix(&mut self, prefix: Option<&str>) {
        let Some(prefix) = prefix.map(str::trim).filter(|p| !p.is_empty()) else {
            self.sign_uri_strip_prefix = None;
            return;
        };
        let mut normalized = prefix.to_string();
        if !normalized.starts_with('/') {
            normalized.insert(0, '/');
        }
        if normalized.len() > 1 && normalized.ends_with('/') {
            normalized.pop();
        }
        self.sign_uri_strip_prefix = Some(normalized);
    }

    /// 生成秒级时间戳（对应 Java `generateTimestamp()`：
    /// `System.currentTimeMillis() / 1000`）。
    pub fn generate_timestamp(&self) -> i64 {
        gen_timestamp()
    }

    /// 生成 32 位随机串（对应 Java `generateNonceStr()`：数字+大小写字母）。
    pub fn generate_nonce_str(&self) -> String {
        gen_nonce_str()
    }

    /// 构造签名串（对应 Java `buildMessage(nonce, timestamp, request)`：
    /// `METHOD\ncanonical_url\ntimestamp\nnonce\nbody\n`，其中
    /// `canonical_url = stripPathPrefix(rawPath) [+ "?" + rawQuery]`）。
    pub fn build_message(
        &self,
        nonce: &str,
        timestamp: i64,
        request: &CredentialsRequest,
    ) -> String {
        let mut canonical_url = self.strip_path_prefix(&request.path).to_string();
        if let Some(query) = &request.query {
            canonical_url.push('?');
            canonical_url.push_str(query);
        }
        build_request_message(
            &request.method,
            &canonical_url,
            timestamp,
            nonce,
            &request.body,
        )
    }

    /// 从 rawPath 中剥离签名前缀（对应 Java 私有 `stripPathPrefix`）：
    /// 无前缀配置/空路径/路径不以前缀开头 → 原样返回；剥离后为空 → `/`；
    /// 不以 `/` 开头 → 补 `/`。
    pub fn strip_path_prefix<'a>(&self, raw_path: &'a str) -> &'a str {
        let Some(prefix) = &self.sign_uri_strip_prefix else {
            return raw_path;
        };
        if raw_path.is_empty() || !raw_path.starts_with(prefix.as_str()) {
            return raw_path;
        }
        let stripped = &raw_path[prefix.len()..];
        if stripped.is_empty() {
            return "/";
        }
        stripped
    }

    /// 生成 token（对应 Java `getToken` 主体：签名串经 `signer.sign` 后拼
    /// `mchid/nonce_str/timestamp/serial_no/signature` 五元组）。
    pub fn build_token(&self, request: &CredentialsRequest) -> Result<String, WxV3CryptoError> {
        let nonce_str = self.generate_nonce_str();
        let timestamp = self.generate_timestamp();
        let message = self.build_message(&nonce_str, timestamp, request);
        let signature = self.signer.sign(message.as_bytes())?;
        Ok(build_authorization_token(
            &self.merchant_id,
            &nonce_str,
            timestamp,
            &signature.certificate_serial_number,
            &signature.sign,
        ))
    }
}

impl Credentials for WxPayCredentials {
    /// Authorization schema（对应 Java `getSchema()`：
    /// `WECHATPAY2-SHA256-RSA2048`）。
    fn get_schema(&self) -> &'static str {
        AUTHORIZATION_SCHEMA
    }

    /// 生成 Authorization token（对应 Java `getToken(HttpRequestWrapper)`）。
    fn get_token(&self, request: &CredentialsRequest) -> Result<String, WxV3CryptoError> {
        self.build_token(request)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rand_core::OsRng;
    use rsa::RsaPrivateKey;

    use crate::v3::auth::{PrivateKeySigner, SignatureResult};

    use super::*;

    /// 无前缀时 token 五元组格式（schema、mchid、nonce、timestamp、
    /// serial_no、signature）与 Java `getToken` 拼串一致。
    #[test]
    fn token_format_and_signature_roundtrip() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");

        let credentials = WxPayCredentials::new(
            "1234567891",
            Arc::new(PrivateKeySigner::new("5F1C72E2A893", private_key)),
        );
        assert_eq!(credentials.get_schema(), "WECHATPAY2-SHA256-RSA2048");

        let request = CredentialsRequest::new("POST", "/v3/pay/transactions", r#"{"amount":1}"#);
        let token = credentials.get_token(&request).unwrap();
        for part in [
            "mchid=\"1234567891\"",
            "nonce_str=\"",
            "timestamp=\"",
            "serial_no=\"5F1C72E2A893\"",
            "signature=\"",
        ] {
            assert!(token.contains(part), "token 缺少片段 {part}: {token}");
        }
    }

    /// `signUriStripPrefix` 规范化与签名串剥离（对应 Java
    /// `setSignUriStripPrefix`/`stripPathPrefix`）。
    #[test]
    fn sign_uri_strip_prefix_normalization() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");

        let mut credentials = WxPayCredentials::new(
            "mch",
            Arc::new(PrivateKeySigner::new("SERIAL", private_key)),
        );

        // 空白 → 清除
        credentials.set_sign_uri_strip_prefix(Some("   "));
        assert_eq!(credentials.sign_uri_strip_prefix(), None);
        // 补齐开头 / 、去结尾 /
        credentials.set_sign_uri_strip_prefix(Some("api-weixin/"));
        assert_eq!(credentials.sign_uri_strip_prefix(), Some("/api-weixin"));

        // 不匹配前缀原样返回；匹配则剥离；剥空 → /
        assert_eq!(
            credentials.strip_path_prefix("/v3/certificates"),
            "/v3/certificates"
        );
        assert_eq!(
            credentials.strip_path_prefix("/api-weixin/v3/certificates"),
            "/v3/certificates"
        );
        assert_eq!(credentials.strip_path_prefix("/api-weixin"), "/");

        // 签名串使用剥离后的 canonical_url（带 query 拼接）
        let request = CredentialsRequest::new("GET", "/api-weixin/v3/certificates", "")
            .with_query("offset=1");
        let message = credentials.build_message("NONCE", 1700000000, &request);
        assert_eq!(
            message,
            "GET\n/v3/certificates?offset=1\n1700000000\nNONCE\n\n"
        );
    }

    /// 确定性签名器（捕获签名串以便断言）。
    #[test]
    fn build_message_matches_java_layout() {
        struct FixedSigner;
        impl super::Signer for FixedSigner {
            fn sign(&self, _message: &[u8]) -> Result<SignatureResult, WxV3CryptoError> {
                Ok(SignatureResult::new("SIG", "SERIAL"))
            }
        }
        let credentials = WxPayCredentials::new("mch", Arc::new(FixedSigner));
        let request = CredentialsRequest::new("GET", "/v3/certificates", "");
        let token = credentials.get_token(&request).unwrap();
        assert!(token.starts_with("mchid=\"mch\",nonce_str=\""));
        assert!(token.contains("\",timestamp=\""));
        assert!(token.ends_with(",serial_no=\"SERIAL\",signature=\"SIG\""));
    }
}
