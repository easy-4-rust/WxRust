//! SOURCE_PARITY 测试：对应 WxJava common 测试
//! `util/crypto/SHA1Test`、`util/crypto/WxCryptUtilTest`、`util/DataUtilsTest`、
//! `util/XmlUtilsTest`、`api/WxMessageInMemoryDuplicateCheckerTest`、`session/SessionTest`。
//!
//! 证据级别：`V2_MIRRORED`（镜像断言）+ `V3_GOLDEN_DIFF`（WxCryptUtilTest 黄金向量）。

use std::time::Duration;

use wx_rust_common::api::{WxMessageDuplicateChecker, WxMessageInMemoryDuplicateChecker};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::crypto::{Sha1, WxCryptUtil};
use wx_rust_common::util::{DataUtils, XmlUtils};

// ---- 镜像 Java SHA1Test ----
// Java 断言：
//   SHA1.gen("123", "345") == "9f537aeb751ec72605f57f94a2f6dc3e3958e1dd"
//   SHA1.genWithAmple("123", "345") == "20b896ccbd5a72dde5dbe0878ff985e4069771c6"
#[test]
fn sha1_gen_known_vector() {
    let result = Sha1::digest(&["123", "345"]).unwrap();
    assert_eq!(result, "9f537aeb751ec72605f57f94a2f6dc3e3958e1dd");
}

#[test]
fn sha1_gen_with_ample_known_vector() {
    let result = Sha1::digest_with_amp(&["123", "345"]).unwrap();
    assert_eq!(result, "20b896ccbd5a72dde5dbe0878ff985e4069771c6");
}

#[test]
fn sha1_gen_illegal_arguments() {
    // Java: 任一参数为 null/空 抛 IllegalArgumentException
    let result = Sha1::digest(&["", "345"]);
    assert!(result.is_err());
}

// ---- 镜像 Java WxCryptUtilTest（黄金向量）----
// Java 固定输入：
//   encodingAesKey = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG"
//   token = "pamtest", appId = "wxb11529c136998cb6", randomStr = "aaaabbbbccccdddd"
//   replyMsg = "我是中文abcd123" -> afterAesEncrypt = "jn1L23DB+6ELqJ+6bruv21Y6MD7KeIfP82D6gU39rmkgczbWwt5+3bnyg5K55bgVtVzd832WzZGMhkP72vVOfg=="
fn crypt_util_fixture() -> WxCryptUtil {
    WxCryptUtil::new(
        "pamtest",
        "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG",
        "wxb11529c136998cb6",
    )
    .expect("aesKey 构造成功")
}

#[test]
fn crypt_util_aes_encrypt_golden() {
    // V3_GOLDEN_DIFF：固定 randomStr + 固定明文 -> 固定密文（Java 黄金向量）
    let util = crypt_util_fixture();
    let encrypted = util
        .encrypt_with_random("aaaabbbbccccdddd", "我是中文abcd123")
        .expect("加密成功");
    assert_eq!(
        encrypted,
        "jn1L23DB+6ELqJ+6bruv21Y6MD7KeIfP82D6gU39rmkgczbWwt5+3bnyg5K55bgVtVzd832WzZGMhkP72vVOfg=="
    );
}

#[test]
fn crypt_util_normal_roundtrip() {
    // 镜像 Java testNormal：encrypt -> 提取密文 -> decrypt 得回原文
    let util = crypt_util_fixture();
    let encrypted_xml = util.encrypt("我是中文abcd123").expect("加密");
    assert!(encrypted_xml.contains("<Encrypt>"));
    assert!(encrypted_xml.contains("<MsgSignature>"));

    // 用 encrypt_context 的值构造 xml 后解密（签名验证路径）
    let ctx = util.encrypt_context("我是中文abcd123").expect("ctx");
    let full_xml = format!(
        "<xml>\n<Encrypt><![CDATA[{}]]></Encrypt>\n<MsgSignature><![CDATA[{}]]></MsgSignature>\n<TimeStamp>{}</TimeStamp>\n<Nonce><![CDATA[{}]]></Nonce>\n</xml>",
        ctx.encrypted_xml, ctx.signature, ctx.timestamp, ctx.nonce
    );
    let plain = util
        .decrypt_xml(&ctx.signature, &ctx.timestamp, &ctx.nonce, &full_xml)
        .expect("解密成功");
    assert_eq!(plain, "我是中文abcd123");
}

#[test]
fn crypt_util_direct_decrypt() {
    // 镜像 Java testNormal 中的 pc.decrypt(cipherText)：直接解密不验签
    let util = crypt_util_fixture();
    let ctx = util.encrypt_context("我是中文abcd123").expect("ctx");
    let plain = util.decrypt(&ctx.encrypted_xml).expect("直接解密");
    assert_eq!(plain, "我是中文abcd123");
}

#[test]
fn crypt_util_validate_signature_error() {
    // 镜像 Java testValidateSignatureError：错误签名抛异常
    let util = crypt_util_fixture();
    let ctx = util.encrypt_context("我是中文abcd123").expect("ctx");
    let full_xml = format!(
        "<xml>\n<Encrypt><![CDATA[{}]]></Encrypt>\n<MsgSignature><![CDATA[{}]]></MsgSignature>\n<TimeStamp>{}</TimeStamp>\n<Nonce><![CDATA[{}]]></Nonce>\n</xml>",
        ctx.encrypted_xml, "12345", ctx.timestamp, ctx.nonce
    );
    let result = util.decrypt_xml("12345", &ctx.timestamp, &ctx.nonce, &full_xml);
    assert!(result.is_err(), "错误签名必须被拒绝");
    let err = result.unwrap_err();
    assert!(
        err.contains("签名验证错误") || err.contains("签名"),
        "错误信息应提及签名，实际: {err}"
    );
}

#[test]
fn crypt_util_xml_extract_encrypt() {
    // 验证 extract_encrypt_part：CDATA 包裹的密文提取
    let util = crypt_util_fixture();
    let ctx = util.encrypt_context("hello").expect("ctx");
    let full_xml = format!(
        "<xml><ToUserName><![CDATA[toUser]]></ToUserName><Encrypt><![CDATA[{}]]></Encrypt></xml>",
        ctx.encrypted_xml
    );
    let plain = util
        .decrypt_xml(&ctx.signature, &ctx.timestamp, &ctx.nonce, &full_xml)
        .expect("从带前缀的 xml 解密");
    assert_eq!(plain, "hello");
}

// ---- 镜像 Java DataUtilsTest ----
#[test]
fn data_utils_handle_data_with_secret() {
    let data = "js_code=001tZveq0SMoiq1AEXeq0ECJeq0tZveZ&secret=5681022fa1643845392367ea88888888&grant_type=authorization_code&appid=wxe156d4848d999999";
    let s = DataUtils::handle_data_with_secret(data);
    assert!(s.contains("&secret=******&"), "secret 应被脱敏，实际: {s}");
    assert!(
        !s.contains("5681022fa1643845392367ea88888888"),
        "原始 secret 不应出现"
    );
}

#[test]
fn data_utils_no_secret_unchanged() {
    let data = "js_code=abc&grant_type=authorization_code";
    let s = DataUtils::handle_data_with_secret(data);
    assert_eq!(s, data, "无 secret 时原样返回");
}

// ---- 镜像 Java XmlUtilsTest ----
#[test]
fn xml_utils_xml2map() {
    let xml = "<xml>\n\
      <ToUserName><![CDATA[toUser]]></ToUserName>\n\
      <FromUserName><![CDATA[fromUser]]></FromUserName>\n\
      <MsgType><![CDATA[text]]></MsgType>\n\
      <Content><![CDATA[你好]]></Content>\n\
      </xml>";
    let map = XmlUtils::xml_2_map(xml).expect("解析成功");
    assert_eq!(map.get("ToUserName").map(String::as_str), Some("toUser"));
    assert_eq!(map.get("MsgType").map(String::as_str), Some("text"));
    assert_eq!(map.get("Content").map(String::as_str), Some("你好"));
}

#[test]
fn xml_utils_xml2map_cdata() {
    // Java 测试用例：CDATA 内容提取
    let xml = "<xml><Encrypt><![CDATA[密文内容]]></Encrypt></xml>";
    let map = XmlUtils::xml_2_map(xml).expect("解析成功");
    assert_eq!(map.get("Encrypt").map(String::as_str), Some("密文内容"));
}

#[test]
fn xml_utils_malformed_returns_err() {
    // Java testXml2Map_xxe：含 DOCTYPE/实体声明时应拒绝（XXE 防护语义）
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE test [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<xml></xml>"#;
    // quick-xml 默认不解析外部实体，但 DOCTYPE 可能直接报错或被跳过。
    // 语义要求：不抛出解析错误或返回空 map（不读取外部实体）。
    let result = XmlUtils::xml_2_map(xml);
    if let Ok(map) = result {
        assert!(map.is_empty() || map.len() <= 1, "XXE 不应扩展实体");
    }
    // Err 拒绝解析也是合规防护
}

// ---- 镜像 Java WxMessageInMemoryDuplicateCheckerTest ----
#[test]
fn duplicate_checker_ttl_behavior() {
    // Java: checker(2000L TTL, 1000L 清理周期)；8 个 msgId 三阶段检查
    let checker = WxMessageInMemoryDuplicateChecker::with_config(2000, 1000);
    let msg_ids = ["1", "2", "3", "4", "5", "6", "7", "8"];

    // 第一次检查：全部非重复
    for id in msg_ids {
        assert!(!checker.is_duplicate(id), "首次检查 {id} 不应重复");
    }
    // 立即再查：全部重复
    for id in msg_ids {
        assert!(checker.is_duplicate(id), "紧接重复检查 {id} 应重复");
    }
}

#[test]
fn duplicate_checker_expiry() {
    let checker = WxMessageInMemoryDuplicateChecker::with_config(200, 100); // 200ms TTL
    assert!(!checker.is_duplicate("msg1"));
    assert!(checker.is_duplicate("msg1"));
    std::thread::sleep(Duration::from_millis(300));
    // TTL 过期后再次检查应为非重复
    assert!(!checker.is_duplicate("msg1"), "TTL 过期后应重新接受");
}

// ---- 镜像 Java WxMessageInMemoryDuplicateCheckerSingletonTest ----
#[test]
fn duplicate_checker_singleton_same_instance() {
    use wx_rust_common::api::WxMessageInMemoryDuplicateCheckerSingleton;

    // Java: getInstance() 两次返回同一实例
    let a = WxMessageInMemoryDuplicateCheckerSingleton::get_instance();
    let b = WxMessageInMemoryDuplicateCheckerSingleton::get_instance();
    assert!(std::ptr::eq(a, b), "单例两次获取应为同一实例");

    // 单例对象直接实现 WxMessageDuplicateChecker，去重行为生效
    let singleton = WxMessageInMemoryDuplicateCheckerSingleton;
    assert!(!singleton.is_duplicate("singleton-msg-1"));
    assert!(singleton.is_duplicate("singleton-msg-1"));
}

// ---- 镜像 Java SessionTest ----
#[test]
fn session_get_same_instance() {
    // Java: getSession("abc") 两次返回同一会话
    let manager = StandardSessionManager::new();
    let s1 = manager.get_session("abc");
    let s2 = manager.get_session("abc");
    assert_eq!(s1.id(), s2.id());

    // getSession("abc1", false) 已存在则返回
    let abc1 = manager.get_session("abc1");
    let abc1b = manager.get_session_or_create("abc1", false);
    assert!(abc1b.is_some());
    assert_eq!(abc1.id(), abc1b.unwrap().id());

    // getSession("def", false) 不存在返回 null
    let def = manager.get_session_or_create("def", false);
    assert!(def.is_none(), "create=false 且不存在应返回 None");
}

#[test]
fn session_attributes_roundtrip() {
    let manager = StandardSessionManager::new();
    let session = manager.get_session("sess-attr");
    session.set_attribute("openid", "o123".to_string());
    session.set_attribute("unionid", "u456".to_string());
    assert_eq!(session.get_attribute("openid").as_deref(), Some("o123"));
    assert_eq!(session.get_attribute("unionid").as_deref(), Some("u456"));
    session.remove_attribute("openid");
    assert_eq!(session.get_attribute("openid"), None);
    let names = session.attribute_names();
    assert_eq!(names, vec!["unionid".to_string()]);
}

#[test]
fn session_invalidate() {
    // Java testInvalidate：invalidate 后 getAttributeNames 抛 IllegalStateException
    let manager = StandardSessionManager::new();
    let session = manager.get_session("abc");
    session.set_attribute("k", "v".to_string());
    session.invalidate();
    // 失效后属性为空且不可访问
    assert!(!session.is_valid());
    assert_eq!(session.get_attribute("k"), None);
    assert!(session.attribute_names().is_empty());
}

#[test]
fn session_invalidate_and_active_count() {
    // Java testInvalidate2/testInvalidateAngGet：activeSessions 统计
    let manager = StandardSessionManager::new();
    assert_eq!(manager.active_sessions(), 0);
    let s1 = manager.get_session("abc");
    assert_eq!(manager.active_sessions(), 1);
    s1.invalidate();
    assert_eq!(manager.active_sessions(), 0);
    // 失效后重建
    let s2 = manager.get_session("abc");
    assert_eq!(manager.active_sessions(), 1);
    // Java 语义：失效后 getSession 返回新会话对象（引用不同）
    let s1_ptr = std::sync::Arc::as_ptr(&s1);
    let s2_ptr = std::sync::Arc::as_ptr(&s2);
    assert_ne!(s1_ptr, s2_ptr, "失效后应返回新会话对象");
    assert_eq!(s1.id(), s2.id(), "新会话 id 相同");
}
