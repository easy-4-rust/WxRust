//! 核心工具语义测试（镜像 Java 测试语义）。
//!
//! 对应 WxJava `weixin-java-common/src/test` 的关键行为：
//! SHA1 签名、PKCS7 填充、WxError 解析与中文翻译。

use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxError;
use wx_rust_common::util::crypto::WxCryptUtil;
use wx_rust_common::util::crypto::{Pkcs7Encoder, Sha1};

use sha1::Digest as _;

#[test]
fn sha1_gen_sorts_and_hashes() {
    // 排序拼接后 SHA1：与 Java DigestUtils.sha1Hex 行为一致
    let result = Sha1::digest(&["c", "a", "b"]).unwrap();
    // "abc" 的 SHA1 已知向量
    assert_eq!(result, "a9993e364706816aba3e25717850c26c9cd0d89d");
}

#[test]
fn sha1_gen_with_amp_joins_with_ampersand() {
    let result = Sha1::digest_with_amp(&["c", "a", "b"]).unwrap();
    // "a&b&c" 的 SHA1（排序后 & 连接）
    let expected = hex::encode(sha1::Sha1::digest(b"a&b&c"));
    assert_eq!(result, expected);
}

#[test]
fn sha1_gen_rejects_empty_args() {
    assert!(Sha1::digest(&["a", ""]).is_err());
    assert!(Sha1::digest(&[]).is_err());
}

#[test]
fn pkcs7_encode_pads_to_32() {
    // 20 字节 -> 补 12 字节（值为 12 的 ASCII 字符）
    let pad = Pkcs7Encoder::encode(20);
    assert_eq!(pad.len(), 12);
    assert!(pad.iter().all(|&b| b == 12));
}

#[test]
fn pkcs7_encode_exact_block_no_pad() {
    // 32 字节 -> 补 32 字节（值为 32）
    let pad = Pkcs7Encoder::encode(32);
    assert_eq!(pad.len(), 32);
}

#[test]
fn pkcs7_decode_removes_padding() {
    let mut data = vec![1u8; 20];
    data.extend(vec![12u8; 12]); // 模拟 PKCS7 填充
    let decoded = Pkcs7DecoderHelper::decode(&data);
    assert_eq!(decoded.len(), 20);
}

struct Pkcs7DecoderHelper;
impl Pkcs7DecoderHelper {
    fn decode(d: &[u8]) -> Vec<u8> {
        Pkcs7Encoder::decode(d)
    }
}

#[test]
fn wx_error_from_json_parses_code_and_msg() {
    let err = WxError::from_json(r#"{"errcode":40001,"errmsg":"invalid credential"}"#);
    assert_eq!(err.error_code, 40001);
    assert_eq!(err.error_msg.as_deref(), Some("invalid credential"));
    assert!(err.json.is_some());
}

#[test]
fn wx_error_translates_to_chinese_by_type() {
    // MP 平台 40001 -> 中文错误信息
    let err = WxError::from_json_with_type(
        r#"{"errcode":40001,"errmsg":"invalid credential"}"#,
        Some(WxType::Mp),
    );
    assert_eq!(err.error_code, 40001);
    // 原文被保存为 error_msg_en
    assert_eq!(err.error_msg_en.as_deref(), Some("invalid credential"));
    // 中文翻译（来自 WxMpErrorMsgEnum 40001）
    let zh = err.error_msg.as_deref().unwrap();
    assert!(
        zh.contains("AppSecret 错误") || zh.contains("access_token"),
        "实际: {zh}"
    );
}

#[test]
fn wx_error_zero_code_no_translation() {
    let err = WxError::from_json_with_type(r#"{"errcode":0,"errmsg":"ok"}"#, Some(WxType::Mp));
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg.as_deref(), Some("ok"));
}

#[test]
fn crypt_util_encrypt_decrypt_roundtrip() {
    // 微信 EncodingAESKey：43 字符 base64（解码后 32 字节）
    let aes_key = "kvuO9BLIAs5iFlXRfwOJXjh3z7O1psxaY6jY1pnFUBQ=";
    let util = WxCryptUtil::new("test-token", aes_key, "wxappid123").expect("aesKey");
    let plain = "<xml><Content><![CDATA[你好微信]]></Content></xml>";
    let encrypted = util.encrypt(plain).expect("加密成功");
    assert!(encrypted.contains("<Encrypt>"));
    assert!(encrypted.contains("<MsgSignature>"));

    // 用 encrypt_context 的值构造完整 xml（对应 Java generateXml 语义）
    let ctx = util.encrypt_context(plain).expect("加密上下文");
    let full_xml = format!(
        "<xml>\n<Encrypt><![CDATA[{}]]></Encrypt>\n<MsgSignature><![CDATA[{}]]></MsgSignature>\n<TimeStamp>{}</TimeStamp>\n<Nonce><![CDATA[{}]]></Nonce>\n</xml>",
        ctx.encrypted_xml, ctx.signature, ctx.timestamp, ctx.nonce
    );
    // 从完整 xml 提取 Encrypt 密文、签名、时间戳、nonce 并解密
    let decrypted = util
        .decrypt_xml(&ctx.signature, &ctx.timestamp, &ctx.nonce, &full_xml)
        .expect("解密成功");
    assert_eq!(decrypted, plain);
}

#[test]
fn crypt_util_wrong_signature_fails() {
    let aes_key = "kvuO9BLIAs5iFlXRfwOJXjh3z7O1psxaY6jY1pnFUBQ=";
    let util = WxCryptUtil::new("test-token", aes_key, "wxappid123").expect("aesKey");
    let ctx = util.encrypt_context("hello").expect("encrypt");
    let full_xml = format!(
        "<xml>\n<Encrypt><![CDATA[{}]]></Encrypt>\n<MsgSignature><![CDATA[{}]]></MsgSignature>\n<TimeStamp>{}</TimeStamp>\n<Nonce><![CDATA[{}]]></Nonce>\n</xml>",
        ctx.encrypted_xml, ctx.signature, ctx.timestamp, ctx.nonce
    );
    // 错误签名应被拒绝
    let result = util.decrypt_xml("bad-signature", &ctx.timestamp, &ctx.nonce, &full_xml);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("签名验证错误"));
}
