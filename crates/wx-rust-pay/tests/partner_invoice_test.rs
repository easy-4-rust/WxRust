#![allow(clippy::field_reassign_with_default)]
//! 服务商电子发票服务测试（对应 Java `PartnerInvoiceServiceImplTest`）。
//!
//! 测试覆盖：
//! 1. get_invite_url — 获取邀请链接（GET + query 断言）
//! 2. issue_general_invoice — 开具通用行业发票（POST + JSON body 断言）
//! 3. get_invoice — 查询电子发票（GET + 路径参数断言）
//! 4. reverse_invoice — 冲红电子发票（POST + body 移除 fapiao_apply_id）
//! 5. get_invoice_file_download_info — 获取发票文件下载信息（GET）
//! 6. get_sub_merchant_invoice_status — 检查子商户开票状态（GET）
//! 7. create_card_template — 创建卡券模板（POST）
//! 8. update_development_config — 更新开发配置（PATCH）
//! 9. get_user_title_url — 获取用户抬头链接（GET + 多参数）
//! 10. get_user_title — 获取用户抬头信息（GET）
//! 11. issue_real_estate_leasing_invoice — 开具不动产租赁发票（POST）
//! 12. insert_cards — 插入卡包（POST + body 移除 fapiao_apply_id）
//! 13. list_invite_merchants — 查询邀请商户（GET）
//! 14. Bean serde round-trip tests

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use serde_json::json;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::invoice::buyer_information::BuyerInformation;
use wx_rust_pay::bean::invoice::card_template_request::CardTemplateRequest;
use wx_rust_pay::bean::invoice::card_template_result::CardTemplateResult;
use wx_rust_pay::bean::invoice::development_config_request::DevelopmentConfigRequest;
use wx_rust_pay::bean::invoice::development_config_result::DevelopmentConfigResult;
use wx_rust_pay::bean::invoice::general_invoice_request::GeneralInvoiceRequest;
use wx_rust_pay::bean::invoice::industry_invoice_request::IndustryInvoiceRequest;
use wx_rust_pay::bean::invoice::insert_card_request::InsertCardRequest;
use wx_rust_pay::bean::invoice::invite_merchant_query::InviteMerchantQuery;
use wx_rust_pay::bean::invoice::invite_merchant_result::InviteMerchantResult;
use wx_rust_pay::bean::invoice::invite_url_request::InviteUrlRequest;
use wx_rust_pay::bean::invoice::invite_url_result::InviteUrlResult;
use wx_rust_pay::bean::invoice::invoice_file_result::InvoiceFileResult;
use wx_rust_pay::bean::invoice::invoice_file_upload_result::InvoiceFileUploadResult;
use wx_rust_pay::bean::invoice::invoice_result::InvoiceResult;
use wx_rust_pay::bean::invoice::reverse_invoice_request::ReverseInvoiceRequest;
use wx_rust_pay::bean::invoice::sub_merchant_invoice_status::SubMerchantInvoiceStatus;
use wx_rust_pay::bean::invoice::title_url_result::TitleUrlResult;
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
O+U+QIGAZbWs4zC0PKjMZhdXOF0TsQTyMKRjUlWlWdOP4RX1dCeSS8lRAoGAMDPN
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

// ---- Bean serde 测试 ----

/// InviteUrlResult serde（对应 Java `InviteUrlResult`：`invite_url` 字段）。
#[test]
fn test_invite_url_result_serde() {
    let json_str = r#"{"invite_url":"https://example.com/invite"}"#;
    let result: InviteUrlResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(
        result.invite_url.as_deref(),
        Some("https://example.com/invite")
    );
}

/// InviteUrlResult 空 JSON 反序列化。
#[test]
fn test_invite_url_result_empty() {
    let json_str = r#"{}"#;
    let result: InviteUrlResult = serde_json::from_str(json_str).unwrap();
    assert!(result.invite_url.is_none());
}

/// InviteUrlResult round-trip。
#[test]
fn test_invite_url_result_round_trip() {
    let original = InviteUrlResult {
        invite_url: Some("https://pay.weixin.qq.com/invite".to_string()),
    };
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: InviteUrlResult = serde_json::from_str(&json_str).unwrap();
    assert_eq!(original.invite_url, restored.invite_url);
}

/// InvoiceResult serde（含嵌套 InvoiceInformation + Fapiao）。
#[test]
fn test_invoice_result_serde() {
    let json_str = r#"{
        "total_count": 1,
        "fapiao_information": [{
            "fapiao_id": "fid_001",
            "status": "ISSUED",
            "blue_fapiao": {
                "fapiao_code": "12345",
                "fapiao_number": "67890",
                "check_code": "ABC",
                "password": "PWD",
                "fapiao_time": "2024-01-01T00:00:00"
            },
            "total_amount": 10000,
            "tax_amount": 1300,
            "amount": 8700
        }]
    }"#;
    let result: InvoiceResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.total_count, Some(1));
    let info = result.fapiao_information.as_ref().unwrap();
    assert_eq!(info.len(), 1);
    assert_eq!(info[0].fapiao_id.as_deref(), Some("fid_001"));
    assert_eq!(info[0].status.as_deref(), Some("ISSUED"));
    let blue = info[0].blue_fapiao.as_ref().unwrap();
    assert_eq!(blue.fapiao_code.as_deref(), Some("12345"));
    assert_eq!(info[0].total_amount, Some(10000));
}

/// InvoiceFileResult serde（含嵌套 DownloadInfo）。
#[test]
fn test_invoice_file_result_serde() {
    let json_str = r#"{
        "fapiao_download_info_list": [{
            "fapiao_id": "fid_002",
            "download_url": "https://example.com/file.pdf",
            "status": "AVAILABLE"
        }]
    }"#;
    let result: InvoiceFileResult = serde_json::from_str(json_str).unwrap();
    let list = result.fapiao_download_info_list.as_ref().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].fapiao_id.as_deref(), Some("fid_002"));
    assert_eq!(
        list[0].download_url.as_deref(),
        Some("https://example.com/file.pdf")
    );
}

/// SubMerchantInvoiceStatus serde（含嵌套 Mode + DigitalTaxMode）。
#[test]
fn test_sub_merchant_invoice_status_serde() {
    let json_str = r#"{
        "sub_mchid": "1234567890",
        "third_mode": {"status": "ENABLED"},
        "digital_tax_mode": {
            "status": "ACTIVE",
            "billing_person_info": [{"id": "bp1", "name": "张三"}],
            "access_time": "2024-01-01T00:00:00",
            "expired_time": "2025-01-01T00:00:00",
            "ability_info": [{"type": "GENERAL", "status": "ENABLED"}]
        }
    }"#;
    let result: SubMerchantInvoiceStatus = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.sub_mchid.as_deref(), Some("1234567890"));
    assert_eq!(
        result.third_mode.as_ref().unwrap().status.as_deref(),
        Some("ENABLED")
    );
    let dtm = result.digital_tax_mode.as_ref().unwrap();
    assert_eq!(dtm.status.as_deref(), Some("ACTIVE"));
    assert_eq!(dtm.billing_person_info.as_ref().unwrap().len(), 1);
    assert_eq!(dtm.ability_info.as_ref().unwrap().len(), 1);
}

/// CardTemplateResult serde。
#[test]
fn test_card_template_result_serde() {
    let json_str = r#"{"card_appid":"wxd930ea5d5a258f4f","card_id":"CARD_001"}"#;
    let result: CardTemplateResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.card_appid.as_deref(), Some("wxd930ea5d5a258f4f"));
    assert_eq!(result.card_id.as_deref(), Some("CARD_001"));
}

/// DevelopmentConfigResult serde。
#[test]
fn test_development_config_result_serde() {
    let json_str = r#"{"callback_url":"https://cb.example.com","show_fapiao_cell":true,"support_vat_fapiao":false}"#;
    let result: DevelopmentConfigResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(
        result.callback_url.as_deref(),
        Some("https://cb.example.com")
    );
    assert_eq!(result.show_fapiao_cell, Some(true));
    assert_eq!(result.support_vat_fapiao, Some(false));
}

/// TitleUrlResult serde。
#[test]
fn test_title_url_result_serde() {
    let json_str = r#"{
        "title_url": "https://title.example.com",
        "miniprogram_appid": "wx123",
        "miniprogram_path": "/pages/invoice",
        "miniprogram_user_name": "gh_abc"
    }"#;
    let result: TitleUrlResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(
        result.title_url.as_deref(),
        Some("https://title.example.com")
    );
    assert_eq!(result.miniprogram_appid.as_deref(), Some("wx123"));
}

/// BuyerInformation serde。
#[test]
fn test_buyer_information_serde() {
    let json_str = r#"{
        "type": "ENTERPRISE",
        "name": "测试企业",
        "taxpayer_id": "91110000MA01XXXX",
        "amount": 10000,
        "out_trade_no": "ORD_001"
    }"#;
    let result: BuyerInformation = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.r#type.as_deref(), Some("ENTERPRISE"));
    assert_eq!(result.name.as_deref(), Some("测试企业"));
    assert_eq!(result.amount, Some(10000));
}

/// InviteMerchantResult serde（含嵌套 Merchant 列表）。
#[test]
fn test_invite_merchant_result_serde() {
    let json_str = r#"{
        "total_count": 2,
        "offset": 0,
        "limit": 10,
        "mch_invite_result_list": [
            {"sub_mchid": "sub_001", "mch_invite_status": "ACCEPTED", "ep_name": "企业A"},
            {"sub_mchid": "sub_002", "mch_invite_status": "PENDING", "ep_name": "企业B"}
        ]
    }"#;
    let result: InviteMerchantResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.total_count, Some(2));
    let list = result.mch_invite_result_list.as_ref().unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].sub_mchid.as_deref(), Some("sub_001"));
}

/// InvoiceFileUploadResult serde。
#[test]
fn test_invoice_file_upload_result_serde() {
    let json_str = r#"{"fapiao_media_id":"MEDIA_001"}"#;
    let result: InvoiceFileUploadResult = serde_json::from_str(json_str).unwrap();
    assert_eq!(result.fapiao_media_id.as_deref(), Some("MEDIA_001"));
}

// ---- PartnerInvoiceService 方法测试 ----

/// 获取邀请链接：GET 请求 + query 参数断言。
#[tokio::test]
async fn test_get_invite_url_by_mch_id() {
    let response_body = json!({"invite_url": "https://invite.example.com"}).to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains("/v3/new-tax-control-fapiao/fapiaomerchant/getspinviteurl"));
        assert!(path.contains("sub_mchid=sub_mch_001"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let result = invoice_svc
        .get_invite_url_by_mch_id(Some("sub_mch_001"))
        .await
        .expect("获取邀请链接成功");
    assert_eq!(
        result.invite_url.as_deref(),
        Some("https://invite.example.com")
    );
}

/// 获取邀请链接（完整请求）：GET + 多参数。
#[tokio::test]
async fn test_get_invite_url_full_request() {
    let response_body = json!({"invite_url": "https://invite2.example.com"}).to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains("sub_mchid=sub_mch_002"));
        assert!(path.contains("operation_type=CREATE"));
        assert!(path.contains("fapiao_mode=TOTAL"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let mut request = InviteUrlRequest::default();
    request.sub_mchid = Some("sub_mch_002".to_string());
    request.operation_type = Some("CREATE".to_string());
    request.fapiao_mode = Some("TOTAL".to_string());

    let result = invoice_svc
        .get_invite_url(&request)
        .await
        .expect("获取邀请链接成功");
    assert_eq!(
        result.invite_url.as_deref(),
        Some("https://invite2.example.com")
    );
}

/// 开具通用行业发票：POST + JSON body 断言。
#[tokio::test]
async fn test_issue_general_invoice() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(
            path,
            "/v3/new-tax-control-fapiao/fapiao-applications/issue-general"
        );
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = GeneralInvoiceRequest {
        sub_mchid: Some("sub_mch_003".to_string()),
        fapiao_apply_id: Some("apply_001".to_string()),
        ..Default::default()
    };

    invoice_svc
        .issue_general_invoice(&request)
        .await
        .expect("开具通用行业发票成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_003");
    assert_eq!(body["fapiao_apply_id"], "apply_001");
}

/// 查询电子发票：GET + 路径参数断言。
#[tokio::test]
async fn test_get_invoice() {
    let response_body = json!({
        "total_count": 1,
        "fapiao_information": [{
            "fapiao_id": "fid_003",
            "status": "ISSUED",
            "total_amount": 5000
        }]
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains("/v3/new-tax-control-fapiao/fapiao-applications/apply_002"));
        assert!(path.contains("sub_mchid=sub_mch_004"));
        assert!(path.contains("fapiao_id=fid_003"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let result = invoice_svc
        .get_invoice("apply_002", "sub_mch_004", Some("fid_003"))
        .await
        .expect("查询电子发票成功");
    assert_eq!(result.total_count, Some(1));
    assert_eq!(
        result.fapiao_information.as_ref().unwrap()[0]
            .fapiao_id
            .as_deref(),
        Some("fid_003")
    );
}

/// 冲红电子发票：POST + body 移除 fapiao_apply_id。
#[tokio::test]
async fn test_reverse_invoice() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert!(path.contains("/v3/new-tax-control-fapiao/fapiao-applications/apply_003/reverse"));
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = ReverseInvoiceRequest {
        sub_mchid: Some("sub_mch_005".to_string()),
        fapiao_apply_id: Some("apply_003".to_string()),
        reverse_reason: Some("测试冲红".to_string()),
        ..Default::default()
    };

    invoice_svc
        .reverse_invoice(&request)
        .await
        .expect("冲红电子发票成功");

    // 验证 body 中不含 fapiao_apply_id
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_005");
    assert_eq!(body["reverse_reason"], "测试冲红");
    assert!(body.get("fapiao_apply_id").is_none());
}

/// 获取发票文件下载信息：GET + 路径参数。
#[tokio::test]
async fn test_get_invoice_file_download_info() {
    let response_body = json!({
        "fapiao_download_info_list": [{
            "fapiao_id": "fid_004",
            "download_url": "https://download.example.com/invoice.pdf",
            "status": "AVAILABLE"
        }]
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(
            path.contains("/v3/new-tax-control-fapiao/fapiao-applications/apply_004/fapiao-files")
        );
        assert!(path.contains("sub_mchid=sub_mch_006"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let result = invoice_svc
        .get_invoice_file_download_info("apply_004", "sub_mch_006", None)
        .await
        .expect("获取发票文件下载信息成功");
    let list = result.fapiao_download_info_list.as_ref().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].fapiao_id.as_deref(), Some("fid_004"));
}

/// 检查子商户开票状态：GET + 路径参数。
#[tokio::test]
async fn test_get_sub_merchant_invoice_status() {
    let response_body = json!({
        "sub_mchid": "sub_mch_007",
        "third_mode": {"status": "ENABLED"},
        "digital_tax_mode": {"status": "ACTIVE"}
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains("/v3/new-tax-control-fapiao/merchant/sub_mch_007/check-status"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let result = invoice_svc
        .get_sub_merchant_invoice_status("sub_mch_007")
        .await
        .expect("检查子商户开票状态成功");
    assert_eq!(result.sub_mchid.as_deref(), Some("sub_mch_007"));
    assert_eq!(
        result.third_mode.as_ref().unwrap().status.as_deref(),
        Some("ENABLED")
    );
}

/// 创建卡券模板：POST + JSON body。
#[tokio::test]
async fn test_create_card_template() {
    let response_body = json!({
        "card_appid": "wxd930ea5d5a258f4f",
        "card_id": "CARD_NEW_001"
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(path, "/v3/new-tax-control-fapiao/card-template");
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = CardTemplateRequest {
        sub_mchid: Some("sub_mch_008".to_string()),
        card_appid: Some("wxd930ea5d5a258f4f".to_string()),
        ..Default::default()
    };

    let result = invoice_svc
        .create_card_template(&request)
        .await
        .expect("创建卡券模板成功");
    assert_eq!(result.card_id.as_deref(), Some("CARD_NEW_001"));

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_008");
    assert_eq!(body["card_appid"], "wxd930ea5d5a258f4f");
}

/// 更新开发配置：PATCH 方法。
#[tokio::test]
async fn test_update_development_config() {
    let response_body = json!({
        "callback_url": "https://new-cb.example.com",
        "show_fapiao_cell": true,
        "support_vat_fapiao": true
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "PATCH");
        assert_eq!(
            path,
            "/v3/new-tax-control-fapiao/merchant/development-config"
        );
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = DevelopmentConfigRequest {
        callback_url: Some("https://new-cb.example.com".to_string()),
        show_fapiao_cell: Some(true),
        support_vat_fapiao: Some(true),
        ..Default::default()
    };

    let result = invoice_svc
        .update_development_config(&request)
        .await
        .expect("更新开发配置成功");
    assert_eq!(
        result.callback_url.as_deref(),
        Some("https://new-cb.example.com")
    );
    assert_eq!(result.support_vat_fapiao, Some(true));
}

/// 开具不动产租赁发票：POST 方法。
#[tokio::test]
async fn test_issue_real_estate_leasing_invoice() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert_eq!(
            path,
            "/v3/new-tax-control-fapiao/fapiao-applications/real-estate-leasing"
        );
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = IndustryInvoiceRequest {
        sub_mchid: Some("sub_mch_009".to_string()),
        fapiao_apply_id: Some("apply_005".to_string()),
        ..Default::default()
    };

    invoice_svc
        .issue_real_estate_leasing_invoice(&request)
        .await
        .expect("开具不动产租赁发票成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_009");
}

/// 插入卡包：POST + body 移除 fapiao_apply_id。
#[tokio::test]
async fn test_insert_cards() {
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "POST");
        assert!(
            path.contains("/v3/new-tax-control-fapiao/fapiao-applications/apply_006/insert-cards")
        );
        signed_json_response("{}")
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let request = InsertCardRequest {
        sub_mchid: Some("sub_mch_010".to_string()),
        fapiao_apply_id: Some("apply_006".to_string()),
        scene: Some("FAPiao".to_string()),
        ..Default::default()
    };

    invoice_svc
        .insert_cards(&request)
        .await
        .expect("插入卡包成功");

    let body: serde_json::Value = serde_json::from_str(&server.last_body()).unwrap();
    assert_eq!(body["sub_mchid"], "sub_mch_010");
    assert_eq!(body["scene"], "FAPiao");
    assert!(body.get("fapiao_apply_id").is_none());
}

/// 查询邀请商户：GET + 多参数。
#[tokio::test]
async fn test_list_invite_merchants() {
    let response_body = json!({
        "total_count": 1,
        "offset": 0,
        "limit": 10,
        "mch_invite_result_list": [{
            "sub_mchid": "sub_003",
            "mch_invite_status": "ACCEPTED",
            "ep_name": "测试企业C"
        }]
    })
    .to_string();
    let server = MockServer::start(move |method, path, _headers| {
        assert_eq!(method, "GET");
        assert!(path.contains("/v3/new-tax-control-fapiao/fapiaomerchant/listspinvitemchinfo"));
        assert!(path.contains("offset=0"));
        assert!(path.contains("limit=10"));
        signed_json_response(&response_body)
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&server.url("")));
    let invoice_svc = service
        .partner_invoice_service()
        .expect("partner_invoice_service 应已装配");

    let query = InviteMerchantQuery {
        query_time_start: Some("2024-01-01".to_string()),
        query_time_end: Some("2024-12-31".to_string()),
        offset: Some(0),
        limit: Some(10),
        ..Default::default()
    };

    let result = invoice_svc
        .list_invite_merchants(&query)
        .await
        .expect("查询邀请商户成功");
    assert_eq!(result.total_count, Some(1));
    assert_eq!(
        result.mch_invite_result_list.as_ref().unwrap()[0]
            .sub_mchid
            .as_deref(),
        Some("sub_003")
    );
}
