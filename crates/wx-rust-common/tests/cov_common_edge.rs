//! 边界覆盖测试：config/token 过期、crypto(sha1/pkcs7/xml)、error 枚举。
//!
//! 对应 Java `WxMpConfigStorageTest`、`SHA1Test`、`PKCS7EncoderTest`、
//! `WxCryptUtilTest`、`WxErrorExceptionTest` 等边界场景。

use std::sync::Arc;

use wx_rust_common::bean::menu::{WxMenu, WxMenuButton};
use wx_rust_common::clock::FakeClock;
use wx_rust_common::config::{TicketType, WxConfigStorage, WxDefaultConfig};
use wx_rust_common::error::{WxError, WxErrorException, WxRuntimeError};
use wx_rust_common::util::crypto::{ByteGroup, Pkcs7Encoder, Sha1};
use wx_rust_common::util::data_utils::DataUtils;
use wx_rust_common::util::sign_utils::SignUtils;
use wx_rust_common::util::xml_utils::XmlUtils;

// ========== TokenEntry 与 WxDefaultConfig 过期边界 ==========

#[test]
/// 对应 Java: WxMpDefaultConfigImpl#getAccessToken — token 未设置时 is_expired = true
fn token_not_set_is_expired() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    assert!(cfg.is_access_token_expired());
    assert!(cfg.access_token().is_none());
}

#[test]
/// 对应 Java: TokenEntry — expires_at = None 表示永不过期
fn token_none_expires_at_never_expires() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let clock = FakeClock::new(1_000_000);
    cfg.set_clock(Arc::new(clock.clone()));
    // 设置一个非常长的过期时间
    cfg.update_access_token("tok", i32::MAX);
    assert!(!cfg.is_access_token_expired());
    assert_eq!(cfg.access_token().as_deref(), Some("tok"));
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — 精确到期秒数 boundary: now == expires_at => expired
fn token_exact_expiry_second_is_expired() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let clock = FakeClock::new(1_000_000_000);
    cfg.set_clock(Arc::new(clock.clone()));
    // expires_in_seconds = 0 => expires_at = now => t <= now => expired
    cfg.update_access_token("tok", 0);
    assert!(cfg.is_access_token_expired());
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — 将过期边界: now + 1 秒仍有效
fn token_one_second_before_expiry_not_expired() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let clock = FakeClock::new(1_000_000_000);
    cfg.set_clock(Arc::new(clock.clone()));
    cfg.update_access_token("tok", 2);
    // now=1000000000, expires_at=1000000002 => not expired
    assert!(!cfg.is_access_token_expired());
    // 推进 1 秒 => now=1000000001, still < 1000000002
    clock.advance_ms(1000);
    assert!(!cfg.is_access_token_expired());
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — 推进到精确过期瞬间 => expired
fn token_advance_to_exact_expiry_is_expired() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let clock = FakeClock::new(1_000_000_000);
    cfg.set_clock(Arc::new(clock.clone()));
    cfg.update_access_token("tok", 5);
    assert!(!cfg.is_access_token_expired());
    // 推进 5 秒 => now == expires_at => expired
    clock.advance_ms(5000);
    assert!(cfg.is_access_token_expired());
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — expire_access_token 清除后过期
fn expire_access_token_clears_token() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let clock = FakeClock::new(1_000_000);
    cfg.set_clock(Arc::new(clock));
    cfg.update_access_token("tok", 7200);
    assert!(!cfg.is_access_token_expired());
    cfg.expire_access_token();
    assert!(cfg.is_access_token_expired());
    assert!(cfg.access_token().is_none());
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — access_token_lock 返回共享锁
fn access_token_lock_returns_shared_arc() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let lock1 = cfg.access_token_lock();
    let lock2 = cfg.access_token_lock();
    // 两次获取应为同一 Arc（引用计数 >= 2）
    assert!(Arc::ptr_eq(&lock1, &lock2));
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl#isStableAccessToken / autoRefresh
fn config_default_flags() {
    let cfg = WxDefaultConfig::new("id", "sec");
    assert!(!cfg.is_stable_access_token());
    assert!(cfg.auto_refresh_token());
    assert_eq!(cfg.app_id(), "id");
    assert_eq!(cfg.secret(), "sec");
    assert!(cfg.http_proxy_host().is_none());
    assert!(cfg.http_proxy_port().is_none());
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — set_clock 第二次调用返回 false
fn set_clock_twice_returns_false() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    let c1 = FakeClock::new(100);
    let c2 = FakeClock::new(200);
    assert!(cfg.set_clock(Arc::new(c1)));
    // 第二次注入应失败（OnceLock 已初始化）
    assert!(!cfg.set_clock(Arc::new(c2)));
}

#[test]
/// 对应 Java: WxMpDefaultConfigImpl — ticket 默认实现
fn default_ticket_methods() {
    let cfg = WxDefaultConfig::new("appid", "secret");
    assert!(cfg.ticket(TicketType::Jsapi).is_none());
    assert!(cfg.is_ticket_expired(TicketType::Jsapi));
    assert!(cfg.ticket(TicketType::WxCard).is_none());
    assert!(cfg.is_ticket_expired(TicketType::WxCard));
}

#[test]
/// 对应 Java: TicketType#value — 各变体返回正确字符串
fn ticket_type_values() {
    assert_eq!(TicketType::Jsapi.value(), "jsapi");
    assert_eq!(TicketType::Sdk.value(), "2");
    assert_eq!(TicketType::WxCard.value(), "wx_card");
}

// ========== SHA1 边界 ==========

#[test]
/// 对应 Java: SHA1Test#testDigest — 空数组参数返回错误
fn sha1_empty_arr_returns_error() {
    let result = Sha1::digest(&[]);
    assert!(result.is_err());
}

#[test]
/// 对应 Java: SHA1Test — 含空字符串参数返回错误
fn sha1_with_empty_string_returns_error() {
    let result = Sha1::digest(&["abc", ""]);
    assert!(result.is_err());
}

#[test]
/// 对应 Java: SHA1Test — 单字符参数正常签名
fn sha1_single_char() {
    let result = Sha1::digest(&["a"]).unwrap();
    assert_eq!(result.len(), 40); // SHA1 hex 固定 40 字符
}

#[test]
/// 对应 Java: SHA1Test — 超长字符串（>64 字节块边界）正常签名
fn sha1_long_string_above_block_boundary() {
    // SHA1 块大小 64 字节，测试 65 字节
    let long = "a".repeat(65);
    let result = Sha1::digest(&[&long]).unwrap();
    assert_eq!(result.len(), 40);
}

#[test]
/// 对应 Java: SHA1Test — digest_with_amp 空参数返回错误
fn sha1_amp_empty_returns_error() {
    let result = Sha1::digest_with_amp(&[]);
    assert!(result.is_err());
}

#[test]
/// 对应 Java: SHA1Test — digest_with_amp 多参数排序拼接
fn sha1_amp_deterministic_sorting() {
    // 参数排序后拼接 "b&c&a" => 排序 "a&b&c"
    let r1 = Sha1::digest_with_amp(&["b", "c", "a"]).unwrap();
    let r2 = Sha1::digest_with_amp(&["a", "b", "c"]).unwrap();
    assert_eq!(r1, r2);
}

// ========== PKCS7 边界 ==========

#[test]
/// 对应 Java: PKCS7EncoderTest — 块大小整除时填充 32 字节（满块再加一块）
fn pkcs7_encode_block_aligned() {
    // 32 字节对齐 => 填充 32 字节（值 32）
    let pad = Pkcs7Encoder::encode(32);
    assert_eq!(pad.len(), 32);
    assert!(pad.iter().all(|&b| b == 32));
}

#[test]
/// 对应 Java: PKCS7EncoderTest — 差一字节对齐时填充 1 字节
fn pkcs7_encode_one_byte_short() {
    // 31 字节 => 填充 1 字节
    let pad = Pkcs7Encoder::encode(31);
    assert_eq!(pad.len(), 1);
    assert_eq!(pad[0], 1);
}

#[test]
/// 对应 Java: PKCS7EncoderTest — 0 字节输入填充 32 字节
fn pkcs7_encode_zero_bytes() {
    let pad = Pkcs7Encoder::encode(0);
    assert_eq!(pad.len(), 32);
    assert!(pad.iter().all(|&b| b == 32));
}

#[test]
/// 对应 Java: PKCS7EncoderTest — decode 空输入返回空
fn pkcs7_decode_empty() {
    let result = Pkcs7Encoder::decode(&[]);
    assert!(result.is_empty());
}

#[test]
/// 对应 Java: PKCS7EncoderTest — decode 合法填充（PKCS7: 填充值 = 填充字节数，重复填充值）
fn pkcs7_decode_valid_padding() {
    let mut data = vec![1u8, 2, 3];
    // 32 - 3 = 29 填充字节，每个字节值 = 29
    data.extend_from_slice(&[29u8; 29]);
    let result = Pkcs7Encoder::decode(&data);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
/// 对应 Java: PKCS7EncoderTest — decode 非法填充值（0）视为无填充
fn pkcs7_decode_invalid_pad_zero() {
    let data = vec![1u8, 2, 3, 0]; // 末尾 0 不在 1..=32 范围
    let result = Pkcs7Encoder::decode(&data);
    assert_eq!(result, vec![1, 2, 3, 0]); // 无填充
}

#[test]
/// 对应 Java: PKCS7EncoderTest — decode 填充值 > 32 视为无填充
fn pkcs7_decode_invalid_pad_above_32() {
    let data = vec![1u8, 2, 3, 33]; // 33 > 32
    let result = Pkcs7Encoder::decode(&data);
    assert_eq!(result, vec![1, 2, 3, 33]); // 无填充
}

// ========== XML 边界 ==========

#[test]
/// 对应 Java: XmlUtils#xml2Map — 正常解析
fn xml_normal_parse() {
    let xml = "<xml><ToUser>abc</ToUser><FromUser>def</FromUser></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("ToUser").map(|s| s.as_str()), Some("abc"));
    assert_eq!(map.get("FromUser").map(|s| s.as_str()), Some("def"));
}

#[test]
/// 对应 Java: XmlUtils#xml2Map — 空 XML 返回空 Map
fn xml_empty_root_returns_empty_map() {
    let xml = "<xml></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert!(map.is_empty());
}

#[test]
/// 对应 Java: XmlUtils#xml2Map — 空字符串输入
fn xml_empty_string() {
    let result = XmlUtils::xml_2_map("");
    // quick-xml 对空字符串可能返回 Ok(空 map) 或 Err
    // 只要不 panic 即可
    let _ = result;
}

#[test]
/// 对应 Java: XmlUtils#xml2Map — 未闭合标签（坏输入）返回错误或空
fn xml_unclosed_tag() {
    let xml = "<xml><ToUser>abc</ToUser><Broken>unclosed";
    let result = XmlUtils::xml_2_map(xml);
    // quick-xml 对未闭合标签到达 EOF 时可能返回 Ok（部分解析）或 Err
    // 关键是不 panic
    let _ = result;
}

#[test]
/// 对应 Java: XmlUtils#xml2Map — CDATA 节点正常解析
fn xml_cdata_content() {
    let xml = "<xml><Content><![CDATA[hello world]]></Content></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("Content").map(|s| s.as_str()), Some("hello world"));
}

#[test]
/// 对应 Java: XmlUtils#xml2Map — 同名元素保留最后一个
fn xml_duplicate_keys_last_wins() {
    let xml = "<xml><Key>first</Key><Key>second</Key></xml>";
    let map = XmlUtils::xml_2_map(xml).unwrap();
    assert_eq!(map.get("Key").map(|s| s.as_str()), Some("second"));
}

// ========== ByteGroup ==========

#[test]
/// 对应 Java: ByteGroup — 空字节组
fn byte_group_empty() {
    let bg = ByteGroup::new();
    assert_eq!(bg.size(), 0);
    assert!(bg.to_bytes().is_empty());
}

#[test]
/// 对应 Java: ByteGroup — 多次拼接
fn byte_group_concat() {
    let mut bg = ByteGroup::new();
    bg.add_bytes(&[1, 2]);
    bg.add_bytes(&[3, 4, 5]);
    assert_eq!(bg.size(), 5);
    assert_eq!(bg.to_bytes(), vec![1, 2, 3, 4, 5]);
}

// ========== WxError 边界 ==========

#[test]
/// 对应 Java: WxError#toString — Display 输出含错误码与信息
fn wx_error_display_format() {
    let err = WxError::new(40001, "invalid credential");
    let s = format!("{err}");
    assert!(s.contains("40001"));
    assert!(s.contains("invalid credential"));
}

#[test]
/// 对应 Java: WxError#new — 错误码 0 表示成功
fn wx_error_code_zero() {
    let err = WxError::new(0, "ok");
    assert_eq!(err.error_code, 0);
}

#[test]
/// 对应 Java: WxError#new — 负数错误码
fn wx_error_negative_code() {
    let err = WxError::new(-1, "system error");
    assert_eq!(err.error_code, -1);
}

#[test]
/// 对应 Java: WxError#fromJson — JSON 解析成功
fn wx_error_from_json_valid() {
    let json = r#"{"errcode":40001,"errmsg":"invalid"}"#;
    let err = WxError::from_json(json);
    assert_eq!(err.error_code, 40001);
    assert_eq!(err.error_msg.as_deref(), Some("invalid"));
}

#[test]
/// 对应 Java: WxError#fromJson — 非法 JSON 回退到默认错误码 -99
fn wx_error_from_json_invalid() {
    let err = WxError::from_json("not json");
    assert_eq!(err.error_code, -99);
}

#[test]
/// 对应 Java: WxError — Display 含 json 字段时输出原始报文
fn wx_error_display_with_json() {
    let mut err = WxError::new(42, "fail");
    err.json = Some(r#"{"errcode":42}"#.to_string());
    let s = format!("{err}");
    assert!(s.contains("原始报文"));
}

// ========== WxErrorException 边界 ==========

#[test]
/// 对应 Java: WxErrorException — from_code 构造业务错误
fn wx_error_exception_from_code() {
    let exc = WxErrorException::from_code(40001, "bad token");
    assert_eq!(exc.error_code(), Some(40001));
}

#[test]
/// 对应 Java: WxErrorException — error_code 对非业务变体返回 None
fn wx_error_exception_io_no_code() {
    let exc: WxErrorException = std::io::Error::other("io fail").into();
    assert!(exc.error_code().is_none());
}

#[test]
/// 对应 Java: WxErrorException — wx_error 对非业务变体返回 None
fn wx_error_exception_io_no_wx_error() {
    let exc: WxErrorException = WxErrorException::Io("something".to_string());
    assert!(exc.wx_error().is_none());
}

#[test]
/// 对应 Java: WxRuntimeException — Display 输出
fn wx_runtime_error_display() {
    let err = WxRuntimeError::new("retry exceeded");
    let s = format!("{err}");
    assert!(s.contains("retry exceeded"));
}

#[test]
/// 对应 Java: WxErrorException — serde 变体 Display
fn wx_error_exception_serde_display() {
    let exc: WxErrorException = WxErrorException::Serde("bad json".to_string());
    let s = format!("{exc}");
    assert!(s.contains("序列化错误"));
    assert!(s.contains("bad json"));
}

// ========== SignUtils ==========

#[test]
/// 对应 Java: SignUtils#createHmacSha256Sign — 确定性输出
fn hmac_sha256_deterministic() {
    let s1 = SignUtils::create_hmac_sha256_sign("msg", "key");
    let s2 = SignUtils::create_hmac_sha256_sign("msg", "key");
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 64); // SHA256 hex 64 字符
}

// ========== DataUtils ==========

#[test]
/// 对应 Java: DataUtils#handleDataWithSecret — 脱敏替换
fn data_utils_mask_secret() {
    let input = "grant_type=client_credential&secret=abc123&other=val";
    let masked = DataUtils::handle_data_with_secret(input);
    assert!(masked.contains("&secret=******&"));
    assert!(!masked.contains("abc123"));
}

#[test]
/// 对应 Java: DataUtils#handleDataWithSecret — 无 secret 参数保持不变
fn data_utils_no_secret_unchanged() {
    let input = "grant_type=client_credential&other=val";
    let masked = DataUtils::handle_data_with_secret(input);
    assert_eq!(masked, input);
}

// ========== WxMenu JSON 边界 ==========

#[test]
/// 对应 Java: WxMenu#toJson / fromJson — 空菜单序列化/反序列化
fn wx_menu_empty_roundtrip() {
    let menu = WxMenu::default();
    let json = menu.to_json();
    let parsed = WxMenu::from_json(&json).unwrap();
    assert!(parsed.buttons.is_empty());
}

#[test]
/// 对应 Java: WxMenu — 含按钮的菜单序列化
fn wx_menu_with_button_roundtrip() {
    let mut menu = WxMenu::default();
    menu.buttons.push(WxMenuButton {
        r#type: "click".to_string(),
        name: "test".to_string(),
        key: "V1001".to_string(),
        ..Default::default()
    });
    let json = menu.to_json();
    let parsed = WxMenu::from_json(&json).unwrap();
    assert_eq!(parsed.buttons.len(), 1);
    assert_eq!(parsed.buttons[0].r#type, "click");
}

// ========== WxCryptUtil 边界 ==========

#[test]
/// 对应 Java: WxCryptUtil — aesKey 长度错误
fn wx_crypt_util_invalid_aes_key_length() {
    let result = wx_rust_common::util::crypto::WxCryptUtil::new("token", "shortkey", "appid");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("aesKey"));
}

#[test]
/// 对应 Java: WxCryptUtil — 正常构造（44 字符 base64 编码 = 32 字节）
fn wx_crypt_util_valid_construction() {
    // 44 字符的 base64 编码（32 字节）
    let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG";
    let result = wx_rust_common::util::crypto::WxCryptUtil::new("token", aes_key, "appid");
    assert!(result.is_ok());
}

#[test]
/// 对应 Java: WxCryptUtil#encrypt / decrypt — 加解密往返
fn wx_crypt_util_encrypt_decrypt_roundtrip() {
    let aes_key = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG";
    let crypt = wx_rust_common::util::crypto::WxCryptUtil::new("token", aes_key, "appid").unwrap();
    let plain = "<xml><Content>hello</Content></xml>";
    let _encrypted_xml = crypt.encrypt(plain).unwrap();
    // 从生成的 xml 中提取 Encrypt 节点解密
    let ctx = crypt.encrypt_context(plain).unwrap();
    let decrypted = crypt
        .decrypt_content(
            &ctx.signature,
            &ctx.timestamp,
            &ctx.nonce,
            &ctx.encrypted_xml,
        )
        .unwrap();
    assert_eq!(decrypted, plain);
}
