//! 验证器构建器。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.VerifierBuilder`（包私有
//! 静态工具，`WxPayConfig.initApiV3HttpClient` 调用）：
//! 1. 有商户私钥 + 证书序列号 + APIv3 密钥 → 构建
//!    `AutoUpdateCertificatesVerifier`（平台证书模式）；
//! 2. 有公钥 + 公钥 ID → 构建 `PublicCertificateVerifier`，并将步骤 1 的
//!    证书验证器经 `setOtherVerifier` 注入为兜底（对应 Java
//!    `publicCertificatesVerifier.setOtherVerifier(certificatesVerifier)`）；
//! 3. 均失败但有异常 → 返回 `Err`（对应 Java `throw new WxPayException(
//!    ex.getMessage(), ex)`）；均未构建 → `Ok(None)`（对应 Java 返回 null）。
//!
//! ADAPTED：
//! - Java `wxPayHttpProxy` 参数（Apache HttpClient 代理）不迁移：Rust 代理
//!   由 reqwest 客户端承载；
//! - Java 的 `AutoUpdateCertificatesVerifier` 构造器内同步下载证书
//!   （失败仅告警）；Rust 下载延迟到首次 `check_and_auto_update`
//!   （见 `v3/auth/auto_update_certificates_verifier.rs` 的 ADAPTED 说明），
//!   故本构建器不产生网络请求、离线可测；
//! - `payBaseUrl` 仅取其 URI rawPath 作为 `signUriStripPrefix`
//!   （对应 Java `new URI(payBaseUrl).getRawPath()`，空白或 `/` 时不设置）。

use std::sync::Arc;

use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::util::crypto::WxPayCertVerifierError;
use crate::v3::auth::{
    AutoUpdateCertificatesVerifier, PrivateKeySigner, PublicCertificateVerifier, Verifier,
    WxPayCredentials,
};

/// 验证器构建（对应 Java `VerifierBuilder.build(...)`）。
///
/// # 参数
/// - `cert_serial_no`：商户证书序列号（平台证书模式依赖参数）
/// - `mch_id`：商户号
/// - `api_v3_key`：APIv3 密钥
/// - `merchant_private_key`：商户 API 私钥（PKCS#8 已解析形态）
/// - `cert_auto_update_minutes`：证书自动更新间隔（分钟）
/// - `pay_base_url`：支付网关地址（仅取路径前缀参与签名剥离）
/// - `public_key_id`：公钥模式 ID（`PUB_KEY_ID_` 前缀）
/// - `public_key`：微信支付公钥
///
/// # 返回
/// 构建成功的验证器（证书模式/公钥模式/公钥+证书兜底组合），或 `Ok(None)`
/// （两组参数均缺失，对应 Java 返回 null）。
#[allow(clippy::too_many_arguments)]
pub fn build_verifier(
    cert_serial_no: Option<&str>,
    mch_id: Option<&str>,
    api_v3_key: Option<&str>,
    merchant_private_key: Option<RsaPrivateKey>,
    cert_auto_update_minutes: u64,
    pay_base_url: Option<&str>,
    public_key_id: Option<&str>,
    public_key: Option<RsaPublicKey>,
) -> Result<Option<Arc<dyn Verifier>>, WxPayCertVerifierError> {
    let mut certificates_verifier: Option<Arc<dyn Verifier>> = None;

    // ---- 1. 平台证书验证器（沿用 Java 逻辑：优先构建，公钥验证器需要它兜底）----
    // 注：Java 此步的 try/catch 捕获构造器内同步下载证书的失败；Rust 构造
    // 不发起网络请求（见模块文档 ADAPTED），不存在失败路径
    if let (Some(private_key), true, Some(serial), Some(mch), Some(key)) = (
        merchant_private_key,
        !cert_serial_no.unwrap_or_default().trim().is_empty(),
        cert_serial_no,
        mch_id,
        api_v3_key,
    ) {
        let mut credentials =
            WxPayCredentials::new(mch, Arc::new(PrivateKeySigner::new(serial, private_key)));
        if let Some(strip_prefix) = sign_uri_strip_prefix(pay_base_url) {
            credentials.set_sign_uri_strip_prefix(Some(strip_prefix.as_str()));
        }
        certificates_verifier = Some(Arc::new(AutoUpdateCertificatesVerifier::new(
            Arc::new(credentials),
            key,
            cert_auto_update_minutes,
        )));
    }

    // ---- 2. 公钥验证器（注入证书验证器兜底）----
    if let (Some(key_id), Some(key)) = (public_key_id, public_key) {
        if !key_id.trim().is_empty() {
            let public_verifier = Arc::new(PublicCertificateVerifier::new(key, key_id));
            public_verifier.set_other_verifier(certificates_verifier.clone());
            return Ok(Some(public_verifier));
        }
    }

    // ---- 3. 只有证书验证器 ----
    if let Some(verifier) = certificates_verifier {
        return Ok(Some(verifier));
    }

    // 无任何验证器：沿用 Java 逻辑返回 null（Rust `Ok(None)`；
    // Java 的 `throw ex` 分支来自构造器内网络下载失败，Rust 无此路径）
    Ok(None)
}

/// 完全公钥场景（对应 Java `VerifierBuilder.buildPublicCertVerifier(
/// publicKeyId, publicKey)`）。
pub fn build_public_cert_verifier(
    public_key_id: &str,
    public_key: RsaPublicKey,
) -> Arc<dyn Verifier> {
    Arc::new(PublicCertificateVerifier::new(public_key, public_key_id))
}

/// 从 payBaseUrl 提取签名剥离前缀（对应 Java `build` 内
/// `new URI(payBaseUrl).getRawPath()`：空白或 `/` 时返回 None）。
fn sign_uri_strip_prefix(pay_base_url: Option<&str>) -> Option<String> {
    let base = pay_base_url?.trim();
    if base.is_empty() {
        return None;
    }
    // URI rawPath：scheme://authority 之后的首个路径段（无 `?` 前）
    let after_scheme = base.split_once("://").map(|(_, rest)| rest).unwrap_or(base);
    let path = after_scheme
        .split_once('/')
        .map(|(_, path)| format!("/{path}"))
        .unwrap_or_default();
    let path = path.split('?').next().unwrap_or_default().to_string();
    if path.is_empty() || path == "/" {
        None
    } else {
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rand_core::OsRng;
    use rsa::{RsaPrivateKey, RsaPublicKey};

    use crate::v3::auth::Verifier;

    use super::*;

    fn random_keypair() -> (RsaPrivateKey, RsaPublicKey) {
        let mut rng = OsRng;
        let private = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥");
        let public = RsaPublicKey::from(&private);
        (private, public)
    }

    #[test]
    fn public_key_only_mode() {
        let (_, public) = random_keypair();
        let verifier = build_verifier(
            None,
            None,
            None,
            None,
            60,
            None,
            Some("PUB_KEY_ID_1A2B3C"),
            Some(public),
        )
        .unwrap()
        .expect("应构建公钥验证器");
        let valid = verifier.get_valid_certificate().unwrap();
        assert_eq!(valid.serial_no(), "1A2B3C");
    }

    #[test]
    fn certificate_only_mode() {
        let (private, _) = random_keypair();
        let verifier = build_verifier(
            Some("SERIAL01"),
            Some("1234567891"),
            Some("a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb"),
            Some(private),
            60,
            None,
            None,
            None,
        )
        .unwrap()
        .expect("应构建证书验证器");
        // 未下载证书前存储为空（对应 Java 构造失败仅告警）
        assert!(verifier.get_valid_certificate().is_err());
    }

    #[test]
    fn both_missing_returns_none() {
        assert!(
            build_verifier(None, None, None, None, 60, None, None, None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn pay_base_url_path_prefix_extraction() {
        // 对应 Java：rawPath 为 "/" 或空白 → 不设置
        assert_eq!(
            sign_uri_strip_prefix(Some("https://api.mch.weixin.qq.com/")),
            None
        );
        assert_eq!(sign_uri_strip_prefix(Some("  ")), None);
        assert_eq!(sign_uri_strip_prefix(None), None);
        // 带路径前缀的反向代理场景
        assert_eq!(
            sign_uri_strip_prefix(Some("https://proxy.example.com/api-weixin")),
            Some("/api-weixin".to_string())
        );
        assert_eq!(
            sign_uri_strip_prefix(Some("https://proxy.example.com/api-weixin/")),
            Some("/api-weixin/".to_string())
        );
    }

    #[test]
    fn build_public_cert_verifier_standalone() {
        let (_, public) = random_keypair();
        let verifier = build_public_cert_verifier("PUB_KEY_ID_X", public);
        assert!(verifier.get_valid_certificate().is_ok());
        // Arc<dyn Verifier> 可组合进其他验证器
        let _: Arc<dyn Verifier> = verifier;
    }
}
