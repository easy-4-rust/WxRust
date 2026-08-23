#![allow(clippy::field_reassign_with_default)]
//! 覆盖率提升: wx_pay_service.rs + base_wx_pay_service_impl.rs Mock 测试。
//!
//! 利用 MockServer 拦截 HTTP 请求，覆盖未测试的服务方法：
//! close_order、micropay、reverse_order、shorturl、report、
//! authcode_to_openid、download_fund_flow、create_order (APP/JSAPI)、
//! v3 错误 JSON 路径、v3 关单、合单支付等。
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java BaseWxPayServiceImpl 对应方法的行为
//! - RUST_OBLIGATION: Rust 错误路径/约束检查
//! - VALUE_ADD: v3 错误响应解析、边界组合

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::notify::SignatureHeader;
use wx_rust_pay::bean::request::wx_pay_unified_order_v3_request::Amount as OrderV3Amount;
use wx_rust_pay::bean::{TradeTypeEnum, WxPayUnifiedOrderRequest, WxPayUnifiedOrderV3Request};
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::{aes_gcm_encrypt, sign_sha256_rsa};
use wx_rust_pay::util::sign_utils::SignUtils;

// ---- 夹具常量（与 wx_pay_service_impl_test.rs 同源） ----

const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";
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

struct MockServer {
    addr: std::net::SocketAddr,
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
                    let mut lines = request.lines();
                    let request_line = lines.next().unwrap_or_default();
                    let path = request_line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("/")
                        .to_string();
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
            last_body,
            stop,
        }
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
        .set_notify_url("https://example.com/pay/notify")
        .set_refund_notify_url("https://example.com/pay/refund-notify")
        .set_api_host_url(host);
    Arc::new(config)
}

fn v2_signed_response(fields: &[(&str, &str)], sign_type: Option<&str>) -> String {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in fields {
        map.insert(k.to_string(), v.to_string());
    }
    let sign = SignUtils::create_sign(&map, sign_type, MCH_KEY, &[]).expect("响应签名计算失败");
    let mut xml = String::from("<xml>");
    for (k, v) in fields {
        xml.push_str(&format!("<{k}><![CDATA[{v}]]></{k}>"));
    }
    xml.push_str(&format!("<sign><![CDATA[{sign}]]></sign></xml>"));
    xml
}

fn v2_xml_response(
    fields: &[(&str, &str)],
    sign_type: Option<&str>,
) -> (u16, String, String, Vec<(String, String)>) {
    (
        200,
        "text/xml".to_string(),
        v2_signed_response(fields, sign_type),
        vec![],
    )
}

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

fn platform_private_key() -> rsa::RsaPrivateKey {
    load_private_key_from_pem(PLATFORM_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥")
}

// ═══════════════════════════════════════════════════════════════════
// v2 关闭订单（SOURCE_PARITY: closeOrder → /pay/closeorder）
// ═══════════════════════════════════════════════════════════════════

/// v2 关闭订单：URL 断言 + 响应解析。
/// 对应 Java: closeOrder(String outTradeNo)
#[tokio::test]
async fn v2_close_order_xml_and_parse() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/closeorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    service
        .close_order("out_trade_no_001")
        .await
        .expect("关闭订单成功");
    let xml = server.last_body();
    assert!(
        xml.contains("<out_trade_no>out_trade_no_001</out_trade_no>"),
        "{xml}"
    );
    assert!(xml.contains("<sign>"), "{xml}");
}

/// v2 关闭订单空 out_trade_no → 报错。
/// 对应 Java: closeOrder → checkConstraints
#[tokio::test]
async fn v2_close_order_empty_out_trade_no() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.close_order("").await.expect_err("应报错");
    assert!(
        err.to_string().contains("out_trade_no不能为空"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// v2 扫码支付（SOURCE_PARITY: micropay → /pay/micropay）
// ═══════════════════════════════════════════════════════════════════

/// v2 扫码支付：请求体断言 + 响应解析。
/// 对应 Java: micropay(WxPayMicropayRequest)
#[tokio::test]
async fn v2_micropay_xml_and_parse() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/micropay"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("openid", "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
                ("trade_type", "MICROPAY"),
                ("total_fee", "100"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = wx_rust_pay::bean::WxPayMicropayRequest::default();
    request.body = Some("测试商品".to_string());
    request.out_trade_no = Some("micropay_001".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.auth_code = Some("134567890123456789".to_string());

    let result = service.micropay(&request).await.expect("扫码支付成功");
    assert_eq!(
        result.openid.as_deref(),
        Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")
    );

    let xml = server.last_body();
    assert!(xml.contains("<total_fee>100</total_fee>"), "{xml}");
    assert!(xml.contains("<auth_code>"), "{xml}");
    assert!(xml.contains("<sign>"), "{xml}");
}

// ═══════════════════════════════════════════════════════════════════
// v2 authcode_to_openid（SOURCE_PARITY: tools/authcodetoopenid）
// ═══════════════════════════════════════════════════════════════════

/// v2 授权码查询 openid（返回原始 XML 字符串）。
/// 对应 Java: authcode2Openid(WxPayAuthcode2OpenidRequest)
#[tokio::test]
async fn v2_authcode_to_openid() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/tools/authcodetoopenid"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "testnonce"),
                ("openid", "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let result = service
        .authcode2_openid("134567890123456789")
        .await
        .expect("授权码查询成功");
    assert!(
        result.contains("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
        "openid: {result}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// v2 短链接（SOURCE_PARITY: tools/shorturl）
// ═══════════════════════════════════════════════════════════════════

/// v2 短链接转换（返回原始 XML 字符串）。
/// 对应 Java: shortUrl(WxPayShorturlRequest)
#[tokio::test]
async fn v2_shorturl() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/tools/shorturl"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "testnonce"),
                ("short_url", "weixin://wxpay/bizpayurl?pr=abc123"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let result = service
        .shorturl("https://pay.weixin.qq.com/wxpay/pay.action?code=abc123")
        .await
        .expect("短链接成功");
    assert!(
        result.contains("weixin://wxpay/bizpayurl?pr=abc123"),
        "short_url: {result}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// v2 交易上报（SOURCE_PARITY: payitil/report）
// ═══════════════════════════════════════════════════════════════════

/// v2 交易上报。
/// 对应 Java: report(WxPayReportRequest)
#[tokio::test]
async fn v2_report() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/payitil/report"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "testnonce"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = wx_rust_pay::bean::WxPayReportRequest::default();
    request.interface_url = Some("https://api.mch.weixin.qq.com/pay/micropay".to_string());
    request.execute_time = Some(100);
    request.return_code = Some("SUCCESS".to_string());
    request.return_msg = Some("OK".to_string());
    request.result_code = Some("SUCCESS".to_string());
    request.user_ip = Some("127.0.0.1".to_string());

    service.report(&request).await.expect("上报成功");
    let xml = server.last_body();
    assert!(xml.contains("<sign>"), "{xml}");
}

// ═══════════════════════════════════════════════════════════════════
// v2 撤销订单（SOURCE_PARITY: secapi/pay/reverse）
// ═══════════════════════════════════════════════════════════════════

/// v2 撤销订单约束检查。
/// 对应 Java: reverseOrder(WxPayOrderReverseRequest) → checkConstraints
#[tokio::test]
async fn v2_reverse_order_constraint_check() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // transaction_id 和 out_trade_no 同时为空 → 报错
    let request = wx_rust_pay::bean::WxPayOrderReverseRequest::default();
    let err = service.reverse_order(&request).await.expect_err("应报错");
    assert!(err.to_string().contains("不能同时为空"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// v2 资金账单下载（SOURCE_PARITY: pay/downloadfundflow）
// ═══════════════════════════════════════════════════════════════════

/// v2 资金账单下载：download_fund_flow 路径覆盖（account_type/tar_type 校验分支）。
/// 对应 Java: downloadFundFlow(WxPayDownloadFundFlowRequest)
///
/// 此处仅验证约束检查（无需网络请求/证书）。
#[tokio::test]
async fn v2_download_fund_flow_account_type_check() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // 无效 account_type → 报错
    let err = service
        .download_fund_flow("20240101", "InvalidType", "")
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("account_type"), "错误信息: {err}");

    // tar_type 非 GZIP → 报错
    let err = service
        .download_fund_flow("20240101", "Basic", "XML")
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("tar_type"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// v2 创建订单 APP/JSAPI（SOURCE_PARITY: createOrder → unified_order）
// ═══════════════════════════════════════════════════════════════════

/// v2 创建订单（APP 支付）：prepay_id → APP 二次签名。
/// 对应 Java: createOrder → APP
#[tokio::test]
async fn v2_create_order_app() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "APP"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.trade_type = Some("APP".to_string());
    request.out_trade_no = Some("app_order_001".to_string());

    let pay_info = service.create_order(&request).await.expect("APP 下单成功");
    let obj = pay_info.as_object().expect("应返回对象");
    assert_eq!(
        obj.get("prepayId").and_then(|v| v.as_str()),
        Some("wx201410272009395522657a690389285100")
    );
    assert_eq!(
        obj.get("packageValue").and_then(|v| v.as_str()),
        Some("Sign=WXPay")
    );
    assert!(obj.get("sign").and_then(|v| v.as_str()).is_some());
    assert!(obj.get("timeStamp").and_then(|v| v.as_str()).is_some());
}

/// v2 创建订单（JSAPI 支付）：prepay_id → JSAPI 二次签名。
/// 对应 Java: createOrder → JSAPI
#[tokio::test]
async fn v2_create_order_jsapi() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "JSAPI"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.trade_type = Some("JSAPI".to_string());
    request.out_trade_no = Some("jsapi_order_001".to_string());
    request.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());

    let pay_info = service
        .create_order(&request)
        .await
        .expect("JSAPI 下单成功");
    let obj = pay_info.as_object().expect("应返回对象");
    assert_eq!(
        obj.get("package").and_then(|v| v.as_str()),
        Some("prepay_id=wx201410272009395522657a690389285100")
    );
    assert_eq!(obj.get("signType").and_then(|v| v.as_str()), Some("MD5"));
    assert!(obj.get("paySign").and_then(|v| v.as_str()).is_some());
}

/// v2 创建订单（NATIVE 支付）：code_url 字符串。
/// 对应 Java: createOrder → NATIVE
#[tokio::test]
async fn v2_create_order_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "NATIVE"),
                ("code_url", "weixin://wxpay/bizpayurl?pr=abc123"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.trade_type = Some("NATIVE".to_string());
    request.out_trade_no = Some("native_order_001".to_string());
    request.product_id = Some("product_001".to_string());

    let pay_info = service
        .create_order(&request)
        .await
        .expect("NATIVE 下单成功");
    // Native 返回 WxPayNativeOrderResult 对象（codeUrl 字段）
    let url = pay_info
        .get("codeUrl")
        .and_then(|v| v.as_str())
        .or_else(|| pay_info.as_str());
    assert!(
        url.unwrap_or_default().contains("weixin://wxpay/bizpayurl"),
        "pay_info: {pay_info}"
    );
}

/// v2 创建订单（MWEB 支付）：mweb_url 字符串。
/// 对应 Java: createOrder → MWEB
#[tokio::test]
async fn v2_create_order_mweb() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "MWEB"),
                ("mweb_url", "https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx201410272009395522657a690389285100"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.trade_type = Some("MWEB".to_string());
    request.out_trade_no = Some("mweb_order_001".to_string());

    let pay_info = service.create_order(&request).await.expect("MWEB 下单成功");
    let obj = pay_info.as_object().expect("应返回对象");
    assert!(
        obj.get("mwebUrl")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .contains("wx.tenpay.com")
    );
}

// ═══════════════════════════════════════════════════════════════════
// v3 错误 JSON 响应路径（RUST_OBLIGATION: convertException）
// ═══════════════════════════════════════════════════════════════════

/// v3 非 200 响应（4xx）→ 解析 code + message → 报错。
/// 对应 Java: convertException → {code, message}
#[tokio::test]
async fn v3_error_json_with_code_and_message() {
    let server = MockServer::start(|_path, _| {
        (
            403,
            "application/json".to_string(),
            r#"{"code":"PARAM_ERROR","message":"缺少必填参数"}"#.to_string(),
            vec![],
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let err = service
        .get_v3(&format!("http://{}/v3/test", server.addr))
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("PARAM_ERROR"), "错误信息: {err}");
    assert!(err.to_string().contains("缺少必填参数"), "错误信息: {err}");
}

/// v3 非 200 响应（500）→ code 为空 → 仅 message。
/// 对应 Java: convertException → code empty → message only
#[tokio::test]
async fn v3_error_json_empty_code() {
    let server = MockServer::start(|_path, _| {
        (
            500,
            "application/json".to_string(),
            r#"{"code":"","message":"内部服务器错误"}"#.to_string(),
            vec![],
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let err = service
        .post_v3(
            &format!("http://{}/v3/test", server.addr),
            r#"{"key":"value"}"#,
        )
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("内部服务器错误"),
        "错误信息: {err}"
    );
}

/// v3 非 200 响应 → 非 JSON 体 → 状态码+原始响应。
/// 对应 Java: convertException → parse fail → status+text
#[tokio::test]
async fn v3_error_non_json_body() {
    let server = MockServer::start(|_path, _| {
        (
            502,
            "text/plain".to_string(),
            "Bad Gateway".to_string(),
            vec![],
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let err = service
        .put_v3(
            &format!("http://{}/v3/test", server.addr),
            r#"{"key":"value"}"#,
        )
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("502"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// v3 关闭订单（SOURCE_PARITY: closeOrderV3）
// ═══════════════════════════════════════════════════════════════════

/// v3 关闭订单：请求体 mchid 回填。
/// 对应 Java: closeOrderV3(String outTradeNo)
#[tokio::test]
async fn v3_close_order_url_and_body() {
    let server =
        MockServer::start(|_path, _| (204, "application/json".to_string(), "".to_string(), vec![]))
            .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    service
        .close_order_v3("out_trade_no_001")
        .await
        .expect("v3 关单成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["mchid"], json!(MCH_ID));
}

/// v3 关闭订单空 out_trade_no → 报错。
/// 对应 Java: closeOrderV3 → checkConstraints
#[tokio::test]
async fn v3_close_order_empty_out_trade_no() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.close_order_v3("").await.expect_err("应报错");
    assert!(
        err.to_string().contains("out_trade_no不能为空"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// v3 撤销订单（SOURCE_PARITY: reverseOrderV3）
// ═══════════════════════════════════════════════════════════════════

/// v3 撤销订单。
/// 对应 Java: reverseOrderV3(String outTradeNo)
#[tokio::test]
async fn v3_reverse_order() {
    let server = MockServer::start(|path, _| {
        assert!(path.contains("/v3/pay/transactions/out-trade-no/reverse"));
        signed_json_response(r#"{"out_trade_no":"reverse_001"}"#)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    service
        .reverse_order_v3("reverse_001")
        .await
        .expect("v3 撤销成功");
}

// ═══════════════════════════════════════════════════════════════════
// v3 create_order_v3 H5/APP 分支（SOURCE_PARITY: createOrderV3）
// ═══════════════════════════════════════════════════════════════════

/// v3 创建订单（H5 支付）：h5_url 字符串。
/// 对应 Java: createOrderV3 → H5
#[tokio::test]
async fn v3_create_order_h5() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/pay/transactions/h5"));
        signed_json_response(
            r#"{"prepay_id":"wx201410272009395522657a690389285100","h5_url":"https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx201410272009395522657a690389285100"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderV3Request::default();
    request.description = Some("测试商品".to_string());
    request.out_trade_no = Some("h5_order_001".to_string());
    request.amount = Some(OrderV3Amount {
        total: Some(100),
        currency: Some("CNY".to_string()),
    });

    let pay_info = service
        .create_order_v3(TradeTypeEnum::H5, &request)
        .await
        .expect("H5 下单成功");
    assert_eq!(
        pay_info.as_str(),
        Some(
            "https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx201410272009395522657a690389285100"
        )
    );
}

/// v3 创建订单（APP 支付）：AppResult 对象。
/// 对应 Java: createOrderV3 → APP
#[tokio::test]
async fn v3_create_order_app() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/pay/transactions/app"));
        signed_json_response(r#"{"prepay_id":"wx201410272009395522657a690389285100"}"#)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderV3Request::default();
    request.description = Some("测试商品".to_string());
    request.out_trade_no = Some("app_v3_order_001".to_string());
    request.amount = Some(OrderV3Amount {
        total: Some(100),
        currency: Some("CNY".to_string()),
    });

    let pay_info = service
        .create_order_v3(TradeTypeEnum::App, &request)
        .await
        .expect("APP v3 下单成功");
    let obj = pay_info.as_object().expect("应返回对象");
    assert_eq!(
        obj.get("prepayid").and_then(|v| v.as_str()),
        Some("wx201410272009395522657a690389285100")
    );
    assert_eq!(
        obj.get("package").and_then(|v| v.as_str()),
        Some("Sign=WXPay")
    );
    assert!(obj.get("sign").and_then(|v| v.as_str()).is_some());
}

// ═══════════════════════════════════════════════════════════════════
// 合单支付 H5 分支（SOURCE_PARITY: combineTransactions → H5）
// ═══════════════════════════════════════════════════════════════════

/// 合单支付（H5）：h5_url 从原始 JSON 提取。
/// 对应 Java: combineTransactions → H5
#[tokio::test]
async fn v3_combine_transactions_h5() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/combine-transactions/h5"));
        signed_json_response(
            r#"{"prepay_id":"wx201410272009395522657a690389285100","h5_url":"https://wx.tenpay.com/h5_url_test"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = wx_rust_pay::bean::CombineTransactionsRequest::default();
    request.combine_out_trade_no = Some("combine_h5_001".to_string());
    request.sub_orders = vec![];

    let pay_info = service
        .combine_transactions(TradeTypeEnum::H5, &request)
        .await
        .expect("合单 H5 成功");
    assert_eq!(pay_info.as_str(), Some("https://wx.tenpay.com/h5_url_test"));
}

// ═══════════════════════════════════════════════════════════════════
// v3 下单约束检查（RUST_OBLIGATION: NATIVE 必须 product_id）
// ═══════════════════════════════════════════════════════════════════

/// v2 统一下单 NATIVE 无 product_id → 报错。
/// 对应 Java: unifiedOrder → checkConstraints → NATIVE must have product_id
#[tokio::test]
async fn v2_unified_order_native_requires_product_id() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.trade_type = Some("NATIVE".to_string());
    request.out_trade_no = Some("native_001".to_string());
    // 未设置 product_id

    let err = service.unified_order(&request).await.expect_err("应报错");
    assert!(err.to_string().contains("product_id"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// v3 get_v3 / put_v3 / delete_v3 / patch_v3 方法覆盖
// ═══════════════════════════════════════════════════════════════════

/// v3 GET 请求。
/// 对应 Java: getV3(String url)
#[tokio::test]
async fn v3_get_request() {
    let server =
        MockServer::start(|_path, _| signed_json_response(r#"{"status":"SUCCESS"}"#)).await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let response = service
        .get_v3(&format!("http://{}/v3/test", server.addr))
        .await
        .expect("GET 成功");
    assert!(response.contains("SUCCESS"));
}

/// v3 PUT 请求。
/// 对应 Java: putV3(String url, String requestStr)
#[tokio::test]
async fn v3_put_request() {
    let server = MockServer::start(|_path, _| signed_json_response(r#"{"status":"ok"}"#)).await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let response = service
        .put_v3(
            &format!("http://{}/v3/test", server.addr),
            r#"{"key":"value"}"#,
        )
        .await
        .expect("PUT 成功");
    assert!(response.contains("ok"));
}

/// v3 DELETE 请求。
/// 对应 Java: deleteV3(String url)
#[tokio::test]
async fn v3_delete_request() {
    let server = MockServer::start(|_path, _| (204, "".to_string(), "".to_string(), vec![])).await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    service
        .delete_v3(&format!("http://{}/v3/test", server.addr))
        .await
        .expect("DELETE 成功");
}

/// v3 PATCH 请求。
/// 对应 Java: patchV3(String url, String requestStr)
#[tokio::test]
async fn v3_patch_request() {
    let server =
        MockServer::start(|_path, _| signed_json_response(r#"{"status":"patched"}"#)).await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let response = service
        .patch_v3(
            &format!("http://{}/v3/test", server.addr),
            r#"{"key":"value"}"#,
        )
        .await
        .expect("PATCH 成功");
    assert!(response.contains("patched"));
}

// ═══════════════════════════════════════════════════════════════════
// 配置管理（SOURCE_PARITY: 多商户配置切换）
// ═══════════════════════════════════════════════════════════════════

/// 多商户配置切换：addConfig → switchover → switchoverTo。
/// 对应 Java: addConfig + switchover + switchoverTo
#[tokio::test]
async fn multi_config_switchover() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    let mut config2 = WxPayDefaultConfig::new();
    config2
        .set_app_id("wx_other_app")
        .set_mch_id("20000100")
        .set_mch_key("other_key_12345678901234567890")
        .set_api_host_url("http://127.0.0.1:1");

    service.add_config("20000100", "wx_other_app", Arc::new(config2));

    // 切换到新商户
    assert!(service.switchover("20000100", "wx_other_app"));
    let current = service.wx_pay_config();
    assert_eq!(current.mch_id(), Some("20000100"));

    // switchoverTo 成功
    service
        .switchover_to("20000100", "wx_other_app")
        .await
        .expect("切换成功");

    // switchoverTo 不存在的配置 → 报错
    let err = service
        .switchover_to("99999999", "wx_nonexist")
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("未找到对应配置"),
        "错误信息: {err}"
    );
}

/// 自定义配置键切换：addConfigWithKey → switchoverWithKey。
/// 对应 Java: addConfig(configKey, config) + switchover(configKey)
#[tokio::test]
async fn multi_config_custom_key_switchover() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    let mut config2 = WxPayDefaultConfig::new();
    config2
        .set_app_id("wx_tenant_app")
        .set_mch_id("30000100")
        .set_mch_key("tenant_key_12345678901234567890")
        .set_api_host_url("http://127.0.0.1:1");

    // 用 add_config 注册（键=30000100_wx_tenant_app）
    service.add_config("30000100", "wx_tenant_app", Arc::new(config2));

    // 精确匹配 mchId_appId 键
    assert!(service.switchover("30000100", "wx_tenant_app"));
    let current = service.wx_pay_config();
    assert_eq!(current.mch_id(), Some("30000100"));

    // 前缀匹配 mchId_*
    assert!(service.switchover_with_key("30000100"));

    // 自定义键
    service.add_config_with_key("tenant_abc", config_with_host("http://127.0.0.1:1"));
    assert!(service.switchover_with_key("tenant_abc"));

    // 不存在 → false
    assert!(!service.switchover_with_key("nonexist_key"));

    // switchover_to_with_key 成功
    service
        .switchover_to_with_key("tenant_abc")
        .await
        .expect("切换成功");

    // switchover_to_with_key 不存在 → 报错
    let err = service
        .switchover_to_with_key("nonexist")
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("未找到对应配置"),
        "错误信息: {err}"
    );
}

/// removeConfig / getConfigByMch。
/// 对应 Java: removeConfig + getConfig(mchId)
#[tokio::test]
async fn multi_config_remove_and_get() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    let mut config2 = WxPayDefaultConfig::new();
    config2
        .set_app_id("wx_app2")
        .set_mch_id("20000100")
        .set_mch_key("key2_2345678901234567890123456")
        .set_api_host_url("http://127.0.0.1:1");

    service.add_config("20000100", "wx_app2", Arc::new(config2));

    // get_config_by_mch
    let c = service.get_config_by_mch("20000100");
    assert!(c.is_some());
    assert_eq!(c.unwrap().mch_id(), Some("20000100"));

    // get_config_by_mch_app
    let c = service.get_config_by_mch_app("20000100", "wx_app2");
    assert!(c.is_some());

    // remove_config
    service.remove_config("20000100", "wx_app2");
    assert!(service.get_config_by_mch("20000100").is_none());

    // remove_config_with_key
    service.add_config_with_key("custom_key", config_with_host("http://127.0.0.1:1"));
    service.remove_config_with_key("custom_key");
}

/// set_multi_config 批量配置。
/// 对应 Java: setMultiConfig(Map<String, WxPayConfig>)
#[tokio::test]
async fn multi_config_set_multi() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    let mut configs = HashMap::new();
    configs.insert("key_a".to_string(), config_with_host("http://127.0.0.1:1"));
    let mut cfg_b = WxPayDefaultConfig::new();
    cfg_b
        .set_app_id("wx_b")
        .set_mch_id("mch_b")
        .set_mch_key("key_b_23456789012345678901234567890")
        .set_api_host_url("http://127.0.0.1:1");
    configs.insert("key_b".to_string(), Arc::new(cfg_b));

    service.set_multi_config(&configs);

    // set_config 自动注册到 config_map
    let mut cfg_c = WxPayDefaultConfig::new();
    cfg_c
        .set_app_id("wx_c")
        .set_mch_id("mch_c")
        .set_mch_key("key_c_23456789012345678901234567890")
        .set_api_host_url("http://127.0.0.1:1");
    service.set_config(Arc::new(cfg_c));
    let c = service.get_config_by_mch_app("mch_c", "wx_c");
    assert!(c.is_some());
}

/// get_wx_api_data 覆盖。
/// 对应 Java: wxApiData ThreadLocal
#[tokio::test]
async fn wx_api_data_record_and_get() {
    let server = MockServer::start(|_, _| {
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "testnonce"),
                ("transaction_id", "4001312001201707262674894706"),
                ("out_trade_no", "111111826"),
                ("trade_state", "SUCCESS"),
                ("total_fee", "1"),
            ],
            None,
        )
    })
    .await;
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_if_save_api_data(true)
        .set_api_host_url(format!("http://{}", server.addr));
    let service = WxPayServiceImpl::new_arc(Arc::new(config));

    // 触发一次请求 → 记录 api_data
    let _result = service
        .query_order(Some("4001312001201707262674894706"), None)
        .await
        .expect("查询成功");

    let data = service.get_wx_api_data();
    assert!(data.is_some());
    let data = data.unwrap();
    assert!(data.url.is_some());
    assert!(data.request_data.is_some());
    assert!(data.response_data.is_some());
}

// ═══════════════════════════════════════════════════════════════════
// 子服务 getter 覆盖（RUST_OBLIGATION: OnceLock 装配）
// ═══════════════════════════════════════════════════════════════════

/// 子服务 getter 全量覆盖。
/// 对应 Java: getXxxService()
#[test]
fn sub_service_getters_all_some() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    assert!(service.ent_pay_service().is_some());
    assert!(service.redpack_service().is_some());
    assert!(service.profit_sharing_service().is_some());
    assert!(service.pay_score_service().is_some());
    assert!(service.ecommerce_service().is_some());
    assert!(service.business_circle_service().is_some());
    assert!(service.merchant_media_service().is_some());
    assert!(service.marketing_media_service().is_some());
    assert!(service.marketing_favor_service().is_some());
    assert!(service.marketing_busi_favor_service().is_some());
    assert!(service.merchant_transfer_service().is_some());
    assert!(service.brand_merchant_transfer_service().is_some());
    assert!(service.subscription_billing_service().is_some());
    assert!(service.merchant_limitation_service().is_some());
    assert!(service.complaints_service().is_some());
    assert!(service.bank_service().is_some());
    assert!(service.transfer_service().is_some());
    assert!(service.business_operation_transfer_service().is_some());
    assert!(service.partner_pay_score_service().is_some());
    assert!(service.partner_pay_score_sign_plan_service().is_some());
    assert!(service.real_name_service().is_some());
    assert!(service.mi_pay_service().is_some());
    assert!(service.apply4_subject_confirm_service().is_some());
    assert!(service.applyment4_sub_service().is_some());
    assert!(service.custom_declaration_service().is_some());
    assert!(service.wx_entrust_pap_service().is_some());
    assert!(service.wx_deposit_service().is_some());
    assert!(service.partner_transfer_service().is_some());
    assert!(service.payroll_service().is_some());
}

// ═══════════════════════════════════════════════════════════════════
// v3 退款通知解析（SOURCE_PARITY: parseRefundNotifyV3Result）
// ═══════════════════════════════════════════════════════════════════

/// v3 退款通知：验签 + AES-GCM 解密。
/// 对应 Java: parseRefundNotifyV3Result
#[tokio::test]
async fn v3_refund_notify_decrypt_full_flow() {
    let decrypted_payload = json!({
        "out_refund_no": "R001",
        "out_trade_no": "T001",
        "refund_status": "SUCCESS",
        "success_time": "2024-01-01T00:00:00+08:00",
        "amount": {"total": 100, "refund": 100, "payer_total": 100, "payer_refund": 100}
    });
    let nonce = "refundnonce1";
    let ciphertext = aes_gcm_encrypt(
        API_V3_KEY,
        "refund",
        nonce.as_bytes(),
        &decrypted_payload.to_string(),
    )
    .expect("AES 加密");
    let notify_json = json!({
        "id": "EV-refund-001",
        "create_time": "2024-01-01T00:00:00+08:00",
        "event_type": "REFUND.SUCCESS",
        "resource_type": "encrypt-resource",
        "resource": {
            "original_type": "refund",
            "algorithm": "AEAD_AES_256_GCM",
            "ciphertext": ciphertext,
            "associated_data": "refund",
            "nonce": nonce
        }
    });
    let notify_data = notify_json.to_string();

    let timestamp = "1700000000";
    let header_nonce = "r0uYIzEaIUX9";
    let sign_message = format!("{timestamp}\n{header_nonce}\n{notify_data}\n");
    let signature =
        sign_sha256_rsa(&platform_private_key(), sign_message.as_bytes()).expect("平台签名");
    let header = SignatureHeader::new(
        Some(timestamp.to_string()),
        Some(header_nonce.to_string()),
        Some(signature),
        Some(PLATFORM_SERIAL.to_string()),
    );

    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service
        .parse_refund_notify_v3_result(&notify_data, &header)
        .await
        .expect("v3 退款通知解析成功");

    let decrypted = result.result.expect("解密结果");
    assert_eq!(decrypted.out_refund_no.as_deref(), Some("R001"));
    assert_eq!(decrypted.out_trade_no.as_deref(), Some("T001"));
    assert_eq!(decrypted.refund_status.as_deref(), Some("SUCCESS"));
}

// ═══════════════════════════════════════════════════════════════════
// v3 通知签名探测流量识别（RUST_OBLIGATION: SIGNTEST 探测）
// ═══════════════════════════════════════════════════════════════════

/// v3 通知签名探测 → 报错（对应 Java WECHATPAY/SIGNTEST/ 识别）。
#[tokio::test]
async fn v3_notify_sign_test_detection() {
    let header = SignatureHeader::new(
        Some("1700000000".to_string()),
        Some("nonce123".to_string()),
        Some("WECHATPAY/SIGNTEST/xxx".to_string()),
        Some(PLATFORM_SERIAL.to_string()),
    );

    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .parse_order_notify_v3_result("{}", &header)
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("签名探测流量"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// v2 get_pay_info 覆盖（SOURCE_PARITY: getPayInfo 已废弃接口）
// ═══════════════════════════════════════════════════════════════════

/// v2 get_pay_info NATIVE → codeUrl。
/// 对应 Java: getPayInfo → NATIVE
#[tokio::test]
async fn v2_get_pay_info_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "NATIVE"),
                ("code_url", "weixin://wxpay/bizpayurl?pr=abc123"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.trade_type = Some("NATIVE".to_string());
    request.out_trade_no = Some("native_001".to_string());
    request.product_id = Some("product_001".to_string());

    let pay_info = service
        .get_pay_info(&request)
        .await
        .expect("get_pay_info 成功");
    assert_eq!(
        pay_info.get("codeUrl").map(String::as_str),
        Some("weixin://wxpay/bizpayurl?pr=abc123")
    );
}

/// v2 get_pay_info APP → 包含 sign/package/prepayid。
/// 对应 Java: getPayInfo → APP
#[tokio::test]
async fn v2_get_pay_info_app() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("prepay_id", "wx201410272009395522657a690389285100"),
                ("trade_type", "APP"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.trade_type = Some("APP".to_string());
    request.out_trade_no = Some("app_001".to_string());

    let pay_info = service
        .get_pay_info(&request)
        .await
        .expect("get_pay_info APP 成功");
    // get_pay_info 返回 HashMap<String, String>，键为 camelCase
    assert!(
        pay_info.contains_key("sign"),
        "keys: {:?}",
        pay_info.keys().collect::<Vec<_>>()
    );
    assert!(
        pay_info.contains_key("prepayId"),
        "keys: {:?}",
        pay_info.keys().collect::<Vec<_>>()
    );
    assert_eq!(
        pay_info.get("prepayId").map(String::as_str),
        Some("wx201410272009395522657a690389285100")
    );
}
