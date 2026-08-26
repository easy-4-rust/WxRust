#![allow(clippy::field_reassign_with_default)]
//! WxPayService 门面全方法实现测试（Wave 2 P2a，MockServer 自含）。
//!
//! Golden 来源标注：
//! - `wxjava-sign-utils-test`：Java `SignUtilsTest` 的官方文档向量
//!   （appid=wxd930ea5d5a258f4f, mchId=10000100,
//!   mchKey=192006250b4c09247ec02edce69f6a2d，MD5/HMAC-SHA256 签名）；
//! - `wxjava-refund-notify-test`：Java `WxPayRefundNotifyResultTest` 的
//!   req_info 明文 golden（`encodeReqInfo` 方法的输出结构，字段值原样）；
//! - `official-wechatpay-java`：APIv3 密钥与商户私钥材料沿用
//!   `wx_pay_v3_crypto_test.rs`（P3）的同源夹具；
//! - 平台（微信支付）密钥对为本测试独立生成（仅测试用途）。
//!
//! MockServer 语义与 wx-rust-miniapp `sub_domain_g1_core.rs` 同构：
//! 记录最近一次请求路径/请求体/请求头，handler 返回 (状态码, Content-Type,
//! body, 附加响应头)；v3 响应由平台私钥按 `timestamp\nnonce\nbody\n` 签名。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use base64::Engine;
use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePrivateKey;
use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::notify::SignatureHeader;
use wx_rust_pay::bean::request::wx_pay_refund_v3_request::Amount as RefundV3Amount;
use wx_rust_pay::bean::request::wx_pay_unified_order_v3_request::{
    Amount as OrderV3Amount, Payer as OrderV3Payer,
};
use wx_rust_pay::bean::{TradeTypeEnum, WxPayUnifiedOrderRequest, WxPayUnifiedOrderV3Request};
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::{
    aes_gcm_encrypt, sign_sha256_rsa, verify_sha256_rsa,
};
use wx_rust_pay::util::sign_utils::SignUtils;

// ---- 夹具常量 ----

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

// ---- 测试辅助 ----

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

/// 构造带 v2 签名的 XML 响应（对应 Java 微信服务器行为：按报文字段
/// MD5/HMAC 签名后随 sign 一起返回）。
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

/// v2 XML 响应快捷构造（带 v2 签名，Content-Type text/xml）。
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

/// 平台私钥惰性加载（响应签名/通知签名）。
fn platform_private_key() -> rsa::RsaPrivateKey {
    load_private_key_from_pem(PLATFORM_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥")
}

// ---- 测试 ----

/// v2 统一下单：XML 请求体元素名/签名/金额断言 + 响应解析。
#[tokio::test]
async fn v2_unified_order_sends_signed_xml() {
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
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(1);
    request.spbill_create_ip = Some("11.1.11.1".to_string());
    request.notify_url = Some("111111".to_string());
    request.trade_type = Some("JSAPI".to_string());
    request.out_trade_no = Some("111111826".to_string());
    request.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());

    let result = service.unified_order(&request).await.expect("统一下单成功");
    assert_eq!(
        result.prepay_id.as_deref(),
        Some("wx201410272009395522657a690389285100")
    );

    let xml = server.last_body();
    // 元素名断言（serde rename 与 Java @XStreamAlias 一致）
    assert!(xml.contains("<appid>"), "缺少appid: {xml}");
    assert!(xml.contains("<mch_id>"), "缺少mch_id: {xml}");
    assert!(xml.contains("<nonce_str>"), "缺少nonce_str: {xml}");
    assert!(xml.contains("<body>test</body>"), "缺少body: {xml}");
    assert!(xml.contains("<total_fee>1</total_fee>"), "缺少金额: {xml}");
    assert!(
        xml.contains("<out_trade_no>111111826</out_trade_no>"),
        "缺少订单号: {xml}"
    );
    assert!(
        xml.contains("<trade_type>JSAPI</trade_type>"),
        "缺少交易类型: {xml}"
    );
    assert!(xml.contains("<sign>"), "缺少签名: {xml}");
    // 配置回填：请求未设置 appid/mch_id/nonce_str，由 checkAndSign 补齐
    assert!(
        xml.contains("<appid>wxd930ea5d5a258f4f</appid>"),
        "appid未回填: {xml}"
    );
    assert!(
        xml.contains("<mch_id>10000100</mch_id>"),
        "mch_id未回填: {xml}"
    );

    // 签名自洽校验：对发送报文重算 MD5 签名与报文 sign 一致
    let map = wx_rust_pay::bean::xml::root_children_map(&xml).expect("报文解析");
    let expected = SignUtils::create_sign(&map, None, MCH_KEY, &[]).expect("重算签名");
    assert_eq!(map.get("sign").map(String::as_str), Some(expected.as_str()));
}

/// v2 订单查询：transaction_id/out_trade_no 二选一约束 + 响应解析。
#[tokio::test]
async fn v2_query_order_xml_and_parse() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/orderquery"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("transaction_id", "4001312001201707262674894706"),
                ("out_trade_no", "111111826"),
                ("trade_state", "SUCCESS"),
                ("total_fee", "1"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .query_order(Some("4001312001201707262674894706"), None)
        .await
        .expect("查询订单成功");
    assert_eq!(
        result.transaction_id.as_deref(),
        Some("4001312001201707262674894706")
    );
    assert_eq!(result.trade_state.as_deref(), Some("SUCCESS"));
    let xml = server.last_body();
    assert!(
        xml.contains("<transaction_id>4001312001201707262674894706</transaction_id>"),
        "{xml}"
    );
    assert!(
        !xml.contains("<out_trade_no>"),
        "out_trade_no不应出现: {xml}"
    );

    // 两个参数同时为空 → 报错（对应 Java checkConstraints 文案）
    let err = service.query_order(None, None).await.expect_err("应报错");
    assert!(
        err.to_string()
            .contains("transaction_id 和 out_trade_no 不能同时存在或同时为空"),
        "错误信息: {err}"
    );
}

/// v2 退款（证书通道 useKey=true）：XML 请求体 + 签名 + 响应解析。
#[tokio::test]
async fn v2_refund_with_cert_channel_and_sign() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/secapi/pay/refund"));
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("return_msg", "OK"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", "1234567891"),
                ("nonce_str", "ibuaiVcKdpRxkhJA"),
                ("transaction_id", "4001312001201707262674894706"),
                ("out_trade_no", "201707260201501501005710775"),
                ("out_refund_no", "R4001312001201707262674894706_4"),
                ("refund_id", "50000203702017072601461713166"),
                ("refund_fee", "15"),
                ("total_fee", "100"),
            ],
            None,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_p12(&server.url("")));

    let mut request = wx_rust_pay::bean::WxPayRefundRequest::default();
    request.out_trade_no = Some("201707260201501501005710775".to_string());
    request.out_refund_no = Some("R4001312001201707262674894706_4".to_string());
    request.total_fee = Some(100);
    request.refund_fee = Some(15);
    request.op_user_id = Some("10000100".to_string());

    let result = service.refund(&request).await.expect("退款成功");
    assert_eq!(
        result.refund_id.as_deref(),
        Some("50000203702017072601461713166")
    );

    let xml = server.last_body();
    assert!(
        xml.contains("<out_refund_no>R4001312001201707262674894706_4</out_refund_no>"),
        "{xml}"
    );
    assert!(xml.contains("<total_fee>100</total_fee>"), "{xml}");
    assert!(xml.contains("<refund_fee>15</refund_fee>"), "{xml}");
    assert!(xml.contains("<sign>"), "{xml}");
    let map = wx_rust_pay::bean::xml::root_children_map(&xml).expect("报文解析");
    let expected = SignUtils::create_sign(&map, None, MCH_KEY, &[]).expect("重算签名");
    assert_eq!(map.get("sign").map(String::as_str), Some(expected.as_str()));
}

/// v2 订单通知解析：JSON 提示/验签通过/篡改拒绝。
#[tokio::test]
async fn v2_order_notify_parse_and_sign_verify() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));

    // 1. V3 JSON 通知数据 → 提示使用 v3 方法（对应 Java 文案）
    let err = service
        .parse_order_notify_result(r#"{"event_type":"TRANSACTION.SUCCESS"}"#)
        .await
        .expect_err("JSON 通知应报错");
    assert!(
        err.to_string().contains("检测到V3版本的JSON格式通知数据"),
        "错误信息: {err}"
    );

    // 2. 正常 XML 通知（带正确 MD5 签名）→ 解析 + 验签通过
    let xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "ibuaiVcKdpRxkhJA"),
            ("out_trade_no", "111111826"),
            ("transaction_id", "4001312001201707262674894706"),
            ("trade_type", "JSAPI"),
            ("total_fee", "1"),
            ("time_end", "20170726024549"),
        ],
        None,
    );
    let result = service
        .parse_order_notify_result(&xml)
        .await
        .expect("通知解析成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("111111826"));
    assert_eq!(result.total_fee, Some(1));

    // 3. 篡改报文 → 验签失败（对应 Java "参数格式校验错误！"）
    let tampered = xml.replace("111111826", "111111827");
    let err = service
        .parse_order_notify_result(&tampered)
        .await
        .expect_err("篡改报文应报错");
    assert!(
        err.to_string().contains("参数格式校验错误！"),
        "错误信息: {err}"
    );
}

/// v2 退款通知 req_info AES-256-ECB 解密（golden 来自 Java
/// `WxPayRefundNotifyResultTest.encodeReqInfo` 的明文结构）。
#[tokio::test]
async fn v2_refund_notify_ecb_decrypt_java_golden() {
    // Java 测试 golden 明文（字段值原样保留）
    let golden_plaintext = "<root>\n<out_refund_no><![CDATA[R4001312001201707262674894706_4]]></out_refund_no>\n<out_trade_no><![CDATA[201707260201501501005710775]]></out_trade_no>\n<refund_account><![CDATA[REFUND_SOURCE_UNSETTLED_FUNDS]]></refund_account>\n<refund_fee><![CDATA[15]]></refund_fee>\n<refund_id><![CDATA[50000203702017072601461713166]]></refund_id>\n<refund_recv_accout><![CDATA[用户零钱]]></refund_recv_accout>\n<refund_request_source><![CDATA[API]]></refund_request_source>\n<refund_status><![CDATA[SUCCESS]]></refund_status>\n<settlement_refund_fee><![CDATA[15]]></settlement_refund_fee>\n<settlement_total_fee><![CDATA[100]]></settlement_total_fee>\n<success_time><![CDATA[2017-07-26 02:45:49]]></success_time>\n<total_fee><![CDATA[100]]></total_fee>\n<transaction_id><![CDATA[4001312001201707262674894706]]></transaction_id>\n</root>";

    // 与 Java encodeReqInfo 相同算法加密：md5(mchKey) 小写 hex → AES-256-ECB
    // （测试侧用 aes crate 的 BlockEncrypt 逐块 + PKCS#7 填充）
    let key_md5 = wx_rust_pay::util::crypto::wx_pay_crypto_utils::md5_hex(MCH_KEY).to_lowercase();
    let ciphertext = {
        use aes::Aes256;
        use aes::cipher::{Block, BlockCipherEncrypt, KeyInit};
        let cipher = Aes256::new_from_slice(key_md5.as_bytes()).expect("AES密钥");
        let mut plain = golden_plaintext.as_bytes().to_vec();
        let pad_len = 16 - (plain.len() % 16);
        plain.extend(std::iter::repeat_n(pad_len as u8, pad_len));
        let mut out = Vec::new();
        for chunk in plain.chunks_exact(16) {
            let mut block = Block::<Aes256>::try_from(chunk).expect("块");
            cipher.encrypt_block(&mut block);
            out.extend_from_slice(block.as_slice());
        }
        base64::engine::general_purpose::STANDARD.encode(out)
    };

    let xml = format!(
        "<xml><return_code><![CDATA[SUCCESS]]></return_code>\
<appid><![CDATA[{APP_ID}]]></appid>\
<mch_id><![CDATA[{MCH_ID}]]></mch_id>\
<nonce_str><![CDATA[1ee38e38b04990449808688cf3a763b7]]></nonce_str>\
<req_info><![CDATA[{ciphertext}]]></req_info></xml>"
    );

    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service
        .parse_refund_notify_result(&xml)
        .await
        .expect("退款通知解析+解密成功");
    let req_info = result.req_info.expect("req_info 应已解密");
    assert_eq!(
        req_info.out_refund_no.as_deref(),
        Some("R4001312001201707262674894706_4")
    );
    assert_eq!(req_info.refund_fee, Some(15));
    assert_eq!(req_info.settlement_total_fee, Some(100));
    assert_eq!(req_info.refund_status.as_deref(), Some("SUCCESS"));
    assert_eq!(req_info.refund_recv_account.as_deref(), Some("用户零钱"));
    assert_eq!(
        req_info.transaction_id.as_deref(),
        Some("4001312001201707262674894706")
    );
}

/// v3 下单（JSAPI）：Authorization 头断言 + JSON 请求体 + RSA paySign 生成。
#[tokio::test]
async fn v3_create_order_jsapi_authorization_and_pay_info() {
    let server = MockServer::start(|path, headers| {
        assert!(path.starts_with("/v3/pay/transactions/jsapi"));
        // Authorization 头由 execute_v3 生成
        let auth = headers
            .get("authorization")
            .expect("缺少Authorization头")
            .clone();
        assert!(
            auth.starts_with("WECHATPAY2-SHA256-RSA2048 mchid=\"10000100\""),
            "Authorization: {auth}"
        );
        assert!(
            auth.contains(&format!("serial_no=\"{MERCHANT_SERIAL}\"")),
            "{auth}"
        );
        assert!(
            headers.contains_key("wechatpay-serial"),
            "缺少Wechatpay-Serial头"
        );
        signed_json_response(r#"{"prepay_id":"wx201410272009395522657a690389285100"}"#)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut request = WxPayUnifiedOrderV3Request::default();
    request.description = Some("测试商品".to_string());
    request.out_trade_no = Some("v3_out_trade_no_001".to_string());
    request.amount = Some(OrderV3Amount {
        total: Some(1),
        currency: Some("CNY".to_string()),
    });
    request.payer = Some(OrderV3Payer {
        openid: Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string()),
        ..Default::default()
    });

    let pay_info = service
        .create_order_v3(TradeTypeEnum::Jsapi, &request)
        .await
        .expect("v3 下单成功");
    // JSAPI → JsapiResult（appId/timeStamp/nonceStr/package/signType/paySign）
    let obj = pay_info.as_object().expect("应返回对象");
    assert_eq!(obj.get("appId").and_then(|v| v.as_str()), Some(APP_ID));
    assert_eq!(obj.get("signType").and_then(|v| v.as_str()), Some("RSA"));
    assert_eq!(
        obj.get("prepayId").and_then(|v| v.as_str()),
        Some("wx201410272009395522657a690389285100")
    );
    assert!(
        obj.get("package")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .starts_with("prepay_id=")
    );

    // 请求体断言：appid/mchid 由配置回填，notify_url 由配置补齐
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体JSON");
    assert_eq!(body["appid"], json!(APP_ID));
    assert_eq!(body["mchid"], json!(MCH_ID));
    assert_eq!(body["description"], json!("测试商品"));
    assert_eq!(body["out_trade_no"], json!("v3_out_trade_no_001"));
    assert_eq!(body["notify_url"], json!("https://example.com/pay/notify"));

    // paySign 用商户公钥验签（对应 Java SignUtils.sign：SHA256withRSA）
    let pay_sign = obj
        .get("paySign")
        .and_then(|v| v.as_str())
        .expect("paySign");
    let sign_str = format!(
        "{}\n{}\n{}\n{}\n",
        obj.get("appId")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        obj.get("timeStamp")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        obj.get("nonceStr")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        obj.get("package")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    );
    let merchant_priv =
        rsa::RsaPrivateKey::from_pkcs8_pem(MERCHANT_PRIVATE_KEY_PEM).expect("商户私钥");
    let merchant_pub: RsaPublicKey = merchant_priv.to_public_key();
    assert!(
        verify_sha256_rsa(&merchant_pub, sign_str.as_bytes(), pay_sign).expect("验签"),
        "paySign 验签失败"
    );
}

/// v3 申请退款：notify_url 默认值回填 + 响应解析。
#[tokio::test]
async fn v3_refund_notify_url_default_and_response() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/refund/domestic/refunds"));
        signed_json_response(
            r#"{"refund_id":"50000000382019052709732678859","out_refund_no":"1217752501201407033233368018","out_trade_no":"1217752501201407033233368018","channel":"ORIGINAL","status":"SUCCESS"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut request = wx_rust_pay::bean::WxPayRefundV3Request::default();
    request.out_trade_no = Some("1217752501201407033233368018".to_string());
    request.out_refund_no = Some("1217752501201407033233368018".to_string());
    request.reason = Some("商品已售完".to_string());
    request.amount = Some(RefundV3Amount {
        refund: Some(888),
        total: Some(888),
        currency: Some("CNY".to_string()),
        ..Default::default()
    });

    let result = service.refund_v3(&request).await.expect("v3 退款成功");
    assert_eq!(
        result.refund_id.as_deref(),
        Some("50000000382019052709732678859")
    );
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体JSON");
    assert_eq!(body["out_refund_no"], json!("1217752501201407033233368018"));
    // notify_url 未设置 → 从配置 refundNotifyUrl 补齐
    assert_eq!(
        body["notify_url"],
        json!("https://example.com/pay/refund-notify")
    );
}

/// v3 支付结果通知全流程：验签（平台证书公钥）+ AES-GCM 解密 +
/// 反序列化（加密向量与 P3 官方向量同构）。
#[tokio::test]
async fn v3_order_notify_decrypt_full_flow() {
    let decrypted_payload = json!({
        "appid": "wxd678efh567hg6787",
        "mchid": "1230000109",
        "out_trade_no": "1217752501201407033233368018",
        "transaction_id": "1217752501201407033233368018",
        "trade_type": "JSAPI",
        "trade_state": "SUCCESS",
        "trade_state_desc": "支付成功",
        "bank_type": "CMC",
        "attach": "自定义数据",
        "success_time": "2018-06-08T10:34:56+08:00",
        "payer": { "openid": "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o" },
        "amount": { "total": 100, "payer_total": 100, "currency": "CNY", "payer_currency": "CNY" }
    });
    // 微信侧：APIv3 密钥 AES-GCM 加密 resource
    let nonce = "fdasflkja4qw";
    let ciphertext = aes_gcm_encrypt(
        API_V3_KEY,
        "transaction",
        nonce.as_bytes(),
        &decrypted_payload.to_string(),
    )
    .expect("AES 加密");
    let notify_json = json!({
        "id": "EV-2018022511223320873",
        "create_time": "2015-05-20T13:29:35+08:00",
        "event_type": "TRANSACTION.SUCCESS",
        "resource_type": "encrypt-resource",
        "summary": "支付成功",
        "resource": {
            "original_type": "transaction",
            "algorithm": "AEAD_AES_256_GCM",
            "ciphertext": ciphertext,
            "associated_data": "transaction",
            "nonce": nonce
        }
    });
    let notify_data = notify_json.to_string();

    // 微信侧：对通知体签名（Wechatpay-Signature 头）
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
        .parse_order_notify_v3_result(&notify_data, &header)
        .await
        .expect("v3 通知解析成功");

    let decrypted = result.result.expect("解密结果");
    assert_eq!(
        decrypted.out_trade_no.as_deref(),
        Some("1217752501201407033233368018")
    );
    assert_eq!(decrypted.trade_state.as_deref(), Some("SUCCESS"));
    assert_eq!(decrypted.trade_state_desc.as_deref(), Some("支付成功"));
    assert_eq!(decrypted.amount.as_ref().and_then(|a| a.total), Some(100));
    assert_eq!(
        decrypted.payer.as_ref().and_then(|p| p.openid.as_deref()),
        Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")
    );
    // rawData 原样保留
    let raw = result.raw_data.expect("rawData");
    assert_eq!(raw.event_type.as_deref(), Some("TRANSACTION.SUCCESS"));
    assert_eq!(
        raw.resource.as_ref().and_then(|r| r.algorithm.as_deref()),
        Some("AEAD_AES_256_GCM")
    );
}

/// v3 响应验签：拒绝被篡改的响应体（对应 Java "应答的微信支付签名验证失败"）。
#[tokio::test]
async fn v3_response_verification_rejects_tampered() {
    // 用错误的签名头（对另一份 body 签名）返回
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/pay/transactions/out-trade-no/"));
        let body = r#"{"appid":"wxd678efh567hg6787","trade_state":"SUCCESS"}"#;
        let timestamp = "1712345678";
        let nonce = "testnonce1234";
        // 对不同的报文签名 → 验签必然失败
        let message = format!("{timestamp}\n{nonce}\n{{\"tampered\":true}}\n");
        let signature = sign_sha256_rsa(&platform_private_key(), message.as_bytes()).expect("签名");
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
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let err = service
        .query_order_v3(None, Some("out_trade_no_001"))
        .await
        .expect_err("篡改响应应报错");
    assert!(
        err.to_string().contains("应答的微信支付签名验证失败"),
        "错误信息: {err}"
    );
}

/// v3 订单查询：URL 路径/query 断言 + 响应解析。
#[tokio::test]
async fn v3_query_order_url_and_parse() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.starts_with("/v3/pay/transactions/out-trade-no/out123?mchid=10000100"),
            "路径: {path}"
        );
        signed_json_response(
            r#"{"appid":"wxd678efh567hg6787","mchid":"1230000109","out_trade_no":"out123","transaction_id":"1217752501201407033233368018","trade_state":"SUCCESS","amount":{"total":100,"payer_total":100,"currency":"CNY"}}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .query_order_v3(None, Some("out123"))
        .await
        .expect("v3 查询成功");
    assert_eq!(result.trade_state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.out_trade_no.as_deref(), Some("out123"));
}

/// v2 对账单下载与文本解析（ALL 布局，Java 格式 golden）。
#[tokio::test]
async fn v2_download_bill_parse_java_format() {
    // Java BaseWxPayServiceImplTest 数据格式（` 分割行、`,` 占位；
    // ALL 布局 27 列：交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,
    // 商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,
    // 代金券金额,微信退款单号,商户退款单号,退款金额,充值券退款金额,退款类型,
    // 退款状态,商品名称,商户数据包,手续费,费率,订单金额,申请退款金额,费率备注）
    let bill_text = "交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,代金券金额,微信退款单号,商户退款单号,退款金额,充值券退款金额,退款类型,退款状态,商品名称,商户数据包,手续费,费率,订单金额,申请退款金额,费率备注\n\
`2018-02-01 04:21:23,`wx2421b1c4370ec43b,`10000100,`,`1000,`50000305742018020103387128253,`201707260201501501005710775,`oUpF8uMuAJO_M2pxb1Q9zNjWeS6o,`JSAPI,`SUCCESS,`CMC,`CNY,`100.00,`0.00,`,`,`15.00,`0.00,`REFUND_SOURCE_RECHARGE_FUNDS,`SUCCESS,`测试商品,`attach-data,`0.01000,`0.60%,`100.00,`0.00,`\n\
总交易单数,应结订单总金额,退款总金额,充值券退款总金额,手续费总金额,订单总金额,申请退款总金额\n\
`1,`100.00,`0.00,`0.00,`0.01000,`100.00,`0.00";
    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"));
        (200, "text/plain".to_string(), bill_text.to_string(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .download_bill("20180201", "ALL", "", None)
        .await
        .expect("下载对账单成功");
    let xml = server.last_body();
    assert!(xml.contains("<bill_type>ALL</bill_type>"), "{xml}");
    assert!(xml.contains("<bill_date>20180201</bill_date>"), "{xml}");
    assert!(xml.contains("<sign>"), "{xml}");

    assert_eq!(result.total_record.as_deref(), Some("1"));
    assert_eq!(result.total_fee.as_deref(), Some("100.00"));
    let info = &result.bill_info_list[0];
    assert_eq!(info.trade_time.as_deref(), Some("2018-02-01 04:21:23"));
    assert_eq!(
        info.transaction_id.as_deref(),
        Some("50000305742018020103387128253")
    );
    assert_eq!(
        info.out_trade_no.as_deref(),
        Some("201707260201501501005710775")
    );
    assert_eq!(info.body.as_deref(), Some("测试商品"));
    assert_eq!(info.total_fee.as_deref(), Some("100.00"));
    assert_eq!(info.poundage.as_deref(), Some("0.01000"));
    assert_eq!(info.fee_remark.as_deref(), Some(""));
}

/// v2 签名算法 golden（官方文档向量，Java SignUtilsTest 同源）。
#[test]
fn v2_sign_utils_golden_official_doc() {
    let params = {
        let mut m = HashMap::new();
        m.insert("appid".to_string(), "wxd930ea5d5a258f4f".to_string());
        m.insert("body".to_string(), "test".to_string());
        m.insert("device_info".to_string(), "1000".to_string());
        m.insert("mch_id".to_string(), "10000100".to_string());
        m.insert("nonce_str".to_string(), "ibuaiVcKdpRxkhJA".to_string());
        m
    };
    // MD5 签名（Java SignUtilsTest.testCreateSign 期望值）
    let md5_sign = SignUtils::create_sign(&params, None, MCH_KEY, &[]).expect("MD5签名");
    assert_eq!(md5_sign, "9A0A8659F005D6984697E2CA0A9CF3B7");
    // HMAC-SHA256 签名（Java SignUtilsTest.testCreateSign_HMACSHA256 期望值）
    let hmac_sign =
        SignUtils::create_sign(&params, Some("HMAC-SHA256"), MCH_KEY, &[]).expect("HMAC签名");
    assert_eq!(
        hmac_sign,
        "6A9AE1657590FD6257D693A078E1C3E4BB6BA4DC30B23E0EE2496E54170DACD6"
    );
}

/// v2 扫码支付通知解析 + 配置签名类型验签。
#[tokio::test]
async fn v2_scan_pay_notify_parse() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_sign_type("MD5")
        .set_api_host_url("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(Arc::new(config));

    let xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "ibuaiVcKdpRxkhJA"),
            ("openid", "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
            ("product_id", "88888"),
            ("is_subscribe", "N"),
        ],
        None,
    );
    let result = service
        .parse_scan_pay_notify_result(&xml)
        .await
        .expect("扫码通知解析成功");
    assert_eq!(result.product_id.as_deref(), Some("88888"));
    assert_eq!(
        result.openid.as_deref(),
        Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")
    );
}

/// v2 订单评价拉取：强制 HMAC-SHA256 签名 + 原始响应透传。
#[tokio::test]
async fn v2_query_comment_hmac_sign() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/billcommentsp/batchquerycomment"));
        (
            200,
            "application/json".to_string(),
            r#"{"result":[]}"#.to_string(),
            vec![],
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_p12(&server.url("")));

    let begin = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .expect("时间")
        .with_timezone(&chrono::Utc);
    let end = chrono::DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
        .expect("时间")
        .with_timezone(&chrono::Utc);
    let response = service
        .query_comment(begin, end, Some(0), Some(10))
        .await
        .expect("查询评价成功");
    assert_eq!(response, r#"{"result":[]}"#);

    let xml = server.last_body();
    assert!(
        xml.contains("<sign_type>HMAC-SHA256</sign_type>"),
        "签名类型: {xml}"
    );
    assert!(
        xml.contains("<begin_time>20240101000000</begin_time>"),
        "{xml}"
    );
    assert!(xml.contains("<end_time>20240102000000</end_time>"), "{xml}");
    let map = wx_rust_pay::bean::xml::root_children_map(&xml).expect("报文解析");
    let expected =
        SignUtils::create_sign(&map, Some("HMAC-SHA256"), MCH_KEY, &[]).expect("重算HMAC签名");
    assert_eq!(map.get("sign").map(String::as_str), Some(expected.as_str()));
}

/// v3 商家券/营销类 get 请求：getV3WithWechatPaySerial 通道（以查询订单 v3
/// 的 transactionId 分支验证 URL 选择）。
#[tokio::test]
async fn v3_query_order_by_transaction_id_url() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.starts_with("/v3/pay/transactions/id/4001312001201707262674894706?mchid=10000100"),
            "路径: {path}"
        );
        signed_json_response(
            r#"{"appid":"wxd678efh567hg6787","mchid":"1230000109","transaction_id":"4001312001201707262674894706","trade_state":"SUCCESS"}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .query_order_v3(Some("4001312001201707262674894706"), None)
        .await
        .expect("按transactionId查询成功");
    assert_eq!(result.trade_state.as_deref(), Some("SUCCESS"));
}

/// 合单支付 v3（NATIVE）：combine 直连 + combineTransactions 组装 code_url。
#[tokio::test]
async fn v3_combine_transactions_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/combine-transactions/native"));
        signed_json_response(r#"{"prepay_id":"wx201410272009395522657a690389285100","code_url":"weixin://wxpay/bizpayurl/up?pr=abc123"}"#)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let mut request = wx_rust_pay::bean::CombineTransactionsRequest::default();
    request.combine_out_trade_no = Some("combine_out_001".to_string());
    request.sub_orders = vec![];

    let pay_info = service
        .combine_transactions(TradeTypeEnum::Native, &request)
        .await
        .expect("合单支付成功");
    // NATIVE → code_url 字符串
    assert_eq!(
        pay_info.as_str(),
        Some("weixin://wxpay/bizpayurl/up?pr=abc123")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体JSON");
    assert_eq!(body["combine_appid"], json!(APP_ID), "combine_appid 应回填");
    assert_eq!(body["combine_mchid"], json!(MCH_ID), "combine_mchid 应回填");
    assert_eq!(body["combine_out_trade_no"], json!("combine_out_001"));
}

/// v3 退款查询（按 out_refund_no，服务商 sub_mchid query 分支）。
#[tokio::test]
async fn v3_refund_query_and_partner_query() {
    let server = MockServer::start(|_path, _| {
        signed_json_response(
            r#"{"refund_id":"50000000382019052709732678859","out_refund_no":"1217752501201407033233368018","out_trade_no":"1217752501201407033233368018","status":"SUCCESS","amount":{"total":888,"refund":888,"payer_total":888,"payer_refund":888}}"#,
        )
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));

    let result = service
        .refund_query_v3("1217752501201407033233368018")
        .await
        .expect("v3 退款查询成功");
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));
    assert!(
        server
            .last_path()
            .starts_with("/v3/refund/domestic/refunds/1217752501201407033233368018"),
        "路径: {}",
        server.last_path()
    );
}

/// 服务商退款 v3：sub_mchid 为空时从配置补齐（对应 Java `partnerRefundV3`
/// 的 `StringUtils.isBlank(subMchid)` → `config.getSubMchId()`）。
#[tokio::test]
async fn partner_refund_v3_fills_sub_mchid_from_config() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/refund/domestic/refunds"));
        signed_json_response(
            r#"{"refund_id":"50000000382019052709732678859","out_refund_no":"REFUND_001","out_trade_no":"TRADE_001","channel":"ORIGINAL","status":"SUCCESS"}"#,
        )
    })
    .await;
    // 配置含 sub_mch_id
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
        .set_refund_notify_url("https://example.com/pay/refund-notify")
        .set_sub_mch_id("SUB_MCH_123")
        .set_api_host_url(server.url(""));
    let service = WxPayServiceImpl::new_arc(Arc::new(config));

    let mut request = wx_rust_pay::bean::WxPayPartnerRefundV3Request::default();
    request.out_trade_no = Some("TRADE_001".to_string());
    request.out_refund_no = Some("REFUND_001".to_string());
    request.reason = Some("测试退款".to_string());
    // WxPayPartnerRefundV3Request.amount 使用 combine_transactions_request::Amount
    // （use super::* 重导出），字段为 total_amount + currency
    request.amount = Some(wx_rust_pay::bean::request::Amount {
        total_amount: Some(888),
        currency: Some("CNY".to_string()),
    });
    // sub_mchid 未设置 → 应从配置补齐
    assert!(request.sub_mchid.is_none());

    let result = service
        .partner_refund_v3(&request)
        .await
        .expect("服务商 v3 退款成功");
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));

    // 验证请求体包含从配置补齐的 sub_mchid
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体JSON");
    assert_eq!(
        body["sub_mchid"],
        json!("SUB_MCH_123"),
        "sub_mchid 应从配置补齐"
    );
    // sp_appid 也应从配置补齐
    assert_eq!(body["sp_appid"], json!(APP_ID));
    // notify_url 应从配置 refund_notify_url 补齐
    assert_eq!(
        body["notify_url"],
        json!("https://example.com/pay/refund-notify")
    );
}
