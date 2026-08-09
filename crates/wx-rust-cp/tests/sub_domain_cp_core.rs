#![allow(clippy::field_reassign_with_default)]
//! 企业微信核心子域（Wave 2b I1）子服务集成测试。
//!
//! 镜像 Java `me.chanjar.weixin.cp.api.impl` 各 `WxCpXxxServiceImplTest`
//! 的语义，经 MockServer 验证（模式照抄 miniapp
//! `tests/sub_domain_g1_core.rs` 与既有 `wx_cp_service_impl_test.rs`，
//! 自含无外部依赖）。
//!
//! 覆盖 12 个子服务的核心方法：
//! - user：create/update/get_by_id/list_by_department/delete/转换类/活跃数
//! - department：create/get/list/simple_list/update/delete
//! - tag：create/update/delete/list_all/get/add_users2_tag/remove_users_from_tag
//! - message：send（agentId 回填）/get_statistics/recall
//! - media：upload（multipart）/upload_img/download（含 JSON 错误检测）
//! - menu：create/get（46003 → None）
//! - oauth2：buildAuthorizationUrl 三态/get_user_info 字段映射
//! - chat：create/get/update/send_msg
//! - group_robot：webhook 发送（postWithoutToken 通道）+ 缺 key 报错
//! - task_card/agent/work_bench：请求体组装与响应解析
//! - 门面释放后子服务调用报 -99

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

use wx_rust_cp::api::r#impl::*;
use wx_rust_cp::api::*;
use wx_rust_cp::bean::*;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

/// 极简 mock HTTP 服务器：按请求路径返回 (Content-Type, body)，记录
/// 最近一次请求路径（含 query）与请求体、请求计数。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> (content_type, body)`）。
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
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求路径（含 query）与请求体（POST 场景）
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
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

    #[allow(dead_code)]
    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
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

/// JSON 响应快捷构造。
fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

/// 指向 mock 服务器的主机配置（api_host 覆盖）。
fn host_config(host: &str) -> WxCpHostConfig {
    let mut config = WxCpHostConfig::new();
    config.api_host = host.to_string();
    config
}

/// 构建指向 mock 服务器的默认配置（corpid=corpid, secret=secret,
/// token=token123, agentid=101）。
fn config_with_host(host: &str) -> Arc<dyn WxCpConfigStorage> {
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_host_config(host_config(host));
    Arc::new(config)
}

/// 构建门面服务（`Arc<dyn WxCpService>`，供子服务 `Weak` 引用）。
fn service_with_host(host: &str) -> Arc<dyn WxCpService> {
    WxCpServiceImpl::new_arc(config_with_host(host))
}

/// 通用路由 handler：token 请求先应答，业务路径按 contains 分派。
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

// ---- user 服务（镜像 Java WxCpUserServiceImplTest / WxCpUserServiceImpl） ----

#[tokio::test]
async fn user_create_and_update_requests() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let user_service = WxCpUserServiceImpl::new(Arc::downgrade(&svc));

    // 镜像 Java testCreate 的字段（user.json 线格式：userid/name/department/
    // email/gender/mobile/position/telephone + extattr）
    let mut user = WxCpUser::default();
    user.user_id = Some("zhangsan".to_string());
    user.name = Some("张三".to_string());
    user.depart_ids = Some(vec![2]);
    user.email = Some("none@none.com".to_string());
    user.gender = Some(wx_rust_cp::bean::Gender::Female);
    user.mobile = Some("13560084979".to_string());
    user.position = Some("woman".to_string());
    user.telephone = Some("3300393".to_string());
    user.add_ext_attr("爱好", "table");

    user_service.create(&user).await.expect("create 成功");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/create"), "路径: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
    let body = server.last_body();
    assert!(body.contains("\"userid\":\"zhangsan\""), "body: {body}");
    assert!(body.contains("\"name\":\"张三\""), "body: {body}");
    assert!(body.contains("\"department\":[2]"), "body: {body}");
    assert!(body.contains("\"mobile\":\"13560084979\""), "body: {body}");
    assert!(body.contains("\"gender\":\"2\""), "body: {body}");
    assert!(body.contains("\"extattr\""), "body: {body}");

    // update：同一线格式走 /cgi-bin/user/update
    user.name = Some("张三丰".to_string());
    user_service.update(&user).await.expect("update 成功");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/update"), "路径: {path}");
    assert!(
        server.last_body().contains("\"name\":\"张三丰\""),
        "body: {}",
        server.last_body()
    );
}

#[tokio::test]
async fn user_get_by_id_and_list_by_department() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/get?userid=") {
            // user.json 线格式（镜像 WxCpUserGsonAdapter 反序列化 golden）
            json(
                r#"{"errcode":0,"errmsg":"ok","userid":"zhangsan","name":"张三",
                "department":[1,2],"order":[1,2],"position":"后台工程师",
                "mobile":"15913215421","gender":"1","email":"zhangsan@gzdev.com",
                "isleader":1,"status":1}"#,
            )
        } else if path.contains("/cgi-bin/user/list?department_id=") {
            json(
                r#"{"errcode":0,"errmsg":"ok","userlist":[
                {"userid":"zhangsan","name":"张三","department":[1,2],"gender":"1"},
                {"userid":"lisi","name":"李四","department":[2],"gender":"2"}]}"#,
            )
        } else if path.contains("/cgi-bin/user/simplelist?department_id=") {
            json(r#"{"errcode":0,"errmsg":"ok","userlist":[{"userid":"zhangsan","name":"张三"}]}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let user_service = WxCpUserServiceImpl::new(Arc::downgrade(&svc));

    // getById：整体响应解析为 WxCpUser
    let user = user_service
        .get_by_id("zhangsan")
        .await
        .expect("getById 成功");
    assert_eq!(user.user_id.as_deref(), Some("zhangsan"));
    assert_eq!(user.name.as_deref(), Some("张三"));
    assert_eq!(user.depart_ids.as_deref(), Some(&[1, 2][..]));
    assert_eq!(user.gender, Some(wx_rust_cp::bean::Gender::Male));
    assert_eq!(user.mobile.as_deref(), Some("15913215421"));
    assert_eq!(user.status, Some(1));

    // listByDepartment：fetch_child=1 + status 缺省补 0，解析 userlist
    let users = user_service
        .list_by_department(2, Some(true), None)
        .await
        .expect("listByDepartment 成功");
    assert_eq!(users.len(), 2);
    assert_eq!(users[1].user_id.as_deref(), Some("lisi"));
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/user/list?department_id=2"),
        "路径: {path}"
    );
    assert!(path.contains("fetch_child=1"), "路径: {path}");
    assert!(path.contains("status=0"), "路径: {path}");

    // listSimpleByDepartment
    let users = user_service
        .list_simple_by_department(2, Some(false), Some(4))
        .await
        .expect("listSimpleByDepartment 成功");
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].user_id.as_deref(), Some("zhangsan"));
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/user/simplelist?department_id=2"),
        "路径: {path}"
    );
    assert!(path.contains("fetch_child=0"), "路径: {path}");
    assert!(path.contains("status=4"), "路径: {path}");
}

#[tokio::test]
async fn user_delete_single_and_batch() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let user_service = WxCpUserServiceImpl::new(Arc::downgrade(&svc));

    // 单成员：GET USER_DELETE + userid
    user_service.delete(&["zhangsan"]).await.expect("单删成功");
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/user/delete?userid=zhangsan"),
        "路径: {path}"
    );

    // 多成员：POST USER_BATCH_DELETE {"useridlist":[...]}
    user_service
        .delete(&["zhangsan", "lisi"])
        .await
        .expect("批量删除成功");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/user/batchdelete"), "路径: {path}");
    let body = server.last_body();
    assert!(
        body.contains("\"useridlist\":[\"zhangsan\",\"lisi\"]"),
        "body: {body}"
    );
}

#[tokio::test]
async fn user_id_conversion_invite_and_active_stat() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/convert_to_openid") {
            json(r#"{"errcode":0,"errmsg":"ok","openid":"OPEN_1","appid":"APP_1"}"#)
        } else if path.contains("/cgi-bin/user/convert_to_userid")
            || path.contains("/cgi-bin/user/getuserid")
        {
            json(r#"{"errcode":0,"errmsg":"ok","userid":"zhangsan"}"#)
        } else if path.contains("/cgi-bin/user/get_active_stat") {
            json(r#"{"errcode":0,"errmsg":"ok","active_cnt":6}"#)
        } else if path.contains("/cgi-bin/batch/invite") {
            json(
                r#"{"errcode":0,"errmsg":"ok","invaliduser":[],"invalidparty":[],"invalidtag":[]}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let user_service = WxCpUserServiceImpl::new(Arc::downgrade(&svc));

    // userId2Openid：返回 map（openid/appid 按存在性组装）
    let map = user_service
        .user_id2_openid("zhangsan", Some(101))
        .await
        .expect("userId2Openid 成功");
    assert_eq!(map.get("openid").map(String::as_str), Some("OPEN_1"));
    assert_eq!(map.get("appid").map(String::as_str), Some("APP_1"));
    let body = server.last_body();
    assert!(body.contains("\"userid\":\"zhangsan\""), "body: {body}");
    assert!(body.contains("\"agentid\":101"), "body: {body}");

    // openid2UserId / getUserId：取 userid
    assert_eq!(
        user_service
            .openid2_user_id("OPEN_1")
            .await
            .expect("openid2UserId 成功"),
        "zhangsan"
    );
    assert_eq!(
        user_service
            .get_user_id("15913215421")
            .await
            .expect("getUserId 成功"),
        "zhangsan"
    );

    // getActiveStat：date 以 yyyy-MM-dd 格式化进请求体
    let date = chrono::DateTime::parse_from_rfc3339("2026-07-15T10:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let cnt = user_service
        .get_active_stat(date)
        .await
        .expect("getActiveStat 成功");
    assert_eq!(cnt, 6);
    let body = server.last_body();
    assert!(body.contains("\"date\":\"2026-07-15\""), "body: {body}");

    // invite：user 数组（party/tag 缺省不输出）
    let result = user_service
        .invite(&["zhangsan"], &[], &[])
        .await
        .expect("invite 成功");
    assert_eq!(result.err_code, 0);
    let body = server.last_body();
    assert!(body.contains("\"user\":[\"zhangsan\"]"), "body: {body}");
    assert!(!body.contains("party"), "body: {body}");
}

// ---- department 服务（镜像 Java WxCpDepartmentServiceImplTest） ----

#[tokio::test]
async fn department_create_get_update_delete() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/department/create") {
            json(r#"{"errcode":0,"errmsg":"created","id":2}"#)
        } else if path.contains("/cgi-bin/department/get?id=") {
            json(r#"{"errcode":0,"errmsg":"ok","department":{"id":2,"name":"研发部","parentid":1,"order":10}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let dept_service = WxCpDepartmentServiceImpl::new(Arc::downgrade(&svc));

    // create：响应取 id
    let mut depart = WxCpDepart::default();
    depart.name = Some("研发部".to_string());
    depart.parent_id = Some(1);
    let id = dept_service.create(&depart).await.expect("create 成功");
    assert_eq!(id, 2);
    let body = server.last_body();
    assert!(body.contains("\"name\":\"研发部\""), "body: {body}");
    assert!(body.contains("\"parentid\":1"), "body: {body}");

    // get：%d 格式化 + department 子对象解析
    let depart = dept_service.get(2).await.expect("get 成功");
    assert_eq!(depart.id, Some(2));
    assert_eq!(depart.name.as_deref(), Some("研发部"));
    assert_eq!(depart.parent_id, Some(1));
    assert_eq!(depart.order, Some(10));
    assert!(
        server.last_path().contains("/cgi-bin/department/get?id=2"),
        "路径: {}",
        server.last_path()
    );

    // update / delete
    dept_service.update(&depart).await.expect("update 成功");
    assert!(
        server.last_path().contains("/cgi-bin/department/update"),
        "路径: {}",
        server.last_path()
    );
    dept_service.delete(2).await.expect("delete 成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/department/delete?id=2"),
        "路径: {}",
        server.last_path()
    );
}

#[tokio::test]
async fn department_list_and_simple_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/department/simplelist") {
            json(r#"{"errcode":0,"errmsg":"ok","department_id":[1,2,3]}"#)
        } else {
            json(
                r#"{"errcode":0,"errmsg":"ok","department":[
                {"id":1,"name":"企业","parentid":0,"order":1},
                {"id":2,"name":"研发部","parentid":1,"order":10}]}"#,
            )
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let dept_service = WxCpDepartmentServiceImpl::new(Arc::downgrade(&svc));

    // list：不带 id（不拼 ?id=）
    let list = dept_service.list(None).await.expect("list 成功");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id, Some(1));
    assert_eq!(list[1].name.as_deref(), Some("研发部"));
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/department/list"), "路径: {path}");
    assert!(!path.contains("?id="), "路径: {path}");

    // list：带 id 拼 ?id=
    let list = dept_service.list(Some(1)).await.expect("list 成功");
    assert_eq!(list.len(), 2);
    assert!(
        server.last_path().contains("/cgi-bin/department/list?id=1"),
        "路径: {}",
        server.last_path()
    );

    // simpleList：department_id 数组映射为仅含 id 的 WxCpDepart
    let list = dept_service
        .simple_list(Some(1))
        .await
        .expect("simpleList 成功");
    assert_eq!(list.len(), 3);
    assert_eq!(list[0].id, Some(1));
    assert_eq!(list[2].id, Some(3));
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/department/simplelist?id=1"),
        "路径: {}",
        server.last_path()
    );
}

// ---- tag 服务（镜像 Java WxCpTagServiceImplTest） ----

#[tokio::test]
async fn tag_crud_and_members() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/tag/create") {
            json(r#"{"errcode":0,"errmsg":"ok","tagid":"12"}"#)
        } else if path.contains("/cgi-bin/tag/list") {
            json(r#"{"errcode":0,"errmsg":"ok","taglist":[{"tagid":"1","tagname":"标签A"},{"tagid":"2","tagname":"标签B"}]}"#)
        } else if path.contains("/cgi-bin/tag/get?tagid=") {
            json(r#"{"errcode":0,"errmsg":"ok","userlist":[{"userid":"zhangsan","name":"张三"}],"partylist":[1,2],"tagname":"标签A"}"#)
        } else if path.contains("/cgi-bin/tag/addtagusers") || path.contains("/cgi-bin/tag/deltagusers") {
            json(r#"{"errcode":0,"errmsg":"ok","invalidlist":"","invalidparty":[]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let tag_service = WxCpTagServiceImpl::new(Arc::downgrade(&svc));

    // create：tagname + 可选 tagid，响应取 tagid 字符串
    let tag_id = tag_service
        .create("标签A", Some(12))
        .await
        .expect("create 成功");
    assert_eq!(tag_id, "12");
    let body = server.last_body();
    assert!(body.contains("\"tagname\":\"标签A\""), "body: {body}");
    assert!(body.contains("\"tagid\":12"), "body: {body}");

    // update：tagid + tagname
    tag_service
        .update("12", "标签A2")
        .await
        .expect("update 成功");
    assert!(
        server.last_path().contains("/cgi-bin/tag/update"),
        "路径: {}",
        server.last_path()
    );

    // delete：%s 格式化 GET
    tag_service.delete("12").await.expect("delete 成功");
    assert!(
        server.last_path().contains("/cgi-bin/tag/delete?tagid=12"),
        "路径: {}",
        server.last_path()
    );

    // listAll：taglist 数组
    let tags = tag_service.list_all().await.expect("listAll 成功");
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].id.as_deref(), Some("1"));
    assert_eq!(tags[0].name.as_deref(), Some("标签A"));

    // get：整体响应解析（userlist/partylist/tagname）
    let result = tag_service.get("1").await.expect("get 成功");
    assert_eq!(result.tagname.as_deref(), Some("标签A"));
    assert_eq!(result.partylist.as_deref(), Some(&[1, 2][..]));
    let users = result.userlist.unwrap_or_default();
    assert_eq!(users[0].user_id.as_deref(), Some("zhangsan"));

    // addUsers2Tag / removeUsersFromTag
    let result = tag_service
        .add_users2_tag("1", &["zhangsan"], &["2"])
        .await
        .expect("addUsers2Tag 成功");
    assert_eq!(result.err_code, 0);
    let body = server.last_body();
    assert!(body.contains("\"tagid\":\"1\""), "body: {body}");
    assert!(body.contains("\"userlist\":[\"zhangsan\"]"), "body: {body}");
    assert!(body.contains("\"partylist\":[\"2\"]"), "body: {body}");
    tag_service
        .remove_users_from_tag("1", &["zhangsan"], &[])
        .await
        .expect("removeUsersFromTag 成功");
    assert!(
        server.last_path().contains("/cgi-bin/tag/deltagusers"),
        "路径: {}",
        server.last_path()
    );
}

// ---- message 服务（镜像 Java WxCpMessageServiceImplTest） ----

#[tokio::test]
async fn message_send_statistics_and_recall() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/send") {
            json(r#"{"errcode":0,"errmsg":"ok","invaliduser":"","invalidparty":"","invalidtag":"","msgid":"MSG_001","response_code":""}"#)
        } else if path.contains("/cgi-bin/message/get_statistics") {
            json(r#"{"errcode":0,"errmsg":"ok","statistics":[{"app_name":"测试应用","agentid":101,"count":3}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let msg_service = WxCpMessageServiceImpl::new(Arc::downgrade(&svc));

    // send：agentId 为空时回填配置 101（镜像 Java setAgentId）
    let message = WxCpMessage::text()
        .to_user("zhangsan")
        .content("hello")
        .build();
    assert!(message.agent_id.is_none(), "builder 默认无 agentid");
    let result = msg_service.send(&message).await.expect("send 成功");
    assert_eq!(result.msg_id, "MSG_001");
    let body = server.last_body();
    assert!(body.contains("\"agentid\":101"), "body: {body}");
    assert!(body.contains("\"msgtype\":\"text\""), "body: {body}");
    assert!(body.contains("\"content\":\"hello\""), "body: {body}");

    // getStatistics：{"time_type":...}
    let stats = msg_service
        .get_statistics(0)
        .await
        .expect("getStatistics 成功");
    assert_eq!(stats.statistics.len(), 1);
    assert_eq!(stats.statistics[0].agent_id, 101);
    assert_eq!(stats.statistics[0].count, 3);
    let body = server.last_body();
    assert!(body.contains("\"time_type\":0"), "body: {body}");

    // recall：{"msgid":...}
    msg_service.recall("MSG_001").await.expect("recall 成功");
    assert!(
        server.last_path().contains("/cgi-bin/message/recall"),
        "路径: {}",
        server.last_path()
    );
    assert!(
        server.last_body().contains("\"msgid\":\"MSG_001\""),
        "body: {}",
        server.last_body()
    );
}

// ---- media 服务（镜像 Java WxCpMediaServiceImplTest） ----

#[tokio::test]
async fn media_upload_img_and_download() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/uploadimg") {
            json(r#"{"errcode":0,"errmsg":"ok","url":"https://example.com/img/1"}"#)
        } else if path.contains("/cgi-bin/media/get?") || path.contains("/cgi-bin/media/get/jssdk?")
        {
            // 下载：二进制内容（非 JSON 视为文件字节）
            (
                "application/octet-stream".to_string(),
                "FILE_DATA_123".to_string(),
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let media_service = WxCpMediaServiceImpl::new(Arc::downgrade(&svc));

    // upload_img：multipart 上传后取 url
    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join(format!("wx_cp_test_{}.png", std::process::id()));
    std::fs::write(&file_path, b"PNG_BYTES").expect("写临时文件");
    let url = media_service
        .upload_img(file_path.to_str().unwrap())
        .await
        .expect("uploadImg 成功");
    assert_eq!(url, "https://example.com/img/1");
    assert!(
        server.last_path().contains("/cgi-bin/media/uploadimg"),
        "路径: {}",
        server.last_path()
    );
    assert!(
        server.last_path().contains("access_token=MOCK_TOKEN"),
        "路径: {}",
        server.last_path()
    );
    let body = server.last_body();
    assert!(
        body.contains("filename=\"wx_cp_test_"),
        "multipart 含文件名: {body}"
    );
    let _ = std::fs::remove_file(&file_path);

    // download：media_id= 查询 + 字节返回
    let bytes = media_service
        .download("MEDIA_1")
        .await
        .expect("download 成功");
    assert_eq!(String::from_utf8(bytes).expect("utf8"), "FILE_DATA_123");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/media/get?"), "路径: {path}");
    assert!(path.contains("media_id=MEDIA_1"), "路径: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

#[tokio::test]
async fn media_upload_parse_result_and_json_error_download() {
    // 远端文件服务器（uploadWithUrl 的下载源）
    let remote = MockServer::start(|_path| {
        (
            "application/octet-stream".to_string(),
            "REMOTE_BYTES".to_string(),
        )
    })
    .await;
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/upload?type=") {
            json(r#"{"url":"https://example.com/upload","type":"image","mediaId":"MEDIA_1","thumbMediaId":"THUMB_1","createdAt":1720000000}"#)
        } else if path.contains("/cgi-bin/media/get?") {
            // JSON 响应视为微信错误报文（镜像 BaseMediaDownloadRequestExecutor）
            json(r#"{"errcode":40003,"errmsg":"invalid media_id"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let media_service = WxCpMediaServiceImpl::new(Arc::downgrade(&svc));

    // upload（InputStream 版）：文件名 UUID.fileType + multipart，解析上传结果
    let result = media_service
        .upload("image", "jpg", b"JPEG_BYTES".to_vec())
        .await
        .expect("upload 成功");
    assert_eq!(result.media_id, "MEDIA_1");
    assert_eq!(result.url, "https://example.com/upload");
    assert_eq!(result.r#type, "image");
    assert_eq!(result.created_at, 1720000000);
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/media/upload?type=image"),
        "路径: {path}"
    );
    let body = server.last_body();
    assert!(body.contains("filename=\""), "multipart 含文件名: {body}");

    // upload_with_url：从远端服务器下载后上传（filename 保留）
    let result = media_service
        .upload_with_url("image", "logo.png", &remote.url("/logo.png"))
        .await
        .expect("uploadWithUrl 成功");
    assert_eq!(result.media_id, "MEDIA_1");
    let body = server.last_body();
    assert!(
        body.contains("filename=\"logo.png\""),
        "multipart 含文件名: {body}"
    );

    // download：JSON 错误报文 → 40003
    let err = media_service.download("BAD_MEDIA").await.unwrap_err();
    assert_eq!(err.error_code(), Some(40003), "错误: {err}");
}

// ---- menu 服务（镜像 Java WxCpMenuServiceImplTest） ----

#[tokio::test]
async fn menu_create_and_get_with_46003_none() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/menu/get?agentid=") {
            json(r#"{"errcode":0,"errmsg":"ok","buttons":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC","sub_button":[]}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let menu_service = WxCpMenuServiceImpl::new(Arc::downgrade(&svc));

    // create：agentid 取配置 101，URL %d 格式化
    let mut menu = wx_rust_common::bean::menu::WxMenu::default();
    menu.buttons.push(wx_rust_common::bean::menu::WxMenuButton {
        r#type: "click".to_string(),
        name: "今日歌曲".to_string(),
        key: "V1001_TODAY_MUSIC".to_string(),
        ..Default::default()
    });
    menu_service.create(&menu).await.expect("create 成功");
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/menu/create?agentid=101"),
        "路径: {path}"
    );
    assert!(
        server.last_body().contains("\"type\":\"click\""),
        "body: {}",
        server.last_body()
    );

    // get：解析 WxMenu
    let menu = menu_service
        .get_with_agent_id(101)
        .await
        .expect("get 成功")
        .expect("菜单存在");
    assert_eq!(menu.buttons.len(), 1);
    assert_eq!(menu.buttons[0].name, "今日歌曲");

    // 46003「不存在的菜单数据」→ None（镜像 Java 返回 null）
    let bad_server = MockServer::start(dispatch(|_path| {
        json(r#"{"errcode":46003,"errmsg":"no menu"}"#)
    }))
    .await;
    let bad_svc = service_with_host(&bad_server.url(""));
    let bad_menu_service = WxCpMenuServiceImpl::new(Arc::downgrade(&bad_svc));
    assert!(
        bad_menu_service
            .get()
            .await
            .expect("46003 不抛错")
            .is_none(),
        "46003 应返回 None"
    );
}

// ---- oauth2 服务（镜像 Java WxCpOAuth2ServiceImplTest） ----

#[tokio::test]
async fn oauth2_authorization_url_and_get_user_info() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/getuserinfo?") {
            // UserId 优先、其次 userid；OpenId 优先、其次 openid
            json(r#"{"errcode":0,"errmsg":"ok","UserId":"zhangsan","DeviceId":"dev_1","OpenId":"OPEN_1","user_ticket":"TICKET_1","expires_in":"7200","external_userid":"EXT_1"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_oauth2_redirect_uri("https://example/oauth2");
    config.set_host_config(host_config(&server.url("")));
    let svc: Arc<dyn WxCpService> = WxCpServiceImpl::new_arc(Arc::new(config));
    let oauth2_service = WxCpOAuth2ServiceImpl::new(Arc::downgrade(&svc));

    // buildAuthorizationUrl(state)：redirect_uri 取配置，scope=snsapi_base
    let url = oauth2_service.build_authorization_url("STATE_1");
    assert!(
        url.starts_with("https://open.weixin.qq.com/connect/oauth2/authorize"),
        "URL: {url}"
    );
    assert!(url.contains("appid=corpid"), "URL: {url}");
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fexample%2Foauth2"),
        "URL: {url}"
    );
    assert!(url.contains("response_type=code"), "URL: {url}");
    assert!(url.contains("scope=snsapi_base"), "URL: {url}");
    assert!(url.contains("state=STATE_1"), "URL: {url}");
    assert!(url.ends_with("#wechat_redirect"), "URL: {url}");

    // buildAuthorizationUrl(redirectUri, state, scope)：snsapi_privateinfo
    // 时追加 agentid
    let url = oauth2_service.build_authorization_url_with_scope(
        "https://example/oauth2",
        "STATE_2",
        "snsapi_privateinfo",
    );
    assert!(url.contains("scope=snsapi_privateinfo"), "URL: {url}");
    assert!(url.contains("agentid=101"), "URL: {url}");

    // snsapi_base 不追加 agentid
    let url = oauth2_service
        .build_authorization_url_with_redirect_uri("https://example/oauth2", "STATE_3");
    assert!(!url.contains("agentid="), "URL: {url}");

    // getUserInfo：agentid 取配置，字段映射（UserId/OpenId 优先）
    let info = oauth2_service
        .get_user_info("CODE_1")
        .await
        .expect("getUserInfo 成功");
    assert_eq!(info.user_id, "zhangsan");
    assert_eq!(info.open_id, "OPEN_1");
    assert_eq!(info.device_id, "dev_1");
    assert_eq!(info.user_ticket, "TICKET_1");
    assert_eq!(info.expires_in, "7200");
    assert_eq!(info.external_user_id, "EXT_1");
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/user/getuserinfo?code=CODE_1&agentid=101"),
        "路径: {path}"
    );
}

// ---- chat 服务（镜像 Java WxCpChatServiceImplTest） ----

#[tokio::test]
async fn chat_create_get_update_send() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/appchat/create") {
            json(r#"{"errcode":0,"errmsg":"ok","chatid":"CHAT_1"}"#)
        } else if path.contains("/cgi-bin/appchat/get?chatid=") {
            json(r#"{"errcode":0,"errmsg":"ok","chat_info":{"chatid":"CHAT_1","name":"测试群","owner":"zhangsan","userlist":["zhangsan","lisi"]}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let chat_service = WxCpChatServiceImpl::new(Arc::downgrade(&svc));

    // create：请求体含 name/owner/userlist，响应取 chatid
    let chat_id = chat_service
        .create("测试群", "zhangsan", &["zhangsan", "lisi"], None)
        .await
        .expect("create 成功");
    assert_eq!(chat_id, "CHAT_1");
    let body = server.last_body();
    assert!(body.contains("\"name\":\"测试群\""), "body: {body}");
    assert!(body.contains("\"owner\":\"zhangsan\""), "body: {body}");
    assert!(
        body.contains("\"userlist\":[\"zhangsan\",\"lisi\"]"),
        "body: {body}"
    );

    // get：chat_info 子对象解析
    let chat = chat_service.get("CHAT_1").await.expect("get 成功");
    assert_eq!(chat.id.as_deref(), Some("CHAT_1"));
    assert_eq!(chat.name.as_deref(), Some("测试群"));
    assert_eq!(chat.owner.as_deref(), Some("zhangsan"));
    assert_eq!(
        chat.users.as_deref(),
        Some(&["zhangsan".to_string(), "lisi".to_string()][..])
    );
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/appchat/get?chatid=CHAT_1"),
        "路径: {}",
        server.last_path()
    );

    // update：add_user_list/del_user_list
    chat_service
        .update("CHAT_1", Some("新群名"), None, &["wangwu"], &["lisi"])
        .await
        .expect("update 成功");
    let body = server.last_body();
    assert!(body.contains("\"chatid\":\"CHAT_1\""), "body: {body}");
    assert!(body.contains("\"name\":\"新群名\""), "body: {body}");
    assert!(
        body.contains("\"add_user_list\":[\"wangwu\"]"),
        "body: {body}"
    );
    assert!(
        body.contains("\"del_user_list\":[\"lisi\"]"),
        "body: {body}"
    );

    // sendMsg：请求体 message.toJson()
    let mut msg = WxCpAppChatMessage::default();
    msg.msg_type = Some("text".to_string());
    msg.chat_id = Some("CHAT_1".to_string());
    msg.content = Some("hello".to_string());
    chat_service.send_msg(&msg).await.expect("sendMsg 成功");
    assert!(
        server.last_path().contains("/cgi-bin/appchat/send"),
        "路径: {}",
        server.last_path()
    );
    let body = server.last_body();
    assert!(body.contains("\"msgtype\":\"text\""), "body: {body}");
    assert!(body.contains("\"chatid\":\"CHAT_1\""), "body: {body}");
}

// ---- group_robot 服务（镜像 Java WxCpGroupRobotServiceImplTest） ----

#[tokio::test]
async fn group_robot_send_text_via_webhook() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/webhook/send?key=") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_webhook_key("KEY_ABC");
    config.set_host_config(host_config(&server.url("")));
    let svc: Arc<dyn WxCpService> = WxCpServiceImpl::new_arc(Arc::new(config));
    let robot_service = WxCpGroupRobotServiceImpl::new(Arc::downgrade(&svc));

    // sendText：默认 webhook（apiUrl(WEBHOOK_SEND) + key），走 postWithoutToken
    robot_service
        .send_text("hello robot", &["zhangsan"], &["13560084979"])
        .await
        .expect("sendText 成功");
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/webhook/send?key=KEY_ABC"),
        "路径: {path}"
    );
    assert!(
        !path.contains("access_token="),
        "webhook 发送不应自动带 token，路径: {path}"
    );
    let body = server.last_body();
    assert!(body.contains("\"msgtype\":\"text\""), "body: {body}");
    assert!(body.contains("\"content\":\"hello robot\""), "body: {body}");
    assert!(
        body.contains("\"mentioned_list\":[\"zhangsan\"]"),
        "body: {body}"
    );
    assert!(
        body.contains("\"mentioned_mobile_list\":[\"13560084979\"]"),
        "body: {body}"
    );

    // sendMarkdownV2 / sendImage
    robot_service
        .send_markdown_v2("**bold**")
        .await
        .expect("sendMarkdownV2 成功");
    assert!(
        server.last_body().contains("\"msgtype\":\"markdown_v2\""),
        "body: {}",
        server.last_body()
    );
    robot_service
        .send_image("BASE64_1", "MD5_1")
        .await
        .expect("sendImage 成功");
    let body = server.last_body();
    assert!(body.contains("\"msgtype\":\"image\""), "body: {body}");
    assert!(body.contains("\"base64\":\"BASE64_1\""), "body: {body}");
    assert!(body.contains("\"md5\":\"MD5_1\""), "body: {body}");

    // webhookKey 未设置：报错（镜像 Java "请先设置WebhookKey"）
    let svc2 = service_with_host(&server.url(""));
    let robot_service2 = WxCpGroupRobotServiceImpl::new(Arc::downgrade(&svc2));
    let err = robot_service2.send_markdown("x").await.unwrap_err();
    assert!(
        format!("{err}").contains("请先设置WebhookKey"),
        "错误信息: {err}"
    );
}

// ---- task_card / agent / work_bench 服务 ----

#[tokio::test]
async fn task_card_update_and_template_card_button() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let task_card_service = WxCpTaskCardServiceImpl::new(Arc::downgrade(&svc));

    // update：userids/agentid(配置 101)/task_id/clicked_key
    task_card_service
        .update(&["zhangsan", "lisi"], "TASK_1", "已确认")
        .await
        .expect("update 成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/message/update_taskcard"),
        "路径: {}",
        server.last_path()
    );
    let body = server.last_body();
    assert!(
        body.contains("\"userids\":[\"zhangsan\",\"lisi\"]"),
        "body: {body}"
    );
    assert!(body.contains("\"agentid\":101"), "body: {body}");
    assert!(body.contains("\"task_id\":\"TASK_1\""), "body: {body}");
    assert!(body.contains("\"clicked_key\":\"已确认\""), "body: {body}");

    // updateTemplateCardButton：partyids/tagids/atall/response_code/button
    task_card_service
        .update_template_card_button(&["zhangsan"], &[1], &[2], 1, "RESP_1", "已确认")
        .await
        .expect("updateTemplateCardButton 成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/message/update_template_card"),
        "路径: {}",
        server.last_path()
    );
    let body = server.last_body();
    assert!(body.contains("\"partyids\":[1]"), "body: {body}");
    assert!(body.contains("\"tagids\":[2]"), "body: {body}");
    assert!(body.contains("\"atall\":1"), "body: {body}");
    assert!(
        body.contains("\"response_code\":\"RESP_1\""),
        "body: {body}"
    );
    assert!(
        body.contains("\"button\":{\"replace_name\":\"已确认\"}"),
        "body: {body}"
    );
}

#[tokio::test]
async fn agent_get_set_and_work_bench_template() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/agent/get?agentid=") {
            json(r#"{"errcode":0,"errmsg":"ok","agentid":101,"name":"测试应用","square_logo_url":"https://example.com/logo.png","close":0}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let agent_service = WxCpAgentServiceImpl::new(Arc::downgrade(&svc));

    // get：%d 格式化 + 整体解析
    let agent = agent_service.get(101).await.expect("get 成功");
    assert_eq!(agent.agent_id, 101);
    assert_eq!(agent.name, "测试应用");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/agent/get?agentid=101"),
        "路径: {}",
        server.last_path()
    );

    // set：POST + errcode 校验
    agent_service.set(&agent).await.expect("set 成功");
    assert!(
        server.last_path().contains("/cgi-bin/agent/set"),
        "路径: {}",
        server.last_path()
    );

    // work bench：setWorkBenchTemplate（keydata 模板线格式）
    let work_bench_service = WxCpAgentWorkBenchServiceImpl::new(Arc::downgrade(&svc));
    let bench = WxCpAgentWorkBench {
        r#type: "keydata".to_string(),
        agent_id: 101,
        key_data_list: vec![WorkBenchKeyData {
            key: "待审批".to_string(),
            data: "5".to_string(),
            jump_url: "https://example.com/jump".to_string(),
            page_path: String::new(),
        }],
        ..Default::default()
    };
    work_bench_service
        .set_work_bench_template(&bench)
        .await
        .expect("setWorkBenchTemplate 成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/agent/set_workbench_template"),
        "路径: {}",
        server.last_path()
    );
    let body = server.last_body();
    assert!(body.contains("\"agentid\":101"), "body: {body}");
    assert!(body.contains("\"type\":\"keydata\""), "body: {body}");
    assert!(
        body.contains("\"keydata\":{\"items\":[{\"key\":\"待审批\",\"data\":\"5\",\"jump_url\":\"https://example.com/jump\",\"pagepath\":\"\"}]}"),
        "body: {body}"
    );

    // setWorkBenchData：userid 形态
    let bench_data = WxCpAgentWorkBench {
        r#type: "keydata".to_string(),
        agent_id: 101,
        user_id: "zhangsan".to_string(),
        key_data_list: vec![WorkBenchKeyData {
            key: "待审批".to_string(),
            data: "5".to_string(),
            jump_url: String::new(),
            page_path: String::new(),
        }],
        ..Default::default()
    };
    work_bench_service
        .set_work_bench_data(&bench_data)
        .await
        .expect("setWorkBenchData 成功");
    let body = server.last_body();
    assert!(body.contains("\"userid\":\"zhangsan\""), "body: {body}");

    // batchSetWorkBenchData：userid_list + data 包裹
    let bench_batch = WxCpAgentWorkBench {
        r#type: "keydata".to_string(),
        agent_id: 101,
        userid_list: vec!["zhangsan".to_string(), "lisi".to_string()],
        key_data_list: vec![WorkBenchKeyData {
            key: "待审批".to_string(),
            data: "5".to_string(),
            jump_url: String::new(),
            page_path: String::new(),
        }],
        ..Default::default()
    };
    work_bench_service
        .batch_set_work_bench_data(&bench_batch)
        .await
        .expect("batchSetWorkBenchData 成功");
    let body = server.last_body();
    assert!(
        body.contains("\"userid_list\":[\"zhangsan\",\"lisi\"]"),
        "body: {body}"
    );
    assert!(
        body.contains("\"data\":{\"type\":\"keydata\""),
        "body: {body}"
    );
}

// ---- 门面释放（Weak 升级失败 → -99） ----

#[tokio::test]
async fn sub_service_upgrade_failure_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak: Weak<dyn WxCpService> = {
        let svc = service_with_host(&server.url(""));
        Arc::downgrade(&svc)
    };
    let user_service = WxCpUserServiceImpl::new(weak);
    let err = user_service.get_by_id("zhangsan").await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99), "错误: {err}");
    assert!(
        format!("{err}").contains("企业微信服务已释放"),
        "错误信息: {err}"
    );
}
