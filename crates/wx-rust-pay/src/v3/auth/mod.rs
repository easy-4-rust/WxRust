//! v3 认证族（对应 Java `com.github.binarywang.wxpay.v3.auth` 包）。
//!
//! - [`Signer`]/[`SignatureResult`]：`v3/auth/Signer`（签名器接口与结果）；
//! - [`Verifier`]：`v3/auth/Verifier`（验签器接口，`setOtherVerifier`
//!   默认空实现）；
//! - [`PrivateKeySigner`]：`v3/auth/PrivateKeySigner`（SHA256withRSA 私钥签名）；
//! - [`CertificatesVerifier`]：`v3/auth/CertificatesVerifier`（平台证书
//!   存储验签，引擎复用
//!   [`crate::util::crypto::WxPayCertificatesVerifier`]）；
//! - [`AutoUpdateCertificatesVerifier`]：`v3/auth/AutoUpdateCertificatesVerifier`
//!   （证书自动更新验签，引擎复用
//!   [`crate::util::crypto::WxPayAutoUpdateCertificatesVerifier`]）；
//! - [`PublicCertificateVerifier`]：`v3/auth/PublicCertificateVerifier`
//!   （微信支付公钥模式验签，证书验证器兜底）；
//! - [`X509PublicCertificate`]：`v3/auth/X509PublicCertificate`
//!   （公钥模式的证书承载对象）；
//! - [`WxPayCredentials`]：`v3/auth/WxPayCredentials`（实现
//!   [`crate::v3::Credentials`]）；
//! - [`WxPayValidator`]：`v3/auth/WxPayValidator`（实现
//!   [`crate::v3::Validator`]）。

pub mod auto_update_certificates_verifier;
pub mod certificates_verifier;
pub mod private_key_signer;
pub mod public_certificate_verifier;
pub mod signer;
pub mod verifier;
pub mod wx_pay_credentials;
pub mod wx_pay_validator;
pub mod x509_public_certificate;

pub use auto_update_certificates_verifier::{AutoUpdateCertificatesVerifier, TimeInterval};
pub use certificates_verifier::CertificatesVerifier;
pub use private_key_signer::PrivateKeySigner;
pub use public_certificate_verifier::PublicCertificateVerifier;
pub use signer::{SignatureResult, Signer};
pub use verifier::{Verifier, WxPayValidCertificate};
pub use wx_pay_credentials::WxPayCredentials;
pub use wx_pay_validator::WxPayValidator;
pub use x509_public_certificate::X509PublicCertificate;
