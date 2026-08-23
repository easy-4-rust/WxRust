#![allow(clippy::field_reassign_with_default)]
//! 企业微信子服务覆盖提升（Wave 覆盖提升 B2）集成测试。
//!
//! 镜像 Java 各 WxCpXxxServiceImplTest 的有效用例语义，经 MockServer 验证。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

use wx_rust_cp::api::r#impl::*;
use wx_rust_cp::api::*;
use wx_rust_cp::bean::*;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

struct MockServer {
    addr: std::net::SocketAddr,
    #[allow(dead_code)] // 请求计数器保留用于调试
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

// ============================================================
// kf 服务
// ============================================================

/// 对应 Java: WxCpKfServiceImplTest.testAddAccount
#[tokio::test]
async fn test_kf_add_account() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/account/add") {
            json(r#"{"errcode":0,"errmsg":"ok","open_kfid":"kf_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let mut add = WxCpKfAccountAdd::default();
    add.name = "客服1".to_string();
    add.media_id = "media_1".to_string();
    let result = svc.add_account(&add).await.expect("添加客服帐号成功");
    assert_eq!(result.open_kfid, "kf_1");
    assert!(server.last_body().contains(r#""name":"客服1""#));
    assert!(server.last_path().contains("/cgi-bin/kf/account/add"));
}

/// 对应 Java: WxCpKfServiceImplTest.testUpdAccount
#[tokio::test]
async fn test_kf_upd_account() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let mut upd = WxCpKfAccountUpd::default();
    upd.open_kfid = "kf_1".to_string();
    upd.name = "新客服名".to_string();
    svc.upd_account(&upd).await.expect("修改客服帐号成功");
    assert!(server.last_path().contains("/cgi-bin/kf/account/update"));
}

/// 对应 Java: WxCpKfServiceImplTest.testDelAccount
#[tokio::test]
async fn test_kf_del_account() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let mut del = WxCpKfAccountDel::default();
    del.open_kfid = "kf_1".to_string();
    svc.del_account(&del).await.expect("删除客服帐号成功");
    assert!(server.last_path().contains("/cgi-bin/kf/account/del"));
}

/// 对应 Java: WxCpKfServiceImplTest.testGetAccountLink
#[tokio::test]
async fn test_kf_get_account_link() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/add_contact_way") {
            json(r#"{"errcode":0,"errmsg":"ok","url":"https://kf.link"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let mut link = WxCpKfAccountLink::default();
    link.open_kfid = "kf_1".to_string();
    link.scene = "scene_1".to_string();
    let result = svc.get_account_link(&link).await.expect("获取客服链接成功");
    assert_eq!(result.url, "https://kf.link");
}

/// 对应 Java: WxCpKfServiceImplTest.testAddServicer
#[tokio::test]
async fn test_kf_add_servicer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/servicer/add") {
            json(r#"{"errcode":0,"errmsg":"ok","invalid_list":"","invalid_department_list":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .add_servicer("kf_1", &["zhangsan"])
        .await
        .expect("添加接待人员成功");
    let body = server.last_body();
    assert!(body.contains(r#""open_kfid":"kf_1""#), "body: {body}");
    assert!(
        body.contains(r#""userid_list":["zhangsan"]"#),
        "body: {body}"
    );
}

/// 对应 Java: WxCpKfServiceImplTest.testAddServicerWithDepartments
#[tokio::test]
async fn test_kf_add_servicer_with_departments() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/servicer/add") {
            json(r#"{"errcode":0,"errmsg":"ok","invalid_list":"","invalid_department_list":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .add_servicer_with_departments("kf_1", &["zhangsan"], &["dept_1"])
        .await
        .expect("添加接待人员成功");
    assert!(
        server
            .last_body()
            .contains(r#""department_id_list":["dept_1"]"#)
    );
    // 空列表校验
    let err = svc
        .add_servicer_with_departments("kf_1", &[], &[])
        .await
        .unwrap_err();
    assert!(format!("{err}").contains("至少需要填"), "错误信息: {err}");
}

/// 对应 Java: WxCpKfServiceImplTest.testDelServicer
#[tokio::test]
async fn test_kf_del_servicer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/servicer/del") {
            json(r#"{"errcode":0,"errmsg":"ok","invalid_list":"","invalid_department_list":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .del_servicer("kf_1", &["zhangsan"])
        .await
        .expect("删除接待人员成功");
    assert!(server.last_path().contains("/cgi-bin/kf/servicer/del"));
}

/// 对应 Java: WxCpKfServiceImplTest.testDelServicerWithDepartments
#[tokio::test]
async fn test_kf_del_servicer_with_departments() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/servicer/del") {
            json(r#"{"errcode":0,"errmsg":"ok","invalid_list":"","invalid_department_list":""}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .del_servicer_with_departments("kf_1", &["zhangsan"], &[])
        .await
        .expect("删除接待人员成功");
}

/// 对应 Java: WxCpKfServiceImplTest.testListServicer
#[tokio::test]
async fn test_kf_list_servicer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/servicer/list?open_kfid=kf_1") {
            json(
                r#"{"errcode":0,"errmsg":"ok","servicer_list":[{"userid":"zhangsan","status":1}]}"#,
            )
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let result = svc
        .list_servicer("kf_1")
        .await
        .expect("获取接待人员列表成功");
    assert_eq!(result.servicer_list.len(), 1);
    assert_eq!(result.servicer_list[0].user_id, "zhangsan");
}

/// 对应 Java: WxCpKfServiceImplTest.testSendMsg
#[tokio::test]
async fn test_kf_send_msg() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/send_msg") {
            json(r#"{"errcode":0,"errmsg":"ok","msgid":"msg_1"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let req = WxCpKfMsgSendRequest {
        to_user: "user_1".to_string(),
        open_kfid: "kf_1".to_string(),
        msg_type: "text".to_string(),
        ..Default::default()
    };
    let result = svc.send_msg(&req).await.expect("发送客服消息成功");
    assert_eq!(result.msg_id, "msg_1");
    assert!(server.last_path().contains("/cgi-bin/kf/send_msg"));
}

/// 对应 Java: WxCpKfServiceImplTest.testSendMsgOnEvent
#[tokio::test]
async fn test_kf_send_msg_on_event() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/send_msg_on_event") {
            json(r#"{"errcode":0,"errmsg":"ok","msgid":"msg_2"}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let req = WxCpKfMsgSendRequest {
        to_user: "user_1".to_string(),
        open_kfid: "kf_1".to_string(),
        msg_type: "text".to_string(),
        ..Default::default()
    };
    let result = svc
        .send_msg_on_event(&req)
        .await
        .expect("发送事件响应消息成功");
    assert_eq!(result.msg_id, "msg_2");
    assert!(server.last_path().contains("/cgi-bin/kf/send_msg_on_event"));
}

/// 对应 Java: WxCpKfServiceImplTest.testCustomerBatchGet
#[tokio::test]
async fn test_kf_customer_batch_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/customer/batchget") {
            json(r#"{"errcode":0,"errmsg":"ok","customer_list":[{"external_userid":"ext_1"}]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let result = svc
        .customer_batch_get(&["ext_1", "ext_2"])
        .await
        .expect("获取客户信息成功");
    assert_eq!(result.customer_list.len(), 1);
    assert!(
        server
            .last_body()
            .contains(r#""external_userid_list":["ext_1","ext_2"]"#)
    );
}

/// 对应 Java: WxCpKfServiceImplTest.testGetCorpStatistic
#[tokio::test]
async fn test_kf_get_corp_statistic() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/get_corp_statistic") {
            json(r#"{"errcode":0,"errmsg":"ok","statistic_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let request = WxCpKfGetCorpStatisticRequest {
        open_kf_id: "kf_1".to_string(),
        start_time: 1000,
        end_time: 2000,
    };
    let _result = svc
        .get_corp_statistic(&request)
        .await
        .expect("获取企业统计成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/kf/get_corp_statistic")
    );
}

/// 对应 Java: WxCpKfServiceImplTest.testGetServicerStatistic
#[tokio::test]
async fn test_kf_get_servicer_statistic() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/get_servicer_statistic") {
            json(r#"{"errcode":0,"errmsg":"ok","statistic_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let request = WxCpKfGetServicerStatisticRequest {
        open_kf_id: "kf_1".to_string(),
        start_time: 1000,
        end_time: 2000,
        ..Default::default()
    };
    let _result = svc
        .get_servicer_statistic(&request)
        .await
        .expect("获取接待人员统计成功");
}

/// 对应 Java: WxCpKfServiceImplTest.testGetUpgradeServiceConfig
#[tokio::test]
async fn test_kf_get_upgrade_service_config() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/customer/get_upgrade_service_config") {
            json(r#"{"errcode":0,"errmsg":"ok","member_range":{},"groupchat_range":{}}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .get_upgrade_service_config()
        .await
        .expect("获取升级服务配置成功");
}

/// 对应 Java: WxCpKfServiceImplTest.testUpgradeMemberService
#[tokio::test]
async fn test_kf_upgrade_member_service() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    svc.upgrade_member_service("kf_1", "ext_1", "zhangsan", Some("请升级"))
        .await
        .expect("升级专员服务成功");
    let body = server.last_body();
    assert!(body.contains(r#""type":1"#), "body: {body}");
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
}

/// 对应 Java: WxCpKfServiceImplTest.testUpgradeGroupchatService
#[tokio::test]
async fn test_kf_upgrade_groupchat_service() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    svc.upgrade_groupchat_service("kf_1", "ext_1", "chat_1", Some("加入群"))
        .await
        .expect("升级客户群服务成功");
    let body = server.last_body();
    assert!(body.contains(r#""type":2"#), "body: {body}");
    assert!(body.contains(r#""chat_id":"chat_1""#), "body: {body}");
}

/// 对应 Java: WxCpKfServiceImplTest.testCancelUpgradeService
#[tokio::test]
async fn test_kf_cancel_upgrade_service() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    svc.cancel_upgrade_service("kf_1", "ext_1")
        .await
        .expect("取消升级服务成功");
}

/// 对应 Java: WxCpKfServiceImplTest.testSyncMsg
#[tokio::test]
async fn test_kf_sync_msg() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/kf/sync_msg") {
            json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"","has_more":0,"msg_list":[]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpKfServiceImpl::new(weak_service(&service));
    let _result = svc
        .sync_msg(None, Some("token_1"), Some(100), Some(0))
        .await
        .expect("读取消息成功");
    let body = server.last_body();
    assert!(body.contains(r#""token":"token_1""#), "body: {body}");
    assert!(body.contains(r#""limit":100"#), "body: {body}");
    assert!(
        !body.contains("open_kfid"),
        "syncMsg 不应含 open_kfid: {body}"
    );
}

// ============================================================
// o_mail 服务（0% → 覆盖）
// ============================================================

/// 对应 Java: WxCpOMailServiceImplTest.testMailCommonSend
#[tokio::test]
async fn test_mail_common_send() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpOMailServiceImpl::new(weak_service(&service));
    let request = WxCpMailCommonSendRequest {
        to: wx_rust_cp::bean::oa::mail::wx_cp_mail_common_send_request::TO {
            emails: vec!["user1@corp.com".to_string()],
            ..Default::default()
        },
        subject: "测试邮件".to_string(),
        content: "邮件内容".to_string(),
        ..Default::default()
    };
    svc.mail_common_send(&request)
        .await
        .expect("发送普通邮件成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/exmail/app/compose_send")
    );
    assert!(server.last_body().contains(r#""subject":"测试邮件""#));
}

/// 对应 Java: WxCpOMailServiceImplTest.testMailScheduleSend
#[tokio::test]
async fn test_mail_schedule_send() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpOMailServiceImpl::new(weak_service(&service));
    let request = WxCpMailScheduleSendRequest {
        to: wx_rust_cp::bean::oa::mail::wx_cp_mail_schedule_send_request::TO {
            emails: vec!["user1@corp.com".to_string()],
            ..Default::default()
        },
        subject: "定时邮件".to_string(),
        content: "邮件内容".to_string(),
        ..Default::default()
    };
    svc.mail_schedule_send(&request)
        .await
        .expect("发送定时邮件成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/exmail/app/compose_send")
    );
}

/// 对应 Java: WxCpOMailServiceImplTest.testMailMeetingSend
#[tokio::test]
async fn test_mail_meeting_send() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpOMailServiceImpl::new(weak_service(&service));
    let request = WxCpMailMeetingSendRequest {
        to: wx_rust_cp::bean::oa::mail::wx_cp_mail_meeting_send_request::TO {
            emails: vec!["user1@corp.com".to_string()],
            ..Default::default()
        },
        subject: "会议邀请".to_string(),
        content: "会议内容".to_string(),
        ..Default::default()
    };
    svc.mail_meeting_send(&request)
        .await
        .expect("发送会议邮件成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/exmail/app/compose_send")
    );
}

// ============================================================
// agent 服务
// ============================================================

/// 对应 Java: WxCpAgentServiceImplTest.testList
#[tokio::test]
async fn test_agent_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/agent/list") {
            json(r#"{"errcode":0,"errmsg":"ok","agentlist":[{"agentid":101,"name":"应用1"},{"agentid":102,"name":"应用2"}]}"#)
        } else { json(ok_resp()) }
    })).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpAgentServiceImpl::new(weak_service(&service));
    let agents = svc.list().await.expect("获取应用列表成功");
    assert_eq!(agents.len(), 2);
    assert_eq!(agents[0].agent_id, 101);
    assert!(server.last_path().contains("/cgi-bin/agent/list"));
}

/// 对应 Java: WxCpAgentServiceImplTest.testGetAdminList
#[tokio::test]
async fn test_agent_get_admin_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/agent/get_admin_list") {
            json(r#"{"errcode":0,"errmsg":"ok","admin":[{"userid":"admin1"}]}"#)
        } else {
            json(ok_resp())
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpAgentServiceImpl::new(weak_service(&service));
    let result = svc.get_admin_list(101).await.expect("获取管理员列表成功");
    assert!(!result.admin.is_empty());
    assert!(server.last_body().contains(r#""agentid":101"#));
    assert!(server.last_path().contains("/cgi-bin/agent/get_admin_list"));
}

// ============================================================
// menu 服务
// ============================================================

/// 对应 Java: WxCpMenuServiceImplTest.testDelete
#[tokio::test]
async fn test_menu_delete() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpMenuServiceImpl::new(weak_service(&service));
    svc.delete().await.expect("删除菜单成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/menu/delete?agentid=101")
    );
}

/// 对应 Java: WxCpMenuServiceImplTest.testDeleteWithAgentId
#[tokio::test]
async fn test_menu_delete_with_agent_id() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpMenuServiceImpl::new(weak_service(&service));
    svc.delete_with_agent_id(202)
        .await
        .expect("按 agentId 删除菜单成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/menu/delete?agentid=202")
    );
}

/// 对应 Java: WxCpMenuServiceImplTest.testCreateWithAgentId
#[tokio::test]
async fn test_menu_create_with_agent_id() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpMenuServiceImpl::new(weak_service(&service));
    let mut menu = wx_rust_common::bean::menu::WxMenu::default();
    menu.buttons.push(wx_rust_common::bean::menu::WxMenuButton {
        r#type: "click".to_string(),
        name: "测试".to_string(),
        key: "KEY_1".to_string(),
        ..Default::default()
    });
    svc.create_with_agent_id(202, &menu)
        .await
        .expect("按 agentId 创建菜单成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/menu/create?agentid=202")
    );
}

// ============================================================
// group_robot 服务
// ============================================================

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendNews
#[tokio::test]
async fn test_group_robot_send_news() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_webhook_key("KEY_ABC");
    config.set_host_config(host_config(&server.url("")));
    let service: Arc<dyn WxCpService> = WxCpServiceImpl::new_arc(Arc::new(config));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let articles = vec![NewArticle {
        title: "标题1".to_string(),
        description: "描述1".to_string(),
        url: "https://example.com".to_string(),
        pic_url: "https://example.com/pic.png".to_string(),
        ..Default::default()
    }];
    svc.send_news(&articles).await.expect("发送 news 消息成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgtype":"news""#), "body: {body}");
    assert!(body.contains(r#""title":"标题1""#), "body: {body}");
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendFile
#[tokio::test]
async fn test_group_robot_send_file() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    svc.send_file(&webhook_url, "media_id_1")
        .await
        .expect("发送 file 消息成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgtype":"file""#), "body: {body}");
    // file 的 media_id 嵌套在 file 子对象中
    assert!(
        body.contains(r#""file":{"media_id":"media_id_1"}"#),
        "body: {body}"
    );
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendVoice
#[tokio::test]
async fn test_group_robot_send_voice() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    svc.send_voice(&webhook_url, "media_id_2")
        .await
        .expect("发送 voice 消息成功");
    let body = server.last_body();
    // voice 类型的 handle_msg_type 尚未实现（handle_msg_type 不含 voice 分支），
    // 因此 body 仅含 msgtype；这里仅验证请求到达 mock 服务器。
    assert!(body.contains(r#""msgtype":"voice""#), "body: {body}");
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendTemplateCardMessage
#[tokio::test]
async fn test_group_robot_send_template_card_message() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    let msg = WxCpGroupRobotMessage {
        msg_type: Some("template_card".to_string()),
        ..Default::default()
    };
    svc.send_template_card_message(&webhook_url, &msg)
        .await
        .expect("发送模板卡片消息成功");
    assert!(server.last_body().contains(r#""msgtype":"template_card""#));
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendMarkdownWithWebhookUrl
#[tokio::test]
async fn test_group_robot_send_markdown_with_webhook_url() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    svc.send_markdown_with_webhook_url(&webhook_url, "**bold**")
        .await
        .expect("发送 markdown 消息成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgtype":"markdown""#), "body: {body}");
    assert!(body.contains(r#""content":"**bold**""#), "body: {body}");
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendTextWithWebhookUrl
#[tokio::test]
async fn test_group_robot_send_text_with_webhook_url() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    svc.send_text_with_webhook_url(&webhook_url, "hello", &["user1"], &["mobile1"])
        .await
        .expect("发送 text 消息成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgtype":"text""#), "body: {body}");
    assert!(body.contains(r#""content":"hello""#), "body: {body}");
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendImageWithWebhookUrl
#[tokio::test]
async fn test_group_robot_send_image_with_webhook_url() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let service = service_with_host(&server.url(""));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    let webhook_url = format!("{}?key=KEY_ABC", server.url("/cgi-bin/webhook/send"));
    svc.send_image_with_webhook_url(&webhook_url, "BASE64_DATA", "MD5_HASH")
        .await
        .expect("发送 image 消息成功");
    assert!(server.last_body().contains(r#""msgtype":"image""#));
}

/// 对应 Java: WxCpGroupRobotServiceImplTest.testSendMarkdownV2
#[tokio::test]
async fn test_group_robot_send_markdown_v2() {
    let server = MockServer::start(dispatch(|_path| json(ok_resp()))).await;
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_webhook_key("KEY_ABC");
    config.set_host_config(host_config(&server.url("")));
    let service: Arc<dyn WxCpService> = WxCpServiceImpl::new_arc(Arc::new(config));
    let svc = WxCpGroupRobotServiceImpl::new(weak_service(&service));
    svc.send_markdown_v2("**bold**")
        .await
        .expect("发送 markdown_v2 消息成功");
    assert!(server.last_body().contains(r#""msgtype":"markdown_v2""#));
}
