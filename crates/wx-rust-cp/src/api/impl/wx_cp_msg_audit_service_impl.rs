//! 会话内容存档服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpMsgAuditServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `cpService`）。
//!
//! 语义镜像要点：
//! - HTTP 类接口（`getPermitUserList`/`getGroupChat`/`checkSingleAgree`）
//!   走会话存档专用 access_token 通道 `postForMsgAudit`（对应 Java
//!   `cpService.postForMsgAudit`）；
//! - 解密类接口（`getDecryptChatData`/`getChatRecordPlainText` 及
//!   `@Deprecated` 的 SDK 句柄版）以 C2b 的
//!   `util::crypto::decrypt_chat_data` 纯实现替代官方 native SDK 的
//!   `Finance.DecryptData`（ADAPTED：Java 两段式 RSA 解密
//!   `encrypt_random_key` → AES 解密 `encrypt_chat_msg`，sdk 句柄参数
//!   不再参与解密）；
//! - 原生 SDK 拉取类接口（`getChatDatas`/`getChatRecords`/
//!   `getMediaFile`/`downloadMediaFile` 及回调版）依赖 `com.tencent.
//!   wework.Finance` native 库的私有网络协议，Rust 无对应实现，返回
//!   `-99 未实现`（PLATFORM_NA：官方 native SDK 私有协议——与 Java
//!   非纯 HTTP 对应，见各方法标注）；
//! - `closeThreadLocalSdk`/`closeAllSdks` 无原生句柄可释放，空实现。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpMsgAuditService, WxCpService};
use crate::bean::{
    WxCpAgreeInfo, WxCpChatData, WxCpChatDatas, WxCpChatModel, WxCpCheckAgreeRequest, WxCpGroupChat,
};
use crate::enums::url_msg_audit;
use crate::util::crypto::decrypt_chat_data;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 会话内容存档服务实现。
pub struct WxCpMsgAuditServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpMsgAuditServiceImpl {
    /// 构建会话存档服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 会话存档消息解密（对应 Java `WxCpMsgAuditServiceImpl.decryptChatData`
    /// 的 RSA + AES 两段式；官方 `Finance.DecryptData` 以纯实现替代，
    /// ADAPTED）。
    fn decrypt_chat_data_impl(
        &self,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        // 企业获取的会话内容，使用企业自行配置的消息加密公钥进行加密，
        // 企业可用自行保存的私钥解开会话内容数据（Java 同注释语义）。
        // msgAuditPriKey 会话存档私钥不能为空
        let pri_key = svc
            .wx_cp_config_storage()
            .msg_audit_pri_key()
            .unwrap_or_default();
        if pri_key.trim().is_empty() {
            return Err(WxErrorException::from_code(
                -99,
                "请配置会话存档私钥【msgAuditPriKey】",
            ));
        }
        decrypt_chat_data(
            &chat_data.encrypt_random_key,
            &chat_data.encrypt_chat_msg,
            &pri_key,
            Some(pkcs1),
        )
        .map_err(|e| WxErrorException::from_code(-99, e))
    }

    /// 拉取聊天记录（SDK 句柄版，对应 Java `getChatDatas`，`@Deprecated`）。
    ///
    /// 依赖官方 native SDK `Finance.GetChatData` 的私有网络协议，Rust
    /// 无对应实现（PLATFORM_NA：官方 native SDK 私有协议——与 Java
    /// 非纯 HTTP 对应，返回 -99）。
    async fn get_chat_datas_unsupported(
        &self,
        _seq: i64,
        _limit: i64,
        _proxy: Option<&str>,
        _passwd: Option<&str>,
        _timeout: i64,
    ) -> Result<WxCpChatDatas, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getChatDatas 依赖官方 native SDK（Finance.GetChatData），Rust 未实现",
        ))
    }

    /// 拉取聊天记录（推荐使用，对应 Java `getChatRecords`）。
    ///
    /// 依赖官方 native SDK，Rust 无对应实现（PLATFORM_NA：官方 native
    /// SDK `Finance.GetChatData` 私有协议——与 Java 非纯 HTTP 对应，
    /// 返回 -99）。
    async fn get_chat_records_unsupported(
        &self,
        _seq: i64,
        _limit: i64,
        _proxy: Option<&str>,
        _passwd: Option<&str>,
        _timeout: i64,
    ) -> Result<Vec<WxCpChatData>, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getChatRecords 依赖官方 native SDK（Finance.GetChatData），Rust 未实现",
        ))
    }

    /// 获取媒体文件（SDK 分片拉取，对应 Java `getMediaFile`）。
    ///
    /// 依赖官方 native SDK `Finance.GetMediaData`，Rust 无对应实现
    /// （PLATFORM_NA：官方 native SDK 私有协议——与 Java 非纯 HTTP
    /// 对应，返回 -99）。
    ///
    /// 注意：本方法 body 不引用回调/句柄参数——async_trait 生成的 `Send`
    /// future 无法持有非 `Send` 的 `&mut dyn FnMut`（ADAPTED）。
    async fn get_media_file_unsupported(
        &self,
        _sdkfileid: &str,
        _proxy: Option<&str>,
        _passwd: Option<&str>,
        _timeout: i64,
    ) -> Result<(), WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "getMediaFile 依赖官方 native SDK（Finance.GetMediaData），Rust 未实现",
        ))
    }
}

#[async_trait]
impl WxCpMsgAuditService for WxCpMsgAuditServiceImpl {
    /// 拉取聊天记录（SDK 句柄版，对应 Java `getChatDatas`，`@Deprecated`；
    /// PLATFORM_NA：依赖官方 native SDK `Finance.GetChatData` 私有协议，
    /// Rust 无对应实现——与 Java 非纯 HTTP 对应，返回 -99）。
    async fn get_chat_datas(
        &self,
        seq: i64,
        limit: i64,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
    ) -> Result<WxCpChatDatas, WxErrorException> {
        self.get_chat_datas_unsupported(seq, limit, proxy, passwd, timeout)
            .await
    }

    /// 拉取聊天记录（推荐使用，对应 Java `getChatRecords`；PLATFORM_NA：
    /// 依赖官方 native SDK `Finance.GetChatData` 私有协议，Rust 无对应
    /// 实现——与 Java 非纯 HTTP 对应，返回 -99）。
    async fn get_chat_records(
        &self,
        seq: i64,
        limit: i64,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
    ) -> Result<Vec<WxCpChatData>, WxErrorException> {
        self.get_chat_records_unsupported(seq, limit, proxy, passwd, timeout)
            .await
    }

    /// 获取解密的聊天数据 Model（SDK 句柄版，对应 Java `getDecryptData`，
    /// `@Deprecated`；解密以纯实现替代，ADAPTED）。
    async fn get_decrypt_data(
        &self,
        _sdk: i64,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<WxCpChatModel, WxErrorException> {
        let plain_text = self.decrypt_chat_data_impl(chat_data, pkcs1)?;
        WxCpChatModel::from_json(&plain_text).map_err(WxErrorException::Serde)
    }

    /// 获取解密的聊天数据 Model（推荐使用，对应 Java `getDecryptChatData`）。
    async fn get_decrypt_chat_data(
        &self,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<WxCpChatModel, WxErrorException> {
        let plain_text = self.decrypt_chat_data_impl(chat_data, pkcs1)?;
        WxCpChatModel::from_json(&plain_text).map_err(WxErrorException::Serde)
    }

    /// 获取解密的聊天数据明文（SDK 句柄版，对应 Java `getChatPlainText`，
    /// `@Deprecated`；解密以纯实现替代，ADAPTED）。
    async fn get_chat_plain_text(
        &self,
        _sdk: i64,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<String, WxErrorException> {
        self.decrypt_chat_data_impl(chat_data, pkcs1)
    }

    /// 获取解密的聊天数据明文（推荐使用，对应 Java `getChatRecordPlainText`）。
    async fn get_chat_record_plain_text(
        &self,
        chat_data: &WxCpChatData,
        pkcs1: i32,
    ) -> Result<String, WxErrorException> {
        self.decrypt_chat_data_impl(chat_data, pkcs1)
    }

    /// 获取媒体文件（SDK 句柄版，写入目标文件，对应 Java `getMediaFile`，
    /// `@Deprecated`；PLATFORM_NA：依赖官方 native SDK `Finance.GetMediaData`
    /// 私有协议，Rust 无对应实现——与 Java 非纯 HTTP 对应，返回 -99）。
    async fn get_media_file(
        &self,
        _sdk: i64,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        _target_file_path: &str,
    ) -> Result<(), WxErrorException> {
        self.get_media_file_unsupported(sdkfileid, proxy, passwd, timeout)
            .await
    }

    /// 获取媒体文件（推荐使用，写入目标文件，对应 Java `downloadMediaFile`；
    /// PLATFORM_NA：依赖官方 native SDK `Finance.GetMediaData` 私有协议，
    /// Rust 无对应实现——与 Java 非纯 HTTP 对应，返回 -99）。
    async fn download_media_file(
        &self,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        _target_file_path: &str,
    ) -> Result<(), WxErrorException> {
        self.get_media_file_unsupported(sdkfileid, proxy, passwd, timeout)
            .await
    }

    /// 获取媒体文件（SDK 句柄版 + 分片回调，对应 Java `getMediaFile`，
    /// `@Deprecated`；PLATFORM_NA：依赖官方 native SDK `Finance.GetMediaData`
    /// 私有协议，Rust 无对应实现——与 Java 非纯 HTTP 对应，返回 -99）。
    ///
    /// 注意：body 不引用回调参数（async_trait `Send` future 约束，ADAPTED）。
    async fn get_media_file_with_callback(
        &self,
        _sdk: i64,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        _action: &mut dyn FnMut(&[u8]),
    ) -> Result<(), WxErrorException> {
        self.get_media_file_unsupported(sdkfileid, proxy, passwd, timeout)
            .await
    }

    /// 获取媒体文件（推荐使用 + 分片回调，对应 Java `downloadMediaFile`；
    /// PLATFORM_NA：依赖官方 native SDK `Finance.GetMediaData` 私有协议，
    /// Rust 无对应实现——与 Java 非纯 HTTP 对应，返回 -99）。
    ///
    /// 注意：body 不引用回调参数（async_trait `Send` future 约束，ADAPTED）。
    async fn download_media_file_with_callback(
        &self,
        sdkfileid: &str,
        proxy: Option<&str>,
        passwd: Option<&str>,
        timeout: i64,
        _action: &mut dyn FnMut(&[u8]),
    ) -> Result<(), WxErrorException> {
        self.get_media_file_unsupported(sdkfileid, proxy, passwd, timeout)
            .await
    }

    /// 获取会话内容存档开启成员列表（对应 Java `getPermitUserList`）。
    async fn get_permit_user_list(
        &self,
        r#type: Option<i32>,
    ) -> Result<Vec<String>, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if let Some(t) = r#type {
            obj.insert("type".to_string(), serde_json::Value::from(t));
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_msg_audit::GET_PERMIT_USER_LIST);
        // Java：postForMsgAudit——会话存档专用 access_token 通道
        let response = svc.post_for_msg_audit(&url, &map_to_string(&obj)).await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let arr = json
            .get("ids")
            .and_then(|v| v.as_array())
            .ok_or_else(|| WxErrorException::from_code(-99, "ids 字段缺失"))?;
        arr.iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| WxErrorException::from_code(-99, "ids 含非字符串元素"))
            })
            .collect()
    }

    /// 获取会话内容存档内部群信息（对应 Java `getGroupChat`）。
    async fn get_group_chat(&self, roomid: &str) -> Result<WxCpGroupChat, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "roomid".to_string(),
            serde_json::Value::String(roomid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_msg_audit::GET_GROUP_CHAT);
        let response = svc.post_for_msg_audit(&url, &map_to_string(&obj)).await?;
        WxCpGroupChat::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取会话同意情况（单聊，对应 Java `checkSingleAgree`）。
    async fn check_single_agree(
        &self,
        check_agree_request: &WxCpCheckAgreeRequest,
    ) -> Result<WxCpAgreeInfo, WxErrorException> {
        let svc = self.service()?;
        let body = check_agree_request
            .to_json()
            .map_err(WxErrorException::Serde)?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_msg_audit::CHECK_SINGLE_AGREE);
        let response = svc.post_for_msg_audit(&url, &body).await?;
        WxCpAgreeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 关闭当前线程持有的 SDK，释放本地资源（对应 Java
    /// `closeThreadLocalSdk`；Rust 无原生 SDK 句柄，空实现，ADAPTED）。
    fn close_thread_local_sdk(&self) {}

    /// 关闭所有会话存档 SDK 实例，释放全部原生资源（对应 Java
    /// `closeAllSdks`；Rust 无原生 SDK 句柄，空实现，ADAPTED）。
    fn close_all_sdks(&self) {}
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：会话存档 HTTP 通道（msg audit 专用 token）与解密全流程
    //! 往返（RSA PKCS1 加密随机密钥 → AES-256-CBC 解出明文）。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };
    use crate::config::r#impl::WxCpDefaultConfig;

    /// 镜像 Java `testGetPermitUserList`：会话存档专用 access_token 通道
    /// （postForMsgAudit），响应解析 ids 数组。
    #[tokio::test]
    async fn test_msg_audit_get_permit_user_list() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/msgaudit/get_permit_user_list") {
                json(r#"{"errcode":0,"errmsg":"ok","ids":["zhangsan","lisi"]}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpMsgAuditServiceImpl::new(weak_service(&service));

        let ids = svc_impl
            .get_permit_user_list(Some(1))
            .await
            .expect("获取成员列表成功");
        assert_eq!(ids, vec!["zhangsan".to_string(), "lisi".to_string()]);
        // 使用会话存档专用 token（对应 Java postForMsgAudit）
        assert!(
            server.last_path().contains("access_token=MSG_AUDIT_TOKEN"),
            "path: {}",
            server.last_path()
        );
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/msgaudit/get_permit_user_list")
        );
        let body = server.last_body();
        assert!(body.contains(r#""type":1"#), "body: {body}");

        // type 为空时不写入（对应 Java `if (type != null)`）
        let ids = svc_impl
            .get_permit_user_list(None)
            .await
            .expect("获取成员列表成功");
        assert_eq!(ids.len(), 2);
        assert!(
            !server.last_body().contains("type"),
            "body: {}",
            server.last_body()
        );
    }

    /// 解密全流程往返（对应 Java `testGetDecryptChatData`/`decryptChatData`
    /// 语义）：生成 RSA 密钥对 → 公钥 PKCS1 加密随机 AES 密钥 →
    /// AES-256-CBC + PKCS7 加密明文 → `getChatRecordPlainText`/
    /// `getDecryptChatData` 解出原文与 Model。
    #[tokio::test]
    async fn test_msg_audit_decrypt_roundtrip() {
        use base64::Engine as _;
        use rand_core::RngCore;
        use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
        use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

        // 生成 RSA-2048 密钥对（对应企业自行保存的会话存档私钥）
        let mut rng = rand_core::OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("生成 RSA 密钥");
        let public_key = RsaPublicKey::from(&private_key);
        let pem = private_key
            .to_pkcs1_pem(LineEnding::LF)
            .expect("私钥转 PKCS1 PEM");

        // 随机 32 字节 AES 密钥
        let mut aes_key = [0u8; 32];
        rng.fill_bytes(&mut aes_key);

        // RSA 加密 AES 密钥 → encrypt_random_key（base64）
        let encrypted_key = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, &aes_key)
            .expect("RSA 加密 AES 密钥");
        let encrypt_random_key = base64::engine::general_purpose::STANDARD.encode(&encrypted_key);

        // 解密侧 `decrypt_pri_key` 以 `String::from_utf8_lossy` 输出 RSA
        // 明文（非法序列替换），AES 密钥取自该字符串字节——加密侧需严格
        // 镜像同一转换（镜像 decrypt_encrypt_chat_msg 语义）
        let aes_key_str = String::from_utf8_lossy(&aes_key).into_owned();
        let key_bytes = aes_key_str.as_bytes();

        // AES-256-CBC + PKCS7 加密明文（镜像 decrypt_encrypt_chat_msg 的
        // 密钥/IV 语义：密钥前 16 字节为 IV）
        let plaintext = r#"{"msgid":"msg_audit_1","action":"send","from":"zhangsan","msgtype":"text","text":{"content":"会话存档解密测试"}}"#;
        use aes::Aes256;
        use cbc::cipher::block_padding::Pkcs7;
        use cbc::cipher::{BlockModeEncrypt, KeyIvInit};
        type Aes256CbcEnc = cbc::Encryptor<Aes256>;
        let cipher =
            Aes256CbcEnc::new_from_slices(&key_bytes[..32], &key_bytes[..16]).expect("AES 初始化");
        let mut buf = vec![0u8; plaintext.len() + 16];
        let encrypted = cipher
            .encrypt_padded_b2b::<Pkcs7>(plaintext.as_bytes(), &mut buf)
            .expect("AES 加密");
        let encrypt_chat_msg = base64::engine::general_purpose::STANDARD.encode(encrypted);

        // 配置会话存档私钥（PKCS1）
        let mut config = WxCpDefaultConfig::new("corpid", "secret");
        config.set_msg_audit_pri_key(pem.as_str());
        let service = crate::api::r#impl::WxCpServiceImpl::new_arc(std::sync::Arc::new(config));
        let svc_impl = WxCpMsgAuditServiceImpl::new(weak_service(&service));

        let chat_data = WxCpChatData {
            seq: 1,
            msg_id: "msg_audit_1".to_string(),
            publickey_ver: 2,
            encrypt_random_key,
            encrypt_chat_msg,
        };

        // 明文往返（pkcs1=1 走 PKCS1 解密）
        let plain = svc_impl
            .get_chat_record_plain_text(&chat_data, 1)
            .await
            .expect("解密明文成功");
        assert_eq!(plain, plaintext);

        // Model 解析往返
        let model = svc_impl
            .get_decrypt_chat_data(&chat_data, 1)
            .await
            .expect("解密 Model 成功");
        assert_eq!(model.msg_id, "msg_audit_1");

        // 未配置私钥 → 报错（对应 Java「请配置会话存档私钥【msgAuditPriKey】」）
        let config2 = WxCpDefaultConfig::new("corpid", "secret");
        let service2 = crate::api::r#impl::WxCpServiceImpl::new_arc(std::sync::Arc::new(config2));
        let svc_impl2 = WxCpMsgAuditServiceImpl::new(weak_service(&service2));
        assert!(
            svc_impl2
                .get_chat_record_plain_text(&chat_data, 1)
                .await
                .is_err()
        );
    }

    /// 镜像 Java `testCheckSingleAgree`：会话存档专用通道查询会话同意情况，
    /// 响应解析 agree_info。
    #[tokio::test]
    async fn test_msg_audit_check_single_agree() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/msgaudit/check_single_agree") {
                json(r#"{"errcode":0,"errmsg":"ok","agreeinfo":[{"status_change_time":1600000000,"userid":"zhangsan"}]}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpMsgAuditServiceImpl::new(weak_service(&service));

        let mut request = WxCpCheckAgreeRequest::default();
        request.info = vec![crate::bean::msgaudit::wx_cp_check_agree_request::Info {
            userid: "zhangsan".to_string(),
            exteranal_open_id: "wmQER2GAAA".to_string(),
        }];
        let agree = svc_impl
            .check_single_agree(&request)
            .await
            .expect("查询同意情况成功");
        assert_eq!(agree.agree_info.len(), 1);
        assert_eq!(agree.agree_info[0].userid, "zhangsan");
        assert!(
            server.last_path().contains("access_token=MSG_AUDIT_TOKEN"),
            "path: {}",
            server.last_path()
        );
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/msgaudit/check_single_agree")
        );
    }
}
