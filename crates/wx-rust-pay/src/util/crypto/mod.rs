//! 微信支付加解密/签名基础工具。
//!
//! 对应 Java 侧散落的 `DigestUtils.md5Hex`（commons-codec）、
//! `me.chanjar.weixin.common.util.SignUtils` 等能力在支付场景的 Rust 基础
//! 封装。Wave 0 提供 v2 签名所需：MD5 / HmacSHA256 / SHA1。
//! Wave 1（P3）追加：p12/PEM 证书解析（Java `PemUtils`/`WxPayConfig#p12ToPem`）、
//! v3 RSA-SHA256 签名/验签（`rsa` crate）、AES-GCM 加解密
//! （Java `v3/util/AesUtils`）、RSA-OAEP 敏感信息加解密（`RsaCryptoUtil`）。

pub mod wx_pay_cert_utils;
pub mod wx_pay_cert_verifier;
pub mod wx_pay_crypto_utils;
pub mod wx_pay_v3_crypto_utils;

pub use wx_pay_cert_utils::{
    WxPayCertError, WxPayCertificate, WxPayP12Data, load_certificate_from_pem,
    load_private_key_and_cert_from_p12, load_private_key_from_pem, load_public_key_from_pem,
};
pub use wx_pay_cert_verifier::{
    CERT_DOWNLOAD_PATH, CertificateUpdateResult, DEFAULT_UPDATE_INTERVAL_MINUTES,
    WxPayAutoUpdateCertificatesVerifier, WxPayCertVerifierError, WxPayCertificatesVerifier,
    deserialize_to_certs,
};
pub use wx_pay_crypto_utils::{hmac_sha256_hex, md5_hex, sha1_hex};
pub use wx_pay_v3_crypto_utils::{
    AUTHORIZATION_SCHEMA, WxV3CryptoError, aes_gcm_decrypt, aes_gcm_decrypt_bytes, aes_gcm_encrypt,
    build_authorization_token, build_request_message, build_response_message, canonical_url,
    canonical_url_from_url, create_authorization_header, gen_gcm_nonce, gen_nonce_str,
    gen_timestamp, rsa_oaep_decrypt, rsa_oaep_encrypt, sign_sha256_rsa, verify_response_signature,
    verify_sha256_rsa,
};
