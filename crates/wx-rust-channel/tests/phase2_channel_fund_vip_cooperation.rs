#![allow(clippy::field_reassign_with_default)]
//! Phase 2 补齐: Fund / VIP / Cooperation 子域 Bean 序列化与 Mock 服务测试。
//!
//! 镜像 Java:
//! - `WxChannelFundServiceImplTest`（资金余额/流水/提现/银行账户/二维码）
//! - `WxChannelVipServiceImplTest`（会员信息/积分/等级）
//! - `WxStoreCooperationServiceImplTest`（合作员列表/状态/二维码/解绑）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/请求路径/请求体断言
//! - RUST_OBLIGATION: serde rename 语义、default 值、Vec 空值
//! - VALUE_ADD: 空值/边界路径

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use wx_rust_channel::api::WxChannelService;
use wx_rust_channel::api::r#impl::WxChannelServiceImpl;
use wx_rust_channel::bean::cooperation::*;
use wx_rust_channel::bean::fund::*;
use wx_rust_channel::bean::vip::*;
use wx_rust_channel::config::WxChannelConfig;
use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
use wx_rust_common::config::WxConfigStorage;

/// 极简 mock HTTP 服务器。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_path: Arc<std::sync::Mutex<String>>,
    stop: Arc<AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_body_clone = last_body.clone();
        let last_path_clone = last_path.clone();
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
                let last_body_clone = last_body_clone.clone();
                let last_path_clone = last_path_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    *last_path_clone.lock().unwrap() = path.clone();
                    let body = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
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
            last_body,
            last_path,
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

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

/// 构建指向 mock 服务器的配置。
fn config_with_host(host: &str) -> Arc<dyn WxChannelConfig> {
    let mut config = WxChannelDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    config.update_access_token("MOCK_TOKEN", 7200);
    config.set_api_host_url(host);
    Arc::new(config)
}

fn last_body_json(server: &MockServer) -> serde_json::Value {
    serde_json::from_str(&server.last_body()).expect("请求体 JSON")
}

fn new_service(config: Arc<dyn WxChannelConfig>) -> Arc<WxChannelServiceImpl> {
    WxChannelServiceImpl::new_arc(config)
}

// ═══ Fund 资金域 ═══

/// 获取账户余额（对应 Java `WxChannelFundServiceImplTest.testGetBalance`）。
/// 对应 Java: WxChannelFundServiceImplTest.testGetBalance
#[tokio::test]
async fn fund_get_balance() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/funds/getbalance") {
            r#"{"errcode":0,"errmsg":"ok","available_amount":10000,"pending_amount":5000,"sub_mchid":"sub123"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let fund = service.fund_service().expect("资金服务已装配");

    let resp = fund.get_balance().await.expect("获取余额成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.available_amount, 10000);
    assert_eq!(resp.pending_amount, 5000);
    let path = server.last_path();
    assert!(
        path.contains("/channels/ec/funds/getbalance"),
        "路径: {path}"
    );
    assert_eq!(server.last_body(), "{}");
}

/// 获取资金流水列表（对应 Java `WxChannelFundServiceImplTest.testListFundsFlow`）。
/// 对应 Java: WxChannelFundServiceImplTest.testListFundsFlow
#[tokio::test]
async fn fund_list_flow() {
    let server = MockServer::start(|path| {
        if path.contains("/channels/ec/funds/getfundsflowlist") {
            r#"{"errcode":0,"errmsg":"ok","flow_ids":["flow1","flow2"],"has_more":true,"next_key":"next123"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let fund = service.fund_service().expect("资金服务已装配");

    let mut param = FundsListParam::default();
    param.page_size = 10;
    let resp = fund.list_funds_flow(param).await.expect("获取流水列表成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.flow_ids, vec!["flow1", "flow2"]);
    assert!(resp.has_more);
    assert_eq!(resp.next_key, "next123");
    let body = last_body_json(&server);
    assert_eq!(body["page_size"], 10);
}

/// 提交提现（对应 Java `WxChannelFundServiceImplTest.testSubmitWithdraw`）。
/// 对应 Java: WxChannelFundServiceImplTest.testSubmitWithdraw
#[tokio::test]
async fn fund_submit_withdraw() {
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let fund = service.fund_service().expect("资金服务已装配");

    let resp = fund
        .submit_withdraw(Some(10000), "提现".to_string(), "备注".to_string())
        .await
        .expect("提交提现成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["amount"], 10000);
    assert_eq!(body["remark"], "提现");
    assert_eq!(body["bank_memo"], "备注");
}

/// 获取提现列表（对应 Java `WxChannelFundServiceImplTest.testListWithdraw`）。
/// 对应 Java: WxChannelFundServiceImplTest.testListWithdraw
#[tokio::test]
async fn fund_list_withdraw() {
    let server = MockServer::start(|_path| {
        r#"{"errcode":0,"errmsg":"ok","withdraw_ids":["w1","w2"]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let fund = service.fund_service().expect("资金服务已装配");

    let resp = fund
        .list_withdraw(Some(1), Some(10), None, None)
        .await
        .expect("获取提现列表成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.withdraw_ids, vec!["w1", "w2"]);
}

// ═══ VIP 会员域 ═══

/// 获取会员信息（对应 Java `WxChannelVipServiceImplTest.testGetVipInfo`）。
/// 对应 Java: WxChannelVipServiceImplTest.testGetVipInfo
#[tokio::test]
async fn vip_get_info() {
    let server = MockServer::start(|_path| {
        r#"{"errcode":0,"errmsg":"ok","info":{"openid":"ox123","union_id":"u1","user_grade_info":{"grade":2,"experience_value":"500"}}}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let vip = service.vip_service().expect("会员服务已装配");

    let resp = vip
        .get_vip_info("ox123".to_string(), None)
        .await
        .expect("获取会员信息成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.vip_info.open_id, "ox123");
    assert_eq!(resp.vip_info.user_grade_info.grade, 2);
    let body = last_body_json(&server);
    assert_eq!(body["openid"], "ox123");
}

/// 获取会员积分（对应 Java `WxChannelVipServiceImplTest.testGetVipScore`）。
/// 对应 Java: WxChannelVipServiceImplTest.testGetVipScore
#[tokio::test]
async fn vip_get_score() {
    let server = MockServer::start(|_path| {
        r#"{"errcode":0,"errmsg":"ok","info":{"score":"1000"}}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let vip = service.vip_service().expect("会员服务已装配");

    let resp = vip
        .get_vip_score("ox123".to_string())
        .await
        .expect("获取积分成功");
    assert_eq!(resp.err_code, 0);
}

/// 增加会员积分（对应 Java `WxChannelVipServiceImplTest.testIncreaseVipScore`）。
/// 对应 Java: WxChannelVipServiceImplTest.testIncreaseVipScore
#[tokio::test]
async fn vip_increase_score() {
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let vip = service.vip_service().expect("会员服务已装配");

    let resp = vip
        .increase_vip_score(
            "ox123".to_string(),
            "100".to_string(),
            "增加积分".to_string(),
            "req1".to_string(),
        )
        .await
        .expect("增加积分成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["openid"], "ox123");
    assert_eq!(body["score"], "100");
}

/// 获取会员列表（对应 Java `WxChannelVipServiceImplTest.testGetVipList`）。
/// 对应 Java: WxChannelVipServiceImplTest.testGetVipList
#[tokio::test]
async fn vip_get_list() {
    let server = MockServer::start(|_path| {
        r#"{"errcode":0,"errmsg":"ok","list":[{"openid":"ox1","union_id":"u1"}],"total_num":1}"#
            .to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let vip = service.vip_service().expect("会员服务已装配");

    let resp = vip
        .get_vip_list(Some(false), Some(1), Some(10))
        .await
        .expect("获取会员列表成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.vip_infos.len(), 1);
    assert_eq!(resp.vip_infos[0].open_id, "ox1");
    assert_eq!(resp.total_num, 1);
}

/// 更新会员等级（对应 Java `WxChannelVipServiceImplTest.testUpdateVipGrade`）。
/// 对应 Java: WxChannelVipServiceImplTest.testUpdateVipGrade
#[tokio::test]
async fn vip_update_grade() {
    let server = MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok"}"#.to_string()).await;
    let service = new_service(config_with_host(&server.url("")));
    let vip = service.vip_service().expect("会员服务已装配");

    let resp = vip
        .update_vip_grade("ox123".to_string(), Some(3))
        .await
        .expect("更新等级成功");
    assert_eq!(resp.err_code, 0);
    let body = last_body_json(&server);
    assert_eq!(body["openid"], "ox123");
    assert_eq!(body["grade"], 3);
}

// ═══ Cooperation 合作员域 ═══

/// 获取合作员列表（对应 Java `WxStoreCooperationServiceImplTest.testListCooperation`）。
/// 对应 Java: WxStoreCooperationServiceImplTest.testListCooperation
#[tokio::test]
async fn cooperation_list() {
    let server = MockServer::start(|_path| {
        r#"{"errcode":0,"errmsg":"ok","data_list":[{"sharer_id":"s1","status":1,"sharer_name":"合作员1"}]}"#.to_string()
    })
    .await;
    let service = new_service(config_with_host(&server.url("")));
    let coop = service.cooperation_service().expect("合作员服务已装配");

    let resp = coop
        .list_cooperation(None)
        .await
        .expect("获取合作员列表成功");
    assert_eq!(resp.err_code, 0);
}

/// 获取合作员状态（对应 Java `WxStoreCooperationServiceImplTest.testGetCooperationStatus`）。
/// 对应 Java: WxStoreCooperationServiceImplTest.testGetCooperationStatus
#[tokio::test]
async fn cooperation_get_status() {
    let server =
        MockServer::start(|_path| r#"{"errcode":0,"errmsg":"ok","data":{"status":1}}"#.to_string())
            .await;
    let service = new_service(config_with_host(&server.url("")));
    let coop = service.cooperation_service().expect("合作员服务已装配");

    let resp = coop
        .get_cooperation_status("sharer1".to_string(), None)
        .await
        .expect("获取合作员状态成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.data.status, 1);
}

// ═══ Bean 序列化（SOURCE_PARITY: Java bean 序列化测试）═══

/// BalanceInfoResponse serde（对应 Java `BalanceInfoResponse`）。
#[test]
fn test_balance_info_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","available_amount":10000,"pending_amount":5000,"sub_mchid":"sub123"}"#;
    let resp: BalanceInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.available_amount, 10000);
    assert_eq!(resp.sub_mchid, "sub123");
}

/// FlowListResponse serde。
#[test]
fn test_flow_list_response_serde() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","flow_ids":["f1","f2"],"has_more":true,"next_key":"nk"}"#;
    let resp: FlowListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.flow_ids, vec!["f1", "f2"]);
    assert!(resp.has_more);
}

/// FundsFlow serde。
#[test]
fn test_funds_flow_serde() {
    let json = r#"{"flow_id":"f1","funds_type":1,"flow_type":2,"amount":100,"balance":500,"bookkeeping_time":"2024-01-01","remark":"备注"}"#;
    let flow: FundsFlow = serde_json::from_str(json).unwrap();
    assert_eq!(flow.flow_id, "f1");
    assert_eq!(flow.amount, 100);
    assert_eq!(flow.balance, 500);
}

/// FundsListParam serde。
#[test]
fn test_funds_list_param_serde() {
    let param = FundsListParam {
        page_size: 10,
        start_time: 1700000000,
        end_time: 1700000100,
        ..Default::default()
    };
    let json = serde_json::to_string(&param).unwrap();
    assert!(json.contains("\"page_size\":10"));
}

/// VipInfo serde。
#[test]
fn test_vip_info_serde() {
    let json = r#"{"openid":"ox1","union_id":"u1","user_grade_info":{"grade":2,"experience_value":"500"}}"#;
    let info: VipInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.open_id, "ox1");
    assert_eq!(info.user_grade_info.grade, 2);
    assert_eq!(info.user_grade_info.experience_value, "500");
}

/// VipInfoResponse serde。
#[test]
fn test_vip_info_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","info":{"openid":"ox1","union_id":"u1","user_grade_info":{"grade":1,"experience_value":"0"}}}"#;
    let resp: VipInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.vip_info.open_id, "ox1");
}

/// VipScoreResponse serde。
#[test]
fn test_vip_score_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","info":{"score":"1000"}}"#;
    let resp: VipScoreResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.score_info.score, "1000");
}

/// CooperationData serde。
#[test]
fn test_cooperation_data_serde() {
    let json = r#"{"sharer_id":"s1","status":1,"sharer_name":"合作员","sharer_type":2,"bind_time":1700000000}"#;
    let data: CooperationData = serde_json::from_str(json).unwrap();
    assert_eq!(data.sharer_id, "s1");
    assert_eq!(data.status, 1);
    assert_eq!(data.sharer_name, "合作员");
    assert_eq!(data.bind_time, 1700000000);
}

/// CooperationStatus serde。
#[test]
fn test_cooperation_status_serde() {
    let json = r#"{"status":1}"#;
    let status: CooperationStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status.status, 1);
}

/// CooperationListResponse serde。
#[test]
fn test_cooperation_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","data_list":[{"sharer_id":"s1","status":1}]}"#;
    let resp: CooperationListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.data_list.len(), 1);
}

/// CooperationStatusResponse serde。
#[test]
fn test_cooperation_status_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":{"status":2}}"#;
    let resp: CooperationStatusResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.data.status, 2);
}

/// WithdrawListResponse serde。
#[test]
fn test_withdraw_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","withdraw_ids":["w1","w2"]}"#;
    let resp: WithdrawListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.withdraw_ids, vec!["w1", "w2"]);
}

// ═══ VALUE_ADD: 空值/边界 ═══

#[test]
fn test_balance_info_response_defaults() {
    let json = r#"{}"#;
    let resp: BalanceInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.available_amount, 0);
}

#[test]
fn test_flow_list_response_empty() {
    let json = r#"{"errcode":0,"errmsg":"ok","flow_ids":[],"has_more":false}"#;
    let resp: FlowListResponse = serde_json::from_str(json).unwrap();
    assert!(resp.flow_ids.is_empty());
    assert!(!resp.has_more);
}

#[test]
fn test_vip_info_response_empty_info() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: VipInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

/// UserGradeInfo serde（VALUE_ADD: 默认值）。
#[test]
fn test_user_grade_info_default() {
    let info = UserGradeInfo::default();
    assert_eq!(info.grade, 0);
    assert!(info.experience_value.is_empty());
}
