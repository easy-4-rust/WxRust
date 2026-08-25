#![allow(clippy::field_reassign_with_default)]
//! 点金计划服务测试（对应 Java `GoldPlanServiceImplTest`）。
//!
//! 测试覆盖：
//! 1. open_gold_plan — 开通点金计划（POST + JSON body 断言）
//! 2. close_gold_plan — 关闭点金计划
//! 3. open_custom_page — 开通商家小票
//! 4. set_advertising_industry_filter — 设置同业过滤标签
//! 5. open_advertising_show — 开通广告展示（PATCH 方法）
//! 6. close_advertising_show — 关闭广告展示
//! 7. GoldPlanResult serde — bean 序列化/反序列化
//!
//! MockServer 与 wx_pay_service_impl_test.rs 同构：v3 响应由平台私钥签名。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::goldplan::gold_plan_result::GoldPlanResult;
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::sign_sha256_rsa;

// ---- 夹具常量（与 wx_pay_service_impl_test.rs 同源） ----

const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";
const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";
const MERCHANT_SERIAL: &str = "5F1C72E2A8931B72A2E13ADE3BB492C7B9C71571";
const PLATFORM_SERIAL: &str = "PLATFORM_SERIAL_TEST_1";

const MERCHANT_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
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
-----END PRIVATE KEY-----";

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

// ---- MockServer（与 wx_pay_service_impl_test.rs 同构） ----

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
        .set_private_key(MERCHANT_PRIVATE_KEY_PEM)
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

// ---- GoldPlanResult bean 测试 ----

/// GoldPlanResult serde（对应 Java `GoldPlanResult`：`sub_mchid` 字段）。
#[test]
fn test_gold_plan_result_serde() {
    let json_str = r#"{"sub_mchid":"1234567890"}"#;
    let result: GoldPlanResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.sub_mch_id.as_deref(), Some("1234567890"));
}

/// GoldPlanResult 空 JSON 反序列化（所有字段 Optional）。
#[test]
fn test_gold_plan_result_empty() {
    let json_str = r#"{}"#;
    let result: GoldPlanResult = serde_json::from_str(json_str).unwrap();
    assert!(result.sub_mch_id.is_none());
}

/// GoldPlanResult 序列化后反序列化 round-trip。
#[test]
fn test_gold_plan_result_round_trip() {
    let original = GoldPlanResult {
        sub_mch_id: Some("9876543210".to_string()),
    };
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: GoldPlanResult = serde_json::from_str(&json_str).unwrap();
    assert_eq!(original.sub_mch_id, restored.sub_mch_id);
}

// ---- GoldPlanService 方法测试 ----

/// 开通点金计划：POST 请求 + JSON body 断言 + 响应解析。
#[tokio::test]
async fn test_open_gold_plan() {
    let response_body = json!({"sub_mchid": "1234567890"}).to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/goldplan/merchants/changegoldplanstatus");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    let result = gold_plan_svc
        .open_gold_plan("1234567890", Some("JSAPI"))
        .await
        .expect("开通点金计划成功");
    assert_eq!(result.sub_mch_id.as_deref(), Some("1234567890"));

    // 断言请求 body 包含正确字段
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "1234567890");
    assert_eq!(body["operation_type"], "OPEN");
    assert_eq!(body["operation_pay_scene"], "JSAPI");
}

/// 关闭点金计划：operation_type 为 CLOSE。
#[tokio::test]
async fn test_close_gold_plan() {
    let response_body = json!({"sub_mchid": "sub_mch_001"}).to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/goldplan/merchants/changegoldplanstatus");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    let result = gold_plan_svc
        .close_gold_plan("sub_mch_001", None)
        .await
        .expect("关闭点金计划成功");
    assert_eq!(result.sub_mch_id.as_deref(), Some("sub_mch_001"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["operation_type"], "CLOSE");
    // operation_pay_scene 不传时不应出现在 body 中
    assert!(body.get("operation_pay_scene").is_none());
}

/// 开通商家小票：POST + changecustompagestatus 路径。
#[tokio::test]
async fn test_open_custom_page() {
    let response_body = json!({"sub_mchid": "sub_mch_002"}).to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/goldplan/merchants/changecustompagestatus");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    let result = gold_plan_svc
        .open_custom_page("sub_mch_002")
        .await
        .expect("开通商家小票成功");
    assert_eq!(result.sub_mch_id.as_deref(), Some("sub_mch_002"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_002");
    assert_eq!(body["operation_type"], "OPEN");
}

/// 设置同业过滤标签：POST + advertising_industry_filters 数组。
#[tokio::test]
async fn test_set_advertising_industry_filter() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(
            path,
            "/v3/goldplan/merchants/set-advertising-industry-filter"
        );
        // 空 body 响应（void 返回）
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    let filters = vec!["餐饮".to_string(), "零售".to_string()];
    gold_plan_svc
        .set_advertising_industry_filter("sub_mch_003", &filters)
        .await
        .expect("设置同业过滤标签成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_003");
    let arr = body["advertising_industry_filters"].as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0], "餐饮");
    assert_eq!(arr[1], "零售");
}

/// 开通广告展示：PATCH 方法 + 可选过滤标签。
#[tokio::test]
async fn test_open_advertising_show() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "PATCH");
        assert_eq!(path, "/v3/goldplan/merchants/open-advertising-show");
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    let filters = vec!["教育".to_string()];
    gold_plan_svc
        .open_advertising_show("sub_mch_004", Some(&filters))
        .await
        .expect("开通广告展示成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_004");
    let arr = body["advertising_industry_filters"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0], "教育");
}

/// 关闭广告展示：POST 方法。
#[tokio::test]
async fn test_close_advertising_show() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/goldplan/merchants/close-advertising-show");
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    gold_plan_svc
        .close_advertising_show("sub_mch_005")
        .await
        .expect("关闭广告展示成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_005");
}

/// 开通广告展示（无过滤标签）：PATCH 方法，body 不含 advertising_industry_filters。
#[tokio::test]
async fn test_open_advertising_show_without_filters() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "PATCH");
        assert_eq!(path, "/v3/goldplan/merchants/open-advertising-show");
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let gold_plan_svc = service.gold_plan_service().expect("gold_plan_service 应已装配");

    gold_plan_svc
        .open_advertising_show("sub_mch_006", None)
        .await
        .expect("开通广告展示成功（无过滤标签）");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_006");
    assert!(body.get("advertising_industry_filters").is_none());
}
