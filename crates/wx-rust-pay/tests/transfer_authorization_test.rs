#![allow(clippy::field_reassign_with_default)]
//! 商家转账用户授权接口测试（对应 Java `TransferUserAuthorizationApiCompatibilityTest`）。
//!
//! 测试覆盖：
//! 1. transfer_bills_with_authorization — 发起转账并完成免确认收款授权（POST + JSON body 断言）
//! 2. transfer_bills_after_authorization — 用户授权后转账
//! 3. user_confirm_authorization — 发起免确认收款授权
//! 4. get_user_confirm_authorization_by_out_authorization_no — 查询免确认收款授权结果
//! 5. close_user_confirm_authorization — 解除免确认收款授权
//! 6. parse_user_authorization_notify_result — 解析免确认收款授权结果通知
//! 7. bean serde round-trip — 各新 bean 序列化/反序列化
//!
//! MockServer 与 gold_plan_test.rs 同构：v3 响应由平台私钥签名。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::transfer::*;
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::sign_sha256_rsa;

// ---- 夹具常量（与 gold_plan_test.rs 同源） ----

const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";
const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";
const MERCHANT_SERIAL: &str = "5F1C72E2A8931B72A2E13ADE3BB492C7B9C71571";
const PLATFORM_SERIAL: &str = "PLATFORM_SERIAL_TEST_1";

const PLATFORM_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCsgx6q/ArrzTov
I8nSX42mKAwrX/Oh6s1n/vZm6OkPYxInYEYWP5FqzlyYRxiyNfpqdVAV6DcKoFfi
6LVILL5eJ9BgaKgf3K83WW9nvp828+oPe55yJaZGS9lzqE5DXLvoYujVVuMn9y9K
Gegp1qQWrtR2QoWZynnCIBlHCrmgSUxE083Hus9nUkD+c1yxfACF4CZoONVWNPU3
dxoo3/82JX2sF04EpvXxvsKpQv/gj2e16ayWp/bgJ9LK8Tm9RcKrmyyZPVLfpaRv
q2q64bk4zl1WuALtFbXN/FiGpgJ6UYGF0U5P8u7Ujf26mYjGTFNzbbbhH+Pz8aKu
j3cKT8hVAgMBAAECggEAKmsBNvTLcKihVmb74KULLOBrAZ1RyC32fMxF5Q7/A4AO
Pi1ffmf6ByOfw/ezXFqGbPfUjdZv9kZv5cTnkkwiMP+kLph9QnxefwMYMA/AkWAP
XOg+EP4t8NoEqXooaErcj2sGqjJ3n2OZtqRBIEx9Q3CQoFWCCUsufiurW3sJIp6L
3QhxRVBcCwNpRKusfQj9ti0kvNiB+DfWbzmJNNWn8RKMTQ6o5GJI1xked+/KdBsX
JJtH1CnZ3zQ/Lzh/JvPqM00RjozXJ8Tcg4I0XxOBoXjFhPhiPzYKED6+vbrhD0jG
+lvOgTaKBHTaG7mPHDpIAI8CS6pr9FWU25s5aikR8QKBgQDaSFgtpzmn8oM7qfm/
4HWZdK7IXS98NPguCgw2T3te3sdtXJaEDsGPE3nIEfQIBR4Sb42uVumrMfTWFyPw
WBdk3CIY/WDJSeB+t6NpQY4UCxg2KCQlsL7oNf+Fs4XbC50EKfiBSTHtC5QjPfE0
xvI4Q5NSTDLqqvb53S5hbpl+LQKBgQDKUiMWFCqt42bnx7cdKKFitCYlIWyj+4jI
n0BbA7VdN80uuy5fG8YykoYu5VaCC8Kekje70j57UpjHyyLSFPZNHEl1qku6/V9X
5doUkJgcqO1l2X/+st7JG/dXzS6lTA9AgQxQfkYfZ7jYe+qJdVNRr/3u0li+nuxs
GPAVsIPzyQKBgByM8dumz8fD1J5tMRmDxl34ARR7+8YHN57t+YLxCnXyNat8PMOx
GmgYaAlfL8gMw04uAR4YiB+4PQnzpOO+4fBzSpJyMcKiQxbP9jPCO08r1FLhFTlI
0O+WgIGAZbWs4zC0PKjMZhdXOF0TsQTyMKRjUlWlWdOP4RX1dCeSS8lRAoGAMDPN
hC8d0/v/wm0EpX/Oo3OEOwhxl8gP2KyIaqbBiQoIan/SFnrJh7b9HSMqryUaIyl2
+dqnZD5ThBZTn3W7ELgSkGQYUt6W3pw2jvu8IeflV4SwSABZr9Rn45VQ7bTnVVjp
lCvhjQ3rLjinskQvq6CggUvANPosHbz7Rk+dwHECgYAJlvp7uj+bczwTal2dI7KK
JvrNvMfakFZYMradg6PtvAnibGsbqWLg64eBbGWjC50b0X/o/sa4el+zfWW1OMQx
DklphGDUkue0ZkRIHA3qcJNXep8XpAVF1Dxvk24W0SCUvBj2AntqtWx1jQ7b+upy
L8MQQXLyOBWk8pn2EJa7rw==
-----END PRIVATE KEY-----";

const PLATFORM_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArIMeqvwK6806LyPJ0l+N
pigMK1/zoerNZ/72ZujpD2MSJ2BGFj+Ras5cmEcYsjX6anVQFeg3CqBX4ui1SCy+
XifQYGioH9yvN1lvZ76fNvPqD3ueciWmRkvZc6hOQ1y76GLo1VbjJ/cvShnoKdak
Fq7UdkKFmcp5wiAZRwq5oElMRNPNx7rPZ1JA/nNcsXwAheAmaDjVVjT1N3caKN//
NiV9rBdOBKb18b7CqUL/4I9ntemslqf24CfSyvE5vUXCq5ssmT1S36Wkb6tquuG5
OM5dVrgC7RW1zfxYhqYCelGBhdFOT/Lu1I39upmIxkxTc2224R/j8/Giro93Ck/I
VQIDAQAB
-----END PUBLIC KEY-----";

// ---- MockServer（与 gold_plan_test.rs 同构） ----

#[allow(dead_code)]
struct MockServer {
    addr: std::net::SocketAddr,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    last_method: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

#[allow(dead_code)]
impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &str, &HashMap<String, String>) -> (u16, String, String, Vec<(String, String)>)
            + Send
            + Sync
            + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let last_method = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let last_path_clone = last_path.clone();
        let last_body_clone = last_body.clone();
        let last_method_clone = last_method.clone();
        let stop_clone = stop.clone();
        tokio::spawn(async move {
            loop {
                if stop_clone.load(Ordering::SeqCst) {
                    break;
                }
                let Ok((mut socket, _)) = listener.accept().await else {
                    continue;
                };
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                let last_method_clone = last_method_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let mut lines = request.lines();
                    let request_line = lines.next().unwrap_or_default();
                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                    let method = parts.first().unwrap_or(&"GET").to_string();
                    let path = parts.get(1).unwrap_or(&"/").to_string();
                    *last_method_clone.lock().unwrap() = method.clone();
                    *last_path_clone.lock().unwrap() = path.clone();
                    let mut headers = HashMap::new();
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        }
                        if let Some((k, v)) = line.split_once(':') {
                            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
                        }
                    }
                    let body = lines.collect::<Vec<&str>>().join("\n");
                    *last_body_clone.lock().unwrap() = body;
                    let (status, content_type, resp_body, extra_headers) =
                        handler(&method, &path, &headers);
                    let mut response = format!(
                        "HTTP/1.1 {status} {}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
                        if status == 200 { "OK" } else { "Error" },
                        resp_body.len()
                    );
                    for (k, v) in extra_headers {
                        response.push_str(&format!("{k}: {v}\r\n"));
                    }
                    response.push_str("\r\n");
                    response.push_str(&resp_body);
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });

        Self {
            addr,
            last_path,
            last_body,
            last_method,
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

    fn last_method(&self) -> String {
        self.last_method.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

// ---- 测试辅助 ----

fn config_with_host(host: &str) -> Arc<dyn WxPayConfig> {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_api_v3_key(API_V3_KEY)
        .set_cert_serial_no(MERCHANT_SERIAL)
        .set_private_key(
            "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCaZzehUwzcxdLg
Gn/UNryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0xK70AP9b2+Ry5q+zXUU+d
PucJmgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPbmDNatzq9k5a/P454QRWM
dkLUJZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx+h7pf/87jcmXXCm+SZqw
5MtILKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU+17wCXqTcZsipAGoqfqr
Hx/sr30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9GdQWKIQCip3E5dLj5ZFkDe
Ov4Hekf9AgMBAAECggEAEsVsqnS90hNMzUj7dHHJHsgQRGeVlGc+tFzsHcGEDd1u
W7SUfKDQN6BjKgiuvBqGyFTFzL7dltnAS5YroWu0fMZCpMGOIhs2N1Go8/2j43PQ
/k9iMVUw2/JPQxmwWJ2BCy4nvA1+hRkohQCVpFQCzn4tdWYUzcdMrUw2y+h1fkCQ
5MJn7iw9QHKQSeFeCl1/xq2PvOtiK/r1LsckyKNSSNgFEfxyWYaKbnK9OH+5rFKQ
QuI+fnAgE6QiLvmW0NqqZUSfqkLKi/FSWI13ns0H6OxjqpLX8VQ6+Cw5qq8fCuv2
gzkVk8A85ZTCQL/q9qDilt9uAE0bE924WU+n2zkBoQKBgQDQvIufN6fKpm27k4yx
RNV23fj9nojewaVqGg/3yuyiAu6w/yFcTXkGMVOicTYraX2mliHTIoyP8ywKGqqa
XS/Kk3tGD1K04KriPiFwWXU54+DmOJEyYoJlmXOm4BoZ1lW0z5HECC9eO/VPSDY+
zQdRYSCTdSHEgYuOGQSLPPwdrQKBgQC9XTrvkUkIthayc+4IhV6m4kT8uwzuoc3f
uaJhFFcpLKqzcpQBH71TYCXfqkucnO0no0sGerBB4HJQoVRK+jsdhNZhcw1JKLau
E+YlCSRLZ62vyzBTzLw1fnFBp82z1VZBujCrMP+DdwXBTsnkRaUtmDEG0s6YsCwd
fayF0PB9kQKBgAHc/P4R2ByV+brH6WSXsbQa7SMObDhY0CovS18x34Tes9S+okSZ
qG/mttFnY01l5qo7AthIoaqTSBxa+pTgKhIL2PjaICnfK4dTeKbxFXvLzfEgJiOl
/3X6ta6Sp4j9gcxYYfu2+v1DWcA4a8uJtvwB+vF2BTQk1+MP1BuOEs4NAoGBAK3y
+HKdOUPBUPQ4vk4hhaMzcz/d67FB/UYo1lrrPm3aVCxnckHeECKIzgG6A58oIEor
HH4lMcgyD5C1wiLl3mvtXKlD8M5lkfoy2VToIukJomk783bnOXTCY/N12+X4cTYL
fS2k4vK24RiD8b25pFRP26ly+MkV/FBS46pBFsmhAoGAOjfl2vGyJo8CRQ/HBLsS
Bw2VQgRvZU7mom1qa2SKA2VlsFz/aiBCcT8XEsTJxkVYkvbdfbyx2Z9kCbI6Y2cz
Z+M/0rQSRv/eOiBW7anBImZMg6WcYNfRfumkuNq+6fcCKKtZuyrd7ZFc0jTcVasI
xNJ1TM6J6kRQdn5O6Ot4ERw=
-----END PRIVATE KEY-----",
        )
        .set_public_key_id("PUB_KEY_ID_TEST")
        .set_public_key_content(PLATFORM_PUBLIC_KEY_PEM.as_bytes().to_vec())
        .set_api_host_url(host);
    Arc::new(config)
}

/// v3 JSON 响应快捷构造（附加正确的微信支付响应签名头）。
fn signed_json_response(body: &str) -> (u16, String, String, Vec<(String, String)>) {
    let timestamp = "1712345678";
    let nonce = "testnonce1234";
    let message = format!("{timestamp}\n{nonce}\n{body}\n");
    let signature = sign_sha256_rsa(
        &load_private_key_from_pem(PLATFORM_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥"),
        message.as_bytes(),
    )
    .expect("响应签名");
    (
        200,
        "application/json".to_string(),
        body.to_string(),
        vec![
            ("Wechatpay-Timestamp".to_string(), timestamp.to_string()),
            ("Wechatpay-Nonce".to_string(), nonce.to_string()),
            ("Wechatpay-Signature".to_string(), signature),
            ("Wechatpay-Serial".to_string(), PLATFORM_SERIAL.to_string()),
        ],
    )
}

// ==================== Bean serde 测试 ====================

/// PreTransferWithAuthorizationRequest serde round-trip。
#[test]
fn test_pre_transfer_with_authorization_request_serde() {
    let original = PreTransferWithAuthorizationRequest {
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        out_bill_no: Some("BILL_001".to_string()),
        transfer_scene_id: Some("1001".to_string()),
        openid: Some("openid_001".to_string()),
        user_name: Some("张三".to_string()),
        transfer_amount: Some(100),
        transfer_remark: Some("转账备注".to_string()),
        notify_url: Some("https://example.com/notify".to_string()),
        user_recv_perception: Some("活动奖励".to_string()),
        transfer_scene_report_infos: vec![PreTransferTransferSceneReportInfo {
            info_type: Some("活动名称".to_string()),
            info_content: Some("测试活动".to_string()),
        }],
        authorization_info: Some(AuthorizationInfo {
            user_display_name: Some("用户A".to_string()),
            out_authorization_no: Some("AUTH_001".to_string()),
            authorization_notify_url: Some("https://example.com/auth_notify".to_string()),
        }),
        sponsor_mchid: None,
    };
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: PreTransferWithAuthorizationRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(original, restored);
}

/// PreTransferWithAuthorizationResult serde。
#[test]
fn test_pre_transfer_with_authorization_result_serde() {
    let json_str = r#"{"out_bill_no":"BILL_001","transfer_bill_no":"TB_001","create_time":"2025-01-01T00:00:00","state":"WAIT_USER_CONFIRM","package_info":"pkg_info","user_display_name":"用户A","out_authorization_no":"AUTH_001"}"#;
    let result: PreTransferWithAuthorizationResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.out_bill_no.as_deref(), Some("BILL_001"));
    assert_eq!(result.transfer_bill_no.as_deref(), Some("TB_001"));
    assert_eq!(result.state.as_deref(), Some("WAIT_USER_CONFIRM"));
    assert_eq!(result.package_info.as_deref(), Some("pkg_info"));
}

/// TransferBillsAfterAuthorizationRequest serde round-trip。
#[test]
fn test_transfer_bills_after_authorization_request_serde() {
    let original = TransferBillsAfterAuthorizationRequest {
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        out_bill_no: Some("BILL_002".to_string()),
        user_name: Some("李四".to_string()),
        transfer_amount: Some(200),
        transfer_remark: Some("授权后转账".to_string()),
        notify_url: None,
        user_recv_perception: None,
        transfer_scene_id: Some("1001".to_string()),
        transfer_scene_report_infos: vec![],
        authorization_id: Some("AUTH_ID_001".to_string()),
        sponsor_mchid: None,
        out_authorization_no: Some("OUT_AUTH_001".to_string()),
    };
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: TransferBillsAfterAuthorizationRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(original, restored);
}

/// TransferBillsAfterAuthorizationResult serde。
#[test]
fn test_transfer_bills_after_authorization_result_serde() {
    let json_str = r#"{"mch_id":"10000100","out_bill_no":"BILL_002","transfer_bill_no":"TB_002","appid":"wxd930ea5d5a258f4f","state":"SUCCESS","transfer_amount":200,"transfer_remark":"授权后转账","openid":"openid_002","create_time":"2025-01-01T00:00:00"}"#;
    let result: TransferBillsAfterAuthorizationResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.mch_id.as_deref(), Some("10000100"));
    assert_eq!(result.state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.transfer_amount, Some(200));
}

/// UserConfirmAuthorizationRequest serde round-trip。
#[test]
fn test_user_confirm_authorization_request_serde() {
    let original = UserConfirmAuthorizationRequest {
        out_authorization_no: Some("AUTH_003".to_string()),
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        openid: Some("openid_003".to_string()),
        transfer_scene_id: Some("1001".to_string()),
        user_display_name: Some("用户C".to_string()),
        user_recv_perception: Some("活动奖励".to_string()),
        authorization_notify_url: Some("https://example.com/auth_notify".to_string()),
        scene_info: Some(AuthSceneInfo {
            client_ip: Some("127.0.0.1".to_string()),
            device_id: Some("device_001".to_string()),
            device_type: Some("IOS".to_string()),
        }),
    };
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: UserConfirmAuthorizationRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(original, restored);
}

/// UserConfirmAuthorizationResult serde。
#[test]
fn test_user_confirm_authorization_result_serde() {
    let json_str = r#"{"out_authorization_no":"AUTH_003","appid":"wxd930ea5d5a258f4f","openid":"openid_003","user_display_name":"用户C","authorization_id":"WX_AUTH_003","state":"TAKING_EFFECT","authorize_time":"2025-01-01T00:00:00","transfer_scene_id":"1001","create_time":"2025-01-01T00:00:00","package_info":"pkg_info_003"}"#;
    let result: UserConfirmAuthorizationResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.out_authorization_no.as_deref(), Some("AUTH_003"));
    assert_eq!(result.state.as_deref(), Some("TAKING_EFFECT"));
    assert_eq!(result.authorization_id.as_deref(), Some("WX_AUTH_003"));
}

/// UserConfirmAuthorizationResult 含 close_info serde。
#[test]
fn test_user_confirm_authorization_result_with_close_info() {
    let json_str = r#"{"out_authorization_no":"AUTH_004","state":"CLOSED","close_info":{"close_time":"2025-01-02T00:00:00","close_reason":"CLOSE_VIA_MCH_API"}}"#;
    let result: UserConfirmAuthorizationResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.state.as_deref(), Some("CLOSED"));
    let close_info = result.close_info.unwrap();
    assert_eq!(
        close_info.close_reason.as_deref(),
        Some("CLOSE_VIA_MCH_API")
    );
}

/// UserAuthorizationNotifyResult serde。
#[test]
fn test_user_authorization_notify_result_serde() {
    let json_str = r#"{"result":{"out_authorization_no":"AUTH_005","appid":"wxd930ea5d5a258f4f","openid":"openid_005","user_display_name":"用户E","authorization_id":"WX_AUTH_005","state":"TAKING_EFFECT","authorize_time":"2025-01-01T00:00:00"}}"#;
    let result: UserAuthorizationNotifyResult = serde_json::from_str(json_str).unwrap();
    let decrypt = result.result.unwrap();
    assert_eq!(decrypt.out_authorization_no.as_deref(), Some("AUTH_005"));
    assert_eq!(decrypt.state.as_deref(), Some("TAKING_EFFECT"));
    assert_eq!(decrypt.authorization_id.as_deref(), Some("WX_AUTH_005"));
}

// ==================== TransferService 方法测试 ====================

/// 发起转账并完成免确认收款授权：POST 请求 + URL 断言 + 响应解析。
#[tokio::test]
async fn test_transfer_bills_with_authorization() {
    let response_body = json!({
        "out_bill_no": "BILL_PRE_001",
        "transfer_bill_no": "TB_PRE_001",
        "create_time": "2025-01-01T00:00:00",
        "state": "WAIT_USER_CONFIRM",
        "package_info": "pkg_info_pre_001",
        "user_display_name": "用户A",
        "out_authorization_no": "AUTH_PRE_001"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(
            path,
            "/v3/fund-app/mch-transfer/transfer-bills/pre-transfer-with-authorization"
        );
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let request = PreTransferWithAuthorizationRequest {
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        out_bill_no: Some("BILL_PRE_001".to_string()),
        transfer_scene_id: Some("1001".to_string()),
        openid: Some("openid_001".to_string()),
        transfer_amount: Some(100),
        transfer_remark: Some("测试转账".to_string()),
        authorization_info: Some(AuthorizationInfo {
            user_display_name: Some("用户A".to_string()),
            out_authorization_no: Some("AUTH_PRE_001".to_string()),
            authorization_notify_url: Some("https://example.com/notify".to_string()),
        }),
        ..Default::default()
    };
    let result = transfer_svc
        .transfer_bills_with_authorization(&request)
        .await
        .expect("发起转账并完成免确认收款授权成功");
    assert_eq!(result.out_bill_no.as_deref(), Some("BILL_PRE_001"));
    assert_eq!(result.state.as_deref(), Some("WAIT_USER_CONFIRM"));
    assert_eq!(result.package_info.as_deref(), Some("pkg_info_pre_001"));

    // 断言请求 body 包含 authorization_info
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["out_bill_no"], "BILL_PRE_001");
    assert_eq!(
        body["authorization_info"]["out_authorization_no"],
        "AUTH_PRE_001"
    );
}

/// 用户授权后转账：POST 请求 + URL 断言 + 响应解析。
#[tokio::test]
async fn test_transfer_bills_after_authorization() {
    let response_body = json!({
        "mch_id": "10000100",
        "out_bill_no": "BILL_AFTER_001",
        "transfer_bill_no": "TB_AFTER_001",
        "appid": "wxd930ea5d5a258f4f",
        "state": "SUCCESS",
        "transfer_amount": 200,
        "transfer_remark": "授权后转账",
        "openid": "openid_002",
        "create_time": "2025-01-01T00:00:00"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/fund-app/mch-transfer/transfer-bills/transfer");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let request = TransferBillsAfterAuthorizationRequest {
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        out_bill_no: Some("BILL_AFTER_001".to_string()),
        transfer_amount: Some(200),
        transfer_remark: Some("授权后转账".to_string()),
        transfer_scene_id: Some("1001".to_string()),
        authorization_id: Some("WX_AUTH_001".to_string()),
        ..Default::default()
    };
    let result = transfer_svc
        .transfer_bills_after_authorization(&request)
        .await
        .expect("用户授权后转账成功");
    assert_eq!(result.state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.transfer_amount, Some(200));
    assert_eq!(result.mch_id.as_deref(), Some("10000100"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["out_bill_no"], "BILL_AFTER_001");
    assert_eq!(body["authorization_id"], "WX_AUTH_001");
}

/// 发起免确认收款授权：POST 请求 + URL 断言 + 响应解析。
#[tokio::test]
async fn test_user_confirm_authorization() {
    let response_body = json!({
        "out_authorization_no": "AUTH_UC_001",
        "appid": "wxd930ea5d5a258f4f",
        "openid": "openid_003",
        "user_display_name": "用户C",
        "authorization_id": "WX_AUTH_UC_001",
        "state": "WAIT_USER_CONFIRM",
        "transfer_scene_id": "1001",
        "create_time": "2025-01-01T00:00:00",
        "package_info": "pkg_info_uc_001"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/fund-app/mch-transfer/user-confirm-authorization");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let request = UserConfirmAuthorizationRequest {
        out_authorization_no: Some("AUTH_UC_001".to_string()),
        appid: Some("wxd930ea5d5a258f4f".to_string()),
        openid: Some("openid_003".to_string()),
        transfer_scene_id: Some("1001".to_string()),
        user_display_name: Some("用户C".to_string()),
        authorization_notify_url: Some("https://example.com/auth_notify".to_string()),
        ..Default::default()
    };
    let result = transfer_svc
        .user_confirm_authorization(&request)
        .await
        .expect("发起免确认收款授权成功");
    assert_eq!(result.out_authorization_no.as_deref(), Some("AUTH_UC_001"));
    assert_eq!(result.state.as_deref(), Some("WAIT_USER_CONFIRM"));
    assert_eq!(result.package_info.as_deref(), Some("pkg_info_uc_001"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["out_authorization_no"], "AUTH_UC_001");
    assert_eq!(body["openid"], "openid_003");
}

/// 查询免确认收款授权结果：GET 请求 + URL 含路径参数和查询参数。
#[tokio::test]
async fn test_get_user_confirm_authorization_by_out_authorization_no() {
    let response_body = json!({
        "out_authorization_no": "AUTH_QRY_001",
        "appid": "wxd930ea5d5a258f4f",
        "openid": "openid_004",
        "state": "TAKING_EFFECT",
        "authorization_id": "WX_AUTH_QRY_001",
        "authorize_time": "2025-01-01T00:00:00"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains(
            "/v3/fund-app/mch-transfer/user-confirm-authorization/out-authorization-no/AUTH_QRY_001"
        ));
        assert!(path.contains("is_display_authorization=true"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let result = transfer_svc
        .get_user_confirm_authorization_by_out_authorization_no("AUTH_QRY_001", Some(true))
        .await
        .expect("查询免确认收款授权结果成功");
    assert_eq!(result.state.as_deref(), Some("TAKING_EFFECT"));
    assert_eq!(result.authorization_id.as_deref(), Some("WX_AUTH_QRY_001"));
}

/// 查询免确认收款授权结果（无 is_display_authorization 参数）。
#[tokio::test]
async fn test_get_user_confirm_authorization_no_display_param() {
    let response_body = json!({
        "out_authorization_no": "AUTH_QRY_002",
        "state": "TAKING_EFFECT"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains(
            "/v3/fund-app/mch-transfer/user-confirm-authorization/out-authorization-no/AUTH_QRY_002"
        ));
        assert!(!path.contains("is_display_authorization"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let result = transfer_svc
        .get_user_confirm_authorization_by_out_authorization_no("AUTH_QRY_002", None)
        .await
        .expect("查询免确认收款授权结果成功");
    assert_eq!(result.state.as_deref(), Some("TAKING_EFFECT"));
}

/// 解除免确认收款授权：POST 请求 + URL 含路径参数。
#[tokio::test]
async fn test_close_user_confirm_authorization() {
    let response_body = json!({
        "out_authorization_no": "AUTH_CLOSE_001",
        "state": "CLOSED",
        "close_info": {
            "close_time": "2025-01-02T00:00:00",
            "close_reason": "CLOSE_VIA_MCH_API"
        }
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(
            path,
            "/v3/fund-app/mch-transfer/user-confirm-authorization/out-authorization-no/AUTH_CLOSE_001/close"
        );
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let transfer_svc = service
        .transfer_service()
        .expect("transfer_service 应已装配");

    let result = transfer_svc
        .close_user_confirm_authorization("AUTH_CLOSE_001")
        .await
        .expect("解除免确认收款授权成功");
    assert_eq!(result.state.as_deref(), Some("CLOSED"));
    let close_info = result.close_info.unwrap();
    assert_eq!(
        close_info.close_reason.as_deref(),
        Some("CLOSE_VIA_MCH_API")
    );
}
