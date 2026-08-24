//! 敏感信息字段标记。
//!
//! 对应 Java `com.github.binarywang.wxpay.v3.SpecEncrypt`（`@Target(FIELD)`
//! 运行时注解）：`RsaCryptoUtil.encryptFields` 反射遍历字段，对标注
//! `@SpecEncrypt` 的 String 字段做 RSA-OAEP 加密。
//!
//! ADAPTED：Rust 无运行时反射/字段注解；等价约定为——
//! - 敏感字段加密在调用侧**显式**进行（[`crate::util::crypto::rsa_oaep_encrypt`]
//!   / [`crate::v3::util::rsa_crypto_util`] 的 Java 命名镜像），不依赖注解扫描；
//! - 需要集中声明敏感字段的 bean 可实现 [`SpecEncrypt`] 标记特性，由宿主
//!   按约定驱动加密（对应 Java 注解的声明式用法）。

/// 敏感信息字段标记（对应 Java `@SpecEncrypt` 注解）。
///
/// 实现 bean 中的敏感字段（姓名/银行卡号/身份证等）应按
/// RSA-OAEP（SHA-1，`RSA/ECB/OAEPWithSHA-1AndMGF1Padding`）加密，
/// 见 <https://wechatpay-api.gitbook.io/wechatpay-api-v3/qian-ming-zhi-nan-1/min-gan-xin-xi-jia-mi>。
pub trait SpecEncrypt {
    /// 需要加密的敏感字段名列表（对应 Java 反射扫描 `@SpecEncrypt` 字段）。
    ///
    /// 默认为空（无敏感字段），与未标注注解的 Java bean 等价。
    fn spec_encrypt_fields(&self) -> Vec<&'static str> {
        Vec::new()
    }
}
