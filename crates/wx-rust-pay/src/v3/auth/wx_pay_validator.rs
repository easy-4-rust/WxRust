//! v3 响应验签默认实现。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.WxPayValidator`：
//! `Content-Type` 非 `application/json` 直接放行（`return true`）；否则取
//! `Wechatpay-Serial`/`Wechatpay-Signature`/`Wechatpay-TimeStamp`/
//! `Wechatpay-Nonce` 四头，任一缺失返回 `false`，验签串
//! `timestamp\nnonce\nbody\n` 交 [`super::Verifier`] 验签。
//!
//! 验签串构造复用 [`crate::util::crypto::build_response_message`]
//! （与既有 `verify_response_signature` 同源）。

use crate::util::crypto::build_response_message;
use crate::v3::{ValidationResponse, Validator};

use super::Verifier;

/// v3 响应验签器（对应 Java `WxPayValidator implements Validator`）。
pub struct WxPayValidator {
    /// 验签器（对应 Java 字段 `verifier`）。
    verifier: std::sync::Arc<dyn Verifier>,
}

impl std::fmt::Debug for WxPayValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WxPayValidator").finish_non_exhaustive()
    }
}

impl WxPayValidator {
    /// 构造（对应 Java `WxPayValidator(Verifier verifier)`）。
    pub fn new(verifier: std::sync::Arc<dyn Verifier>) -> Self {
        Self { verifier }
    }
}

impl Validator for WxPayValidator {
    /// 校验响应（对应 Java `validate(CloseableHttpResponse)`）：
    /// 1. `Content-Type` mimeType 非 `application/json` → 放行（Java
    ///    `ContentType.parse(...).getMimeType()` 比较）；
    /// 2. 四个 `Wechatpay-*` 头任一缺失 → `false`（Java `todo: check
    ///    timestamp` 的空实现同样保留——不校验时间戳新鲜度）；
    /// 3. `timestamp\nnonce\nbody\n` 交 `verifier.verify`。
    fn validate(&self, response: &ValidationResponse) -> bool {
        if let Some(content_type) = &response.content_type {
            let mime = content_type
                .split(';')
                .next()
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase();
            if mime != "application/json" {
                return true;
            }
        } else {
            // 无 Content-Type 头：对应 Java getFirstHeader("Content-Type") NPE
            // 场景按缺失处理，不放行
            return false;
        }
        let (Some(serial), Some(sign), Some(timestamp), Some(nonce)) = (
            response.wechatpay_serial.as_deref(),
            response.wechatpay_signature.as_deref(),
            response.wechatpay_timestamp.as_deref(),
            response.wechatpay_nonce.as_deref(),
        ) else {
            return false;
        };
        let message = build_response_message(timestamp, nonce, &response.body);
        self.verifier.verify(serial, message.as_bytes(), sign)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rand_core::OsRng;
    use rsa::{RsaPrivateKey, RsaPublicKey};

    use crate::util::crypto::{sign_sha256_rsa, verify_sha256_rsa};
    use crate::v3::auth::PublicCertificateVerifier;

    use super::*;

    /// 平台签名验证器包装（公钥验签直通）。
    struct PlainPublicKeyVerifier {
        public_key: RsaPublicKey,
    }
    impl Verifier for PlainPublicKeyVerifier {
        fn verify(&self, serial_number: &str, message: &[u8], signature: &str) -> bool {
            serial_number == "PLATFORM"
                && verify_sha256_rsa(&self.public_key, message, signature).unwrap_or(false)
        }
        fn get_valid_certificate(
            &self,
        ) -> Result<crate::v3::WxPayValidCertificate, crate::util::crypto::WxPayCertVerifierError>
        {
            Err(crate::util::crypto::WxPayCertVerifierError::NoCertificates)
        }
    }

    #[test]
    fn non_json_content_type_passes_through() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let verifier = PlainPublicKeyVerifier {
            public_key: RsaPublicKey::from(&private_key),
        };
        let validator = WxPayValidator::new(Arc::new(verifier));

        let resp = ValidationResponse {
            content_type: Some("text/plain".into()),
            wechatpay_serial: None,
            wechatpay_signature: None,
            wechatpay_timestamp: None,
            wechatpay_nonce: None,
            body: String::new(),
        };
        assert!(validator.validate(&resp));
    }

    #[test]
    fn json_response_requires_headers_and_valid_signature() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = RsaPublicKey::from(&private_key);

        let validator = WxPayValidator::new(Arc::new(PlainPublicKeyVerifier {
            public_key: public_key.clone(),
        }));

        let body = r#"{"code":"SUCCESS"}"#;
        let message = build_response_message("1700000000", "NONCE", body);
        let sign = sign_sha256_rsa(&private_key, message.as_bytes()).unwrap();

        // 缺头 → false
        let missing = ValidationResponse {
            content_type: Some("application/json".into()),
            wechatpay_serial: None,
            wechatpay_signature: None,
            wechatpay_timestamp: None,
            wechatpay_nonce: None,
            body: body.into(),
        };
        assert!(!validator.validate(&missing));

        // 完整头 + 正确签名 → true；body 篡改 → false
        let ok = ValidationResponse::new(
            Some("application/json; charset=utf-8"),
            "PLATFORM",
            &sign,
            "1700000000",
            "NONCE",
            body,
        );
        assert!(validator.validate(&ok));

        let tampered = ValidationResponse::new(
            Some("application/json"),
            "PLATFORM",
            &sign,
            "1700000000",
            "NONCE",
            r#"{"code":"FAIL"}"#,
        );
        assert!(!validator.validate(&tampered));
    }

    /// 与 `PublicCertificateVerifier` 组合的端到端验签（离线）。
    #[test]
    fn validates_against_public_certificate_verifier() {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = RsaPublicKey::from(&private_key);

        let validator = WxPayValidator::new(Arc::new(PublicCertificateVerifier::new(
            public_key,
            "PUB_KEY_ID_1",
        )));

        let body = r#"{"code":"SUCCESS"}"#;
        let message = build_response_message("1700000000", "NONCE", body);
        let sign = sign_sha256_rsa(&private_key, message.as_bytes()).unwrap();

        let ok = ValidationResponse::new(
            Some("application/json"),
            "PUB_KEY_ID_1",
            &sign,
            "1700000000",
            "NONCE",
            body,
        );
        assert!(validator.validate(&ok));
    }
}
