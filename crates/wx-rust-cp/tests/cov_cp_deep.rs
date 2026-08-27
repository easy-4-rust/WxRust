#![allow(clippy::field_reassign_with_default, clippy::if_same_then_else)]
//! 企业微信 OA/weDoc/日历/日程/外部联系人深度覆盖测试。
//!
//! 镜像 Java `me.chanjar.weixin.cp.api.impl` 各 `WxCpXxxServiceImplTest`
//! 的语义，经 MockServer 验证（模式照抄 `sub_domain_cp_core.rs`）。
//!
//! 覆盖目标：
//! - OA approval：提交审批/详情/模板/提交单据 各分支（GET vs POST 语义）
//! - OA weDoc：新建文档/编辑/获取/分享/删除/管理员 分支
//! - OA calendar：日历增删改查
//! - OA schedule：日程增删改查 + listByCalendar
//! - external contact：联系我 CRUD、unionid 路径、客户标签分组 CRUD、
//!   客户群进群方式、群发消息、敏感词规则、客户朋友圈、获客链接

use wx_rust_cp::api::r#impl::*;
use wx_rust_cp::api::*;
use wx_rust_cp::bean::*;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

// ---------------------------------------------------------------------------
// 极简 MockServer（与 sub_domain_cp_core.rs 同模式）
// ---------------------------------------------------------------------------

struct MockServer {
    addr: std::net::SocketAddr,
    #[allow(dead_code)]
    requests: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    last_method: std::sync::Arc<std::sync::Mutex<String>>,
    last_path: std::sync::Arc<std::sync::Mutex<String>>,
    last_body: std::sync::Arc<std::sync::Mutex<String>>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &str) -> (String, String) + Send + Sync + 'static,
    {
        use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().expect("addr");
        let requests = std::sync::Arc::new(AtomicUsize::new(0));
        let last_method = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let stop = std::sync::Arc::new(AtomicBool::new(false));
        let handler = std::sync::Arc::new(handler);
        let (r, lm, lp, lb, s) = (
            requests.clone(),
            last_method.clone(),
            last_path.clone(),
            last_body.clone(),
            stop.clone(),
        );
        tokio::spawn(async move {
            loop {
                if s.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                r.fetch_add(1, Ordering::SeqCst);
                let handler = handler.clone();
                let lm = lm.clone();
                let lp = lp.clone();
                let lb = lb.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let mut method = String::from("GET");
                    let mut path = String::new();
                    if let Some(request_line) = request.lines().next() {
                        let mut parts = request_line.split_whitespace();
                        if let Some(m) = parts.next() {
                            method = m.to_string();
                        }
                        if let Some(p) = parts.next() {
                            path = p.to_string();
                        }
                    }
                    *lm.lock().unwrap() = method.clone();
                    *lp.lock().unwrap() = path.clone();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        *lb.lock().unwrap() = request[idx + 4..].to_string();
                    }
                    let (ct, body) = handler(&method, &path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {ct}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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
            last_method,
            last_path,
            last_body,
            stop,
        }
    }
    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }
    fn last_method(&self) -> String {
        self.last_method.lock().unwrap().clone()
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
        self.stop.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str, &str) -> (String, String) + Send + Sync + 'static {
    move |_method: &str, path: &str| {
        if path.contains("/cgi-bin/gettoken") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

fn host_config(host: &str) -> WxCpHostConfig {
    let mut config = WxCpHostConfig::new();
    config.api_host = host.to_string();
    config
}

fn config_with_host(host: &str) -> std::sync::Arc<dyn WxCpConfigStorage> {
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("token123");
    config.set_agent_id(Some(101));
    config.set_host_config(host_config(host));
    std::sync::Arc::new(config)
}

fn service_with_host(host: &str) -> std::sync::Arc<WxCpServiceImpl> {
    WxCpServiceImpl::new_arc(config_with_host(host))
}

fn weak(service: &std::sync::Arc<WxCpServiceImpl>) -> std::sync::Weak<dyn WxCpService> {
    let svc: std::sync::Arc<dyn WxCpService> = service.clone();
    std::sync::Arc::downgrade(&svc)
}

fn ts(secs: i64) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(secs, 0).expect("valid timestamp")
}

// ===========================================================================
// OA 审批（WxCpOaServiceImpl）
// ===========================================================================

/// 对应 Java: `WxCpOaServiceImplTest.testApply` — 提交审批申请。
#[tokio::test]
async fn oa_apply_returns_sp_no() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/applyevent") {
            json(r#"{"errcode":0,"errmsg":"ok","sp_no":"SP20240001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let mut req = WxCpOaApplyEventRequest::default();
    req.creator_user_id = "zhangsan".to_string();
    req.template_id = "TPL_001".to_string();
    let sp_no = oa.apply(&req).await.expect("提交审批成功");
    assert_eq!(sp_no, "SP20240001");
    assert!(server.last_path().contains("/cgi-bin/oa/applyevent"));
    let body = server.last_body();
    assert!(
        body.contains(r#""creator_userid":"zhangsan""#),
        "body: {body}"
    );
    assert!(body.contains(r#""template_id":"TPL_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetApprovalDetail` — 获取审批详情。
#[tokio::test]
async fn oa_get_approval_detail_post_sp_no() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/getapprovaldetail") {
            json(r#"{"errcode":0,"errmsg":"ok","info":{"sp_no":"SP001","sp_name":"请假","sp_status":"1"}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let detail = oa.get_approval_detail("SP001").await.expect("获取详情成功");
    assert_eq!(detail.info.sp_no, "SP001");
    assert!(server.last_path().contains("/cgi-bin/oa/getapprovaldetail"));
    let body = server.last_body();
    assert!(body.contains(r#""sp_no":"SP001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetTemplateDetail` — 获取审批模板详情。
#[tokio::test]
async fn oa_get_template_detail() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/gettemplatedetail") {
            json(r#"{"errcode":0,"errmsg":"ok","template_names":[{"text":"请假模板","lang":"zh_CN"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let result = oa
        .get_template_detail("TPL_001")
        .await
        .expect("获取模板成功");
    assert_eq!(result.template_names.len(), 1);
    assert!(server.last_path().contains("/cgi-bin/oa/gettemplatedetail"));
    let body = server.last_body();
    assert!(body.contains(r#""template_id":"TPL_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetApprovalData` — 获取审批数据（旧版）。
#[tokio::test]
async fn oa_get_approval_data_with_next_sp_num() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/corp/getapprovaldata") {
            json(r#"{"errcode":0,"errmsg":"ok","count":2,"sp_no_list":["SP001","SP002"]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let data = oa
        .get_approval_data(1600000000, 1600003600, Some(100))
        .await
        .expect("获取审批数据成功");
    assert_eq!(data.count, 2);
    assert!(server.last_path().contains("/cgi-bin/corp/getapprovaldata"));
    let body = server.last_body();
    assert!(body.contains(r#""starttime":1600000000"#), "body: {body}");
    assert!(body.contains(r#""endtime":1600003600"#), "body: {body}");
    assert!(body.contains(r#""next_spnum":100"#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetApprovalData` — 无 next_sp_num 分支。
#[tokio::test]
async fn oa_get_approval_data_without_next_sp_num() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/corp/getapprovaldata") {
            json(r#"{"errcode":0,"errmsg":"ok","count":0,"sp_no_list":[]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let data = oa
        .get_approval_data(1600000000, 1600003600, None)
        .await
        .expect("获取审批数据成功");
    assert_eq!(data.count, 0);
    let body = server.last_body();
    assert!(
        !body.contains("next_spnum"),
        "body 不应含 next_spnum: {body}"
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCorpConf` — 获取企业假期配置（GET 语义）。
#[tokio::test]
async fn oa_get_corp_conf_uses_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/vacation/getcorpconf") {
            json(r#"{"errcode":0,"errmsg":"ok","lists":[{"id":1,"name":"年假"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let conf = oa.get_corp_conf().await.expect("获取企业假期配置成功");
    assert_eq!(conf.lists.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/oa/vacation/getcorpconf")
    );
    assert_eq!(server.last_method(), "GET");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetUserVacationQuota` — 获取成员假期余额。
#[tokio::test]
async fn oa_get_user_vacation_quota() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/vacation/getuservacationquota") {
            json(r#"{"errcode":0,"errmsg":"ok","lists":[{"id":1,"vacation_name":"年假"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let quota = oa
        .get_user_vacation_quota("zhangsan")
        .await
        .expect("获取假期余额成功");
    assert_eq!(quota.lists.len(), 1);
    let body = server.last_body();
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testSetOneUserQuota` — 修改成员假期余额。
#[tokio::test]
async fn oa_set_one_user_quota_with_remarks() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let resp = oa
        .set_one_user_quota("zhangsan", 1, 5, 0, Some("调整假期"))
        .await
        .expect("修改假期余额成功");
    assert_eq!(resp.errcode, 0);
    let body = server.last_body();
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
    assert!(body.contains(r#""vacation_id":1"#), "body: {body}");
    assert!(body.contains(r#""leftduration":5"#), "body: {body}");
    assert!(body.contains(r#""remarks":"调整假期""#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testSetOneUserQuota` — 无 remarks 分支。
#[tokio::test]
async fn oa_set_one_user_quota_without_remarks() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    oa.set_one_user_quota("zhangsan", 1, 5, 0, None)
        .await
        .expect("修改假期余额成功");
    let body = server.last_body();
    assert!(!body.contains("remarks"), "body 不应含 remarks: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testCreateOaApprovalTemplate` — 创建审批模板。
#[tokio::test]
async fn oa_create_approval_template() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/approval/create_template") {
            json(r#"{"errcode":0,"errmsg":"ok","template_id":"NEW_TPL_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let tpl = WxCpOaApprovalTemplate::default();
    let template_id = oa
        .create_oa_approval_template(&tpl)
        .await
        .expect("创建模板成功");
    assert_eq!(template_id, "NEW_TPL_001");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/oa/approval/create_template")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testUpdateOaApprovalTemplate` — 更新审批模板。
#[tokio::test]
async fn oa_update_approval_template() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let tpl = WxCpOaApprovalTemplate::default();
    oa.update_oa_approval_template(&tpl)
        .await
        .expect("更新模板成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/oa/approval/update_template")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCheckinOption` — 获取打卡规则。
#[tokio::test]
async fn oa_get_checkin_option() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/checkin/getcheckinoption") {
            json(r#"{"errcode":0,"errmsg":"ok","info":[{"userid":"zhangsan","group":{"groupname":"默认","checkintype":1}}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let options = oa
        .get_checkin_option(ts(1600000000), &["zhangsan"])
        .await
        .expect("获取打卡规则成功");
    assert_eq!(options.len(), 1);
    let body = server.last_body();
    assert!(body.contains(r#""datetime":1600000000"#), "body: {body}");
    assert!(
        body.contains(r#""useridlist":["zhangsan"]"#),
        "body: {body}"
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCropCheckinOption` — 获取企业打卡规则。
#[tokio::test]
async fn oa_get_crop_checkin_option() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/checkin/getcorpcheckinoption") {
            json(r#"{"errcode":0,"errmsg":"ok","group":[{"groupname":"默认","checkintype":3}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let groups = oa
        .get_crop_checkin_option()
        .await
        .expect("获取企业打卡规则成功");
    assert_eq!(groups.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/checkin/getcorpcheckinoption")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetDialRecord` — 获取公费电话记录。
#[tokio::test]
async fn oa_get_dial_record() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/dial/get_dial_record") {
            json(
                r#"{"errcode":0,"errmsg":"ok","record":[{"dial_time":1600000000,"duration":120}]}"#,
            )
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let records = oa
        .get_dial_record(ts(1600000000), ts(1600003600), Some(0), Some(50))
        .await
        .expect("获取公费电话记录成功");
    assert_eq!(records.len(), 1);
    let body = server.last_body();
    assert!(body.contains(r#""offset":0"#), "body: {body}");
    assert!(body.contains(r#""limit":50"#), "body: {body}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetDialRecord` — 时间跨度超 30 天报错。
#[tokio::test]
async fn oa_get_dial_record_time_range_error() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let err = oa
        .get_dial_record(ts(1600000000), ts(1600000000 + 31 * 86400), None, None)
        .await
        .unwrap_err();
    assert!(format!("{err}").contains("30天"), "错误: {err}");
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCheckinDayData` — 打卡日报数据。
#[tokio::test]
async fn oa_get_checkin_day_data() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/checkin/getcheckin_daydata") {
            json(r#"{"errcode":0,"errmsg":"ok","datas":[{"base_check_time":1600000000}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let data = oa
        .get_checkin_day_data(ts(1600000000), ts(1600003600), &["zhangsan"])
        .await
        .expect("获取日报成功");
    assert_eq!(data.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/checkin/getcheckin_daydata")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCheckinMonthData` — 打卡月报数据。
#[tokio::test]
async fn oa_get_checkin_month_data() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/checkin/getcheckin_monthdata") {
            json(r#"{"errcode":0,"errmsg":"ok","datas":[{"userid":"zhangsan","days":20}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let data = oa
        .get_checkin_month_data(ts(1600000000), ts(1600003600), &["zhangsan"])
        .await
        .expect("获取月报成功");
    assert_eq!(data.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/checkin/getcheckin_monthdata")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetCheckinScheduleList` — 获取排班信息。
#[tokio::test]
async fn oa_get_checkin_schedule_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/checkin/getcheckinschedulist") {
            json(r#"{"errcode":0,"errmsg":"ok","schedule_list":[{"userid":"zhangsan"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let list = oa
        .get_checkin_schedule_list(ts(1600000000), ts(1600003600), &["zhangsan"])
        .await
        .expect("获取排班成功");
    assert_eq!(list.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/checkin/getcheckinschedulist")
    );
}

/// 对应 Java: `WxCpOaServiceImplTest.testGetApprovalInfo` — size 越界报错。
#[tokio::test]
async fn oa_get_approval_info_with_cursor_size_error() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let oa = WxCpOaServiceImpl::new(weak(&svc));
    let err = oa
        .get_approval_info_with_cursor(ts(1600000000), ts(1600003600), None, Some(101), None)
        .await
        .unwrap_err();
    assert!(format!("{err}").contains("size"), "错误: {err}");
}

/// OA 门面释放 → -99。
#[tokio::test]
async fn oa_service_released_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak_ref = {
        let svc = service_with_host(&server.url(""));
        weak(&svc)
    };
    let oa = WxCpOaServiceImpl::new(weak_ref);
    let err = oa.get_corp_conf().await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
    assert!(format!("{err}").contains("已释放"), "错误: {err}");
}

// ===========================================================================
// OA weDoc（WxCpOaWeDocServiceImpl）
// ===========================================================================

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocCreate` — 新建文档。
#[tokio::test]
async fn wedoc_create_returns_doc_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/create_doc") {
            json(r#"{"errcode":0,"errmsg":"ok","docid":"DOC_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocCreateRequest {
        space_id: "SPACE_1".to_string(),
        doc_type: 1,
        doc_name: "测试文档".to_string(),
        ..Default::default()
    };
    let data = doc_svc.doc_create(&req).await.expect("创建文档成功");
    assert_eq!(data.doc_id, "DOC_001");
    assert!(server.last_path().contains("/cgi-bin/wedoc/create_doc"));
    let body = server.last_body();
    assert!(body.contains(r#""spaceid":"SPACE_1""#), "body: {body}");
    assert!(body.contains(r#""doc_name":"测试文档""#), "body: {body}");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocRename` — 重命名文档。
#[tokio::test]
async fn wedoc_rename_post_docid_and_new_name() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/rename_doc") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocRenameRequest {
        doc_id: "DOC_001".to_string(),
        new_name: "新名称".to_string(),
        ..Default::default()
    };
    doc_svc.doc_rename(&req).await.expect("重命名成功");
    assert!(server.last_path().contains("/cgi-bin/wedoc/rename_doc"));
    let body = server.last_body();
    assert!(body.contains(r#""docid":"DOC_001""#), "body: {body}");
    assert!(body.contains(r#""new_name":"新名称""#), "body: {body}");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocDelete` — 删除文档（docid 分支）。
#[tokio::test]
async fn wedoc_delete_with_doc_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/del_doc") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    doc_svc
        .doc_delete(Some("DOC_001"), None)
        .await
        .expect("删除成功");
    assert!(server.last_path().contains("/cgi-bin/wedoc/del_doc"));
    let body = server.last_body();
    assert!(body.contains(r#""docid":"DOC_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocDelete` — 删除文档（formid 分支）。
#[tokio::test]
async fn wedoc_delete_with_form_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/del_doc") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    doc_svc
        .doc_delete(None, Some("FORM_001"))
        .await
        .expect("删除成功");
    let body = server.last_body();
    assert!(body.contains(r#""formid":"FORM_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocInfo` — 获取文档信息。
#[tokio::test]
async fn wedoc_info_post_docid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/get_doc_base_info") {
            json(r#"{"errcode":0,"errmsg":"ok","doc_base_info":{"docid":"DOC_001","doc_name":"测试文档","doc_type":1}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let info = doc_svc.doc_info("DOC_001").await.expect("获取文档信息成功");
    assert_eq!(info.doc_base_info.doc_name, "测试文档");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/wedoc/get_doc_base_info")
    );
    let body = server.last_body();
    assert!(body.contains(r#""docid":"DOC_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocShare(String)` — 分享文档。
#[tokio::test]
async fn wedoc_share_post_docid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/doc_share") {
            json(r#"{"errcode":0,"errmsg":"ok","share_url":"https://example.com/share"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let share = doc_svc.doc_share("DOC_001").await.expect("分享成功");
    assert_eq!(share.share_url, "https://example.com/share");
    assert!(server.last_path().contains("/cgi-bin/wedoc/doc_share"));
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocShare(WxCpDocShareRequest)` — 分享文档（formid 分支）。
#[tokio::test]
async fn wedoc_share_with_request_form_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/doc_share") {
            json(r#"{"errcode":0,"errmsg":"ok","share_url":"https://example.com/fshare"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocShareRequest {
        form_id: "FORM_001".to_string(),
        ..Default::default()
    };
    let share = doc_svc
        .doc_share_with_request(&req)
        .await
        .expect("分享成功");
    assert_eq!(share.share_url, "https://example.com/fshare");
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocGetAuth` — 获取文档权限。
#[tokio::test]
async fn wedoc_get_auth() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/doc_get_auth") {
            json(r#"{"errcode":0,"errmsg":"ok","auth_info":{"can_read":true}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    doc_svc.doc_get_auth("DOC_001").await.expect("获取权限成功");
    assert!(server.last_path().contains("/cgi-bin/wedoc/doc_get_auth"));
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocModifySafetySetting` — 修改安全设置。
#[tokio::test]
async fn wedoc_modify_safety_setting() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/mod_doc_safty_setting") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocModifySafetySettingRequest {
        doc_id: "DOC_001".to_string(),
        enable_readonly_copy: true,
        ..Default::default()
    };
    doc_svc
        .doc_modify_safety_setting(&req)
        .await
        .expect("修改安全设置成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/wedoc/mod_doc_safty_setting")
    );
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocAddAdmin` — 添加管理员。
#[tokio::test]
async fn wedoc_add_admin() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/add_admin") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocAdminRequest {
        doc_id: "DOC_001".to_string(),
        user_id: "zhangsan".to_string(),
        ..Default::default()
    };
    doc_svc.doc_add_admin(&req).await.expect("添加管理员成功");
    assert!(server.last_path().contains("/cgi-bin/wedoc/add_admin"));
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocDeleteAdmin` — 删除管理员。
#[tokio::test]
async fn wedoc_delete_admin() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/del_admin") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let req = WxCpDocAdminRequest {
        doc_id: "DOC_001".to_string(),
        user_id: "zhangsan".to_string(),
        ..Default::default()
    };
    doc_svc
        .doc_delete_admin(&req)
        .await
        .expect("删除管理员成功");
    assert!(server.last_path().contains("/cgi-bin/wedoc/del_admin"));
}

/// 对应 Java: `WxCpOaWeDocServiceImplTest.testDocGetAdminList` — 获取管理员列表。
#[tokio::test]
async fn wedoc_get_admin_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/wedoc/get_admin_list") {
            json(r#"{"errcode":0,"errmsg":"ok","admin_list":[{"userid":"zhangsan"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak(&svc));
    let result = doc_svc
        .doc_get_admin_list("DOC_001")
        .await
        .expect("获取管理员列表成功");
    assert_eq!(result.admin_list.len(), 1);
    assert!(server.last_path().contains("/cgi-bin/wedoc/get_admin_list"));
}

/// weDoc 门面释放 → -99。
#[tokio::test]
async fn wedoc_service_released_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak_ref = {
        let svc = service_with_host(&server.url(""));
        weak(&svc)
    };
    let doc_svc = WxCpOaWeDocServiceImpl::new(weak_ref);
    let err = doc_svc.doc_info("DOC_001").await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
}

// ===========================================================================
// OA 日历（WxCpOaCalendarServiceImpl）
// ===========================================================================

/// 对应 Java: `WxCpOaCalendarServiceImplTest.testAdd` — 添加日历。
#[tokio::test]
async fn calendar_add_returns_raw_response() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/calendar/add") {
            json(r#"{"errcode":0,"errmsg":"ok","cal_id":"CAL_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let cal_svc = WxCpOaCalendarServiceImpl::new(weak(&svc));
    let cal = WxCpOaCalendar {
        cal_id: "CAL_001".to_string(),
        summary: "我的日历".to_string(),
        ..Default::default()
    };
    let resp = cal_svc.add(&cal).await.expect("添加日历成功");
    assert!(resp.contains("CAL_001"), "响应: {resp}");
    assert!(server.last_path().contains("/cgi-bin/oa/calendar/add"));
    let body = server.last_body();
    assert!(body.contains(r#""summary":"我的日历""#), "body: {body}");
}

/// 对应 Java: `WxCpOaCalendarServiceImplTest.testUpdate` — 更新日历。
#[tokio::test]
async fn calendar_update_post_body() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/calendar/update") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let cal_svc = WxCpOaCalendarServiceImpl::new(weak(&svc));
    let cal = WxCpOaCalendar {
        cal_id: "CAL_001".to_string(),
        summary: "更新后的日历".to_string(),
        ..Default::default()
    };
    cal_svc.update(&cal).await.expect("更新日历成功");
    assert!(server.last_path().contains("/cgi-bin/oa/calendar/update"));
    let body = server.last_body();
    assert!(body.contains(r#""cal_id":"CAL_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaCalendarServiceImplTest.testGet` — 获取日历。
#[tokio::test]
async fn calendar_get_parses_calendar_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/calendar/get") {
            json(r#"{"errcode":0,"errmsg":"ok","calendar_list":[{"cal_id":"CAL_001","summary":"日历一"},{"cal_id":"CAL_002","summary":"日历二"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let cal_svc = WxCpOaCalendarServiceImpl::new(weak(&svc));
    let list = cal_svc
        .get(&["CAL_001", "CAL_002"])
        .await
        .expect("获取日历成功");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].cal_id, "CAL_001");
    assert_eq!(list[0].summary, "日历一");
    assert_eq!(list[1].cal_id, "CAL_002");
    assert!(server.last_path().contains("/cgi-bin/oa/calendar/get"));
    let body = server.last_body();
    assert!(body.contains(r#"["CAL_001","CAL_002"]"#), "body: {body}");
}

/// 对应 Java: `WxCpOaCalendarServiceImplTest.testDelete` — 删除日历。
#[tokio::test]
async fn calendar_delete_post_cal_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/calendar/del") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let cal_svc = WxCpOaCalendarServiceImpl::new(weak(&svc));
    cal_svc.delete("CAL_001").await.expect("删除日历成功");
    assert!(server.last_path().contains("/cgi-bin/oa/calendar/del"));
    let body = server.last_body();
    assert!(body.contains(r#""cal_id":"CAL_001""#), "body: {body}");
}

/// 日历门面释放 → -99。
#[tokio::test]
async fn calendar_service_released_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak_ref = {
        let svc = service_with_host(&server.url(""));
        weak(&svc)
    };
    let cal_svc = WxCpOaCalendarServiceImpl::new(weak_ref);
    let err = cal_svc.get(&["CAL_001"]).await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
}

// ===========================================================================
// OA 日程（WxCpOaScheduleServiceImpl）
// ===========================================================================

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testAdd` — 添加日程（无 agentId）。
#[tokio::test]
async fn schedule_add_without_agent_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/add") {
            json(r#"{"errcode":0,"errmsg":"ok","schedule_id":"SCH_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    let schedule = WxCpOaSchedule {
        schedule_id: "SCH_001".to_string(),
        summary: "测试日程".to_string(),
        ..Default::default()
    };
    let resp = sch_svc.add(&schedule, None).await.expect("添加日程成功");
    assert!(resp.contains("SCH_001"), "响应: {resp}");
    assert!(server.last_path().contains("/cgi-bin/oa/schedule/add"));
    let body = server.last_body();
    assert!(!body.contains("agentid"), "body 不应含 agentid: {body}");
}

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testAdd` — 添加日程（带 agentId）。
#[tokio::test]
async fn schedule_add_with_agent_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/add") {
            json(r#"{"errcode":0,"errmsg":"ok","schedule_id":"SCH_002"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    let schedule = WxCpOaSchedule::default();
    sch_svc
        .add(&schedule, Some(3010040))
        .await
        .expect("添加日程成功");
    let body = server.last_body();
    assert!(body.contains(r#""agentid":3010040"#), "body: {body}");
}

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testUpdate` — 更新日程。
#[tokio::test]
async fn schedule_update_post_body() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/update") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    let schedule = WxCpOaSchedule {
        schedule_id: "SCH_001".to_string(),
        summary: "更新后的日程".to_string(),
        ..Default::default()
    };
    sch_svc.update(&schedule).await.expect("更新日程成功");
    assert!(server.last_path().contains("/cgi-bin/oa/schedule/update"));
    let body = server.last_body();
    assert!(body.contains(r#""schedule_id":"SCH_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testGetDetails` — 获取日程详情。
#[tokio::test]
async fn schedule_get_details_parses_schedule_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/get") {
            json(r#"{"errcode":0,"errmsg":"ok","schedule_list":[{"schedule_id":"SCH_001","summary":"日程一"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    let list = sch_svc
        .get_details(&["SCH_001"])
        .await
        .expect("获取日程详情成功");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].schedule_id, "SCH_001");
    assert_eq!(list[0].summary, "日程一");
    assert!(server.last_path().contains("/cgi-bin/oa/schedule/get"));
    let body = server.last_body();
    assert!(body.contains(r#"["SCH_001"]"#), "body: {body}");
}

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testDelete` — 删除日程。
#[tokio::test]
async fn schedule_delete_post_schedule_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/del") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    sch_svc.delete("SCH_001").await.expect("删除日程成功");
    assert!(server.last_path().contains("/cgi-bin/oa/schedule/del"));
    let body = server.last_body();
    assert!(body.contains(r#""schedule_id":"SCH_001""#), "body: {body}");
}

/// 对应 Java: `WxCpOaOaScheduleServiceImplTest.testListByCalendar` — 按日历获取日程。
#[tokio::test]
async fn schedule_list_by_calendar_with_offset_limit() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/oa/schedule/get_by_calendar") {
            json(r#"{"errcode":0,"errmsg":"ok","schedule_list":[{"schedule_id":"SCH_001"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak(&svc));
    let list = sch_svc
        .list_by_calendar("CAL_001", Some(0), Some(10))
        .await
        .expect("获取日程列表成功");
    assert_eq!(list.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/oa/schedule/get_by_calendar")
    );
    let body = server.last_body();
    assert!(body.contains(r#""cal_id":"CAL_001""#), "body: {body}");
    assert!(body.contains(r#""offset":0"#), "body: {body}");
    assert!(body.contains(r#""limit":10"#), "body: {body}");
}

/// 日程门面释放 → -99。
#[tokio::test]
async fn schedule_service_released_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak_ref = {
        let svc = service_with_host(&server.url(""));
        weak(&svc)
    };
    let sch_svc = WxCpOaScheduleServiceImpl::new(weak_ref);
    let err = sch_svc.get_details(&["SCH_001"]).await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
}

// ===========================================================================
// 外部联系人（WxCpExternalContactServiceImpl）
// ===========================================================================

/// 对应 Java: `WxCpExternalContactServiceImplTest.testGetContactWay` — 获取「联系我」方式。
#[tokio::test]
async fn ext_contact_get_contact_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_contact_way") {
            json(r#"{"errcode":0,"errmsg":"ok","contact_way":{"config_id":"CFG_001","remark":"测试","style":1}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let info = ext
        .get_contact_way("CFG_001")
        .await
        .expect("获取联系我方式成功");
    assert_eq!(info.contact_way.config_id, "CFG_001");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/get_contact_way")
    );
    let body = server.last_body();
    assert!(body.contains(r#""config_id":"CFG_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testUpdateContactWay` — 更新「联系我」方式。
#[tokio::test]
async fn ext_contact_update_contact_way() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let mut info = WxCpContactWayInfo::default();
    info.contact_way.config_id = "CFG_001".to_string();
    info.contact_way.remark = "更新备注".to_string();
    ext.update_contact_way(&info).await.expect("更新成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/update_contact_way")
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testUpdateContactWay` — 缺 configId 报错。
#[tokio::test]
async fn ext_contact_update_contact_way_no_config_id() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let info = WxCpContactWayInfo::default(); // config_id 为空
    let err = ext.update_contact_way(&info).await.unwrap_err();
    assert!(format!("{err}").contains("configId"), "错误: {err}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testDeleteContactWay` — 删除「联系我」方式。
#[tokio::test]
async fn ext_contact_delete_contact_way() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.delete_contact_way("CFG_001").await.expect("删除成功");
    let body = server.last_body();
    assert!(body.contains(r#""config_id":"CFG_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testConvertToOpenid` — external_userid 转 openid。
#[tokio::test]
async fn ext_contact_convert_to_openid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/convert_to_openid") {
            json(r#"{"errcode":0,"errmsg":"ok","openid":"OPEN_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let openid = ext.convert_to_openid("EXT_001").await.expect("转换成功");
    assert_eq!(openid, "OPEN_001");
    let body = server.last_body();
    assert!(
        body.contains(r#""external_userid":"EXT_001""#),
        "body: {body}"
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testUnionidToExternalUserid` — unionid 转换。
#[tokio::test]
async fn ext_contact_unionid_to_external_userid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/unionid_to_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid":"EXT_FROM_UNION"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let ext_id = ext
        .unionid_to_external_userid("UNION_001", "OPEN_001")
        .await
        .expect("转换成功");
    assert_eq!(ext_id, "EXT_FROM_UNION");
    let body = server.last_body();
    assert!(body.contains(r#""unionid":"UNION_001""#), "body: {body}");
    assert!(body.contains(r#""openid":"OPEN_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testGetCorpTagList` — 获取标签（含 group_id）。
#[tokio::test]
async fn ext_contact_get_corp_tag_list_with_group_id() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_corp_tag_list") {
            json(r#"{"errcode":0,"errmsg":"ok","tag_group":[{"group_id":"GRP_001","group_name":"分组A","tag":[{"id":"TAG_001","name":"标签1"}]}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let result = ext
        .get_corp_tag_list_with_group_id(&[], &["GRP_001"])
        .await
        .expect("获取标签成功");
    assert_eq!(result.tag_group_list.len(), 1);
    assert_eq!(result.tag_group_list[0].group_id, "GRP_001");
    let body = server.last_body();
    assert!(body.contains(r#""group_id":["GRP_001"]"#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testAddCorpTag` — 添加企业标签。
#[tokio::test]
async fn ext_contact_add_corp_tag() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/add_corp_tag") {
            json(r#"{"errcode":0,"errmsg":"ok","tag_group":{"group_id":"GRP_NEW","tag":[{"id":"TAG_NEW","name":"新标签"}]}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let info = WxCpUserExternalTagGroupInfo {
        tag_group: TagGroup {
            group_name: "新分组".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = ext.add_corp_tag(&info).await.expect("添加标签成功");
    assert_eq!(result.tag_group.group_id, "GRP_NEW");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/add_corp_tag")
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testEditCorpTag` — 编辑标签。
#[tokio::test]
async fn ext_contact_edit_corp_tag() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.edit_corp_tag("TAG_001", Some("新名称"), Some(5))
        .await
        .expect("编辑标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""id":"TAG_001""#), "body: {body}");
    assert!(body.contains(r#""name":"新名称""#), "body: {body}");
    assert!(body.contains(r#""order":5"#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testDelCorpTag` — 删除标签。
#[tokio::test]
async fn ext_contact_del_corp_tag() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.del_corp_tag(&["TAG_001"], &["GRP_001"])
        .await
        .expect("删除标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""tag_id":["TAG_001"]"#), "body: {body}");
    assert!(body.contains(r#""group_id":["GRP_001"]"#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testMarkTag` — 标记客户标签。
#[tokio::test]
async fn ext_contact_mark_tag() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.mark_tag("zhangsan", "EXT_001", &["TAG_A"], &["TAG_B"])
        .await
        .expect("标记标签成功");
    let body = server.last_body();
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
    assert!(
        body.contains(r#""external_userid":"EXT_001""#),
        "body: {body}"
    );
    assert!(body.contains(r#""add_tag":["TAG_A"]"#), "body: {body}");
    assert!(body.contains(r#""remove_tag":["TAG_B"]"#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testAddJoinWay` — 添加客户群进群方式。
#[tokio::test]
async fn ext_contact_add_join_way() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/groupchat/add_join_way") {
            json(r#"{"errcode":0,"errmsg":"ok","config_id":"JW_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let info = WxCpGroupJoinWayInfo {
        join_way: JoinWay {
            scene: 1,
            remark: "进群".to_string(),
            chat_id_list: vec!["CHAT_001".to_string()],
            ..Default::default()
        },
    };
    let result = ext.add_join_way(&info).await.expect("添加进群方式成功");
    assert_eq!(result.config_id, "JW_001");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/groupchat/add_join_way")
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testAddJoinWay` — 超过 5 个群 ID 报错。
#[tokio::test]
async fn ext_contact_add_join_way_chat_id_limit() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let info = WxCpGroupJoinWayInfo {
        join_way: JoinWay {
            chat_id_list: vec![
                "1".into(),
                "2".into(),
                "3".into(),
                "4".into(),
                "5".into(),
                "6".into(),
            ],
            ..Default::default()
        },
    };
    let err = ext.add_join_way(&info).await.unwrap_err();
    assert!(format!("{err}").contains("5个"), "错误: {err}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testTransferCustomer` — 转接客户。
#[tokio::test]
async fn ext_contact_transfer_customer() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/transfer_customer") {
            json(r#"{"errcode":0,"errmsg":"ok","customer":[{"external_userid":"EXT_001","errcode":0}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let req = WxCpUserTransferCustomerReq {
        hand_over_userid: "zhangsan".to_string(),
        take_over_userid: "lisi".to_string(),
        external_userid: vec!["EXT_001".to_string()],
        transfer_msg: "转接".to_string(),
    };
    let resp = ext.transfer_customer(&req).await.expect("转接成功");
    assert_eq!(resp.customer.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/transfer_customer")
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testRemindGroupMsgSend` — 提醒群发。
#[tokio::test]
async fn ext_contact_remind_group_msg_send() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.remind_group_msg_send("MSG_001")
        .await
        .expect("提醒成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgid":"MSG_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testCancelGroupMsgSend` — 停止群发。
#[tokio::test]
async fn ext_contact_cancel_group_msg_send() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.cancel_group_msg_send("MSG_001")
        .await
        .expect("停止成功");
    let body = server.last_body();
    assert!(body.contains(r#""msgid":"MSG_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testGetInterceptRuleList` — 获取敏感词规则列表。
#[tokio::test]
async fn ext_contact_get_intercept_rule_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_intercept_rule_list") {
            json(r#"{"errcode":0,"errmsg":"ok","rule_list":[{"rule_id":"RULE_001","rule_name":"敏感词1"}]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let list = ext
        .get_intercept_rule_list()
        .await
        .expect("获取敏感词列表成功");
    assert_eq!(list.rule_list.len(), 1);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/get_intercept_rule_list")
    );
    assert_eq!(server.last_method(), "GET");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testAddInterceptRule` — 添加敏感词规则。
#[tokio::test]
async fn ext_contact_add_intercept_rule() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/add_intercept_rule") {
            json(r#"{"errcode":0,"errmsg":"ok","rule_id":"RULE_NEW"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let req = WxCpInterceptRuleAddRequest::default();
    let rule_id = ext
        .add_intercept_rule(&req)
        .await
        .expect("添加敏感词规则成功");
    assert_eq!(rule_id, "RULE_NEW");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/add_intercept_rule")
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testCloseTempChat` — 结束临时会话。
#[tokio::test]
async fn ext_contact_close_temp_chat() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.close_temp_chat("zhangsan", "EXT_001")
        .await
        .expect("结束会话成功");
    let body = server.last_body();
    assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
    assert!(
        body.contains(r#""external_userid":"EXT_001""#),
        "body: {body}"
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testGetFollowUserList` — 获取配置了客户联系功能的成员。
#[tokio::test]
async fn ext_contact_list_followers() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/get_follow_user_list") {
            json(r#"{"errcode":0,"errmsg":"ok","follow_user":["zhangsan","lisi"]}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let followers = ext.list_followers().await.expect("获取成员列表成功");
    assert_eq!(followers, vec!["zhangsan".to_string(), "lisi".to_string()]);
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/externalcontact/get_follow_user_list")
    );
    assert_eq!(server.last_method(), "GET");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testOpengidToChatid` — opengid 转 chatid。
#[tokio::test]
async fn ext_contact_opengid_to_chatid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/opengid_to_chatid") {
            json(r#"{"errcode":0,"errmsg":"ok","chat_id":"CHAT_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let chat_id = ext
        .opengid_to_chatid("OPENGID_001")
        .await
        .expect("转换成功");
    assert_eq!(chat_id, "CHAT_001");
    let body = server.last_body();
    assert!(body.contains(r#""opengid":"OPENGID_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testToServiceExternalUserid` — 代开发应用转换。
#[tokio::test]
async fn ext_contact_to_service_external_userid() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/to_service_external_userid") {
            json(r#"{"errcode":0,"errmsg":"ok","external_userid":"SVC_EXT_001"}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let ext_id = ext
        .to_service_external_userid("EXT_001")
        .await
        .expect("转换成功");
    assert_eq!(ext_id, "SVC_EXT_001");
    let body = server.last_body();
    assert!(
        body.contains(r#""external_userid":"EXT_001""#),
        "body: {body}"
    );
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testGetGroupWelcomeTemplate` — 获取入群欢迎语。
#[tokio::test]
async fn ext_contact_get_group_welcome_template() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/externalcontact/group_welcome_template/get") {
            json(r#"{"errcode":0,"errmsg":"ok","template_id":"TPL_001","text":{"content":"欢迎"}}"#)
        } else {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        }
    }))
    .await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    let result = ext
        .get_group_welcome_template("TPL_001")
        .await
        .expect("获取欢迎语成功");
    assert_eq!(result.template_id, "TPL_001");
    let body = server.last_body();
    assert!(body.contains(r#""template_id":"TPL_001""#), "body: {body}");
}

/// 对应 Java: `WxCpExternalContactServiceImplTest.testDelGroupWelcomeTemplate` — 删除入群欢迎语。
#[tokio::test]
async fn ext_contact_del_group_welcome_template_with_agent_id() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let svc = service_with_host(&server.url(""));
    let ext = WxCpExternalContactServiceImpl::new(weak(&svc));
    ext.del_group_welcome_template("TPL_001", Some("201"))
        .await
        .expect("删除成功");
    let body = server.last_body();
    assert!(body.contains(r#""template_id":"TPL_001""#), "body: {body}");
    assert!(body.contains(r#""agentid":"201""#), "body: {body}");
}

/// 外部联系人门面释放 → -99。
#[tokio::test]
async fn ext_contact_service_released_returns_99() {
    let server = MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
    let weak_ref = {
        let svc = service_with_host(&server.url(""));
        weak(&svc)
    };
    let ext = WxCpExternalContactServiceImpl::new(weak_ref);
    let err = ext.list_followers().await.unwrap_err();
    assert_eq!(err.error_code(), Some(-99));
}
