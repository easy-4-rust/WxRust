//! 平台证书验签器（v3/auth Java 命名镜像）。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.auth.CertificatesVerifier`：
//! 以 `HashMap<BigInteger, X509Certificate>` 存储平台证书，按序列号路由
//! 公钥做 SHA256withRSA 验签；`getValidCertificate` 返回首张有效期内证书。
//!
//! 引擎复用 [`crate::util::crypto::WxPayCertificatesVerifier`]（P3 全量实现 +
//! golden 测试，见 `tests/wx_pay_cert_verifier_test.rs`）；本文件提供 Java
//! 命名镜像并实现 [`super::Verifier`] 特性，供 [`crate::v3`] 认证族组合。

use crate::util::crypto::{WxPayCertVerifierError, WxPayCertificate, WxPayCertificatesVerifier};

use super::{Verifier, WxPayValidCertificate};

/// 平台证书验签器（对应 Java `CertificatesVerifier implements Verifier`）。
#[derive(Debug, Clone, Default)]
pub struct CertificatesVerifier {
    /// 证书存储引擎（对应 Java `HashMap<BigInteger, X509Certificate>`）。
    inner: WxPayCertificatesVerifier,
}

impl CertificatesVerifier {
    /// 由证书列表构建（对应 Java 构造器
    /// `CertificatesVerifier(List<X509Certificate>)`：逐个
    /// `certificates.put(cert.getSerialNumber(), cert)`）。
    pub fn new(certificates: Vec<WxPayCertificate>) -> Self {
        Self {
            inner: WxPayCertificatesVerifier::with_certificates(certificates),
        }
    }

    /// 存储中的全部序列号（对应 Java `certificateMap.keySet()`）。
    pub fn get_serial_numbers(&self) -> Vec<String> {
        self.inner.get_serial_numbers()
    }

    /// 存储中的证书数量。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 存储是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Verifier for CertificatesVerifier {
    /// 按序列号路由公钥验签（对应 Java `verify(serialNumber, message,
    /// signature)`：`containsKey(val) && verify(cert, message, signature)`）。
    fn verify(&self, serial_number: &str, message: &[u8], signature: &str) -> bool {
        self.inner.verify(serial_number, message, signature)
    }

    /// 返回首张有效期内证书（对应 Java `getValidCertificate`；全失效时
    /// `Err`，对应抛 `NoSuchElementException("没有有效的微信支付平台证书")`）。
    fn get_valid_certificate(&self) -> Result<WxPayValidCertificate, WxPayCertVerifierError> {
        self.inner
            .get_valid_certificate()
            .cloned()
            .map(|c| WxPayValidCertificate::Certificate(Box::new(c)))
    }
}
