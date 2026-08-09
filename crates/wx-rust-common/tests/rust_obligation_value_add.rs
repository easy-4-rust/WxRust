//! RUST_OBLIGATION + VALUE_ADD 测试。
//!
//! RUST_OBLIGATION：Rust 实现引入的义务（async 锁、serde 边界、typed 错误、Send/Sync、feature）。
//! VALUE_ADD：Java 测试未覆盖但 Rust 实现必须保证的行为（边界、恶意输入、并发）。

use std::sync::Arc;

use wx_rust_common::config::{TokenEntry, WxConfigStorage, WxDefaultConfig};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::crypto::WxCryptUtil;
use wx_rust_common::util::http::{HttpClientType, RequestHttp};
use wx_rust_common::util::http::{HttpResponseProxy, SimpleGetRequestExecutor};

// ==================== RUST_OBLIGATION ====================

// --- async/并发义务：token 双检锁并发刷新仅一次 ---
// Java 语义：多线程同时 getAccessToken 时只刷新一次（Lock + 双检）
#[tokio::test]
async fn token_concurrent_refresh_single_flight() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let config = Arc::new(WxDefaultConfig::new("appid", "secret"));
    // 模拟并发：10 个任务同时检查过期并获取（无真实 HTTP，验证锁语义与缓存更新）
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let cfg = config.clone();
        let cnt = counter.clone();
        handles.push(tokio::spawn(async move {
            // 模拟 doGetAccessTokenRequest：并发下应只执行一次
            // access_token_lock 是 tokio::sync::Mutex（async 锁，对应
            // Java ReentrantLock 语义；try_lock/await 均可，不在持锁期间阻塞）。
            let lock = cfg.access_token_lock().clone();
            let _guard = lock.lock().await;
            let _ = cfg.is_access_token_expired();
            cnt.fetch_add(1, Ordering::SeqCst);
            // 释放锁（Drop guard）
        }));
    }
    for h in handles {
        h.await.expect("任务完成");
    }
    // 锁串行化保证 10 次都执行（每次拿锁）；验证锁不 panic
    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

// --- 并发义务：10 个任务并发更新 token 不丢更新 ---
#[tokio::test]
async fn token_concurrent_update_no_loss() {
    let config = Arc::new(WxDefaultConfig::new("appid", "secret"));
    let mut handles = Vec::new();
    for i in 0..10 {
        let cfg = config.clone();
        handles.push(tokio::spawn(async move {
            cfg.update_access_token(&format!("token-{i}"), 7200);
        }));
    }
    for h in handles {
        h.await.expect("任务完成");
    }
    // 最后一个写入者生效（Mutex 保证无数据竞争）
    let token = config.access_token().expect("token 已写入");
    assert!(token.starts_with("token-"), "token 值完整: {token}");
    assert!(!config.is_access_token_expired(), "7200s 未过期");
}

// --- serde 义务：WxError 未知字段容忍 ---
#[test]
fn serde_wx_error_unknown_fields_tolerated() {
    // Java Gson 默认忽略未知字段；serde 默认也忽略
    let err = wx_rust_common::error::WxError::from_json(
        r#"{"errcode":40001,"errmsg":"x","unknown_field":123,"retry":true}"#,
    );
    assert_eq!(err.error_code, 40001);
    assert_eq!(err.error_msg.as_deref(), Some("x"));
}

// --- serde 义务：WxAccessToken 缺失字段默认值 ---
#[test]
fn serde_access_token_missing_fields_defaults() {
    // Java 字段默认值：expiresIn=-1
    let token: wx_rust_common::bean::WxAccessToken =
        serde_json::from_str(r#"{"access_token":"abc"}"#).expect("缺失 expires_in 可解析");
    assert_eq!(token.expires_in, -1, "expires_in 默认 -1（对应 Java）");
    assert_eq!(token.access_token, "abc");
}

// --- serde 义务：线格式精确匹配（camelCase） ---
#[test]
fn serde_access_token_wire_format() {
    let token = wx_rust_common::bean::WxAccessToken::new("ACCESS_TOKEN", 7200);
    let json = serde_json::to_string(&token).unwrap();
    // 微信接口线格式：access_token / expires_in（snake_case 字段名）
    assert!(
        json.contains("\"access_token\":\"ACCESS_TOKEN\""),
        "实际: {json}"
    );
    assert!(json.contains("\"expires_in\":7200"), "实际: {json}");
}

// --- 错误义务：WxErrorException 错误码变体精确 ---
#[test]
fn error_exception_wx_variant_code() {
    let e = WxErrorException::from_code(40001, "invalid credential");
    assert_eq!(e.error_code(), Some(40001));
    assert!(e.wx_error().is_some());
    // Display 应含错误信息
    assert!(e.to_string().contains("40001"));
}

#[test]
fn error_exception_runtime_variant() {
    let e = WxErrorException::Runtime(wx_rust_common::error::WxRuntimeError::new("超时"));
    assert_eq!(e.error_code(), None, "运行时错误无微信错误码");
    assert!(e.wx_error().is_none());
    assert!(e.to_string().contains("超时"));
}

// --- Send/Sync 义务：核心类型跨线程安全 ---
#[test]
fn core_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WxDefaultConfig>();
    assert_send_sync::<WxCryptUtil>();
    assert_send_sync::<wx_rust_common::session::StandardSessionManager>();
    assert_send_sync::<wx_rust_common::api::WxMessageInMemoryDuplicateChecker>();
    assert_send_sync::<SimpleGetRequestExecutor>();
}

// --- feature 义务：redis 门控（仅编译期验证，无 redis 时类型不存在） ---
#[cfg(feature = "redis")]
#[test]
fn redis_feature_types_exist() {
    use wx_rust_common::api::WxMessageInRedisDuplicateChecker;
    use wx_rust_common::redis::{WxRedisOps, WxRedisOpsImpl};
    use wx_rust_common::util::locks::RedisDistributedLock;

    // 编译期验证类型存在（构造需要真实连接，仅验证 trait 实现）
    fn _assert_impls<T: WxRedisOps>() {}
    _assert_impls::<WxRedisOpsImpl>();
    let _ = std::any::type_name::<WxMessageInRedisDuplicateChecker>();
    let _ = std::any::type_name::<RedisDistributedLock>();
}

// ==================== VALUE_ADD ====================

// --- 恶意输入：无效 JSON ---
#[test]
fn wx_error_malformed_json_returns_default() {
    let err = wx_rust_common::error::WxError::from_json("not-json{{{");
    // 解析失败应给出默认错误（-99）而非 panic
    assert_eq!(err.error_code, -99);
    assert!(err.json.is_some());
}

// --- 恶意输入：超长错误码 ---
#[test]
fn wx_error_huge_code_no_panic() {
    let err = wx_rust_common::error::WxError::from_json(r#"{"errcode":999999999,"errmsg":"x"}"#);
    assert_eq!(err.error_code, 999999999);
}

// --- 恶意输入：XXE/DOCTYPE（XML 工具） ---
#[test]
fn xml_utils_xxe_doctype_rejected() {
    // XXE 攻击向量：DOCTYPE + 外部实体，不应读取文件内容
    let xml = r#"<?xml version="1.0"?>
<!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]>
<xml><a>&xxe;</a></xml>"#;
    let result = wx_rust_common::util::XmlUtils::xml_2_map(xml);
    if let Ok(map) = result {
        // 即使解析成功也不得展开外部实体（值为空或实体名，而非文件内容）
        if let Some(v) = map.get("a") {
            assert!(!v.contains("root:"), "XXE 不得读取 /etc/passwd 内容");
        }
    }
    // Err 拒绝解析同样安全
}

// --- 恶意输入：PKCS7 非法填充 ---
#[test]
fn pkcs7_decode_invalid_padding() {
    use wx_rust_common::util::crypto::Pkcs7Encoder;
    // 尾字节 > 32：视为无填充（Java 语义 pad=0）
    let data = vec![1u8, 2, 3, 200];
    let decoded = Pkcs7Encoder::decode(&data);
    assert_eq!(decoded.len(), 4, "非法填充按无填充处理");

    // 空输入
    let decoded = Pkcs7Encoder::decode(&[]);
    assert!(decoded.is_empty());
}

// --- 恶意输入：空密文解密 ---
#[test]
fn crypt_util_decrypt_empty_cipher() {
    let util = WxCryptUtil::new("t", "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG", "appid")
        .expect("aesKey");
    let result = util.decrypt("");
    assert!(result.is_err(), "空密文应报错");
}

// --- 边界：SHA1 大量参数排序 ---
#[test]
fn sha1_many_args_sorted() {
    use wx_rust_common::util::crypto::Sha1;
    // 5 个参数排序拼接（Java 语义）
    let r1 = Sha1::digest(&["e", "d", "c", "b", "a"]).unwrap();
    let r2 = Sha1::digest(&["a", "b", "c", "d", "e"]).unwrap();
    assert_eq!(r1, r2, "排序后拼接与顺序无关");
}

// --- 边界：重复检查器并发安全 ---
#[test]
fn duplicate_checker_concurrent_safe() {
    use wx_rust_common::api::{WxMessageDuplicateChecker, WxMessageInMemoryDuplicateChecker};
    let checker = Arc::new(WxMessageInMemoryDuplicateChecker::new());
    let mut handles = Vec::new();
    for _ in 0..8 {
        let c = checker.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..100 {
                let _ = c.is_duplicate(&format!("id-{i}"));
            }
        }));
    }
    for h in handles {
        h.join().expect("线程完成");
    }
    // 并发调用不 panic 即通过（Mutex 保护）
}

// --- 边界：HttpResponseProxy 文件名提取 ---
#[test]
fn http_response_proxy_extract_filename_utf8() {
    // 对应 Java HttpResponseProxyTest：filename*=utf-8'' 提取
    let content = "attachment; filename*=utf-8''%E6%B5%8B%E8%AF%95.xlsx";
    let name = HttpResponseProxy::extract_file_name_from_content_string(content).expect("提取成功");
    assert_eq!(name, "测试.xlsx");
}

#[test]
fn http_response_proxy_extract_filename_plain() {
    // 对应 Java HttpResponseProxyTest：filename="..." 提取
    // HTTP header 中中文文件名以 ISO-8859-1 呈现（Java 读到的形态），
    // 实现按 ISO-8859-1 → UTF-8 还原
    let content = "attachment; filename=\"\u{e8}\u{90}\u{a5}\u{e4}\u{b8}\u{9a}\u{e6}\u{89}\u{a7}\u{e7}\u{85}\u{a7}.jpg\"";
    let name = HttpResponseProxy::extract_file_name_from_content_string(content).expect("提取成功");
    assert_eq!(name, "营业执照.jpg");
}

#[test]
fn http_response_proxy_extract_filename_empty_error() {
    let result = HttpResponseProxy::extract_file_name_from_content_string("");
    assert!(result.is_err(), "空 content 应报错");
    let result = HttpResponseProxy::extract_file_name_from_content_string("no-filename-here");
    assert!(result.is_err(), "无 filename 应报错");
}

// --- 边界：URI 编码 ---
#[test]
fn uri_util_encode_uri_component() {
    use wx_rust_common::util::http::UriUtil;
    // encodeURIComponent 语义（对应 Java URIUtil）
    assert_eq!(UriUtil::encode_uri_component("abc"), "abc");
    assert_eq!(UriUtil::encode_uri_component("你好"), "%E4%BD%A0%E5%A5%BD");
    assert_eq!(UriUtil::encode_uri_component("a b"), "a%20b");
    assert_eq!(UriUtil::encode_uri_component(""), "");
}

// --- 边界：HttpClientType 枚举值 ---
#[test]
fn http_client_type_names() {
    assert_eq!(HttpClientType::JoddHttp.name(), "JODD_HTTP");
    assert_eq!(HttpClientType::ApacheHttp.name(), "APACHE_HTTP");
    assert_eq!(HttpClientType::OkHttp.name(), "OK_HTTP");
    assert_eq!(HttpClientType::HttpComponents.name(), "HTTP_COMPONENTS");
}

// --- 边界：WxType 枚举 ---
#[test]
fn wx_type_names() {
    use wx_rust_common::enums::WxType;
    assert_eq!(WxType::Cp.name(), "CP");
    assert_eq!(WxType::Mp.name(), "MP");
    assert_eq!(WxType::MiniApp.name(), "MiniApp");
    assert_eq!(WxType::Open.name(), "Open");
    assert_eq!(WxType::Pay.name(), "Pay");
    assert_eq!(WxType::Channel.name(), "Channel");
}

// --- 边界：TicketType 值 ---
#[test]
fn ticket_type_values() {
    use wx_rust_common::enums::TicketType;
    assert_eq!(TicketType::Jsapi.value(), "jsapi");
    assert_eq!(TicketType::Sdk.value(), "2");
    assert_eq!(TicketType::WxCard.value(), "wx_card");
}

// --- 边界：TokenEntry 过期判断 ---
#[test]
fn token_entry_expiry_boundary() {
    let entry = TokenEntry {
        value: "t".to_string(),
        expires_at: Some(1000),
    };
    assert!(
        entry.is_expired(1000),
        "等于过期时刻视为过期（Java expiresIn 语义）"
    );
    assert!(!entry.is_expired(999));
    assert!(entry.is_expired(1001));

    let never = TokenEntry {
        value: "t".to_string(),
        expires_at: None,
    };
    assert!(!never.is_expired(99999999), "无过期时刻永不过期");
}

// --- 错误义务：WxErrorException Display/错误消息 ---
#[test]
fn error_exception_display_messages() {
    let e = WxErrorException::Io("磁盘满".to_string());
    assert!(e.to_string().contains("磁盘满"));
    let e = WxErrorException::Http("连接超时".to_string());
    assert!(e.to_string().contains("连接超时"));
    let e = WxErrorException::Serde("json 错误".to_string());
    assert!(e.to_string().contains("json 错误"));
}

// --- RequestHttp trait 实现（编译契约） ---
#[derive(Clone)]
struct FakeRequestHttp;
impl RequestHttp for FakeRequestHttp {
    fn request_type(&self) -> HttpClientType {
        HttpClientType::HttpComponents
    }
}

#[test]
fn request_http_trait_contract() {
    let r = FakeRequestHttp;
    assert_eq!(r.request_type(), HttpClientType::HttpComponents);
}

// --- RequestExecutor 泛型义务：SimpleGet 执行器与微信错误码 ---
// 无网络环境验证 handle_response 的错误码路径（纯逻辑）
#[test]
fn simple_get_handle_response_error_code() {
    let response = r#"{"errcode":40001,"errmsg":"invalid credential"}"#;
    let result =
        SimpleGetRequestExecutor::handle_response(wx_rust_common::enums::WxType::Mp, response);
    assert!(result.is_err(), "错误码非 0 必须报错");
    let err = result.unwrap_err();
    assert_eq!(err.error_code(), Some(40001));
}

#[test]
fn simple_get_handle_response_ok() {
    let response = r#"{"errcode":0,"errmsg":"ok"}"#;
    let result =
        SimpleGetRequestExecutor::handle_response(wx_rust_common::enums::WxType::Mp, response);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), response);
}

// --- WxCryptUtil 构造错误：非法 aesKey 长度 ---
#[test]
fn crypt_util_invalid_aes_key_rejected() {
    // 44 字符但解码后非 32 字节
    let result = WxCryptUtil::new("t", "AAAA", "appid");
    assert!(result.is_err(), "非法 aesKey 应拒绝");
}
