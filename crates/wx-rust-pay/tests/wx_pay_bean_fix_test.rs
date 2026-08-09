#![allow(clippy::field_reassign_with_default)]
//! Wave 2c F2：bean 生成器缺陷修复后的门面方法线格式测试（MockServer 自含）。
//!
//! Golden 来源标注：
//! - `wxjava-combine-close-test`：Java `CombineCloseRequestTest.testSerialization`
//!   的 Gson 线格式断言（`combine_appid`/`sub_mchid`/`sub_appid`/`mchid`/
//!   `out_trade_no` 包含，transient `combineOutTradeNo` 不输出）；
//! - `wxjava-base-pay-service-impl`：Java `BaseWxPayServiceImpl` 的
//!   `closeOrderV3(request)`/`closePartnerOrderV3(request)`/`closeCombine`/
//!   `reverseOrderV3(request)`/`downloadFundFlow(request)`/`applyFundFlowBill`
//!   实现语义（URL 路径、transient 字段不入请求体、配置回填、GZIP/文本通道）；
//! - 签名与证书夹具与 `wx_pay_service_impl_test.rs`（P2a）同源
//!   （`wxjava-sign-utils-test` 商户密钥 / official-wechatpay-java APIv3 材料 /
//!   平台密钥对为本测试独立生成）。
//!
//! MockServer 语义与 wx-rust-miniapp `sub_domain_g1_core.rs` 同构：
//! 记录最近一次请求路径/请求体/请求头，handler 返回 (状态码, Content-Type,
//! body, 附加响应头)；v3 响应由平台私钥按 `timestamp\nnonce\nbody\n` 签名。

use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use base64::Engine;
use flate2::Compression;
use flate2::write::GzEncoder;
use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::request::combine_close_request::SubOrders;
use wx_rust_pay::bean::{
    CombineCloseRequest, WxPayApplyFundFlowBillV3Request, WxPayDownloadFundFlowRequest,
    WxPayOrderCloseV3Request, WxPayOrderReverseV3Request, WxPayPartnerOrderCloseV3Request,
};
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::sign_sha256_rsa;

/// 官方文档样例商户密钥（Java SignUtilsTest 同源）。
const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
/// 官方文档样例 appid（Java SignUtilsTest 同源）。
const APP_ID: &str = "wxd930ea5d5a258f4f";
/// 官方文档样例商户号（Java SignUtilsTest 同源）。
const MCH_ID: &str = "10000100";
/// APIv3 密钥（official-wechatpay-java `TestConfig.API_V3_KEY`，P3 同源）。
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";
/// 商户 API 证书序列号（v3 Authorization 头 serial_no 取值）。
const MERCHANT_SERIAL: &str = "5F1C72E2A8931B72A2E13ADE3BB492C7B9C71571";
/// 平台证书序列号（Wechatpay-Serial 响应头，测试用固定值）。
const PLATFORM_SERIAL: &str = "PLATFORM_SERIAL_TEST_1";

/// 商户 API 私钥（PKCS#8，official-wechatpay-java `merchant_private_key.pem`，
/// 与 P3 测试同源）。
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

/// 平台（微信支付）私钥（本测试独立生成，仅测试用途；mock 响应签名用）。
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

/// 平台（微信支付）公钥（SPKI，与 PLATFORM_PRIVATE_KEY_PEM 配对）。
const PLATFORM_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArIMeqvwK6806LyPJ0l+N
pigMK1/zoerNZ/72ZujpD2MSJ2BGFj+Ras5cmEcYsjX6anVQFeg3CqBX4ui1SCy+
XifQYGioH9yvN1lvZ76fNvPqD3ueciWmRkvZc6hOQ1y76GLo1VbjJ/cvShnoKdak
Fq7UdkKFmcp5wiAZRwq5oElMRNPNx7rPZ1JA/nNcsXwAheAmaDjVVjT1N3caKN//
NiV9rBdOBKb18b7CqUL/4I9ntemslqf24CfSyvE5vUXCq5ssmT1S36Wkb6tquuG5
OM5dVrgC7RW1zfxYhqYCelGBhdFOT/Lu1I39upmIxkxTc2224R/j8/Giro93Ck/I
VQIDAQAB
-----END PUBLIC KEY-----";

/// 商户 p12 证书夹具（openssl pkcs12 -export -legacy，密码=商户号
/// `1234567891`；与 P3 测试同源，v2 退款/证书通道测试使用）。
const MERCHANT_P12_BASE64: &str =
    "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
BMfvJQQQ8QQsT1mnFF3iTOfoHbyj2wICCAA=";

// ---- MockServer（与 wx-rust-miniapp 测试同构，扩展响应头/请求头捕获） ----

/// 极简 mock HTTP 服务器：记录最近一次请求路径（含 query）/请求体/请求头，
/// handler 返回 (状态码, Content-Type, body, 附加响应头)。
struct MockServer {
    addr: std::net::SocketAddr,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &HashMap<String, String>) -> (u16, String, String, Vec<(String, String)>)
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
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

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
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let mut lines = request.lines();
                    let request_line = lines.next().unwrap_or_default();
                    let path = request_line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("/")
                        .to_string();
                    *last_path_clone.lock().unwrap() = path.clone();
                    // 解析请求头
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
                    let (status, content_type, body, extra_headers) = handler(&path, &headers);
                    let mut response = format!(
                        "HTTP/1.1 {status} {}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
                        if status == 200 { "OK" } else { "Error" },
                        body.len()
                    );
                    for (k, v) in extra_headers {
                        response.push_str(&format!("{k}: {v}\r\n"));
                    }
                    response.push_str("\r\n");
                    response.push_str(&body);
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });

        Self {
            addr,
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

/// 二进制响应 mock（GZIP 资金账单通道测试：响应为原始 gzip 字节，MockServer
/// 的 String 响应体会被 `from_utf8_lossy` 破坏二进制内容，故独立实现）。
struct ByteMockServer {
    addr: std::net::SocketAddr,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl ByteMockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> Vec<u8> + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

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
                let handler = handler.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    let body = request
                        .split("\r\n\r\n")
                        .nth(1)
                        .unwrap_or_default()
                        .to_string();
                    let bytes = handler(&body);
                    *last_body_clone.lock().unwrap() = body;
                    let mut response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        bytes.len()
                    )
                    .into_bytes();
                    response.extend_from_slice(&bytes);
                    let _ = socket.write_all(&response).await;
                });
            }
        });

        Self {
            addr,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn last_body(&self) -> String {
        self.last_body.lock().unwrap().clone()
    }
}

impl Drop for ByteMockServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}
/// p12 夹具 DER 字节。
fn p12_der() -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(MERCHANT_P12_BASE64.replace('\n', ""))
        .expect("p12 base64")
}

/// 构建指向 mock 服务器的支付配置（v3 公钥模式：publicKeyId + 平台公钥）。
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
        .set_notify_url("https://example.com/pay/notify")
        .set_refund_notify_url("https://example.com/pay/refund-notify")
        .set_api_host_url(host);
    Arc::new(config)
}

/// 构建带 p12 证书的配置（v2 退款等 useKey=true 场景；p12 密码=商户号）。
fn config_with_p12(host: &str) -> Arc<dyn WxPayConfig> {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id("1234567891")
        .set_mch_key(MCH_KEY)
        .set_key_content(p12_der())
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

// ---- 测试 ----

/// 关闭订单 v3（请求版）：URL 路径 + JSON 请求体线格式。
///
/// 对应 Java `closeOrderV3(WxPayOrderCloseV3Request)`：`mchid` 空白时从配置
/// 回填（`StringUtils.isBlank`），URL 含 `out_trade_no`，请求体仅 `mchid`
/// （Java `outTradeNo` 为 transient，GSON 跳过 → Rust `#[serde(skip)]`）。
#[tokio::test]
async fn close_order_v3_with_request_wire_format() {
    let server =
        MockServer::start(|_path, _| (200, "application/json".to_string(), String::new(), vec![]))
            .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let mut request = WxPayOrderCloseV3Request::default();
    request.out_trade_no = Some("1217752501201407033233368018".to_string());
    // mchid 留空 → 配置回填
    service
        .close_order_v3_with_request(&request)
        .await
        .expect("close_order_v3_with_request 应成功");
    assert_eq!(
        server.last_path(),
        "/v3/pay/transactions/out-trade-no/1217752501201407033233368018/close"
    );
    let body: serde_json::Value =
        serde_json::from_str(&server.last_body()).expect("请求体应为 JSON");
    assert_eq!(body, json!({"mchid": MCH_ID}));
    // Java transient out_trade_no：线格式不含（仅在 URL 路径）
    assert!(
        !server.last_body().contains("1217752501201407033233368018"),
        "out_trade_no 不应出现在请求体: {}",
        server.last_body()
    );
}

/// 服务商关闭订单 v3（请求版）：URL 路径 + 请求体线格式。
///
/// 对应 Java `closePartnerOrderV3(WxPayPartnerOrderCloseV3Request)`：
/// `sp_mchid`/`sub_mchid` 空白时从配置回填，请求体仅
/// `sp_mchid`/`sub_mchid`（transient `out_trade_no` 不入体）。
#[tokio::test]
async fn close_partner_order_v3_with_request_wire_format() {
    let server =
        MockServer::start(|_path, _| (200, "application/json".to_string(), String::new(), vec![]))
            .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let mut request = WxPayPartnerOrderCloseV3Request::default();
    request.out_trade_no = Some("P20150806125346".to_string());
    request.sub_mch_id = Some("1230000109".to_string()); // sp_mchid 留空 → 配置回填
    service
        .close_partner_order_v3_with_request(&request)
        .await
        .expect("close_partner_order_v3_with_request 应成功");
    assert_eq!(
        server.last_path(),
        "/v3/pay/partner/transactions/out-trade-no/P20150806125346/close"
    );
    let body: serde_json::Value =
        serde_json::from_str(&server.last_body()).expect("请求体应为 JSON");
    assert_eq!(body, json!({"sp_mchid": MCH_ID, "sub_mchid": "1230000109"}));
    assert!(!server.last_body().contains("P20150806125346"));
}

/// 合单关闭订单：URL 路径 + 请求体线格式（对照 Java
/// `CombineCloseRequestTest` golden：`combine_out_trade_no` 为 transient，
/// 仅作 URL 路径参数，不入请求体）。
#[tokio::test]
async fn close_combine_wire_format() {
    let server =
        MockServer::start(|_path, _| (200, "application/json".to_string(), String::new(), vec![]))
            .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let mut request = CombineCloseRequest::default();
    request.combine_appid = Some("wxd678efh567hg6787".to_string());
    request.combine_out_trade_no = Some("P20150806125346".to_string());
    let mut sub_order = SubOrders::default();
    sub_order.mchid = Some("1900000109".to_string());
    sub_order.out_trade_no = Some("20150806125346".to_string());
    sub_order.sub_mchid = Some("1230000109".to_string());
    sub_order.sub_appid = Some("wxd678efh567hg6999".to_string());
    request.sub_orders = vec![sub_order];
    service
        .close_combine(&request)
        .await
        .expect("close_combine 应成功");
    assert_eq!(
        server.last_path(),
        "/v3/combine-transactions/out-trade-no/P20150806125346/close"
    );
    let body = server.last_body();
    for key in [
        "\"combine_appid\":\"wxd678efh567hg6787\"",
        "\"sub_mchid\":\"1230000109\"",
        "\"sub_appid\":\"wxd678efh567hg6999\"",
        "\"mchid\":\"1900000109\"",
        "\"out_trade_no\":\"20150806125346\"",
    ] {
        assert!(body.contains(key), "请求体应包含 {key}: {body}");
    }
    // Java transient combineOutTradeNo：Gson 不输出
    assert!(
        !body.contains("P20150806125346"),
        "combine_out_trade_no 不应出现在请求体: {body}"
    );
}

/// v3 撤销订单（请求版）：URL 路径 + 请求体线格式 + 响应解析。
///
/// 对应 Java `reverseOrderV3(WxPayOrderReverseV3Request)`：`appid`/`mchid`
/// 空白时从配置回填；请求体仅 `appid`/`mchid`（transient `out_trade_no`
/// 仅作 URL 路径参数）；响应反序列化为 `WxPayOrderReverseV3Result`。
#[tokio::test]
async fn reverse_order_v3_with_request_wire_format_and_response() {
    let server = MockServer::start(|_path, _| {
        signed_json_response(
            r#"{"appid":"wxd930ea5d5a258f4f","mchid":"10000100","out_trade_no":"R123456"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let mut request = WxPayOrderReverseV3Request::default();
    request.out_trade_no = Some("R123456".to_string());
    // appid/mchid 留空 → 配置回填
    let result = service
        .reverse_order_v3_with_request(&request)
        .await
        .expect("reverse_order_v3_with_request 应成功");
    assert_eq!(
        server.last_path(),
        "/v3/pay/transactions/out-trade-no/R123456/reverse"
    );
    let body: serde_json::Value =
        serde_json::from_str(&server.last_body()).expect("请求体应为 JSON");
    assert_eq!(body, json!({"appid": APP_ID, "mchid": MCH_ID}));
    assert_eq!(result.out_trade_no.as_deref(), Some("R123456"));
}

/// v3 申请资金账单：URL 查询参数（含/不含 tar_type 两分支）+ 响应解析。
///
/// 对应 Java `applyFundFlowBill(WxPayApplyFundFlowBillV3Request)`：
/// `tar_type` 为空（`StringUtils.isBlank`）时不携带该查询参数。
#[tokio::test]
async fn apply_fund_flow_bill_url_and_response() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.contains("bill_date=2019-06-11")
                && path.contains("account_type=BASIC")
                && path.contains("tar_type=GZIP"),
            "URL 应包含账单参数: {path}"
        );
        signed_json_response(
            r#"{"hash_type":"SHA1","hash_value":"f2c38e8e3d3f6e0b","download_url":"https://api.mch.weixin.qq.com/v3/billdownload/file"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let mut request = WxPayApplyFundFlowBillV3Request::default();
    request.bill_date = Some("2019-06-11".to_string());
    request.account_type = Some("BASIC".to_string());
    request.tar_type = Some("GZIP".to_string());
    let result = service
        .apply_fund_flow_bill(&request)
        .await
        .expect("apply_fund_flow_bill 应成功");
    assert_eq!(result.hash_type.as_deref(), Some("SHA1"));
    assert_eq!(
        result.download_url.as_deref(),
        Some("https://api.mch.weixin.qq.com/v3/billdownload/file")
    );

    // tar_type 为空分支：URL 无 tar_type 参数
    let server2 = MockServer::start(|path, _| {
        assert!(!path.contains("tar_type"), "URL 不应含 tar_type: {path}");
        signed_json_response(
            r#"{"hash_type":"SHA1","hash_value":"abc","download_url":"https://x"}"#,
        )
    })
    .await;
    let service2 = WxPayServiceImpl::new_arc(config_with_host(&server2.url("")));
    let mut request2 = WxPayApplyFundFlowBillV3Request::default();
    request2.bill_date = Some("2019-06-11".to_string());
    request2.account_type = Some("BASIC".to_string());
    service2
        .apply_fund_flow_bill(&request2)
        .await
        .expect("apply_fund_flow_bill 应成功");
    assert!(
        server2
            .last_path()
            .starts_with("/v3/bill/fundflowbill?bill_date=2019-06-11&account_type=BASIC")
    );
}

/// 下载资金账单（请求版）：XML 请求体线格式（文本通道）。
///
/// 对应 Java `downloadFundFlow(WxPayDownloadFundFlowRequest)`：`checkAndSign`
/// （配置回填 + HMAC-SHA256 签名）后 POST `/pay/downloadfundflow`（证书通道），
/// 请求体 XML 含 `bill_date`/`account_type`/`tar_type`（Wave 1 被误生成到
/// `AccountType` 的字段已归位）；响应按 Java `handleFundFlow` 解析。
#[tokio::test]
async fn download_fund_flow_with_request_sends_signed_xml() {
    // 对应 Java 资金账单原始文本格式：明细段（标题 + 反引号分隔字段）+ 汇总段
    let fund_flow_text = "记账时间 微信支付业务单号 资金流水单号 业务名称 业务类型 收支类型 收支金额（元） 账户结余（元） 资金变更提交申请人 备注 业务凭证号`2018-02-01 04:21:23`50000305742018020103387128253`1900009231201802015884652186`退款`退款`支出`0.02`0.17`system`缺货`REF4200000068201801293084726067\n资金流水总笔数`1`1`0.02`0`0";
    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadfundflow"), "path={path}");
        (
            200,
            "text/plain".to_string(),
            fund_flow_text.to_string(),
            vec![],
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_p12(&server.url("")));
    let mut request = WxPayDownloadFundFlowRequest::default();
    request.bill_date = Some("20180819".to_string());
    request.account_type = Some("Basic".to_string());
    // tar_type 不设置（对应 Java 测试 null tarType → 非 GZIP 通道）
    let result = service
        .download_fund_flow_with_request(&request)
        .await
        .expect("download_fund_flow_with_request 应成功");
    let body = server.last_body();
    assert!(
        body.contains("<bill_date>20180819</bill_date>"),
        "body={body}"
    );
    assert!(
        body.contains("<account_type>Basic</account_type>"),
        "body={body}"
    );
    assert!(
        body.contains("<appid>wxd930ea5d5a258f4f</appid>"),
        "body={body}"
    );
    assert!(body.contains("<mch_id>1234567891</mch_id>"), "body={body}");
    assert!(
        body.contains("<sign_type>HMAC-SHA256</sign_type>"),
        "body={body}"
    );
    assert!(body.contains("<sign>"), "body={body}");
    // 响应解析（对应 Java handleFundFlow）
    assert_eq!(result.wx_pay_fund_flow_base_result_list.len(), 1);
    assert_eq!(
        result.wx_pay_fund_flow_base_result_list[0]
            .billing_time
            .as_deref(),
        Some("2018-02-01 04:21:23")
    );
    assert_eq!(result.total_record.as_deref(), Some("1"));
    assert_eq!(result.income_amount.as_deref(), Some("0.02"));
}

/// 下载资金账单（请求版）：GZIP 通道（对应 Java `handleGzipFundFlow`：
/// `postForBytes` + `ZipUtils.unGzip`，gzip 响应解压为文本后解析）。
#[tokio::test]
async fn download_fund_flow_with_request_gzip_channel() {
    let fund_flow_text = "记账时间 微信支付业务单号 资金流水单号 业务名称 业务类型 收支类型 收支金额（元） 账户结余（元） 资金变更提交申请人 备注 业务凭证号`2018-02-01 04:21:23`50000305742018020103387128253`1900009231201802015884652186`退款`退款`支出`0.02`0.17`system`缺货`REF4200000068201801293084726067\n资金流水总笔数`2`1`0.02`1`0.10";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(fund_flow_text.as_bytes())
        .expect("gzip 写入");
    let gz_bytes = encoder.finish().expect("gzip 完成");
    let server = ByteMockServer::start(move |_path| {
        // 请求 URL 断言在 body 侧完成；响应返回原始 gzip 字节
        gz_bytes.clone()
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_p12(&server.url("")));
    let mut request = WxPayDownloadFundFlowRequest::default();
    request.bill_date = Some("20180819".to_string());
    request.account_type = Some("Basic".to_string());
    request.tar_type = Some("GZIP".to_string());
    let result = service
        .download_fund_flow_with_request(&request)
        .await
        .expect("gzip 通道应成功");
    let body = server.last_body();
    assert!(body.contains("<tar_type>GZIP</tar_type>"), "body={body}");
    assert_eq!(result.wx_pay_fund_flow_base_result_list.len(), 1);
    assert_eq!(result.total_record.as_deref(), Some("2"));
    assert_eq!(result.expenditure_amount.as_deref(), Some("0.10"));
}

/// 合单关闭请求 bean JSON 线格式（对照 Java `CombineCloseRequestTest` golden：
/// Gson 序列化断言 + 反序列化）。
#[test]
fn combine_close_request_json_golden() {
    let mut request = CombineCloseRequest::default();
    request.combine_appid = Some("wxd678efh567hg6787".to_string());
    request.combine_out_trade_no = Some("P20150806125346".to_string());
    let mut sub_order = SubOrders::default();
    sub_order.mchid = Some("1900000109".to_string());
    sub_order.out_trade_no = Some("20150806125346".to_string());
    sub_order.sub_mchid = Some("1230000109".to_string());
    sub_order.sub_appid = Some("wxd678efh567hg6999".to_string());
    request.sub_orders = vec![sub_order];

    let json = serde_json::to_string(&request).expect("序列化");
    assert!(json.contains("\"sub_mchid\":\"1230000109\""));
    assert!(json.contains("\"sub_appid\":\"wxd678efh567hg6999\""));
    assert!(json.contains("\"combine_appid\":\"wxd678efh567hg6787\""));
    assert!(json.contains("\"mchid\":\"1900000109\""));
    assert!(json.contains("\"out_trade_no\":\"20150806125346\""));
    // Java transient combineOutTradeNo：Gson 不输出（Rust #[serde(skip)] 同语义）
    assert!(!json.contains("P20150806125346"));

    // 反序列化（对应 Java `gson.fromJson`）
    let back: CombineCloseRequest = serde_json::from_str(&json).expect("反序列化");
    assert_eq!(back.combine_appid.as_deref(), Some("wxd678efh567hg6787"));
    assert_eq!(back.sub_orders.len(), 1);
    assert_eq!(back.sub_orders[0].sub_mchid.as_deref(), Some("1230000109"));
    assert_eq!(
        back.sub_orders[0].sub_appid.as_deref(),
        Some("wxd678efh567hg6999")
    );
}

/// 资金账单请求 bean 线格式：v3 申请 JSON + v2 下载 XML；`AccountType`
/// 还原为常量占位结构（不再持有账单字段）。
#[test]
fn fund_flow_beans_wire_format() {
    // v3 申请资金账单（Java WxPayApplyFundFlowBillV3Request：@SerializedName
    // bill_date/account_type/tar_type）
    let mut apply = WxPayApplyFundFlowBillV3Request::default();
    apply.bill_date = Some("2019-06-11".to_string());
    apply.account_type = Some("BASIC".to_string());
    apply.tar_type = Some("GZIP".to_string());
    let json = serde_json::to_string(&apply).expect("序列化");
    assert!(json.contains("\"bill_date\":\"2019-06-11\""));
    assert!(json.contains("\"account_type\":\"BASIC\""));
    assert!(json.contains("\"tar_type\":\"GZIP\""));

    // v2 下载资金账单 XML（Java WxPayDownloadFundFlowRequest：@XStreamAlias
    // bill_date/account_type/tar_type）
    let mut download = WxPayDownloadFundFlowRequest::default();
    download.bill_date = Some("20180819".to_string());
    download.account_type = Some("Basic".to_string());
    download.tar_type = Some("GZIP".to_string());
    let xml = download.to_xml().expect("to_xml");
    assert!(xml.contains("<bill_date>20180819</bill_date>"), "xml={xml}");
    assert!(
        xml.contains("<account_type>Basic</account_type>"),
        "xml={xml}"
    );
    assert!(xml.contains("<tar_type>GZIP</tar_type>"), "xml={xml}");
    // 从 XML 解析回填（对应 Java fromXML）
    let back = WxPayDownloadFundFlowRequest::from_xml(&xml).expect("from_xml");
    assert_eq!(back.bill_date.as_deref(), Some("20180819"));

    // AccountType 仅为常量占位（Java 无实例字段），线格式为空对象
    let account_json = serde_json::to_string(
        &wx_rust_pay::bean::request::wx_pay_apply_fund_flow_bill_v3_request::AccountType {},
    )
    .expect("序列化");
    assert_eq!(account_json, "{}");
}
