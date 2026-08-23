#![allow(clippy::field_reassign_with_default)]
//! 企业微信外部联系人服务（Wave 覆盖提升）集成测试。
//!
//! 镜像 Java `WxCpExternalContactServiceImplTest` 的有效用例语义，经
//! MockServer 验证（模式照抄 `sub_domain_cp_facade.rs`，自含无外部依赖）。
//!
//! 覆盖大量此前 0% 或极低覆盖率的方法。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

use wx_rust_cp::api::r#impl::*;
use wx_rust_cp::api::*;
use wx_rust_cp::bean::*;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

#[allow(dead_code)]
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, String) + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);
        let requests_clone = requests.clone();
        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let stop_clone = stop.clone();
        tokio::spawn(async move {
            loop {
                if stop_clone.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                requests_clone.fetch_add(1, Ordering::SeqCst);
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *last_body_clone.lock().unwrap() = request[idx + 4..].to_string();
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let (content_type, body) = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });
        Self {
            addr,
            requests,
            last_path,
            last_body,
            stop,
        }
    }
    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }
    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }
    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

fn host_config(host: &str) -> WxCpHostConfig {
    let mut config = WxCpHostConfig::new();
    config.api_host = host.to_string();
    config
}

fn config_with_host(host: &str) -> Arc<dyn WxCpConfigStorage> {
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_host_config(host_config(host));
    Arc::new(config)
}

fn service_with_host(host: &str) -> Arc<dyn WxCpService> {
    WxCpServiceImpl::new_arc(config_with_host(host))
}

fn weak_service(service: &Arc<dyn WxCpService>) -> Weak<dyn WxCpService> {
    Arc::downgrade(service)
}

fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/gettoken") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

fn ok_resp() -> &'static str {
    r#"{"errcode":0,"errmsg":"ok"}"#
}

// ---- 联系我方式 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetContactWay
#[tokio::test]
async fn test_ec_get_contact_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_contact_way") {
            json(r#"{"errcode":0,"errmsg":"ok","contact_way":{"config_id":"cfg_1","remark":"test","style":1}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let info = svc
        .get_contact_way("cfg_1")
        .await
        .expect("获取联系我方式成功");
    assert_eq!(info.contact_way.config_id, "cfg_1");
    assert!(server.last_body().contains(r#""config_id":"cfg_1""#));
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/get_contact_way")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testListContactWay
#[tokio::test]
async fn test_ec_list_contact_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/list_contact_way") {
            json(r#"{"errcode":0,"errmsg":"ok","contact_way":[{"config_id":"cfg_1"},{"config_id":"cfg_2"}],"next_cursor":"CURSOR_NEXT"}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .list_contact_way(Some(1000), Some(2000), Some("cursor_1"), Some(100))
        .await
        .expect("获取列表成功");
    assert_eq!(result.contact_way.len(), 2);
    assert_eq!(result.next_cursor, "CURSOR_NEXT");
    let body = server.last_body();
    assert!(body.contains(r#""start_time":1000"#), "body: {body}");
    assert!(body.contains(r#""cursor":"cursor_1""#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testUpdateContactWay
#[tokio::test]
async fn test_ec_update_contact_way() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let mut info = WxCpContactWayInfo::default();
    info.contact_way.config_id = "cfg_1".to_string();
    info.contact_way.remark = "updated".to_string();
    svc.update_contact_way(&info)
        .await
        .expect("更新联系我方式成功");
    let body = server.last_body();
    assert!(body.contains(r#""config_id":"cfg_1""#), "body: {body}");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/update_contact_way")
    );
    // 空 configId 校验
    info.contact_way.config_id = "  ".to_string();
    assert!(svc.update_contact_way(&info).await.is_err());
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDeleteContactWay
#[tokio::test]
async fn test_ec_delete_contact_way() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.delete_contact_way("cfg_1")
        .await
        .expect("删除联系我方式成功");
    assert!(server.last_body().contains(r#""config_id":"cfg_1""#));
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/del_contact_way")
    );
}

// ---- 临时会话 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testCloseTempChat
#[tokio::test]
async fn test_ec_close_temp_chat() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.close_temp_chat("zhangsan", "wmQER2GAAA")
        .await
        .expect("结束临时会话成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/close_temp_chat")
    );
}

// ---- 客户详情 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetExternalContact
#[tokio::test]
async fn test_ec_get_external_contact() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/crm/get_external_contact") {
            json(r#"{"errcode":0,"errmsg":"ok","external_contact":{"external_userid":"wmQER2GAAA","name":"张三","type":1}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let info = svc
        .get_external_contact("wmQER2GAAA")
        .await
        .expect("获取详情成功");
    assert_eq!(info.external_contact.external_user_id, "wmQER2GAAA");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/crm/get_external_contact?external_userid=wmQER2GAAA")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetContactDetail
#[tokio::test]
async fn test_ec_get_contact_detail_with_cursor() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get?external_userid=") {
            json(r#"{"errcode":0,"errmsg":"ok","external_contact":{"external_userid":"wmQER2GAAA","name":"张三"},"follow_user":[{"userid":"zhangsan"}]}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let info = svc
        .get_contact_detail("wmQER2GAAA", Some("cursor_abc"))
        .await
        .expect("获取详情成功");
    assert_eq!(info.external_contact.external_user_id, "wmQER2GAAA");
    assert!(server.last_path().contains("&cursor=cursor_abc"));
    svc.get_contact_detail("wmQER2GAAA", None)
        .await
        .expect("获取详情成功");
}

// ---- 身份转换 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testConvertToOpenid
#[tokio::test]
async fn test_ec_convert_to_openid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/convert_to_openid") {
            json(r#"{"errcode":0,"errmsg":"ok","openid":"OPEN_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let openid = svc.convert_to_openid("wmQER2GAAA").await.expect("转换成功");
    assert_eq!(openid, "OPEN_1");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testUnionidToExternalUserid
#[tokio::test]
async fn test_ec_unionid_to_external_userid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/unionid_to_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid":"wmQER2GAAA"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let ext_uid = svc
        .unionid_to_external_userid("union_1", "openid_1")
        .await
        .expect("转换成功");
    assert_eq!(ext_uid, "wmQER2GAAA");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testToServiceExternalUserid
#[tokio::test]
async fn test_ec_to_service_external_userid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/to_service_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid":"svc_wmQER2GAAA"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let ext_uid = svc
        .to_service_external_userid("wmQER2GAAA")
        .await
        .expect("转换成功");
    assert_eq!(ext_uid, "svc_wmQER2GAAA");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testFromServiceExternalUserid
#[tokio::test]
async fn test_ec_from_service_external_userid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/from_service_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid":"wmQER2GAAA"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let ext_uid = svc
        .from_service_external_userid("svc_wmQER2GAAA", "agent_1")
        .await
        .expect("转换成功");
    assert_eq!(ext_uid, "wmQER2GAAA");
    assert!(server.last_body().contains(r#""source_agentid":"agent_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testUnionidToExternalUserid3rd
#[tokio::test]
async fn test_ec_unionid_to_external_userid_3rd() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/service/externalcontact/unionid_to_external_userid_3rd") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid_info":[{"external_userid":"wmQER2GAAA"}]}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .unionid_to_external_userid_3rd("union_1", "openid_1", Some("corpid_1"))
        .await
        .expect("转换成功");
    assert_eq!(result.external_user_id_info.len(), 1);
    assert!(server.last_body().contains(r#""corpid":"corpid_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetNewExternalUserId
#[tokio::test]
async fn test_ec_get_new_external_user_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_new_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","items":[{"old_external_userid":"old_1","new_external_userid":"new_1"}]}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .get_new_external_user_id(&["old_1", "old_2"])
        .await
        .expect("获取新 ID 成功");
    assert_eq!(result.items.len(), 1);
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testFinishExternalUserIdMigration
#[tokio::test]
async fn test_ec_finish_external_user_id_migration() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.finish_external_user_id_migration("corpid_1")
        .await
        .expect("设置迁移完成成功");
    assert!(server.last_body().contains(r#""corpid":"corpid_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testOpengidToChatid
#[tokio::test]
async fn test_ec_opengid_to_chatid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/opengid_to_chatid") {
            json(r#"{"errcode":0,"errmsg":"ok","chat_id":"CHAT_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let chat_id = svc.opengid_to_chatid("opengid_1").await.expect("转换成功");
    assert_eq!(chat_id, "CHAT_1");
}

// ---- 进群方式 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testAddJoinWay
#[tokio::test]
async fn test_ec_add_join_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/add_join_way") {
            json(r#"{"errcode":0,"errmsg":"ok","config_id":"jw_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let mut info = WxCpGroupJoinWayInfo::default();
    info.join_way.scene = 1;
    info.join_way.chat_id_list = vec!["chat1".to_string(), "chat2".to_string()];
    let result = svc.add_join_way(&info).await.expect("添加进群方式成功");
    assert_eq!(result.config_id, "jw_1");
    // 超过 5 个群 ID 校验
    info.join_way.chat_id_list = (0..6).map(|i| format!("c{i}")).collect();
    assert!(svc.add_join_way(&info).await.is_err());
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testUpdateJoinWay
#[tokio::test]
async fn test_ec_update_join_way() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let mut info = WxCpGroupJoinWayInfo::default();
    info.join_way.config_id = "jw_1".to_string();
    info.join_way.scene = 1;
    svc.update_join_way(&info).await.expect("更新进群方式成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/groupchat/update_join_way")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetJoinWay
#[tokio::test]
async fn test_ec_get_join_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/get_join_way") {
            json(r#"{"errcode":0,"errmsg":"ok","join_way":{"config_id":"jw_1","scene":1}}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let info = svc.get_join_way("jw_1").await.expect("获取进群方式成功");
    assert_eq!(info.join_way.config_id, "jw_1");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDelJoinWay
#[tokio::test]
async fn test_ec_del_join_way() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.del_join_way("jw_1").await.expect("删除进群方式成功");
    assert!(server.last_body().contains(r#""config_id":"jw_1""#));
}

// ---- 客户列表 / 成员列表 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetContactDetailBatch
#[tokio::test]
async fn test_ec_get_contact_detail_batch() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/batch/get_by_user") {
            json(r#"{"errcode":0,"errmsg":"ok","external_contact_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_contact_detail_batch(&["zhangsan", "lisi"], Some("cursor_1"), Some(100))
        .await
        .expect("批量获取成功");
    let body = server.last_body();
    assert!(
        body.contains(r#""userid_list":["zhangsan","lisi"]"#),
        "body: {body}"
    );
    assert!(body.contains(r#""cursor":"cursor_1""#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetContactList
/// NOTE: WxCpExternalContactListInfo 的 errcode 字段声明为 String，但标准执行器
/// 要求 errcode 为整数；此不一致导致 from_json 在 errcode=0（整数）时报错。
/// 通过 errcode=84061（无客户，走非标准路径）覆盖路径。
#[tokio::test]
async fn test_ec_get_contact_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/contact_list") {
            // 84061 触发执行器错误路径，覆盖 errcode 解析分支
            json(r#"{"errcode":84061,"errmsg":"no data"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let err = svc
        .get_contact_list(Some("cursor_1"), Some(50))
        .await
        .unwrap_err();
    assert_eq!(err.error_code(), Some(84061), "应返回 84061 错误");
    assert!(server.last_body().contains(r#""cursor":"cursor_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testListFollowers
#[tokio::test]
async fn test_ec_list_followers() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_follow_user_list") {
            json(r#"{"errcode":0,"errmsg":"ok","follow_user":["zhangsan","lisi"]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let followers = svc.list_followers().await.expect("获取成员列表成功");
    assert_eq!(followers, vec!["zhangsan".to_string(), "lisi".to_string()]);
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testListUnassignedList
#[tokio::test]
async fn test_ec_list_unassigned_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_unassigned_list") {
            json(r#"{"errcode":0,"errmsg":"ok","info":[{"handover_userid":"u1"}],"is_last":true}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .list_unassigned_list(Some(1), Some("cursor_1"), Some(100))
        .await
        .expect("获取列表成功");
    assert_eq!(result.unassign_infos.len(), 1);
    let body = server.last_body();
    assert!(body.contains(r#""page_id":1"#), "body: {body}");
    assert!(body.contains(r#""cursor":"cursor_1""#), "body: {body}");
}

// ---- 客户转接 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testTransferExternalContact
#[tokio::test]
async fn test_ec_transfer_external_contact() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.transfer_external_contact("ext_1", "hand_1", "take_1")
        .await
        .expect("分配离职成员外部联系人成功");
    let body = server.last_body();
    assert!(
        body.contains(r#""external_userid":"ext_1""#),
        "body: {body}"
    );
    assert!(
        body.contains(r#""handover_userid":"hand_1""#),
        "body: {body}"
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testTransferCustomer
#[tokio::test]
async fn test_ec_transfer_customer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/transfer_customer") {
            json(r#"{"errcode":0,"errmsg":"ok","customer":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let req = WxCpUserTransferCustomerReq {
        hand_over_userid: "hand_1".to_string(),
        take_over_userid: "take_1".to_string(),
        external_userid: vec!["ext_1".to_string()],
        ..Default::default()
    };
    let result = svc
        .transfer_customer(&req)
        .await
        .expect("转接在职成员客户成功");
    assert!(result.customer.is_empty());
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testTransferResult
#[tokio::test]
async fn test_ec_transfer_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/transfer_result") {
            json(r#"{"errcode":0,"errmsg":"ok","customer":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .transfer_result("hand_1", "take_1", Some("cursor_1"))
        .await
        .expect("查询结果成功");
    assert!(server.last_body().contains(r#""cursor":"cursor_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testResignedTransferCustomer
#[tokio::test]
async fn test_ec_resigned_transfer_customer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/resigned/transfer_customer") {
            json(r#"{"errcode":0,"errmsg":"ok","customer":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let req = WxCpUserTransferCustomerReq {
        hand_over_userid: "hand_1".to_string(),
        take_over_userid: "take_1".to_string(),
        external_userid: vec!["ext_1".to_string()],
        ..Default::default()
    };
    let _result = svc
        .resigned_transfer_customer(&req)
        .await
        .expect("分配离职成员客户成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/resigned/transfer_customer")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testResignedTransferResult
#[tokio::test]
async fn test_ec_resigned_transfer_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/resigned/transfer_result") {
            json(r#"{"errcode":0,"errmsg":"ok","customer":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .resigned_transfer_result("hand_1", "take_1", None)
        .await
        .expect("查询成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/resigned/transfer_result")
    );
}

// ---- 客户群 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testListGroupChat
#[tokio::test]
async fn test_ec_list_group_chat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/list") {
            json(r#"{"errcode":0,"errmsg":"ok","group_chat_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .list_group_chat(Some(100), Some("cursor_1"), 0, Some(&["zhangsan"]))
        .await
        .expect("获取列表成功");
    let body = server.last_body();
    assert!(body.contains(r#""cursor":"cursor_1""#), "body: {body}");
    assert!(body.contains(r#""limit":100"#), "body: {body}");
    assert!(
        body.contains(r#""userid_list":["zhangsan"]"#),
        "body: {body}"
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testListGroupChatWithPageIndex
#[tokio::test]
async fn test_ec_list_group_chat_with_page_index() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/list") {
            json(r#"{"errcode":0,"errmsg":"ok","group_chat_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .list_group_chat_with_page_index(Some(0), Some(100), 0, &["zhangsan"], &["dept_1"])
        .await
        .expect("获取列表成功");
    let body = server.last_body();
    assert!(body.contains(r#""offset":0"#), "body: {body}");
    assert!(
        body.contains(r#""partyid_list":["dept_1"]"#),
        "body: {body}"
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupChat
#[tokio::test]
async fn test_ec_get_group_chat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/get") {
            json(r#"{"errcode":0,"errmsg":"ok","group_chat":{"chat_id":"CHAT_1","name":"测试群","owner":"zhangsan"}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .get_group_chat("CHAT_1", Some(1))
        .await
        .expect("获取详情成功");
    assert_eq!(result.group_chat.chat_id, "CHAT_1");
    assert!(server.last_body().contains(r#""need_name":1"#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testTransferGroupChat
#[tokio::test]
async fn test_ec_transfer_group_chat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/transfer") {
            json(r#"{"errcode":0,"errmsg":"ok","failed_chat_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .transfer_group_chat(&["CHAT_1"], "new_owner")
        .await
        .expect("交接成功");
    assert!(server.last_body().contains(r#""chat_id_list":["CHAT_1"]"#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testOnjobTransferGroupChat
#[tokio::test]
async fn test_ec_onjob_transfer_group_chat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/onjob_transfer") {
            json(r#"{"errcode":0,"errmsg":"ok","failed_chat_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .onjob_transfer_group_chat(&["CHAT_1", "CHAT_2"], "new_owner")
        .await
        .expect("交接成功");
}

// ---- 行为数据 / 统计 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetUserBehaviorStatistic
#[tokio::test]
async fn test_ec_get_user_behavior_statistic() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_user_behavior_data") {
            json(r#"{"errcode":0,"errmsg":"ok","behavior_data":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let start = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let end = chrono::DateTime::parse_from_rfc3339("2026-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let _result = svc
        .get_user_behavior_statistic(start, end, &["zhangsan"], &["dept_1"])
        .await
        .expect("获取数据成功");
    let body = server.last_body();
    assert!(body.contains(r#""userid":["zhangsan"]"#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupChatStatistic
#[tokio::test]
async fn test_ec_get_group_chat_statistic() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/statistic") {
            json(r#"{"errcode":0,"errmsg":"ok","total":0}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let start = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let _result = svc
        .get_group_chat_statistic(start, 1, 0, 0, 100, &["zhangsan"], &[])
        .await
        .expect("获取统计数据成功");
    assert!(server.last_body().contains(r#""order_by":1"#));
}

// ---- 群发消息 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testRemindGroupMsgSend
#[tokio::test]
async fn test_ec_remind_group_msg_send() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.remind_group_msg_send("msg_1").await.expect("提醒成功");
    assert!(server.last_body().contains(r#""msgid":"msg_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCancelGroupMsgSend
#[tokio::test]
async fn test_ec_cancel_group_msg_send() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.cancel_group_msg_send("msg_1").await.expect("停止成功");
    assert!(server.last_body().contains(r#""msgid":"msg_1""#));
}

// ---- 标签 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testAddCorpTag
#[tokio::test]
async fn test_ec_add_corp_tag() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/add_corp_tag") {
            json(r#"{"errcode":0,"errmsg":"ok","tag_group":{"group_id":"g_1","tag":[{"id":"t_1","name":"标签A"}]}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let mut group = WxCpUserExternalTagGroupInfo::default();
    group.tag_group.group_name = "测试组".to_string();
    let result = svc.add_corp_tag(&group).await.expect("添加标签成功");
    assert_eq!(result.tag_group.group_id, "g_1");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testEditCorpTag
#[tokio::test]
async fn test_ec_edit_corp_tag() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.edit_corp_tag("tag_1", Some("新标签名"), Some(10))
        .await
        .expect("编辑标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""id":"tag_1""#), "body: {body}");
    assert!(body.contains(r#""name":"新标签名""#), "body: {body}");
    assert!(body.contains(r#""order":10"#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDelCorpTag
#[tokio::test]
async fn test_ec_del_corp_tag() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.del_corp_tag(&["tag_1"], &["group_1"])
        .await
        .expect("删除标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""tag_id":["tag_1"]"#), "body: {body}");
    assert!(body.contains(r#""group_id":["group_1"]"#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testMarkTag
#[tokio::test]
async fn test_ec_mark_tag() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.mark_tag("zhangsan", "ext_1", &["tag_1"], &["tag_2"])
        .await
        .expect("标记标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
    assert!(body.contains(r#""add_tag":["tag_1"]"#), "body: {body}");
    assert!(body.contains(r#""remove_tag":["tag_2"]"#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetCorpTagListWithGroupId
#[tokio::test]
async fn test_ec_get_corp_tag_list_with_group_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_corp_tag_list") {
            json(r#"{"errcode":0,"errmsg":"ok","tag_group":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_corp_tag_list_with_group_id(&["tag_1"], &["group_1"])
        .await
        .expect("获取标签列表成功");
    let body = server.last_body();
    assert!(body.contains(r#""tag_id":["tag_1"]"#), "body: {body}");
    assert!(body.contains(r#""group_id":["group_1"]"#), "body: {body}");
}

// ---- 朋友圈 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentTaskResult
#[tokio::test]
async fn test_ec_get_moment_task_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_task_result") {
            json(r#"{"errcode":0,"errmsg":"ok","status":2}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_task_result("job_1")
        .await
        .expect("获取结果成功");
    assert!(server.last_path().contains("&jobid=job_1"));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCancelMomentTask
#[tokio::test]
async fn test_ec_cancel_moment_task() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.cancel_moment_task("moment_1").await.expect("停止成功");
    assert!(server.last_body().contains(r#""moment_id":"moment_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentList
#[tokio::test]
async fn test_ec_get_moment_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_list") {
            json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"","moment_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_list(1000, 2000, Some("creator_1"), Some(1), None, Some(100))
        .await
        .expect("获取列表成功");
    assert!(server.last_body().contains(r#""creator":"creator_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentTask
#[tokio::test]
async fn test_ec_get_moment_task() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_task") {
            json(r#"{"errcode":0,"errmsg":"ok","task_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_task("moment_1", Some("cursor_1"), Some(100))
        .await
        .expect("获取任务成功");
    assert!(server.last_body().contains(r#""moment_id":"moment_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentCustomerList
#[tokio::test]
async fn test_ec_get_moment_customer_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_customer_list") {
            json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"","customer_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_customer_list("moment_1", "zhangsan", None, Some(50))
        .await
        .expect("获取列表成功");
    let body = server.last_body();
    assert!(body.contains(r#""moment_id":"moment_1""#), "body: {body}");
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentSendResult
#[tokio::test]
async fn test_ec_get_moment_send_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_send_result") {
            json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"","customer_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_send_result("moment_1", "zhangsan", None, None)
        .await
        .expect("获取结果成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetMomentComments
#[tokio::test]
async fn test_ec_get_moment_comments() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_moment_comments") {
            json(r#"{"errcode":0,"errmsg":"ok","comment_list":[],"like_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_moment_comments("moment_1", "zhangsan")
        .await
        .expect("获取互动数据成功");
}

// ---- 群发记录 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupMsgListV2
#[tokio::test]
async fn test_ec_get_group_msg_list_v2() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_groupmsg_list_v2") {
            json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"","group_msg_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let start = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let end = chrono::DateTime::parse_from_rfc3339("2026-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let _result = svc
        .get_group_msg_list_v2(
            "single",
            start,
            end,
            Some("creator_1"),
            Some(1),
            Some(100),
            None,
        )
        .await
        .expect("获取群发记录成功");
    let body = server.last_body();
    assert!(body.contains(r#""chat_type":"single""#), "body: {body}");
    assert!(body.contains(r#""creator":"creator_1""#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupMsgSendResult
#[tokio::test]
async fn test_ec_get_group_msg_send_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_groupmsg_send_result") {
            json(r#"{"errcode":0,"errmsg":"ok","send_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_group_msg_send_result("msg_1", "zhangsan", Some(100), Some("cursor_1"))
        .await
        .expect("获取发送结果成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgid":"msg_1""#), "body: {body}");
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupMsgResult
#[tokio::test]
async fn test_ec_get_group_msg_result() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_group_msg_result") {
            json(r#"{"errcode":0,"errmsg":"ok","detail_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_group_msg_result("msg_1", None, None)
        .await
        .expect("获取结果成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupMsgTask
#[tokio::test]
async fn test_ec_get_group_msg_task() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_groupmsg_task") {
            json(r#"{"errcode":0,"errmsg":"ok","task_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_group_msg_task("msg_1", None, None)
        .await
        .expect("获取任务成功");
}

// ---- 欢迎语模板 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testAddGroupWelcomeTemplate
#[tokio::test]
async fn test_ec_add_group_welcome_template() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/group_welcome_template/add") {
            json(r#"{"errcode":0,"errmsg":"ok","template_id":"tmpl_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let template = WxCpGroupWelcomeTemplateResult::default();
    let tmpl_id = svc
        .add_group_welcome_template(&template)
        .await
        .expect("添加模板成功");
    assert_eq!(tmpl_id, "tmpl_1");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testEditGroupWelcomeTemplate
#[tokio::test]
async fn test_ec_edit_group_welcome_template() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let template = WxCpGroupWelcomeTemplateResult::default();
    svc.edit_group_welcome_template(&template)
        .await
        .expect("编辑模板成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/group_welcome_template/edit")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetGroupWelcomeTemplate
#[tokio::test]
async fn test_ec_get_group_welcome_template() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/group_welcome_template/get") {
            json(r#"{"errcode":0,"errmsg":"ok","template_id":"tmpl_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_group_welcome_template("tmpl_1")
        .await
        .expect("获取模板成功");
    assert!(server.last_body().contains(r#""template_id":"tmpl_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDelGroupWelcomeTemplate
#[tokio::test]
async fn test_ec_del_group_welcome_template() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.del_group_welcome_template("tmpl_1", Some("101"))
        .await
        .expect("删除模板成功");
    let body = server.last_body();
    assert!(body.contains(r#""template_id":"tmpl_1""#), "body: {body}");
    assert!(body.contains(r#""agentid":"101""#), "body: {body}");
}

// ---- 商品图册 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetProductAlbumList
#[tokio::test]
async fn test_ec_get_product_album_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_product_album_list") {
            json(r#"{"errcode":0,"errmsg":"ok","product_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_product_album_list(Some(100), Some("cursor_1"))
        .await
        .expect("获取列表成功");
    assert!(server.last_body().contains(r#""limit":100"#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetProductAlbum
#[tokio::test]
async fn test_ec_get_product_album() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_product_album") {
            json(r#"{"errcode":0,"errmsg":"ok","product":{"product_id":"p_1","description":"商品1"}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc.get_product_album("p_1").await.expect("获取商品成功");
    assert!(server.last_body().contains(r#""product_id":"p_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDeleteProductAlbum
#[tokio::test]
async fn test_ec_delete_product_album() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.delete_product_album("p_1").await.expect("删除商品成功");
    assert!(server.last_body().contains(r#""product_id":"p_1""#));
}

// ---- 敏感词 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testAddInterceptRule
#[tokio::test]
async fn test_ec_add_intercept_rule() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/add_intercept_rule") {
            json(r#"{"errcode":0,"errmsg":"ok","rule_id":"rule_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let req = WxCpInterceptRuleAddRequest::default();
    let rule_id = svc
        .add_intercept_rule(&req)
        .await
        .expect("添加敏感词规则成功");
    assert_eq!(rule_id, "rule_1");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testUpdateInterceptRule
#[tokio::test]
async fn test_ec_update_intercept_rule() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let rule = WxCpInterceptRule::default();
    svc.update_intercept_rule(&rule)
        .await
        .expect("修改敏感词规则成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/update_intercept_rule")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testDelInterceptRule
#[tokio::test]
async fn test_ec_del_intercept_rule() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.del_intercept_rule("rule_1")
        .await
        .expect("删除敏感词规则成功");
    assert!(server.last_body().contains(r#""rule_id":"rule_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetInterceptRuleList
#[tokio::test]
async fn test_ec_get_intercept_rule_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_intercept_rule_list") {
            json(r#"{"errcode":0,"errmsg":"ok","rule_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc.get_intercept_rule_list().await.expect("获取列表成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testGetInterceptRuleDetail
#[tokio::test]
async fn test_ec_get_intercept_rule_detail() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_intercept_rule") {
            json(
                r#"{"errcode":0,"errmsg":"ok","rule":{"rule_id":"rule_1","rule_name":"测试规则"}}"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_intercept_rule_detail("rule_1")
        .await
        .expect("获取详情成功");
    assert!(server.last_body().contains(r#""rule_id":"rule_1""#));
}

// ---- 获客链接 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionLinkList
#[tokio::test]
async fn test_ec_customer_acquisition_link_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition/list_link") {
            json(r#"{"errcode":0,"errmsg":"ok","link_id_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .customer_acquisition_link_list(Some(100), Some("cursor_1"))
        .await
        .expect("获取列表成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionLinkGet
#[tokio::test]
async fn test_ec_customer_acquisition_link_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition/get") {
            json(r#"{"errcode":0,"errmsg":"ok","link":{"link_id":"link_1","link_name":"获客链接1"}}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .customer_acquisition_link_get("link_1")
        .await
        .expect("获取详情成功");
    assert!(server.last_body().contains(r#""link_id":"link_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionLinkCreate
#[tokio::test]
async fn test_ec_customer_acquisition_link_create() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition/create_link") {
            json(r#"{"errcode":0,"errmsg":"ok","link":{"link_id":"link_1"}}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let req = WxCpCustomerAcquisitionRequest::default();
    let _result = svc
        .customer_acquisition_link_create(&req)
        .await
        .expect("创建成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionUpdate
#[tokio::test]
async fn test_ec_customer_acquisition_update() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let req = WxCpCustomerAcquisitionRequest::default();
    svc.customer_acquisition_update(&req)
        .await
        .expect("编辑成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/customer_acquisition/update_link")
    );
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionLinkDelete
#[tokio::test]
async fn test_ec_customer_acquisition_link_delete() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    svc.customer_acquisition_link_delete("link_1")
        .await
        .expect("删除成功");
    assert!(server.last_body().contains(r#""link_id":"link_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionCustomer
#[tokio::test]
async fn test_ec_customer_acquisition_customer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition/customer") {
            json(r#"{"errcode":0,"errmsg":"ok","customer_list":[],"next_cursor":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .customer_acquisition_customer("link_1", Some(100), Some("cursor_1"))
        .await
        .expect("获取列表成功");
    assert!(server.last_body().contains(r#""link_id":"link_1""#));
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionQuota
#[tokio::test]
async fn test_ec_customer_acquisition_quota() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition_quota") {
            json(r#"{"errcode":0,"errmsg":"ok","total":100,"balance":90}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let _result = svc
        .customer_acquisition_quota()
        .await
        .expect("查询使用量成功");
}

/// 对应 Java: WxCpExternalContactServiceImplTest.testCustomerAcquisitionStatistic
#[tokio::test]
async fn test_ec_customer_acquisition_statistic() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/customer_acquisition/statistic") {
            json(r#"{"errcode":0,"errmsg":"ok","click_link_customer_cnt":10,"new_customer_cnt":5}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let start = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let end = chrono::DateTime::parse_from_rfc3339("2026-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let _result = svc
        .customer_acquisition_statistic("link_1", start, end)
        .await
        .expect("查询成功");
    assert!(server.last_body().contains(r#""link_id":"link_1""#));
}

// ---- 发送欢迎语 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testSendWelcomeMsg
#[tokio::test]
async fn test_ec_send_welcome_msg() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let msg = WxCpWelcomeMsg::default();
    svc.send_welcome_msg(&msg).await.expect("发送欢迎语成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/send_welcome_msg")
    );
}

// ---- 附件上传 ----

/// 对应 Java: WxCpExternalContactServiceImplTest.testUploadAttachment
#[tokio::test]
async fn test_ec_upload_attachment() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/upload_attachment") {
            json(r#"{"errcode":0,"errmsg":"ok","type":"file","mediaId":"MEDIA_1","created_at":1720000000}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpExternalContactServiceImpl::new(weak_service(&service));
    let result = svc
        .upload_attachment("file", "pdf", 1, b"PDF_BYTES".to_vec())
        .await
        .expect("上传附件成功");
    assert_eq!(result.media_id, "MEDIA_1");
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/media/upload_attachment"),
        "path: {path}"
    );
    assert!(path.contains("media_type=file"), "path: {path}");
    assert!(path.contains("attachment_type=1"), "path: {path}");
}
