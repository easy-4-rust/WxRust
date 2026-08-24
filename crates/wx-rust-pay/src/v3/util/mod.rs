//! v3 工具包（对应 Java `com.github.binarywang.wxpay.v3.util` 包）。
//!
//! Java 命名镜像，实现复用 [`crate::util::crypto`] 既有纯函数
//! （`wx_pay_v3_crypto_utils.rs`/`wx_pay_cert_utils.rs`），零重复：
//! - [`aes_utils`]：`v3/util/AesUtils`（AES-256-GCM）；
//! - [`pem_utils`]：`v3/util/PemUtils`（PEM 私钥/公钥/证书加载）；
//! - [`rsa_crypto_util`]：`v3/util/RsaCryptoUtil`（RSA-OAEP 敏感信息加解密）。
//!
//! Java `v3/util/SignUtils`（SHA256withRSA 静态签名）已由
//! [`crate::util::crypto::sign_sha256_rsa`] 承载（文件位于 `util/sign_utils.rs`
//! 与 `util/crypto/`，见审计映射）。

pub mod aes_utils;
pub mod pem_utils;
pub mod rsa_crypto_util;
