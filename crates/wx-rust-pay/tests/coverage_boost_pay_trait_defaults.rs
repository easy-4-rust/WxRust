#![allow(clippy::field_reassign_with_default)]
//! 覆盖率提升：`WxPayService` trait 默认实现（`wx_pay_service.rs`）全量补齐。
//!
//! 覆盖（与 coverage_boost_pay_service_mock.rs 互补，不重复其用例）：
//! - trait 默认实现的配置管理（addConfig/removeConfig/switchover 族，
//!   默认 no-op/false/Err，经仅实现两个必需方法的 BareService 触发）；
//! - 29 个子服务 getter 默认 None + getWxApiData 默认 None；
//! - getPayBaseUrl 沙箱/非沙箱两分支；
//! - v2 证书通道（refund/refundV2/reverseOrder/sendCoupon/queryComment，
//!   p12 夹具）与代金券查询、汇率、刷脸；
//! - partner 变体（queryPartnerOrderV3/closePartnerOrderV3/
//!   createPartnerOrderV3/unifiedPartnerOrderV3/partnerRefundV3/
//!   refundPartnerQueryV3/codepay 服务商模式）；
//! - 合单（queryCombine/closeCombine/combine/combineTransactions
//!   JSAPI/APP/NATIVE 分支）；
//! - 境外支付（createOrderV3Global 四种 tradeType，死代理保离线）；
//! - 沙箱 key/人脸核身固定域名方法（死代理客户端保离线）；
//! - v2/v3 通知解析全家桶（含 refund req_info AES-ECB 解密、SIGNTEST）；
//! - 对账单/资金账单（文本/GZIP/错误 XML/约束检查/v3 账单申请）；
//! - 扫码模式一二维码 URL 生成与两个未实现占位。
//!
//! 测试三层：
//! - SOURCE_PARITY: 镜像 Java BaseWxPayServiceImpl 对应方法行为
//! - RUST_OBLIGATION: Rust 错误路径/约束检查/默认占位
//! - VALUE_ADD: 死代理离线策略、GZIP 账单、通知解密全流程

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use chrono::TimeZone;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::bean::notify::SignatureHeader;
use wx_rust_pay::bean::request::wx_pay_partner_unified_order_v3_request::Amount as PartnerAmount;
use wx_rust_pay::bean::request::wx_pay_unified_order_v3_request::Amount as OrderV3Amount;
use wx_rust_pay::bean::{
    CombineCloseRequest, GlobalTradeTypeEnum, TradeTypeEnum, WxPayCodepayRequest,
    WxPayCouponInfoQueryRequest, WxPayCouponSendRequest, WxPayCouponStockQueryRequest,
    WxPayDownloadFundFlowRequest, WxPayFacepayRequest, WxPayPartnerOrderCloseV3Request,
    WxPayPartnerRefundV3Request, WxPayPartnerUnifiedOrderV3Request, WxPayQueryCommentRequest,
    WxPayQueryExchangeRateRequest, WxPayRefundQueryRequest, WxPayRefundQueryV3Request,
    WxPayRefundRequest, WxPayRefundV3Request, WxPayUnifiedOrderRequest,
    WxPayUnifiedOrderV3GlobalRequest,
};
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::constant::wx_pay_constants::WxPaySpecificTradeType;
use wx_rust_pay::util::crypto::wx_pay_cert_utils::load_private_key_from_pem;
use wx_rust_pay::util::crypto::wx_pay_v3_crypto_utils::{aes_gcm_encrypt, sign_sha256_rsa};
use wx_rust_pay::util::sign_utils::SignUtils;

// ---- 夹具常量（与 coverage_boost_pay_service_mock.rs 同源） ----

const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";
const API_V3_KEY: &str = "a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb";
const MERCHANT_SERIAL: &str = "5F1C72E2A8931B72A2E13ADE3BB492C7B9C71571";
const PLATFORM_SERIAL: &str = "PLATFORM_SERIAL_TEST_1";
const P12_MCH_ID: &str = "1234567891";

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

/// 商户 p12 证书夹具（openssl pkcs12 -export -legacy，密码=商户号
/// `1234567891`；与 wx_pay_service_impl_test.rs 同源）。
const MERCHANT_P12_BASE64: &str =
    "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
BMfvJQQQ8QQsT1mnFF3iTOfoHbyj2wICCAA=";

// ---- MockServer（与 coverage_boost_pay_service_mock.rs 同构，body 改为字节以支持 GZIP 账单） ----

struct MockServer {
    addr: std::net::SocketAddr,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str, &HashMap<String, String>) -> (u16, String, Vec<u8>, Vec<(String, String)>)
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
                    let mut bytes = response.into_bytes();
                    bytes.extend_from_slice(&body);
                    let _ = socket.write_all(&bytes).await;
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

// ---- 测试辅助（与 coverage_boost_pay_service_mock.rs 同源） ----

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

/// p12 夹具 DER 字节。
fn p12_der() -> Vec<u8> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(MERCHANT_P12_BASE64.replace('\n', ""))
        .expect("p12 base64")
}

/// 构建带 p12 证书的配置（v2 退款等 useKey=true 场景；p12 密码=商户号）。
fn config_with_p12(host: &str) -> Arc<dyn WxPayConfig> {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(P12_MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_key_content(p12_der())
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
) -> (u16, String, Vec<u8>, Vec<(String, String)>) {
    (
        200,
        "text/xml".to_string(),
        v2_signed_response(fields, sign_type).into_bytes(),
        vec![],
    )
}

fn signed_json_response(body: &str) -> (u16, String, Vec<u8>, Vec<(String, String)>) {
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
        body.as_bytes().to_vec(),
        vec![
            ("Wechatpay-Timestamp".to_string(), timestamp.to_string()),
            ("Wechatpay-Nonce".to_string(), nonce.to_string()),
            ("Wechatpay-Signature".to_string(), signature),
            ("Wechatpay-Serial".to_string(), PLATFORM_SERIAL.to_string()),
        ],
    )
}

fn no_content_response() -> (u16, String, Vec<u8>, Vec<(String, String)>) {
    (204, "application/json".to_string(), Vec::new(), vec![])
}

/// GZIP 压缩文本（对应 Java ZipUtils.gzip 的响应侧）。
fn gzip_bytes(text: &str) -> Vec<u8> {
    use std::io::Write;
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(text.as_bytes()).expect("gzip 写入");
    encoder.finish().expect("gzip 完成")
}

/// AES-256-ECB + PKCS7 加密（`decrypt_refund_req_info` 的逆运算，
/// 对应 Java `SecureUtil.aes(md5Hex(key))` 加密 refund req_info）。
fn aes_256_ecb_encrypt_pkcs7(key: &[u8], plain: &[u8]) -> Vec<u8> {
    use aes::Aes256;
    use aes::cipher::{Block, BlockCipherEncrypt, KeyInit};
    use md5::{Digest, Md5};
    let key_md5 = hex::encode(Md5::digest(key));
    let cipher = Aes256::new_from_slice(key_md5.as_bytes()).expect("AES-256 密钥长度");
    let mut data = plain.to_vec();
    let pad = 16 - (data.len() % 16);
    data.resize(data.len() + pad, pad as u8);
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks_exact(16) {
        let mut block = Block::<Aes256>::try_from(chunk).expect("AES 块长度");
        cipher.encrypt_block(&mut block);
        out.extend_from_slice(block.as_slice());
    }
    out
}

/// 仅实现两个必需方法的裸服务：其余方法全部走 trait 默认实现
/// （对应 Java `BaseWxPayServiceImpl` 未被子类覆写的方法体）。
struct BarePayService {
    config: Arc<dyn WxPayConfig>,
    client: reqwest::Client,
}

#[async_trait]
impl WxPayService for BarePayService {
    fn wx_pay_config(&self) -> Arc<dyn WxPayConfig> {
        self.config.clone()
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}

fn bare_service(config: Arc<dyn WxPayConfig>) -> BarePayService {
    BarePayService {
        config,
        client: reqwest::Client::new(),
    }
}

/// 死代理客户端：所有请求指向本机未监听端口，连接拒绝即返回
/// （固定域名方法 get_sandbox_sign_key/get_wx_pay_face_auth_info/
/// 境外 apihk 基地址的离线保底策略，VALUE_ADD）。
fn dead_proxy_client() -> reqwest::Client {
    reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:9").expect("死代理配置"))
        .build()
        .expect("构建死代理客户端")
}

// ═══════════════════════════════════════════════════════════════════
// RUST_OBLIGATION：trait 默认实现——配置管理族（默认 no-op/false/Err）
// ═══════════════════════════════════════════════════════════════════

/// 配置管理默认实现：addConfig/removeConfig/setMultiConfig 均 no-op；
/// switchover 族默认 false/Err；getConfig 回退主配置。
/// 对应 Java: BaseWxPayServiceImpl 配置管理（子类未覆写时）
#[tokio::test]
async fn trait_default_config_management() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    // no-op 方法可直接调用不 panic
    let mut extra = WxPayDefaultConfig::new();
    extra.set_app_id("wx_extra").set_mch_id("mch_extra");
    service.add_config("mch_extra", "wx_extra", Arc::new(extra));
    let mut extra2 = WxPayDefaultConfig::new();
    extra2.set_app_id("wx_key").set_mch_id("mch_key");
    service.add_config_with_key("tenant_key", Arc::new(extra2));
    service.remove_config("mch_extra", "wx_extra");
    service.remove_config_with_key("tenant_key");
    let mut configs = HashMap::new();
    configs.insert("k".to_string(), config_with_host("http://127.0.0.1:1"));
    service.set_multi_config(&configs);
    service.set_multi_config_with_default(&configs, "mch_default");
    let mut cfg = WxPayDefaultConfig::new();
    cfg.set_app_id("wx_set").set_mch_id("mch_set");
    service.set_config(Arc::new(cfg));

    // 默认切换实现：boolean false / Result Err
    assert!(!service.switchover("mch_extra", "wx_extra"));
    assert!(!service.switchover_with_key("tenant_key"));
    let err = service
        .switchover_to("mch_extra", "wx_extra")
        .await
        .expect_err("默认 switchover_to 应报错");
    assert!(
        err.to_string().contains("未找到对应配置"),
        "错误信息: {err}"
    );
    let err = service
        .switchover_to_with_key("tenant_key")
        .await
        .expect_err("默认 switchover_to_with_key 应报错");
    assert!(
        err.to_string().contains("未找到对应配置"),
        "错误信息: {err}"
    );

    // get_config 回退主配置；按 mch 查询默认 None
    assert_eq!(service.get_config().mch_id(), Some(MCH_ID));
    assert!(
        service
            .get_config_by_mch_app("mch_extra", "wx_extra")
            .is_none()
    );
    assert!(service.get_config_by_mch("mch_extra").is_none());
    // get_wx_api_data 默认 None（BareService 未覆写记录逻辑）
    assert!(service.get_wx_api_data().is_none());
}

/// getPayBaseUrl 沙箱/非沙箱两分支。
/// 对应 Java: getPayBaseUrl() → useSandboxEnv 拼接沙箱后缀
#[test]
fn trait_default_get_pay_base_url_sandbox() {
    let service = bare_service(config_with_host("http://mock.pay.host"));
    assert_eq!(service.get_pay_base_url(), "http://mock.pay.host");

    let mut sandbox = WxPayDefaultConfig::new();
    sandbox
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_api_host_url("http://mock.pay.host")
        .set_use_sandbox_env(true);
    let sandbox_service = bare_service(Arc::new(sandbox));
    assert_eq!(
        sandbox_service.get_pay_base_url(),
        "http://mock.pay.host/xdc/apiv2sandbox"
    );
}

/// 29 个子服务 getter 默认返回 None（对应 Java `getXxxService()` 未装配）。
#[test]
fn trait_default_sub_service_getters_none() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));
    assert!(service.wx_entrust_pap_service().is_none());
    assert!(service.wx_deposit_service().is_none());
    assert!(service.partner_transfer_service().is_none());
    assert!(service.payroll_service().is_none());
    assert!(service.ent_pay_service().is_none());
    assert!(service.redpack_service().is_none());
    assert!(service.profit_sharing_service().is_none());
    assert!(service.pay_score_service().is_none());
    assert!(service.ecommerce_service().is_none());
    assert!(service.business_circle_service().is_none());
    assert!(service.merchant_media_service().is_none());
    assert!(service.marketing_media_service().is_none());
    assert!(service.marketing_favor_service().is_none());
    assert!(service.marketing_busi_favor_service().is_none());
    assert!(service.merchant_transfer_service().is_none());
    assert!(service.brand_merchant_transfer_service().is_none());
    assert!(service.subscription_billing_service().is_none());
    assert!(service.merchant_limitation_service().is_none());
    assert!(service.complaints_service().is_none());
    assert!(service.bank_service().is_none());
    assert!(service.transfer_service().is_none());
    assert!(service.business_operation_transfer_service().is_none());
    assert!(service.partner_pay_score_service().is_none());
    assert!(service.partner_pay_score_sign_plan_service().is_none());
    assert!(service.real_name_service().is_none());
    assert!(service.mi_pay_service().is_none());
    assert!(service.apply4_subject_confirm_service().is_none());
    assert!(service.applyment4_sub_service().is_none());
    assert!(service.custom_declaration_service().is_none());
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：HTTP 执行引擎默认实现（postWithMimeType/postForBytes/
// postV3WithRequest/requestV3/getV3WithWechatPaySerial/downloadV3）
// ═══════════════════════════════════════════════════════════════════

/// HTTP 引擎默认实现直连调用。
/// 对应 Java: post(url, str, useKey, mimeType) / postForBytes /
/// postV3(url, httpPost) / requestV3 / getV3WithWechatPaySerial / downloadV3
#[tokio::test]
async fn engine_default_methods_direct_call() {
    let server = MockServer::start(|path, _| {
        if path.starts_with("/v3/binary") {
            (
                200,
                "application/octet-stream".to_string(),
                vec![1, 2, 3],
                vec![],
            )
        } else {
            signed_json_response(r#"{"status":"ok"}"#)
        }
    })
    .await;
    let base = format!("http://{}", server.addr);
    let service = bare_service(config_with_host(&base));

    // post_with_mime_type（默认实现）
    let resp = service
        .post_with_mime_type(
            &format!("{base}/v3/test"),
            "<xml/>",
            false,
            "application/xml",
        )
        .await
        .expect("postWithMimeType 成功");
    assert!(resp.contains("status"));

    // post_for_bytes（默认实现，非证书通道）
    let bytes = service
        .post_for_bytes(&format!("{base}/v3/test"), "raw", false)
        .await
        .expect("postForBytes 成功");
    assert!(!bytes.is_empty());

    // post_v3_with_request / request_v3（预构建 reqwest::Request）
    let url = format!("{base}/v3/req");
    let mut request = reqwest::Request::new(reqwest::Method::POST, url.parse().expect("URL 解析"));
    *request.body_mut() = Some(reqwest::Body::from(r#"{"k":"v"}"#));
    let resp = service
        .post_v3_with_request(&url, &request)
        .await
        .expect("postV3(url, httpPost) 成功");
    assert!(resp.contains("ok"));
    let mut request2 = reqwest::Request::new(reqwest::Method::PUT, url.parse().expect("URL 解析"));
    *request2.body_mut() = Some(reqwest::Body::from(r#"{"k":"v2"}"#));
    let resp = service
        .request_v3(&url, &request2)
        .await
        .expect("requestV3 成功");
    assert!(resp.contains("ok"));

    // get_v3_with_wechat_pay_serial（默认实现）
    let resp = service
        .get_v3_with_wechat_pay_serial(&format!("{base}/v3/test"))
        .await
        .expect("getV3WithWechatPaySerial 成功");
    assert!(resp.contains("ok"));

    // download_v3（默认实现，返回字节流）
    let bytes = service
        .download_v3(&format!("{base}/v3/binary"))
        .await
        .expect("downloadV3 成功");
    assert_eq!(bytes, vec![1, 2, 3]);

    // download_bill_with_url（downloadV3 的便捷委托）
    let bytes = service
        .download_bill_with_url(&format!("{base}/v3/binary"))
        .await
        .expect("downloadBill(url) 成功");
    assert_eq!(bytes, vec![1, 2, 3]);
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v2 查询订单 out_trade_no 分支 + v3 查询变体（含 partner）
// ═══════════════════════════════════════════════════════════════════

/// query_order(out_trade_no) 分支 + query_order_v3 两种 URL + 非法 JSON 报错。
/// 对应 Java: queryOrder/queryOrderV3(String transactionId, String outTradeNo)
#[tokio::test]
async fn v2_and_v3_query_order_variants() {
    let server = MockServer::start(|path, _| {
        if path.starts_with("/pay/orderquery") {
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", MCH_ID),
                    ("nonce_str", "n1"),
                    ("out_trade_no", "out_query_001"),
                    ("trade_state", "SUCCESS"),
                ],
                None,
            )
        } else {
            signed_json_response(r#"{"out_trade_no":"out_query_001","trade_state":"SUCCESS"}"#)
        }
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // v2：out_trade_no 分支（此前仅覆盖 transaction_id 分支）
    let result = service
        .query_order(None, Some("out_query_001"))
        .await
        .expect("v2 查询成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("out_query_001"));

    // v3：out-trade-no URL / id URL
    let r = service
        .query_order_v3(None, Some("out_query_001"))
        .await
        .expect("v3 out-trade-no 查询成功");
    assert_eq!(r.out_trade_no.as_deref(), Some("out_query_001"));
    let r = service
        .query_order_v3(Some("4200001234"), None)
        .await
        .expect("v3 id 查询成功");
    assert_eq!(r.out_trade_no.as_deref(), Some("out_query_001"));
}

/// query_order_v3 非法 JSON → "解析响应失败"。
#[tokio::test]
async fn v3_query_order_invalid_json() {
    let server = MockServer::start(|_, _| signed_json_response("not-json")).await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));
    let err = service
        .query_order_v3(None, Some("out_1"))
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("解析响应失败"), "错误信息: {err}");
}

/// 服务商模式查询订单 v3：sp_mchid/sub_mchid 配置回填 + 两种 URL。
/// 对应 Java: queryPartnerOrderV3
#[tokio::test]
async fn v3_query_partner_order_variants() {
    let server = MockServer::start(|_, _| {
        signed_json_response(r#"{"out_trade_no":"p1","trade_state":"SUCCESS"}"#)
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let r = service
        .query_partner_order_v3(None, Some("partner_out_001"))
        .await
        .expect("服务商 out-trade-no 查询成功");
    assert_eq!(r.out_trade_no.as_deref(), Some("p1"));

    let r = service
        .query_partner_order_v3(Some("4200005678"), None)
        .await
        .expect("服务商 id 查询成功");
    assert_eq!(r.out_trade_no.as_deref(), Some("p1"));

    // with_request 变体：请求字段回填分支
    let mut request = wx_rust_pay::bean::WxPayPartnerOrderQueryV3Request::default();
    request.out_trade_no = Some("partner_out_002".to_string());
    let r = service
        .query_partner_order_v3_with_request(&request)
        .await
        .expect("with_request 成功");
    assert_eq!(r.out_trade_no.as_deref(), Some("p1"));
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v3 关单变体（partner/with_request/combine）
// ═══════════════════════════════════════════════════════════════════

/// closePartnerOrderV3 族 + closeOrderV3(request) + closeCombine。
/// 对应 Java: closePartnerOrderV3/closeCombine
#[tokio::test]
async fn v3_close_partner_and_combine_variants() {
    let server = MockServer::start(|path, _| {
        assert!(path.contains("/close"), "关单请求应命中 close 路径: {path}");
        no_content_response()
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // close_partner_order_v3(String)
    service
        .close_partner_order_v3("partner_out_001")
        .await
        .expect("服务商关单成功");

    // close_partner_order_v3(request)：sp_mchid/sub_mchid 回填
    let mut request = WxPayPartnerOrderCloseV3Request::default();
    request.out_trade_no = Some("partner_out_002".to_string());
    service
        .close_partner_order_v3_with_request(&request)
        .await
        .expect("服务商关单(request) 成功");

    // close_order_v3(request)：mchid 回填
    let mut close_request = wx_rust_pay::bean::WxPayOrderCloseV3Request::default();
    close_request.out_trade_no = Some("out_close_001".to_string());
    service
        .close_order_v3_with_request(&close_request)
        .await
        .expect("关单(request) 成功");

    // close_combine
    let mut combine_request = CombineCloseRequest::default();
    combine_request.combine_out_trade_no = Some("combine_close_001".to_string());
    service
        .close_combine(&combine_request)
        .await
        .expect("合单关单成功");
}

/// close_partner_order_v3 空 out_trade_no → 报错。
#[tokio::test]
async fn v3_close_partner_order_empty_no() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));
    let err = service
        .close_partner_order_v3("")
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("out_trade_no不能为空"),
        "错误信息: {err}"
    );
}

/// queryCombine：GET 合单查询。
/// 对应 Java: queryCombine(String combineOutTradeNo)
#[tokio::test]
async fn v3_query_combine() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.starts_with("/v3/combine-transactions/out-trade-no/"),
            "{path}"
        );
        signed_json_response(
            r#"{"combine_out_trade_no":"c1","combine_payer_info":{"openid":"o1"},"sub_orders":[]}"#,
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let result = service.query_combine("c1").await.expect("合单查询成功");
    assert_eq!(result.combine_out_trade_no.as_deref(), Some("c1"));
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v3 下单（unifiedOrderV3 / unifiedPartnerOrderV3 /
// createPartnerOrderV3 / combine / combineTransactions 各分支）
// ═══════════════════════════════════════════════════════════════════

/// unifiedOrderV3(JSAPI) + unifiedPartnerOrderV3(NATIVE) + createPartnerOrderV3(JSAPI)。
/// 对应 Java: unifiedOrderV3/unifiedPartnerOrderV3/createPartnerOrderV3
#[tokio::test]
async fn v3_unified_and_partner_order() {
    let server = MockServer::start(|path, _| {
        if path.contains("/partner/") {
            signed_json_response(
                r#"{"prepay_id":"partner_prepay_1","code_url":"weixin://wxpay/bizpayurl?pr=p1"}"#,
            )
        } else {
            signed_json_response(r#"{"prepay_id":"jsapi_prepay_1"}"#)
        }
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // unified_order_v3（JSAPI，appid/mchid/notify_url 从配置回填）
    let mut request = wx_rust_pay::bean::WxPayUnifiedOrderV3Request::default();
    request.description = Some("测试".to_string());
    request.out_trade_no = Some("v3_jsapi_001".to_string());
    request.amount = Some(OrderV3Amount {
        total: Some(100),
        currency: Some("CNY".to_string()),
    });
    let result = service
        .unified_order_v3(TradeTypeEnum::Jsapi, &request)
        .await
        .expect("v3 统一下单成功");
    assert_eq!(result.prepay_id.as_deref(), Some("jsapi_prepay_1"));

    // unified_partner_order_v3（NATIVE，sp_appid/sp_mchid/notify_url 回填）
    let mut partner_request = WxPayPartnerUnifiedOrderV3Request::default();
    partner_request.description = Some("服务商测试".to_string());
    partner_request.out_trade_no = Some("v3_partner_001".to_string());
    partner_request.amount = Some(PartnerAmount {
        total: Some(100),
        currency: Some("CNY".to_string()),
    });
    let result = service
        .unified_partner_order_v3(TradeTypeEnum::Native, &partner_request)
        .await
        .expect("服务商 v3 下单成功");
    assert_eq!(
        result.code_url.as_deref(),
        Some("weixin://wxpay/bizpayurl?pr=p1")
    );

    // create_partner_order_v3（JSAPI 二次签名）
    let pay_info = service
        .create_partner_order_v3(TradeTypeEnum::Jsapi, &partner_request)
        .await
        .expect("服务商 create 成功");
    let obj = pay_info.as_object().expect("应返回对象");
    assert_eq!(
        obj.get("package").and_then(|v| v.as_str()),
        Some("prepay_id=partner_prepay_1")
    );
    assert!(obj.get("paySign").and_then(|v| v.as_str()).is_some());
}

/// combine(基座方法) + combineTransactions JSAPI/APP/NATIVE 分支。
/// 对应 Java: combine/combineTransactions
#[tokio::test]
async fn v3_combine_base_and_transactions_branches() {
    let server = MockServer::start(|path, _| {
        let body = if path.ends_with("/native") {
            r#"{"prepay_id":"combine_prepay_1","code_url":"weixin://wxpay/bizpayurl?pr=cc"}"#
        } else {
            r#"{"prepay_id":"combine_prepay_1"}"#
        };
        signed_json_response(body)
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let mut request = wx_rust_pay::bean::CombineTransactionsRequest::default();
    request.combine_out_trade_no = Some("combine_001".to_string());
    request.sub_orders = vec![];

    // combine（基座，返回 CombineTransactionsResult）
    let result = service
        .combine(TradeTypeEnum::Jsapi, &request)
        .await
        .expect("合单支付成功");
    assert_eq!(result.prepay_id.as_deref(), Some("combine_prepay_1"));

    // combine_transactions：JSAPI（RSA 二次签名）
    let pay_info = service
        .combine_transactions(TradeTypeEnum::Jsapi, &request)
        .await
        .expect("合单 JSAPI 成功");
    assert!(pay_info.get("paySign").is_some(), "{pay_info}");

    // combine_transactions：APP（不二次签名，AppResult）
    let pay_info = service
        .combine_transactions(TradeTypeEnum::App, &request)
        .await
        .expect("合单 APP 成功");
    assert_eq!(
        pay_info.get("prepayid").and_then(|v| v.as_str()),
        Some("combine_prepay_1")
    );

    // combine_transactions：NATIVE（code_url 字符串）
    let pay_info = service
        .combine_transactions(TradeTypeEnum::Native, &request)
        .await
        .expect("合单 NATIVE 成功");
    assert_eq!(pay_info.as_str(), Some("weixin://wxpay/bizpayurl?pr=cc"));
}

/// 境外支付 v3：四种 GlobalTradeTypeEnum 全覆盖（死代理保离线，
/// 仅验证回填/URL 组装在发送前完成并返回 Http 错误）。
/// 对应 Java: createOrderV3Global/unifiedOrderV3Global（apihk 固定基地址）
#[tokio::test]
async fn v3_global_order_all_trade_types_offline() {
    let service = BarePayService {
        config: config_with_host("http://127.0.0.1:1"),
        client: dead_proxy_client(),
    };

    for trade_type in [
        GlobalTradeTypeEnum::App,
        GlobalTradeTypeEnum::Jsapi,
        GlobalTradeTypeEnum::Native,
        GlobalTradeTypeEnum::H5,
    ] {
        let mut request = WxPayUnifiedOrderV3GlobalRequest::default();
        request.description = Some("境外测试".to_string());
        request.out_trade_no = Some(format!("global_{trade_type:?}"));
        // 死代理：发送即失败（Http 错误），前置回填分支均已执行
        let err = service
            .unified_order_v3_global(trade_type, &request)
            .await
            .expect_err("死代理应失败");
        assert!(
            err.to_string().to_lowercase().contains("http") || err.to_string().contains("error"),
            "错误信息: {err}"
        );
    }

    // create_order_v3_global：回填 + global→domestic 映射后发送失败
    let mut request = WxPayUnifiedOrderV3GlobalRequest::default();
    request.description = Some("境外 create".to_string());
    request.out_trade_no = Some("global_create_1".to_string());
    let err = service
        .create_order_v3_global(GlobalTradeTypeEnum::App, &request)
        .await
        .expect_err("死代理应失败");
    let _ = err;
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v2 退款族（p12 证书通道）
// ═══════════════════════════════════════════════════════════════════

/// refund/refundV2 全流程（/secapi/pay/refund 与 refundv2，需证书）。
/// 对应 Java: refund(WxPayRefundRequest)
#[tokio::test]
async fn v2_refund_and_refund_v2_full_flow() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/secapi/pay/refund"), "{path}");
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", P12_MCH_ID),
                ("nonce_str", "n2"),
                ("out_refund_no", "R001"),
                ("out_trade_no", "T001"),
                ("refund_id", "5000001"),
                ("refund_fee", "100"),
            ],
            None,
        )
    })
    .await;
    let service = bare_service(config_with_p12(&format!("http://{}", server.addr)));

    let mut request = WxPayRefundRequest::default();
    request.out_trade_no = Some("T001".to_string());
    request.out_refund_no = Some("R001".to_string());
    request.total_fee = Some(100);
    request.refund_fee = Some(100);

    let result = service.refund(&request).await.expect("退款成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));
    let xml = server.last_body();
    assert!(xml.contains("<out_refund_no>R001</out_refund_no>"), "{xml}");
    assert!(xml.contains("<sign>"), "{xml}");

    // refund_v2：/secapi/pay/refundv2
    let result = service.refund_v2(&request).await.expect("退款 v2 成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));
}

/// 退款约束检查：refund_account 非法 + 双单号同时为空。
/// 对应 Java: WxPayRefundRequest#checkConstraints
#[tokio::test]
async fn v2_refund_constraint_checks() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    let mut request = WxPayRefundRequest::default();
    request.out_trade_no = Some("T001".to_string());
    request.refund_account = Some("INVALID_ACCOUNT".to_string());
    let err = service.refund(&request).await.expect_err("应报错");
    assert!(
        err.to_string().contains("refund_account"),
        "错误信息: {err}"
    );

    let empty = WxPayRefundRequest::default();
    let err = service.refund(&empty).await.expect_err("应报错");
    assert!(err.to_string().contains("不能同时为空"), "错误信息: {err}");
}

/// refundQuery 四参数填充 + 四选一约束 + refundQueryV2。
/// 对应 Java: refundQuery/refundQueryV2
#[tokio::test]
async fn v2_refund_query_family() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/refundquery"), "退款查询路径: {path}");
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("nonce_str", "n3"),
                ("out_refund_no", "R001"),
                ("refund_id", "5000001"),
                ("refund_count", "1"),
            ],
            None,
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // refund_query(String...) 参数填充分支
    let result = service
        .refund_query(None, Some("T001"), None, None)
        .await
        .expect("退款查询成功");
    assert_eq!(result.refund_count, Some(1));

    // refund_query_v2（/pay/refundqueryv2）
    let mut request = WxPayRefundQueryRequest::default();
    request.out_trade_no = Some("T001".to_string());
    let result = service
        .refund_query_v2(&request)
        .await
        .expect("退款查询 v2 成功");
    assert_eq!(result.refund_count, Some(1));

    // 四选一约束：全空 → 报错
    let err = service
        .refund_query(None, None, None, None)
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("四选一"), "错误信息: {err}");

    // 四选一约束：全填 → 报错
    let mut all = WxPayRefundQueryRequest::default();
    all.transaction_id = Some("t".to_string());
    all.out_trade_no = Some("o".to_string());
    all.out_refund_no = Some("r".to_string());
    all.refund_id = Some("i".to_string());
    let err = service
        .refund_query_with_request(&all)
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("四选一"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v3 退款族
// ═══════════════════════════════════════════════════════════════════

/// refundV3/partnerRefundV3/refundQueryV3 族。
/// 对应 Java: refundV3/partnerRefundV3/refundQueryV3/refundPartnerQueryV3
#[tokio::test]
async fn v3_refund_family() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/refund/domestic/refunds"), "{path}");
        signed_json_response(
            r#"{"out_refund_no":"R001","out_trade_no":"T001","refund_id":"5000001","status":"SUCCESS"}"#,
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // refund_v3（notify_url 从配置回填）
    let mut request = WxPayRefundV3Request::default();
    request.out_trade_no = Some("T001".to_string());
    request.out_refund_no = Some("R001".to_string());
    request.amount = Some(wx_rust_pay::bean::request::wx_pay_refund_v3_request::Amount::default());
    let result = service.refund_v3(&request).await.expect("v3 退款成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));

    // partner_refund_v3（sp_appid/notify_url 回填）
    let mut partner = WxPayPartnerRefundV3Request::default();
    partner.out_trade_no = Some("T001".to_string());
    partner.out_refund_no = Some("R001".to_string());
    let result = service
        .partner_refund_v3(&partner)
        .await
        .expect("服务商 v3 退款成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));

    // refund_query_v3(String)
    let result = service
        .refund_query_v3("R001")
        .await
        .expect("v3 退款查询成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));

    // refund_query_v3(request)
    let mut q = WxPayRefundQueryV3Request::default();
    q.out_refund_no = Some("R001".to_string());
    let result = service
        .refund_query_v3_with_request(&q)
        .await
        .expect("v3 退款查询(request) 成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));

    // refund_partner_query_v3（sub_mchid 查询参数）
    let mut pq = WxPayRefundQueryV3Request::default();
    pq.out_refund_no = Some("R001".to_string());
    pq.sub_mchid = Some("1900000109".to_string());
    let result = service
        .refund_partner_query_v3(&pq)
        .await
        .expect("服务商退款查询成功");
    assert_eq!(result.out_refund_no.as_deref(), Some("R001"));
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v2 通知解析（订单/扫码/退款 req_info 解密）
// ═══════════════════════════════════════════════════════════════════

/// parseOrderNotifyResult：正常 XML + V3 JSON 检测 + 指定签名类型。
/// 对应 Java: parseOrderNotifyResult
#[tokio::test]
async fn v2_parse_order_notify_result_variants() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    // 正常通知（MD5 签名，checkResult 成功）
    let xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "n4"),
            ("out_trade_no", "T001"),
            ("transaction_id", "4200001"),
        ],
        None,
    );
    let result = service
        .parse_order_notify_result(&xml)
        .await
        .expect("通知解析成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("T001"));

    // 指定签名类型变体
    service
        .parse_order_notify_result_with_sign_type(&xml, Some("MD5"))
        .await
        .expect("指定签名类型解析成功");

    // V3 JSON 检测 → 建议使用 v3 方法
    let err = service
        .parse_order_notify_result("{}")
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("parseOrderNotifyV3Result"),
        "错误信息: {err}"
    );
}

/// parseScanPayNotifyResult：XML 解析 + 验签。
/// 对应 Java: parseScanPayNotifyResult
#[tokio::test]
async fn v2_parse_scan_pay_notify_result() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));
    let xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "n5"),
            ("openid", "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
        ],
        None,
    );
    let result = service
        .parse_scan_pay_notify_result(&xml)
        .await
        .expect("扫码通知解析成功");
    assert_eq!(
        result.openid.as_deref(),
        Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")
    );
    service
        .parse_scan_pay_notify_result_with_sign_type(&xml, "MD5")
        .await
        .expect("指定签名类型扫码通知解析成功");
}

/// parseRefundNotifyResult：FAIL 直返 + SUCCESS 解密 req_info + 无 req_info。
/// 对应 Java: parseRefundNotifyResult（decryptReqInfo AES-256-ECB）
#[tokio::test]
async fn v2_parse_refund_notify_result_decrypt() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    // return_code=FAIL：直接返回，不解密
    let fail_xml = v2_signed_response(&[("return_code", "FAIL"), ("return_msg", "失败")], None);
    let result = service
        .parse_refund_notify_result(&fail_xml)
        .await
        .expect("FAIL 通知直返");
    assert_eq!(result.return_code.as_deref(), Some("FAIL"));

    // SUCCESS + req_info：AES-256-ECB（md5(mch_key) 为密钥）解密
    let req_info_xml = "<xml><out_refund_no><![CDATA[R001]]></out_refund_no>\
<out_trade_no><![CDATA[T001]]></out_trade_no>\
<refund_status><![CDATA[SUCCESS]]></refund_status>\
<settlement_total_fee>100</settlement_total_fee></xml>";
    let encrypted = aes_256_ecb_encrypt_pkcs7(MCH_KEY.as_bytes(), req_info_xml.as_bytes());
    let req_info_b64 = {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(encrypted)
    };
    let ok_xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "n6"),
            ("req_info", &req_info_b64),
        ],
        None,
    );
    let result = service
        .parse_refund_notify_result(&ok_xml)
        .await
        .expect("退款通知解密成功");
    let req_info = result.req_info.expect("req_info 应解密");
    assert_eq!(req_info.out_refund_no.as_deref(), Some("R001"));
    assert_eq!(req_info.out_trade_no.as_deref(), Some("T001"));

    // SUCCESS 无 req_info：原样返回
    let plain_xml = v2_signed_response(
        &[
            ("return_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
        ],
        None,
    );
    let result = service
        .parse_refund_notify_result(&plain_xml)
        .await
        .expect("无 req_info 通知解析成功");
    assert!(result.req_info.is_none());
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：v3 通知解析全家桶（verifyNotifySign + 7 个 parse 方法）
// ═══════════════════════════════════════════════════════════════════

/// 构造 AES-GCM 加密 + 平台签名的 v3 通知（与既有退款通知测试同一手法）。
fn encrypted_notify_v3(event_type: &str, plaintext: &str) -> (String, SignatureHeader) {
    let nonce = "notifynonce1";
    let ciphertext =
        aes_gcm_encrypt(API_V3_KEY, "event", nonce.as_bytes(), plaintext).expect("AES 加密");
    let notify_json = serde_json::json!({
        "id": "EV-001",
        "create_time": "2024-01-01T00:00:00+08:00",
        "event_type": event_type,
        "resource_type": "encrypt-resource",
        "resource": {
            "original_type": "event",
            "algorithm": "AEAD_AES_256_GCM",
            "ciphertext": ciphertext,
            "associated_data": "event",
            "nonce": nonce
        }
    });
    let notify_data = notify_json.to_string();
    let timestamp = "1700000000";
    let header_nonce = "h0uYIzEaIUX9";
    let sign_message = format!("{timestamp}\n{header_nonce}\n{notify_data}\n");
    let signature =
        sign_sha256_rsa(&platform_private_key(), sign_message.as_bytes()).expect("平台签名");
    let header = SignatureHeader::new(
        Some(timestamp.to_string()),
        Some(header_nonce.to_string()),
        Some(signature),
        Some(PLATFORM_SERIAL.to_string()),
    );
    (notify_data, header)
}

fn platform_private_key() -> rsa::RsaPrivateKey {
    load_private_key_from_pem(PLATFORM_PRIVATE_KEY_PEM.as_bytes()).expect("平台私钥")
}

/// verifyNotifySign：平台公钥验签 true/false。
/// 对应 Java: verifyNotifySign(SignatureHeader, String)
#[tokio::test]
async fn v3_verify_notify_sign() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));
    let (data, header) = encrypted_notify_v3("TRANSACTION.SUCCESS", r#"{"out_trade_no":"T1"}"#);
    assert!(
        service
            .verify_notify_sign(&header, &data)
            .await
            .expect("验签执行")
    );
    let bad_header = SignatureHeader::new(
        Some("1700000000".to_string()),
        Some("h0uYIzEaIUX9".to_string()),
        Some("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()),
        Some(PLATFORM_SERIAL.to_string()),
    );
    assert!(
        !service
            .verify_notify_sign(&bad_header, &data)
            .await
            .expect("验签执行")
    );
}

/// v3 通知解析全家桶：partner order / base / combine / transfer batches /
/// transfer bills / partner refund / partner subscribe / complaint。
/// 对应 Java: parseXxxNotifyV3Result 系列
#[tokio::test]
async fn v3_notify_parsers_full_family() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    let (data, header) = encrypted_notify_v3(
        "TRANSACTION.SUCCESS",
        r#"{"out_trade_no":"T1","mchid":"10000100"}"#,
    );
    let result = service
        .parse_partner_order_notify_v3_result(&data, &header)
        .await
        .expect("服务商支付通知解析成功");
    assert!(result.result.is_some());

    let result = service
        .base_parse_order_notify_v3_result(&data, &header)
        .await
        .expect("泛型通知解析成功");
    assert!(result.get("rawData").is_some(), "{result}");

    let (data, header) = encrypted_notify_v3(
        "COMBINE.TRANSACTION.SUCCESS",
        r#"{"combine_out_trade_no":"C1"}"#,
    );
    let result = service
        .parse_combine_notify_result(&data, &header)
        .await
        .expect("合单通知解析成功");
    assert!(result.result.is_some());

    let (data, header) = encrypted_notify_v3(
        "REFUND.SUCCESS",
        r#"{"out_refund_no":"R1","refund_status":"SUCCESS"}"#,
    );
    let result = service
        .parse_partner_refund_notify_v3_result(&data, &header)
        .await
        .expect("服务商退款通知解析成功");
    assert!(result.result.is_some());

    let (data, header) = encrypted_notify_v3("TRANSFERBATCH.FINISHED", r#"{"batch_id":"B1"}"#);
    let result = service
        .parse_transfer_batches_notify_v3_result(&data, &header)
        .await
        .expect("转账批次通知解析成功");
    assert!(result.result.is_some());

    let (data, header) = encrypted_notify_v3("TRANSFER.BILL.FINISHED", r#"{"bill_id":"b1"}"#);
    let result = service
        .parse_transfer_bills_notify_v3_result(&data, &header)
        .await
        .expect("商家转账通知解析成功");
    assert!(result.result.is_some());

    let (data, header) = encrypted_notify_v3("SUBSCRIBE", r#"{"contract_id":"c1"}"#);
    let result = service
        .parse_partner_subscribe_notify(&data, &header)
        .await
        .expect("订阅通知解析成功");
    assert!(result.result.is_some());

    let (data, header) = encrypted_notify_v3("COMPLAINT.CREATE", r#"{"complaint_id":"cp1"}"#);
    let result = service
        .parse_complaint_notify_result(&data, &header)
        .await
        .expect("投诉通知解析成功");
    assert!(result.result.is_some());
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：对账单/资金账单（文本/GZIP/错误 XML/约束/v3 申请）
// ═══════════════════════════════════════════════════════════════════

const BILL_TEXT: &str = "交易时间,公众账号ID,商户号,子商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,总金额,代金券金额,订单金额,费率,费率备注,商品名称,商户数据包,手续费,退款金额,优惠券退款金额,退款类型,退款状态`2024-01-01T00:00:00,wxd930ea5d5a258f4f,10000100,,,,4200001,T001,o1,MICROPAY,SUCCESS,CFT,CNY,100,0,100,0.6%,,测试,0,0,0,,,`总交易单数`1`总交易额`100`手续费`0`";

/// downloadRawBill/downloadBill：文本 + 结构化解析。
/// 对应 Java: downloadRawBill/downloadBill
#[tokio::test]
async fn v2_download_bill_text_and_structured() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/downloadbill"), "{path}");
        (
            200,
            "text/plain".to_string(),
            BILL_TEXT.as_bytes().to_vec(),
            vec![],
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // 文本对账单（tar_type 空）
    let raw = service
        .download_raw_bill("20240101", "ALL", "", None)
        .await
        .expect("文本账单下载成功");
    assert!(raw.contains("总交易单数"), "{raw}");

    // 结构化解析（SUCCESS 类型）
    let bill = service
        .download_bill("20240101", "SUCCESS", "", None)
        .await
        .expect("结构化账单成功");
    assert_eq!(bill.total_record.as_deref(), Some("1"));
}

/// downloadRawBill（GZIP）：post_for_bytes + gunzip 全流程。
/// 对应 Java: downloadRawBill → handleGzipBill
#[tokio::test]
async fn v2_download_bill_gzip() {
    let gzip = gzip_bytes(BILL_TEXT);
    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"), "{path}");
        (200, "application/x-gzip".to_string(), gzip.clone(), vec![])
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let raw = service
        .download_raw_bill("20240102", "ALL", "GZIP", Some("dev_001"))
        .await
        .expect("GZIP 账单下载成功");
    assert!(raw.contains("总交易单数"), "{raw}");
}

/// downloadRawBill 错误 XML → common_result_error 组合文案。
#[tokio::test]
async fn v2_download_bill_error_xml() {
    let error_xml = v2_signed_response(
        &[("return_code", "FAIL"), ("return_msg", "账单日期无效")],
        None,
    );
    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"), "{path}");
        (
            200,
            "text/xml".to_string(),
            error_xml.clone().into_bytes(),
            vec![],
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let err = service
        .download_raw_bill("20240103", "ALL", "", None)
        .await
        .expect_err("应报错");
    assert!(
        err.to_string().contains("账单日期无效") && err.to_string().contains("原始报文"),
        "错误信息: {err}"
    );
}

/// 对账单约束检查：非法 bill_type / tar_type。
/// 对应 Java: WxPayDownloadBillRequest#checkConstraints
#[tokio::test]
async fn v2_download_bill_constraint_checks() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    let err = service
        .download_bill("20240101", "INVALID", "", None)
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("bill_type"), "错误信息: {err}");

    let err = service
        .download_bill("20240101", "ALL", "ZIP", None)
        .await
        .expect_err("应报错");
    assert!(err.to_string().contains("tar_type"), "错误信息: {err}");
}

/// downloadFundFlow（三参 + request 变体）：文本路径全流程。
/// 对应 Java: downloadFundFlow（HMAC-SHA256 + 证书通道）
#[tokio::test]
async fn v2_download_fund_flow_full_and_request() {
    let fund_text = "记账时间,业务类型,业务单号,资金流向,收入金额,支出金额,账户结余,资金账户账号,费用备注,业务凭证号,备注`2024-01-01T00:00:00,收入,5001,D,100,0,100,20000100,,memo,v1`资金流水总笔数`1`收入笔数`1`收入金额`100`支出笔数`0`支出金额`0";
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/downloadfundflow"), "{path}");
        (
            200,
            "text/plain".to_string(),
            fund_text.as_bytes().to_vec(),
            vec![],
        )
    })
    .await;
    let service = bare_service(config_with_p12(&format!("http://{}", server.addr)));

    let result = service
        .download_fund_flow("20240101", "Basic", "")
        .await
        .expect("资金账单下载成功");
    assert_eq!(result.total_record.as_deref(), Some("1"));

    // with_request 变体（bean 字段映射）
    let mut request = WxPayDownloadFundFlowRequest::default();
    request.bill_date = Some("20240101".to_string());
    request.account_type = Some("Basic".to_string());
    request.tar_type = Some("".to_string());
    let result = service
        .download_fund_flow_with_request(&request)
        .await
        .expect("资金账单(request) 成功");
    assert_eq!(result.total_record.as_deref(), Some("1"));
}

/// applyTradeBill/applyFundFlowBill：tar_type 空/非空两分支。
/// 对应 Java: applyTradeBill/applyFundFlowBill
#[tokio::test]
async fn v3_apply_bills_url_variants() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/v3/bill/"), "{path}");
        signed_json_response(
            r#"{"hash_type":"SHA256","hash_value":"abc","download_url":"https://download.example.com/bill.gz"}"#,
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // 交易账单：无 tar_type / 有 tar_type
    let mut request = wx_rust_pay::bean::WxPayApplyTradeBillV3Request::default();
    request.bill_date = Some("2024-01-01".to_string());
    request.bill_type = Some("ALL".to_string());
    let result = service
        .apply_trade_bill(&request)
        .await
        .expect("交易账单申请成功");
    assert!(
        result
            .download_url
            .as_deref()
            .unwrap_or_default()
            .contains("download.example.com")
    );
    request.tar_type = Some("GZIP".to_string());
    service
        .apply_trade_bill(&request)
        .await
        .expect("交易账单申请(GZIP) 成功");

    // 资金账单：无 tar_type / 有 tar_type
    let mut fund = wx_rust_pay::bean::WxPayApplyFundFlowBillV3Request::default();
    fund.bill_date = Some("2024-01-01".to_string());
    fund.account_type = Some("Basic".to_string());
    let result = service
        .apply_fund_flow_bill(&fund)
        .await
        .expect("资金账单申请成功");
    assert!(result.download_url.is_some());
    fund.tar_type = Some("GZIP".to_string());
    service
        .apply_fund_flow_bill(&fund)
        .await
        .expect("资金账单申请(GZIP) 成功");
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：撤销/人脸/汇率/评价/代金券
// ═══════════════════════════════════════════════════════════════════

/// reverseOrder（证书通道）+ reverseOrderV3(request)。
/// 对应 Java: reverseOrder/reverseOrderV3
#[tokio::test]
async fn v2_reverse_order_and_v3_request() {
    let server = MockServer::start(|path, _| {
        if path.starts_with("/secapi/pay/reverse") {
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", P12_MCH_ID),
                    ("nonce_str", "n7"),
                ],
                None,
            )
        } else {
            assert!(path.contains("/reverse"), "{path}");
            signed_json_response(r#"{"out_trade_no":"rev_001"}"#)
        }
    })
    .await;
    let base = format!("http://{}", server.addr);
    let service = bare_service(config_with_p12(&base));
    let v3_service = bare_service(config_with_host(&base));

    let mut request = wx_rust_pay::bean::WxPayOrderReverseRequest::default();
    request.out_trade_no = Some("rev_001".to_string());
    service.reverse_order(&request).await.expect("v2 撤销成功");

    // reverse_order_v3_with_request（appid/mchid 回填 + 反序列化）
    let mut v3 = wx_rust_pay::bean::WxPayOrderReverseV3Request::default();
    v3.out_trade_no = Some("rev_001".to_string());
    let result = v3_service
        .reverse_order_v3_with_request(&v3)
        .await
        .expect("v3 撤销(request) 成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("rev_001"));
}

/// facepay + queryExchangeRate（v2 XML 通道）。
/// 对应 Java: facepay/queryExchangeRate
#[tokio::test]
async fn v2_facepay_and_exchange_rate() {
    let server = MockServer::start(|path, _| {
        if path.starts_with("/pay/facepay") {
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", MCH_ID),
                    ("nonce_str", "n8"),
                    ("openid", "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o"),
                ],
                None,
            )
        } else {
            assert!(path.starts_with("/pay/queryexchagerate"), "{path}");
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", MCH_ID),
                    ("nonce_str", "n9"),
                    ("rate", "6.5000"),
                ],
                None,
            )
        }
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let mut facepay = WxPayFacepayRequest::default();
    facepay.body = Some("刷脸".to_string());
    facepay.out_trade_no = Some("face_001".to_string());
    facepay.total_fee = Some(100);
    facepay.spbill_create_ip = Some("127.0.0.1".to_string());
    facepay.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());
    facepay.face_code = Some("face_code_1".to_string());
    let result = service.facepay(&facepay).await.expect("人脸支付成功");
    assert_eq!(
        result.openid.as_deref(),
        Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")
    );

    let mut rate = WxPayQueryExchangeRateRequest::default();
    rate.fee_type = Some("USD".to_string());
    rate.date = Some("20240101".to_string());
    let _result = service
        .query_exchange_rate("USD", "20240101")
        .await
        .expect("汇率查询成功");
    let _ = rate;
}

/// 代金券三件套：sendCoupon（证书）/ queryCouponStock / queryCouponInfo。
/// 对应 Java: sendCoupon/queryCouponStock/queryCouponInfo
#[tokio::test]
async fn v2_coupon_family() {
    let server = MockServer::start(|path, _| {
        if path.starts_with("/mmpaymkttransfers/send_coupon") {
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", P12_MCH_ID),
                    ("nonce_str", "n10"),
                    ("coupon_stock_id", "175084655"),
                    ("errmsg", "ok"),
                ],
                None,
            )
        } else {
            v2_xml_response(
                &[
                    ("return_code", "SUCCESS"),
                    ("result_code", "SUCCESS"),
                    ("appid", APP_ID),
                    ("mch_id", P12_MCH_ID),
                    ("nonce_str", "n11"),
                ],
                None,
            )
        }
    })
    .await;
    let service = bare_service(config_with_p12(&format!("http://{}", server.addr)));

    let mut send = WxPayCouponSendRequest::default();
    send.coupon_stock_id = Some("175084655".to_string());
    send.partner_trade_no = Some("10000098".to_string());
    send.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());
    send.openid_count = Some(1);
    let _ = service.send_coupon(&send).await.expect("发券成功");

    let mut stock = WxPayCouponStockQueryRequest::default();
    stock.coupon_stock_id = Some("175084655".to_string());
    let _ = service
        .query_coupon_stock(&stock)
        .await
        .expect("批次查询成功");

    let mut info = WxPayCouponInfoQueryRequest::default();
    info.coupon_id = Some("coupon_1".to_string());
    info.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());
    info.stock_id = Some("175084655".to_string());
    let _ = service
        .query_coupon_info(&info)
        .await
        .expect("代金券信息查询成功");
}

/// queryComment（Date 重载 + request 变体，HMAC-SHA256 + 证书）。
/// 对应 Java: queryComment(Date, Date, Integer, Integer)
#[tokio::test]
async fn v2_query_comment_date_overload() {
    let server = MockServer::start(|path, _| {
        assert!(
            path.starts_with("/billcommentsp/batchquerycomment"),
            "{path}"
        );
        (
            200,
            "text/plain".to_string(),
            b"comment-data".to_vec(),
            vec![],
        )
    })
    .await;
    let service = bare_service(config_with_p12(&format!("http://{}", server.addr)));

    let begin = chrono::Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = chrono::Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();
    let result = service
        .query_comment(begin, end, Some(0), Some(10))
        .await
        .expect("评价查询成功");
    assert_eq!(result, "comment-data");

    let mut request = WxPayQueryCommentRequest::default();
    request.begin_time = Some("20240101000000".to_string());
    request.end_time = Some("20240102000000".to_string());
    request.offset = Some(0);
    request.limit = Some(10);
    let result = service
        .query_comment_with_request(&request)
        .await
        .expect("评价查询(request) 成功");
    assert_eq!(result, "comment-data");
}

/// codepay：直连模式与服务商模式（sp 字段判定 + 四字段回填）。
/// 对应 Java: codepay(WxPayCodepayRequest)
#[tokio::test]
async fn v3_codepay_direct_and_partner() {
    let server = MockServer::start(|path, _| {
        if path.contains("/partner/transactions/codepay") {
            signed_json_response(r#"{"out_trade_no":"codepay_p"}"#)
        } else {
            assert!(path.starts_with("/v3/pay/transactions/codepay"), "{path}");
            signed_json_response(r#"{"out_trade_no":"codepay_d"}"#)
        }
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    // 直连模式（无 sp/sub 字段 → appid/mchid 回填）
    let mut direct = WxPayCodepayRequest::default();
    direct.description = Some("直连".to_string());
    direct.out_trade_no = Some("codepay_d".to_string());
    let result = service.codepay(&direct).await.expect("直连 codepay 成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("codepay_d"));

    // 服务商模式（sp_mchid 触发 → 四字段回填）
    let mut partner = WxPayCodepayRequest::default();
    partner.description = Some("服务商".to_string());
    partner.out_trade_no = Some("codepay_p".to_string());
    partner.sp_mchid = Some("1600000109".to_string());
    let result = service
        .codepay(&partner)
        .await
        .expect("服务商 codepay 成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("codepay_p"));
}

// ═══════════════════════════════════════════════════════════════════
// RUST_OBLIGATION：扫码二维码 + 未实现占位
// ═══════════════════════════════════════════════════════════════════

/// createScanPayQrcodeMode1 URL 生成 + 两个未实现占位（-99）。
/// 对应 Java: createScanPayQrcodeMode1/createScanPayQrcodeMode1(logo)/Mode2
#[tokio::test]
async fn scan_pay_qrcode_mode1_and_unimplemented_stubs() {
    let service = bare_service(config_with_host("http://127.0.0.1:1"));

    let url = service
        .create_scan_pay_qrcode_mode1("product_001")
        .await
        .expect("模式一 URL 生成成功");
    assert!(url.starts_with("weixin://wxpay/bizpayurl?"), "{url}");
    assert!(url.contains("appid=wxd930ea5d5a258f4f"), "{url}");
    assert!(url.contains("mch_id=10000100"), "{url}");
    assert!(url.contains("product_id=product_001"), "{url}");
    assert!(url.contains("sign="), "{url}");

    let err = service
        .create_scan_pay_qrcode_mode1_with_logo("p1", None, None)
        .await
        .expect_err("含 logo 版未实现");
    assert_eq!(err.error_code(), Some(-99));

    let err = service
        .create_scan_pay_qrcode_mode2("weixin://wxpay/bizpayurl", None, None)
        .await
        .expect_err("模式二未实现");
    assert_eq!(err.error_code(), Some(-99));
}

// ═══════════════════════════════════════════════════════════════════
// VALUE_ADD：固定域名方法离线策略（死代理：发送即失败，不出网）
// ═══════════════════════════════════════════════════════════════════

/// get_sandbox_sign_key / get_wx_pay_face_auth_info：固定域名
/// （api.mch.weixin.qq.com / payapp.weixin.qq.com），以死代理保离线，
/// 验证前置流程（签名/MD5 回填）执行后在 HTTP 层失败。
/// 对应 Java: getSandboxSignKey/getWxPayFaceAuthInfo
#[tokio::test]
async fn fixed_host_methods_offline_via_dead_proxy() {
    let service = BarePayService {
        config: config_with_host("http://127.0.0.1:1"),
        client: dead_proxy_client(),
    };

    let err = service
        .get_sandbox_sign_key()
        .await
        .expect_err("死代理应失败");
    assert!(
        matches!(err, wx_rust_common::error::WxErrorException::Http(_)),
        "应为 Http 错误: {err}"
    );

    let mut face = wx_rust_pay::bean::WxPayFaceAuthInfoRequest::default();
    face.store_name = Some("门店".to_string());
    face.device_id = Some("device_1".to_string());
    face.rawdata = Some("raw".to_string());
    face.mch_id = Some(MCH_ID.to_string());
    let err = service
        .get_wx_pay_face_auth_info(&face)
        .await
        .expect_err("死代理应失败");
    assert!(
        matches!(err, wx_rust_common::error::WxErrorException::Http(_)),
        "应为 Http 错误: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：createOrder 子商户号切换分支 + 不支持交易类型 +
// createOrder(Specific)（与既有用例互补）
// ═══════════════════════════════════════════════════════════════════

/// createOrder：响应含 sub_appid/sub_mch_id 时 APP/JSAPI 切换子商户参数；
/// 未知 trade_type 报错；createOrder(Specific) 覆盖 trade_type。
/// 对应 Java: createOrder(WxPayUnifiedOrderRequest)
#[tokio::test]
async fn v2_create_order_sub_merchant_and_unsupported_type() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"), "{path}");
        v2_xml_response(
            &[
                ("return_code", "SUCCESS"),
                ("result_code", "SUCCESS"),
                ("appid", APP_ID),
                ("mch_id", MCH_ID),
                ("sub_appid", "wx_sub_app_01"),
                ("sub_mch_id", "1900000109"),
                ("nonce_str", "n12"),
                ("prepay_id", "wx_sub_prepay_001"),
                ("trade_type", "APP"),
                ("code_url", "weixin://wxpay/bizpayurl?pr=sub001"),
            ],
            None,
        )
    })
    .await;
    let service = bare_service(config_with_host(&format!("http://{}", server.addr)));

    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.out_trade_no = Some("sub_app_order_001".to_string());

    // APP：sub_appid/sub_mch_id 非空 → 切换为开放平台子商户参数
    request.trade_type = Some("APP".to_string());
    let pay_info = service
        .create_order(&request)
        .await
        .expect("APP 子商户下单成功");
    assert_eq!(
        pay_info.get("appId").and_then(|v| v.as_str()),
        Some("wx_sub_app_01")
    );
    assert_eq!(
        pay_info.get("partnerId").and_then(|v| v.as_str()),
        Some("1900000109")
    );

    // JSAPI：sub_appid 切换 + 默认 MD5 签名类型
    request.trade_type = Some("JSAPI".to_string());
    request.openid = Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string());
    let pay_info = service
        .create_order(&request)
        .await
        .expect("JSAPI 子商户下单成功");
    assert_eq!(
        pay_info.get("appId").and_then(|v| v.as_str()),
        Some("wx_sub_app_01")
    );
    assert_eq!(
        pay_info.get("signType").and_then(|v| v.as_str()),
        Some("MD5")
    );

    // create_order_with_specific：Specific 覆盖 request.trade_type
    let mut native_req = request.clone();
    native_req.trade_type = Some("JSAPI".to_string());
    native_req.product_id = Some("product_001".to_string());
    let pay_info = service
        .create_order_with_specific(WxPaySpecificTradeType::Native, &native_req)
        .await
        .expect("Specific NATIVE 下单成功");
    assert!(
        pay_info.get("codeUrl").and_then(|v| v.as_str()).is_some() || pay_info.as_str().is_some(),
        "{pay_info}"
    );

    // 不支持的 trade_type → 报错
    let mut bad = request.clone();
    bad.trade_type = Some("FUTURE_TYPE".to_string());
    let err = service.create_order(&bad).await.expect_err("应报错");
    assert!(
        err.to_string().contains("该交易类型暂不支持"),
        "错误信息: {err}"
    );
}
