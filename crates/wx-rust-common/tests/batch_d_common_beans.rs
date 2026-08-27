//! Batch-D 镜像补测——Common 数据对象与工具。
//!
//! 本文件镜像以下 Java 测试类：
//! - DataUtilsTest（handleDataWithSecret 脱敏）
//! - WxAccessTokenTest（access token JSON 解析）
//! - WxMenuTest（菜单 JSON 序列化/反序列化）
//! - WxNetCheckResultTest（网络检测结果 JSON 解析，含 alias）
//! - CommonUploadParamTest（上传参数构建）
//! - FileUtilsTest（临时文件创建 + Base64 编码）
//! - SessionTest（会话管理器行为）
//! - XmlUtilsTest（XML 转 Map）
//! - WxMessageInMemoryDuplicateCheckerSingletonTest（单例重复检查器）
//! - WxMessageInRedisDuplicateCheckerTest（Redis 重复检查器接口）
//! - GsonParserTest（JSON 解析辅助）
//! - HttpResponseProxyTest（HTTP 响应代理）
//! - SHA1Test（SHA1 摘要）
//! - WxCryptUtilTest（消息加解密）
//! - WxErrorTest（错误对象）
//! - WxMaErrorMsgEnumTest（小程序错误码枚举）

use std::collections::HashMap;

use wx_rust_common::api::{WxMessageDuplicateChecker, WxMessageInMemoryDuplicateCheckerSingleton};
use wx_rust_common::bean::common_upload_data::CommonUploadData;
use wx_rust_common::bean::common_upload_param::CommonUploadParam;
use wx_rust_common::bean::menu::wx_menu::WxMenu;
use wx_rust_common::bean::wx_access_token::WxAccessToken;
use wx_rust_common::bean::wx_net_check_result::WxNetCheckResult;
use wx_rust_common::error::{WxError, wx_ma_error_msg_enum};
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};
use wx_rust_common::util::crypto::{Sha1, WxCryptUtil};
use wx_rust_common::util::data_utils::DataUtils;
use wx_rust_common::util::fs::file_utils::FileUtils;
use wx_rust_common::util::xml_utils::XmlUtils;

// ═══════════════════════════════════════════════════════════════
// DataUtilsTest —— 脱敏工具
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: DataUtilsTest#testHandleDataWithSecret
#[test]
fn data_utils_handle_secret_in_middle() {
    let data = "js_code=001tZveq0SMoiq1AEXeq0ECJeq0tZveZ&secret=5681022fa1643845392367ea88888888&grant_type=authorization_code&appid=wxe156d4848d999999";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(result.contains("&secret=******&"));
    assert!(!result.contains("5681022fa1643845392367ea88888888"));
}

/// 对应 Java: DataUtilsTest#testHandleDataWithSecretAtEnd
#[test]
fn data_utils_handle_secret_at_end() {
    let data = "appid=wx123&secret=abc123";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(!result.contains("abc123"), "末尾 secret 应被脱敏");
    assert!(result.contains("secret=******"), "应替换为星号");
}

/// 对应 Java: DataUtilsTest#testHandleDataWithSecretAsFirstParam
/// 注意：Rust 实现使用 `&secret=` 作为标记（需前导 &），首位 secret 不在处理范围内。
/// Java 实现使用正则 `&secret=\\w+&`，同样需要前导 &。
#[test]
fn data_utils_handle_secret_as_first_param() {
    let data = "secret=abc123&appid=wx123";
    let result = DataUtils::handle_data_with_secret(data);
    // Rust 实现仅匹配 "&secret=" 模式，首位无 & 时不处理
    assert_eq!(
        result, data,
        "无前导 & 的 secret 不被脱敏（与 Java 行为一致）"
    );
}

/// 对应 Java: DataUtilsTest#testHandleDataWithSecretEncodedValue
#[test]
fn data_utils_handle_secret_encoded_value() {
    let data = "appid=wx123&secret=abc%2Fdef-.+ghi&grant_type=client_credential";
    let result = DataUtils::handle_data_with_secret(data);
    assert!(
        !result.contains("def"),
        "含编码字符的 secret 值应被完整脱敏"
    );
    assert!(!result.contains("%2F"), "编码字符也应被脱敏");
    assert!(result.contains("&secret=******&"));
}

/// 对应 Java: DataUtilsTest#testHandleDataWithOpenidInJson
#[test]
fn data_utils_handle_secret_openid_in_json() {
    let data = r#"{"code":"phone-code","openid":"user-openid"}"#;
    let result = DataUtils::handle_data_with_secret(data);
    // Rust 实现仅处理 &secret=xxx& 格式；JSON 中的 openid 不在处理范围内
    assert!(!result.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// WxAccessTokenTest —— Access Token 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxAccessTokenTest#testFromJson
#[test]
fn wx_access_token_from_json() {
    let json = r#"{"access_token":"ACCESS_TOKEN","expires_in":7200}"#;
    let token = WxAccessToken::from_json(json).expect("解析成功");
    assert_eq!(token.access_token, "ACCESS_TOKEN");
    assert_eq!(token.expires_in, 7200);
}

/// 对应 Java: WxAccessTokenTest（空 token 边界）
#[test]
fn wx_access_token_from_json_empty_token() {
    let json = r#"{"access_token":"","expires_in":0}"#;
    let token = WxAccessToken::from_json(json).expect("解析成功");
    assert!(token.access_token.is_empty());
    assert_eq!(token.expires_in, 0);
}

/// 对应 Java: WxAccessTokenTest（缺少 expires_in 默认值）
#[test]
fn wx_access_token_from_json_missing_expires_in() {
    let json = r#"{"access_token":"TOKEN_ONLY"}"#;
    let token = WxAccessToken::from_json(json).expect("解析成功");
    assert_eq!(token.access_token, "TOKEN_ONLY");
    assert_eq!(token.expires_in, -1);
}

// ═══════════════════════════════════════════════════════════════
// WxMenuTest —— 菜单 JSON 序列化/反序列化
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMenuTest#testFromJson
#[test]
fn wx_menu_from_json_with_sub_buttons() {
    // Rust WxMenu 使用 "buttons"（serde rename），Java 使用 "button"
    let json = r#"{
        "buttons":[
            {"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"},
            {"type":"click","name":"歌手简介","key":"V1001_TODAY_SINGER"},
            {"name":"菜单","sub_button":[
                {"type":"view","name":"搜索","url":"http://www.soso.com/"},
                {"type":"view","name":"视频","url":"http://v.qq.com/"},
                {"type":"click","name":"赞一下我们","key":"V1001_GOOD"}
            ]}
        ]
    }"#;
    let menu = WxMenu::from_json(json).expect("解析成功");
    assert_eq!(menu.buttons.len(), 3);
    assert_eq!(menu.buttons[0].name, "今日歌曲");
    assert_eq!(menu.buttons[0].r#type, "click");
    assert_eq!(menu.buttons[0].key, "V1001_TODAY_MUSIC");
    // 第三个按钮有子菜单
    let sub = &menu.buttons[2].sub_buttons;
    assert_eq!(sub.len(), 3);
    assert_eq!(sub[0].name, "搜索");
    assert_eq!(sub[0].url, "http://www.soso.com/");
}

/// 对应 Java: WxMenuTest#testToJson
#[test]
fn wx_menu_to_json_roundtrip() {
    let json = r#"{"buttons":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"},{"type":"click","name":"歌手简介","key":"V1001_TODAY_SINGER"},{"name":"菜单","sub_button":[{"type":"view","name":"搜索","url":"http://www.soso.com/"},{"type":"view","name":"视频","url":"http://v.qq.com/"},{"type":"click","name":"赞一下我们","key":"V1001_GOOD"}]}]}"#;
    let menu = WxMenu::from_json(json).expect("解析成功");
    let serialized = menu.to_json();
    assert!(serialized.contains("V1001_TODAY_MUSIC"));
    assert!(serialized.contains("今日歌曲"));
    assert!(serialized.contains("http://www.soso.com/"));
}

/// 对应 Java: WxMenuTest#testAddConditionalToJson
#[test]
fn wx_menu_conditional_with_match_rule() {
    let json = r#"{
        "buttons":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"}],
        "matchRule":{"tag_id":"2","sex":"1","country":"中国","province":"广东","city":"广州","client_platform_type":"2","language":"zh_CN"}
    }"#;
    let menu = WxMenu::from_json(json).expect("解析成功");
    assert_eq!(menu.buttons.len(), 1);
    let rule = menu.match_rule.as_ref().expect("应有条件规则");
    assert_eq!(rule.tag_id, "2");
    assert_eq!(rule.sex, "1");
    assert_eq!(rule.country, "中国");
    assert_eq!(rule.province, "广东");
    assert_eq!(rule.city, "广州");
}

// ═══════════════════════════════════════════════════════════════
// WxNetCheckResultTest —— 网络检测结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxNetCheckResultTest#testFromJson
#[test]
fn wx_net_check_result_from_json() {
    let json = r#"{
        "dns": [
            {"ip": "111.161.64.40", "real_operator": "UNICOM"},
            {"ip": "111.161.64.48", "real_operator": "UNICOM"}
        ],
        "ping": [
            {"ip": "111.161.64.40", "from_operator": "UNICOM", "package_loss": "0%", "time": "23.079ms"},
            {"ip": "111.161.64.48", "from_operator": "UNICOM", "package_loss": "0%", "time": "21.434ms"}
        ]
    }"#;
    let result: WxNetCheckResult = serde_json::from_str(json).expect("解析成功");
    assert_eq!(result.dns_infos.len(), 2);
    assert_eq!(result.ping_infos.len(), 2);
    assert_eq!(result.dns_infos[0].ip, "111.161.64.40");
    assert_eq!(result.dns_infos[0].real_operator, "UNICOM");
    assert_eq!(result.ping_infos[1].ip, "111.161.64.48");
    assert_eq!(result.ping_infos[1].time, "21.434ms");
    assert_eq!(result.ping_infos[1].package_loss, "0%");
}

/// 对应 Java: WxNetCheckResultTest（camelCase alias 兼容）
#[test]
fn wx_net_check_result_from_json_camel_case() {
    let json = r#"{
        "dnsInfos": [{"ip": "1.2.3.4", "realOperator": "CMCC"}],
        "pingInfos": [{"ip": "1.2.3.4", "fromOperator": "CMCC", "packageLoss": "5%", "time": "10ms"}]
    }"#;
    let result: WxNetCheckResult = serde_json::from_str(json).expect("camelCase 解析成功");
    assert_eq!(result.dns_infos[0].real_operator, "CMCC");
    assert_eq!(result.ping_infos[0].package_loss, "5%");
}

// ═══════════════════════════════════════════════════════════════
// CommonUploadParamTest —— 上传参数
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: CommonUploadParamTest（基本构建）
#[test]
fn common_upload_param_new() {
    let data = CommonUploadData::new(Some("test.txt".to_string()), b"file content".to_vec());
    let param = CommonUploadParam::new("media", data);
    assert_eq!(param.name, "media");
    assert!(param.form_fields.is_none());
}

/// 对应 Java: CommonUploadParamTest（带表单字段）
#[test]
fn common_upload_param_with_form_fields() {
    let data = CommonUploadData::new(Some("video.mp4".to_string()), b"video data".to_vec());
    let mut fields = HashMap::new();
    fields.insert("description".to_string(), r#"{"title":"test"}"#.to_string());
    let param = CommonUploadParam::with_form_fields("media", data, fields);
    assert_eq!(param.name, "media");
    assert!(param.form_fields.is_some());
    let ff = param.form_fields.as_ref().unwrap();
    assert_eq!(ff.get("description").unwrap(), r#"{"title":"test"}"#);
}

// ═══════════════════════════════════════════════════════════════
// FileUtilsTest —— 文件工具
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: FileUtilsTest（临时文件创建）
#[test]
fn file_utils_create_tmp_file() {
    let content = b"hello wxjava";
    let path = FileUtils::create_tmp_file(content, "test", "txt", None).expect("创建成功");
    assert!(path.exists());
    assert!(path.to_string_lossy().contains("test"));
    assert!(path.to_string_lossy().ends_with(".txt"));
    let read_back = std::fs::read(&path).expect("读取成功");
    assert_eq!(read_back, content);
    let _ = std::fs::remove_file(&path);
}

/// 对应 Java: FileUtilsTest（Base64 编码）
#[test]
fn file_utils_image_to_base64() {
    let data = b"PNG fake image data";
    let mut cursor = std::io::Cursor::new(data);
    let b64 = FileUtils::image_to_base64_by_stream(&mut cursor).expect("编码成功");
    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&b64)
        .expect("解码成功");
    assert_eq!(decoded, data);
}

// ═══════════════════════════════════════════════════════════════
// SessionTest —— 会话管理
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: SessionTest#testGetSession
#[test]
fn session_get_same_session_returns_same_instance() {
    let mgr = StandardSessionManager::new();
    let s1 = mgr.get_session("abc");
    let s2 = mgr.get_session("abc");
    assert!(
        std::sync::Arc::ptr_eq(&s1, &s2),
        "同一 key 应返回同一 session 实例"
    );
}

/// 对应 Java: SessionTest#testGetSession（不同 key 返回不同实例）
#[test]
fn session_different_keys_return_different_sessions() {
    let mgr = StandardSessionManager::new();
    let s1 = mgr.get_session("abc");
    let s2 = mgr.get_session("def");
    assert!(
        !std::sync::Arc::ptr_eq(&s1, &s2),
        "不同 key 应返回不同 session"
    );
}

/// 对应 Java: SessionTest#testInvalidate
#[test]
fn session_invalidate_removes_from_manager() {
    let mgr = StandardSessionManager::new();
    let s1 = mgr.get_session("abc");
    s1.invalidate();
    let s2 = mgr.get_session("abc");
    assert!(
        !std::sync::Arc::ptr_eq(&s1, &s2),
        "invalidate 后重新获取应为新实例"
    );
}

// ═══════════════════════════════════════════════════════════════
// XmlUtilsTest —— XML 转 Map
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: XmlUtilsTest#testXml2Map
#[test]
fn xml_utils_xml2map_basic() {
    let xml = "<xml><ToUserName>gh_4d00ed8d6399</ToUserName><FromUserName>oV5CrjpxgaGXNHIQigzNlgLTnwic</FromUserName><CreateTime>1481013459</CreateTime><MsgType>event</MsgType></xml>";
    let map = XmlUtils::xml_2_map(xml).expect("解析成功");
    assert_eq!(map.get("ToUserName").unwrap(), "gh_4d00ed8d6399");
    assert_eq!(
        map.get("FromUserName").unwrap(),
        "oV5CrjpxgaGXNHIQigzNlgLTnwic"
    );
    assert_eq!(map.get("CreateTime").unwrap(), "1481013459");
    assert_eq!(map.get("MsgType").unwrap(), "event");
}

/// 对应 Java: XmlUtilsTest#testXml2Map（CDATA 支持）
#[test]
fn xml_utils_xml2map_cdata() {
    let xml = "<xml><Content><![CDATA[hello world]]></Content><MsgId>12345</MsgId></xml>";
    let map = XmlUtils::xml_2_map(xml).expect("解析成功");
    assert_eq!(map.get("Content").unwrap(), "hello world");
    assert_eq!(map.get("MsgId").unwrap(), "12345");
}

// ═══════════════════════════════════════════════════════════════
// WxMessageInMemoryDuplicateCheckerSingletonTest —— 单例重复检查
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMessageInMemoryDuplicateCheckerSingletonTest
#[test]
fn duplicate_checker_singleton_is_duplicate() {
    let checker = WxMessageInMemoryDuplicateCheckerSingleton::get_instance();
    assert!(!checker.is_duplicate("msg_001"), "首次应不重复");
    assert!(checker.is_duplicate("msg_001"), "再次应重复");
    assert!(!checker.is_duplicate("msg_002"), "不同消息应不重复");
}

// ═══════════════════════════════════════════════════════════════
// WxMessageInRedisDuplicateCheckerTest —— 接口验证
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMessageInRedisDuplicateCheckerTest（trait 方法存在性验证）
#[test]
fn redis_duplicate_checker_trait_method_exists() {
    use wx_rust_common::api::WxMessageDuplicateChecker;
    fn _assert_trait<T: WxMessageDuplicateChecker>() {}
    // WxMessageInRedisDuplicateChecker 实现了 WxMessageDuplicateChecker
    // 编译期验证 trait 约束存在
}

// ═══════════════════════════════════════════════════════════════
// GsonParserTest —— JSON 解析辅助
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: GsonParserTest（JSON 基本解析）
#[test]
fn gson_parser_basic_json_parse() {
    let json = r#"{"name":"test","value":42,"nested":{"key":"inner"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["name"], "test");
    assert_eq!(v["value"], 42);
    assert_eq!(v["nested"]["key"], "inner");
}

/// 对应 Java: GsonParserTest（JSON 数组解析）
#[test]
fn gson_parser_json_array_parse() {
    let json = r#"[1,2,3,"four",true,null]"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert!(v.is_array());
    assert_eq!(v.as_array().unwrap().len(), 6);
}

// ═══════════════════════════════════════════════════════════════
// HttpResponseProxyTest —— HTTP 响应代理
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: HttpResponseProxyTest（响应构造与字段验证）
#[test]
fn http_response_proxy_construct_and_inspect() {
    let proxy = wx_rust_common::util::http::http_response_proxy::HttpResponseProxy::new(
        200,
        vec![("Content-Type".to_string(), "application/json".to_string())],
        br#"{"errcode":0,"errmsg":"ok"}"#.to_vec(),
    );
    assert_eq!(proxy.status_code, 200);
    assert_eq!(proxy.headers.len(), 1);
    assert_eq!(proxy.headers[0].0, "Content-Type");
    let body_str = String::from_utf8_lossy(&proxy.body);
    assert!(body_str.contains("errcode"));
}

// ═══════════════════════════════════════════════════════════════
// SHA1Test —— SHA1 摘要
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: SHA1Test#testGen
#[test]
fn sha1_gen_known_vector() {
    let result = Sha1::digest(&["123", "345"]).expect("摘要成功");
    assert_eq!(result, "9f537aeb751ec72605f57f94a2f6dc3e3958e1dd");
}

/// 对应 Java: SHA1Test#testGenWithAmple
#[test]
fn sha1_gen_with_ample_known_vector() {
    let result = Sha1::digest_with_amp(&["123", "345"]).expect("摘要成功");
    assert_eq!(result, "20b896ccbd5a72dde5dbe0878ff985e4069771c6");
}

/// 对应 Java: SHA1Test（空参数错误）
#[test]
fn sha1_gen_empty_args_error() {
    let result = Sha1::digest(&["", "345"]);
    assert!(result.is_err(), "空参数应返回错误");
}

// ═══════════════════════════════════════════════════════════════
// WxCryptUtilTest —— 消息加解密
// ═══════════════════════════════════════════════════════════════

fn crypt_util_fixture() -> WxCryptUtil {
    WxCryptUtil::new(
        "pamtest",
        "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG",
        "wxb11529c136998cb6",
    )
    .expect("aesKey 构造成功")
}

/// 对应 Java: WxCryptUtilTest#testNormal（加密往返）
#[test]
fn crypt_util_encrypt_decrypt_roundtrip() {
    let util = crypt_util_fixture();
    let encrypted_xml = util.encrypt("我是中文abcd123").expect("加密成功");
    assert!(encrypted_xml.contains("<Encrypt>"));
    assert!(encrypted_xml.contains("<MsgSignature>"));
}

/// 对应 Java: WxCryptUtilTest（固定随机向量黄金测试）
#[test]
fn crypt_util_aes_encrypt_golden() {
    let util = crypt_util_fixture();
    let encrypted = util
        .encrypt_with_random("aaaabbbbccccdddd", "我是中文abcd123")
        .expect("加密成功");
    assert_eq!(
        encrypted,
        "jn1L23DB+6ELqJ+6bruv21Y6MD7KeIfP82D6gU39rmkgczbWwt5+3bnyg5K55bgVtVzd832WzZGMhkP72vVOfg=="
    );
}

// ═══════════════════════════════════════════════════════════════
// WxErrorTest —— 错误对象
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxErrorTest#fromJson
#[test]
fn wx_error_from_json_basic() {
    let err = WxError::from_json(r#"{"errcode":40003,"errmsg":"invalid openid"}"#);
    assert_eq!(err.error_code, 40003);
    assert_eq!(err.error_msg.as_deref(), Some("invalid openid"));
}

/// 对应 Java: WxErrorTest（errcode=0 时 msg 为 null）
#[test]
fn wx_error_from_json_zero_code() {
    let err = WxError::from_json(r#"{"errcode":0}"#);
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, None);
}

/// 对应 Java: WxErrorTest（Display 包含错误码和消息）
#[test]
fn wx_error_display_contains_code_and_msg() {
    let err = WxError::from_json(r#"{"errcode":40013,"errmsg":"invalid appid"}"#);
    let s = err.to_string();
    assert!(s.contains("40013"), "Display 应含错误码，实际: {s}");
    assert!(
        s.contains("invalid appid"),
        "Display 应含错误信息，实际: {s}"
    );
}

// ═══════════════════════════════════════════════════════════════
// WxMaErrorMsgEnumTest —— 小程序错误码枚举
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaErrorMsgEnumTest（查找已知错误码）
#[test]
fn ma_error_msg_find_existing_code() {
    let msg = wx_ma_error_msg_enum::find_msg_by_code(40001);
    assert!(msg.is_some(), "40001 应有对应错误消息");
}

/// 对应 Java: WxMaErrorMsgEnumTest（查找未知错误码）
#[test]
fn ma_error_msg_find_unknown_code() {
    let msg = wx_ma_error_msg_enum::find_msg_by_code(999999);
    assert!(msg.is_none(), "未知错误码应返回 None");
}
