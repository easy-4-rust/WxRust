//! Criterion benchmarks for wx-rust-common crypto primitives.
//!
//! Benchmarks the most CPU-heavy pure functions:
//! - `WxCryptUtil` encrypt / decrypt (AES-256-CBC + PKCS7 + SHA1 sign)
//! - `Sha1::digest_with_amp` (SHA1 message signing)
//! - `SignUtils::create_hmac_sha256_sign` (HMAC-SHA256)

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use wx_rust_common::util::SignUtils;
use wx_rust_common::util::crypto::{Sha1, WxCryptUtil};

/// Typical WeChat message XML payload (~500 bytes).
const SAMPLE_PLAIN_TEXT: &str = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[Hello World from WxRust benchmark]]></Content>
<MsgId>1234567890123456</MsgId>
</xml>"#;

/// Test token (configured in WeChat backend).
const TEST_TOKEN: &str = "test_token_abc123";

/// Test EncodingAESKey (43 chars, standard WeChat format).
const TEST_AES_KEY: &str = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG";

/// Test appid.
const TEST_APPID: &str = "wx1234567890abcdef";

/// Build a `WxCryptUtil` instance for benchmarks.
fn make_crypt_util() -> WxCryptUtil {
    WxCryptUtil::new(TEST_TOKEN, TEST_AES_KEY, TEST_APPID)
        .expect("WxCryptUtil construction should succeed with valid inputs")
}

fn bench_wx_crypt_util(c: &mut Criterion) {
    let crypt = make_crypt_util();

    // Pre-encrypt a message so we have a valid ciphertext for decrypt benchmarks.
    let encrypted_xml = crypt
        .encrypt(SAMPLE_PLAIN_TEXT)
        .expect("encrypt should succeed for benchmark setup");

    // Extract the Base64 ciphertext from the encrypted XML for raw decrypt.
    let cipher_text = {
        let start = encrypted_xml.find("<Encrypt>").unwrap() + "<Encrypt>".len();
        let end = encrypted_xml[start..].find("</Encrypt>").unwrap();
        let raw = &encrypted_xml[start..start + end];
        raw.trim()
            .strip_prefix("<![CDATA[")
            .and_then(|s| s.strip_suffix("]]>"))
            .unwrap_or(raw)
            .to_string()
    };

    // Pre-compute signature for decrypt_content benchmarks.
    let timestamp = "1348831860";
    let nonce = "test_nonce_12345";
    let signature = Sha1::digest_with_amp(&[TEST_TOKEN, timestamp, nonce, &cipher_text])
        .expect("signature should succeed");

    c.bench_function("encrypt", |b| {
        b.iter(|| {
            black_box(
                crypt
                    .encrypt(black_box(SAMPLE_PLAIN_TEXT))
                    .expect("encrypt should not fail"),
            );
        })
    });

    c.bench_function("decrypt", |b| {
        b.iter(|| {
            black_box(
                crypt
                    .decrypt(black_box(&cipher_text))
                    .expect("decrypt should not fail"),
            );
        })
    });

    c.bench_function("decrypt_xml", |b| {
        b.iter(|| {
            black_box(
                crypt
                    .decrypt_xml(
                        black_box(&signature),
                        black_box(timestamp),
                        black_box(nonce),
                        black_box(&encrypted_xml),
                    )
                    .expect("decrypt_xml should not fail"),
            );
        })
    });
}

fn bench_sha1(c: &mut Criterion) {
    let params = vec![
        "token_abc",
        "1348831860",
        "nonce_xyz",
        "encrypted_base64_content",
    ];

    c.bench_function("sha1_digest_with_amp_4_params", |b| {
        b.iter(|| {
            black_box(Sha1::digest_with_amp(black_box(&params)).expect("sha1 should not fail"));
        })
    });

    c.bench_function("sha1_digest_4_params", |b| {
        b.iter(|| {
            black_box(Sha1::digest(black_box(&params)).expect("sha1 should not fail"));
        })
    });
}

fn bench_hmac_sha256(c: &mut Criterion) {
    let key = "test_hmac_secret_key_1234567890";
    let message = "appid=wx1234567890abcdef&mch_id=1234567890&nonce_str=nonce123&body=WxRust+Test";

    c.bench_function("hmac_sha256_sign", |b| {
        b.iter(|| {
            black_box(SignUtils::create_hmac_sha256_sign(
                black_box(message),
                black_box(key),
            ));
        })
    });
}

criterion_group!(benches, bench_wx_crypt_util, bench_sha1, bench_hmac_sha256);
criterion_main!(benches);
