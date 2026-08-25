//! RSA mitigation 验证测试。
//!
//! 背景：`rsa 0.9.10` 存在 RUSTSEC-2023-0071（Marvin Attack，medium），
//! 无修复版本可用（0.10 尚为 RC）。本测试验证当前 rsa 版本在 WxRust
//! 使用路径上的基本正确性与错误行为稳定性，作为 mitigation 证据链的一环。
//!
//! 覆盖点：
//! 1. RSA-OAEP 加解密往返——对应微信支付 v3 敏感信息加密
//! 2. PKCS#1 v1.5 SHA256withRSA 签名/验签往返——对应 v3 请求签名
//! 3. 错误输入行为：不 panic、返回可控错误
//!
//! 注意：OAEP 测试使用 SHA-256 而非 SHA-1 作为哈希算法，原因是 workspace
//! sha1 0.11 依赖 digest 0.11，与 rsa 0.9.10 的 digest 0.10 不兼容。
//! 使用 rsa crate 自带的 sha2（digest 0.10）可正确验证 RSA-OAEP 机制本身。
//! wx-rust-pay 中的实际 OAEP 路径使用 sha1 0.10.6（独立依赖），已通过
//! `wx_pay_v3_crypto_test::rsa_oaep_roundtrip` 验证。

use base64::Engine;
use rsa::Oaep;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::sha2::Digest;
use rsa::{RsaPrivateKey, RsaPublicKey};

/// 生成 2048-bit RSA 密钥对（测试专用）。
fn test_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = rand_core::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成测试密钥失败");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

// ===========================================================================
// 1. RSA-OAEP 加解密往返
//    对应微信支付 v3 敏感信息加密（RsaCryptoUtil.encryptOAEP / decryptOAEP）。
//    使用 SHA-256 作为 OAEP 哈希（兼容 rsa 0.9 digest 0.10）。
// ===========================================================================

/// RSA-OAEP 加解密往返：ASCII 明文。
#[test]
fn rsa_oaep_roundtrip_ascii() {
    let (private_key, public_key) = test_keypair();
    let message = "sensitive_data_12345";

    let ciphertext = public_key
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("OAEP 加密不应失败");
    assert!(!ciphertext.is_empty(), "密文不应为空");

    let plaintext = private_key
        .decrypt(Oaep::new::<rsa::sha2::Sha256>(), &ciphertext)
        .expect("OAEP 解密不应失败");
    assert_eq!(String::from_utf8(plaintext).unwrap(), message);
}

/// RSA-OAEP 加解密往返：中文明文（姓名/身份证等敏感字段场景）。
#[test]
fn rsa_oaep_roundtrip_chinese() {
    let (private_key, public_key) = test_keypair();
    let message = "张三#110101199001011234";

    let ciphertext = public_key
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("OAEP 加密中文不应失败");

    let plaintext = private_key
        .decrypt(Oaep::new::<rsa::sha2::Sha256>(), &ciphertext)
        .expect("OAEP 解密中文不应失败");
    assert_eq!(String::from_utf8(plaintext).unwrap(), message);
}

/// RSA-OAEP 加解密往返：明文长度接近上限。
/// RSA-2048 OAEP with SHA-256: max plaintext = 256 - 2*32 - 2 = 190 bytes。
#[test]
fn rsa_oaep_roundtrip_near_max_length() {
    let (private_key, public_key) = test_keypair();
    let message = "A".repeat(190);

    let ciphertext = public_key
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("190 字节应为 OAEP SHA-256 上限，加密不应失败");

    let plaintext = private_key
        .decrypt(Oaep::new::<rsa::sha2::Sha256>(), &ciphertext)
        .expect("解密不应失败");
    assert_eq!(String::from_utf8(plaintext).unwrap(), message);
}

/// RSA-OAEP 加密空消息：应成功（空明文合法）。
#[test]
fn rsa_oaep_encrypt_empty_message() {
    let (_private_key, public_key) = test_keypair();
    let result = public_key.encrypt(&mut rand_core::OsRng, Oaep::new::<rsa::sha2::Sha256>(), b"");
    assert!(result.is_ok(), "空明文 OAEP 加密应成功");
}

/// RSA-OAEP 密文 Base64 编解码往返（模拟实际 API 传输路径）。
#[test]
fn rsa_oaep_base64_roundtrip() {
    let (private_key, public_key) = test_keypair();
    let message = "13800138000";

    let ciphertext = public_key
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("加密");
    let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&ciphertext_b64)
        .expect("Base64 解码");
    let plaintext = private_key
        .decrypt(Oaep::new::<rsa::sha2::Sha256>(), &decoded)
        .expect("解密");
    assert_eq!(String::from_utf8(plaintext).unwrap(), message);
}

// ===========================================================================
// 2. PKCS#1 v1.5 SHA256withRSA 签名/验签往返
//    对应微信支付 v3 请求签名（PrivateKeySigner.sign / PublicCertificateVerifier.verify）
// ===========================================================================

/// SHA256withRSA 签名/验签往返：确定性签名（PKCS#1 v1.5 不含随机性）。
#[test]
fn rsa_pkcs1v15_sign_verify_roundtrip() {
    let (private_key, public_key) = test_keypair();
    let message = b"GET\n/v3/pay/transactions/id/123\n1652750623\nnonce123\n\n";

    let digest = rsa::sha2::Sha256::digest(message);
    let signature = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名不应失败");
    assert_eq!(signature.len(), 256, "RSA-2048 签名应为 256 字节");

    // 验签通过
    let verify_result = public_key.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &digest,
        &signature,
    );
    assert!(verify_result.is_ok(), "验签应通过");
}

/// SHA256withRSA 签名确定性：同一消息两次签名结果一致。
#[test]
fn rsa_pkcs1v15_sign_deterministic() {
    let (private_key, _public_key) = test_keypair();
    let message = b"deterministic test message";

    let digest = rsa::sha2::Sha256::digest(message);
    let sig1 = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名 1");
    let sig2 = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名 2");
    assert_eq!(sig1, sig2, "PKCS#1 v1.5 签名应为确定性");
}

/// SHA256withRSA 签名 Base64 编码往返（模拟 Wechatpay-Signature 头传输）。
#[test]
fn rsa_pkcs1v15_sign_base64_roundtrip() {
    let (private_key, public_key) = test_keypair();
    let message = b"POST\n/v3/refund\n1700000000\nabc123\n{\"out_trade_no\":\"x\"}\n";

    let digest = rsa::sha2::Sha256::digest(message);
    let signature = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名");
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(&signature);

    let decoded_sig = base64::engine::general_purpose::STANDARD
        .decode(&sig_b64)
        .expect("Base64 解码签名");
    let verify_result = public_key.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &digest,
        &decoded_sig,
    );
    assert!(verify_result.is_ok(), "Base64 往返后验签应通过");
}

// ===========================================================================
// 3. 错误输入行为验证：不 panic、返回可控错误
// ===========================================================================

/// 错误私钥解密 OAEP 密文：必须失败但不 panic。
#[test]
fn rsa_oaep_decrypt_wrong_key_returns_error() {
    let (_sk1, pk1) = test_keypair();
    let (sk2, _pk2) = test_keypair();

    let message = "secret";
    let ciphertext = pk1
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("加密");

    // 用另一个私钥解密
    let result = sk2.decrypt(Oaep::new::<rsa::sha2::Sha256>(), &ciphertext);
    assert!(result.is_err(), "错误私钥解密应返回 Err");
}

/// 篡改密文后 OAEP 解密：必须失败但不 panic。
#[test]
fn rsa_oaep_decrypt_tampered_ciphertext_returns_error() {
    let (private_key, public_key) = test_keypair();
    let message = "tamper_test";

    let mut ciphertext = public_key
        .encrypt(
            &mut rand_core::OsRng,
            Oaep::new::<rsa::sha2::Sha256>(),
            message.as_bytes(),
        )
        .expect("加密");
    // 篡改一个字节
    ciphertext[0] ^= 0xFF;

    let result = private_key.decrypt(Oaep::new::<rsa::sha2::Sha256>(), &ciphertext);
    assert!(result.is_err(), "篡改密文解密应返回 Err");
}

/// 短密文 OAEP 解密：长度不合法时必须失败但不 panic。
#[test]
fn rsa_oaep_decrypt_short_ciphertext_returns_error() {
    let (private_key, _public_key) = test_keypair();
    // 构造一个长度远小于 RSA 块大小的密文
    let short_ciphertext = base64::engine::general_purpose::STANDARD.encode(b"short");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&short_ciphertext)
        .unwrap();
    let result = private_key.decrypt(Oaep::new::<rsa::sha2::Sha256>(), &decoded);
    assert!(result.is_err(), "长度错误的密文解密应返回 Err");
}

/// 验签拒绝篡改消息。
#[test]
fn rsa_pkcs1v15_verify_rejects_tampered_message() {
    let (private_key, public_key) = test_keypair();
    let message = b"original message";

    let digest = rsa::sha2::Sha256::digest(message);
    let signature = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名");

    // 用不同消息的摘要验签
    let tampered_digest = rsa::sha2::Sha256::digest(b"tampered message");
    let result = public_key.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &tampered_digest,
        &signature,
    );
    assert!(result.is_err(), "篡改消息验签应失败");
}

/// 验签拒绝伪造签名（全零）。
#[test]
fn rsa_pkcs1v15_verify_rejects_forged_signature() {
    let (private_key, public_key) = test_keypair();
    let message = b"verify forged test";

    let digest = rsa::sha2::Sha256::digest(message);
    let _signature = private_key
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名");

    // 伪造签名（全零）
    let forged_signature = vec![0u8; 256];
    let result = public_key.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &digest,
        &forged_signature,
    );
    assert!(result.is_err(), "伪造签名验签应失败");
}

/// 不同密钥对验签：公钥不匹配应失败。
#[test]
fn rsa_pkcs1v15_verify_wrong_public_key_fails() {
    let (sk1, _pk1) = test_keypair();
    let (_sk2, pk2) = test_keypair();
    let message = b"cross key test";

    let digest = rsa::sha2::Sha256::digest(message);
    let signature = sk1
        .sign_with_rng(
            &mut rand_core::OsRng,
            Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
            &digest,
        )
        .expect("签名");

    // 用另一个公钥验签
    let result = pk2.verify(
        Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
        &digest,
        &signature,
    );
    assert!(result.is_err(), "不匹配公钥验签应失败");
}
