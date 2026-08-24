//! 微信支付 v3 认证与加密基础设施。
//!
//! 对应 Java（weixin-java-pay）`com.github.binarywang.wxpay.v3` 包，
//! 按 Java 目录结构镜像：
//! - [`auth`]：`v3/auth` 包——[`auth::Signer`]/[`auth::Verifier`] 特性族
//!   （`PrivateKeySigner`、`CertificatesVerifier`、
//!   `AutoUpdateCertificatesVerifier`、`PublicCertificateVerifier`）、
//!   [`auth::WxPayCredentials`]（Authorization token 构造）、
//!   [`auth::WxPayValidator`]（响应验签）、
//!   [`auth::X509PublicCertificate`]（公钥模式的"证书"承载）；
//! - [`credentials`]/[`validator`]：`v3/Credentials.java`/`v3/Validator.java`
//!   两个顶层接口（Java 以 Apache HttpClient 请求/响应类型为参数，Rust 以
//!   轻量值对象适配，见各文件 ADAPTED 说明）；
//! - [`spec_encrypt`]：`v3/SpecEncrypt.java` 敏感字段标记的 Rust 等价约定；
//! - [`util`]：`v3/util` 包——`AesUtils`/`PemUtils`/`RsaCryptoUtil` 的
//!   Java 命名镜像（实现复用 [`crate::util::crypto`] 既有纯函数，零重复）。
//!
//! Java 侧 `v3` 包中基于 Apache HttpClient 的执行机构
//! （`SignatureExec`/`WechatPayUploadHttpPost`/`WxPayV3DownloadHttpGet`/
//! `WxPayV3HttpClientBuilder`）不迁移：Rust 以 reqwest 单一后端承载，
//! 签名注入内联于请求构造（[`crate::util::crypto::create_authorization_header`]），
//! 见 `docs/verification/V0-gap-closure.md` 的 PLATFORM_NA 处置表。

pub mod auth;
pub mod credentials;
pub mod spec_encrypt;
pub mod util;
pub mod validator;

pub use auth::{
    AutoUpdateCertificatesVerifier, CertificatesVerifier, PrivateKeySigner,
    PublicCertificateVerifier, SignatureResult, Signer, TimeInterval, Verifier, WxPayCredentials,
    WxPayValidCertificate, WxPayValidator, X509PublicCertificate,
};
pub use credentials::{Credentials, CredentialsRequest};
pub use validator::{ValidationResponse, Validator};
