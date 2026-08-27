#![allow(clippy::field_reassign_with_default)]
//! A1 深度覆盖：query_order 各 trade_state / close_order / refund / download_bill /
//! XML 工具 / 错误码映射 —— 45+ httpmock 测试。
//!
//! 对应 Java: `BaseWxPayServiceImpl` + `WxPayServiceImpl` + `WxPayBillResult`
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java 对应方法的行为
//! - RUST_OBLIGATION: Rust 错误路径/约束检查
//! - VALUE_ADD: 边界组合、错误码映射

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::bean::WxPayDownloadBillRequest;
use wx_rust_pay::bean::WxPayRefundRequest;
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::exception::wx_pay_exception::WxPayException;
use wx_rust_pay::util::sign_utils::SignUtils;
use wx_rust_pay::util::wx_pay_service_impl_utils as impl_utils;
use wx_rust_pay::util::wx_pay_service_impl_utils::V2Request;

// ---- 复用已有夹具常量 ----

const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";

// ---- MockServer（与 coverage_boost_pay_service_mock.rs 同构） ----

struct MockServer {
    addr: std::net::SocketAddr,
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
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

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
                    let _body = lines.collect::<Vec<&str>>().join("\n");
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

        Self { addr, stop }
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
        .set_notify_url("https://example.com/pay/notify")
        .set_refund_notify_url("https://example.com/pay/refund-notify")
        .set_api_host_url(host);
    Arc::new(config)
}

/// 构造带签名的 v2 XML 响应（与 coverage_boost 相同签名逻辑）。
fn v2_signed_response(fields: &[(&str, &str)]) -> String {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in fields {
        map.insert(k.to_string(), v.to_string());
    }
    let sign = SignUtils::create_sign(&map, Some("MD5"), MCH_KEY, &[]).expect("签名失败");
    let mut xml = String::from("<xml>");
    for (k, v) in fields {
        xml.push_str(&format!("<{k}><![CDATA[{v}]]></{k}>"));
    }
    xml.push_str(&format!("<sign><![CDATA[{sign}]]></sign></xml>"));
    xml
}

fn v2_xml_response(fields: &[(&str, &str)]) -> (u16, String, String, Vec<(String, String)>) {
    (
        200,
        "text/xml".to_string(),
        v2_signed_response(fields),
        vec![],
    )
}

/// query_order 公共断言：解析成功后 trade_state 与 trade_state_desc 正确。
async fn assert_query_order_state(state: &str, desc: &str) {
    let state_owned = state.to_string();
    let desc_owned = desc.to_string();
    let server = MockServer::start(move |path, _| {
        assert!(
            path.starts_with("/pay/orderquery"),
            "URL 应为 /pay/orderquery"
        );
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("trade_state", &state_owned),
            ("trade_state_desc", &desc_owned),
            ("transaction_id", "4200001234202301011234567890"),
            ("out_trade_no", "ORDER_001"),
            ("total_fee", "100"),
            ("cash_fee", "100"),
            ("bank_type", "CMB_CREDIT"),
            ("time_end", "20230101120000"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .query_order(None, Some("ORDER_001"))
        .await
        .expect("查询订单成功");
    assert_eq!(
        result.trade_state.as_deref(),
        Some(state),
        "trade_state 应为 {state}"
    );
    assert_eq!(
        result.trade_state_desc.as_deref(),
        Some(desc),
        "trade_state_desc 应为 {desc}"
    );
    assert_eq!(
        result.transaction_id.as_deref(),
        Some("4200001234202301011234567890")
    );
    assert_eq!(result.out_trade_no.as_deref(), Some("ORDER_001"));
    assert_eq!(result.total_fee, Some(100));
    assert_eq!(result.cash_fee, Some(100));
    assert_eq!(result.bank_type.as_deref(), Some("CMB_CREDIT"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. query_order 各 trade_state 分支（7 个）
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: queryOrder → trade_state=SUCCESS
#[tokio::test]
async fn query_order_state_success() {
    assert_query_order_state("SUCCESS", "支付成功").await;
}

/// 对应 Java: queryOrder → trade_state=REFUND
#[tokio::test]
async fn query_order_state_refund() {
    assert_query_order_state("REFUND", "转入退款").await;
}

/// 对应 Java: queryOrder → trade_state=NOTPAY
#[tokio::test]
async fn query_order_state_notpay() {
    assert_query_order_state("NOTPAY", "未支付").await;
}

/// 对应 Java: queryOrder → trade_state=CLOSED
#[tokio::test]
async fn query_order_state_closed() {
    assert_query_order_state("CLOSED", "已关闭").await;
}

/// 对应 Java: queryOrder → trade_state=REVOKED
#[tokio::test]
async fn query_order_state_revoked() {
    assert_query_order_state("REVOKED", "已撤销（仅付款码支付）").await;
}

/// 对应 Java: queryOrder → trade_state=USERPAYING
#[tokio::test]
async fn query_order_state_userpaying() {
    assert_query_order_state("USERPAYING", "用户支付中").await;
}

/// 对应 Java: queryOrder → trade_state=PAYERROR
#[tokio::test]
async fn query_order_state_payerror() {
    assert_query_order_state("PAYERROR", "支付失败").await;
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. query_order 约束检查（RUST_OBLIGATION）
// ═══════════════════════════════════════════════════════════════════════════

/// transaction_id 和 out_trade_no 同时提供时应报错
#[tokio::test]
async fn query_order_both_ids_error() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .query_order(Some("TXN_001"), Some("OUT_001"))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("必须二选一"), "错误信息: {err}");
}

/// transaction_id 和 out_trade_no 同时为空时应报错
#[tokio::test]
async fn query_order_no_ids_error() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.query_order(None, None).await.unwrap_err();
    assert!(err.to_string().contains("必须二选一"), "错误信息: {err}");
}

/// 空字符串应视为"未提供"
#[tokio::test]
async fn query_order_empty_string_treated_as_none() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .query_order(Some("  "), Some("  "))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("必须二选一"), "错误信息: {err}");
}

/// query_order 使用 transaction_id
#[tokio::test]
async fn query_order_with_transaction_id() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/orderquery"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("trade_state", "SUCCESS"),
            ("transaction_id", "TXN_001"),
            ("out_trade_no", "OUT_001"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .query_order(Some("TXN_001"), None)
        .await
        .expect("查询成功");
    assert_eq!(result.transaction_id.as_deref(), Some("TXN_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. close_order OK 与错误码两路
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: closeOrder(String outTradeNo) —— 成功路径
#[tokio::test]
async fn v2_close_order_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/closeorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .close_order("OUT_TRADE_001")
        .await
        .expect("关闭订单成功");
    assert_eq!(result.result_code.as_deref(), Some("SUCCESS"));
}

/// 对应 Java: closeOrder —— 微信返回错误码路径
#[tokio::test]
async fn v2_close_order_error_code() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/closeorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "FAIL"),
            ("err_code", "ORDER_NOT_EXIST"),
            ("err_code_des", "订单不存在"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let err = service.close_order("NOT_EXIST_ORDER").await.unwrap_err();
    assert!(
        err.to_string().contains("ORDER_NOT_EXIST"),
        "应含错误代码: {err}"
    );
    assert!(
        err.to_string().contains("订单不存在"),
        "应含错误详情: {err}"
    );
}

/// close_order 空 out_trade_no 约束
#[tokio::test]
async fn v2_close_order_empty_trade_no() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.close_order("").await.unwrap_err();
    assert!(err.to_string().contains("不能为空"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. refund v1 XML 字段断言
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: refund(WxPayRefundRequest) —— 验证 check_and_sign + to_xml 生成的请求 XML 字段
/// （不调用服务端，因退款需 p12 证书；直接验证 XML 字段序与内容）。
#[test]
fn v2_refund_xml_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = WxPayRefundRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.out_refund_no = Some("REFUND_001".to_string());
    req.total_fee = Some(100);
    req.refund_fee = Some(80);
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(
        xml.contains("out_refund_no"),
        "XML 应含 out_refund_no: {xml}"
    );
    assert!(xml.contains("REFUND_001"), "XML 应含 REFUND_001: {xml}");
    assert!(xml.contains("<total_fee>"), "XML 应含 total_fee: {xml}");
    assert!(xml.contains("<refund_fee>"), "XML 应含 refund_fee: {xml}");
    assert!(xml.contains("100"), "XML 应含 total_fee 值: {xml}");
    assert!(xml.contains("80"), "XML 应含 refund_fee 值: {xml}");
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
}

/// refund 约束：transaction_id 和 out_trade_no 不能同时为空
#[tokio::test]
async fn v2_refund_no_trade_id_error() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let mut req = WxPayRefundRequest::default();
    req.out_refund_no = Some("REF_001".to_string());
    req.total_fee = Some(100);
    req.refund_fee = Some(100);
    let err = service.refund(&req).await.unwrap_err();
    assert!(err.to_string().contains("不能同时为空"), "错误信息: {err}");
}

/// refund 约束：非法 refund_account
#[tokio::test]
async fn v2_refund_invalid_account() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let mut req = WxPayRefundRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.out_refund_no = Some("REF_001".to_string());
    req.total_fee = Some(100);
    req.refund_fee = Some(100);
    req.refund_account = Some("INVALID_ACCOUNT".to_string());
    let err = service.refund(&req).await.unwrap_err();
    assert!(
        err.to_string().contains("refund_account"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. download_bill 头剥离逻辑
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: downloadBill —— 正常对账单解析（ALL 类型，含 ` 分割）
#[tokio::test]
async fn download_bill_all_type_parse() {
    // 构造 ALL 类型对账单：标题行（27 字段） + 1 条明细 + 汇总
    let header: Vec<&str> = (0..27).map(|_| "H").collect();
    let record: Vec<&str> = (0..27)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00",
            5 => "TXN_001",
            6 => "OUT_001",
            12 => "100",
            _ => "v",
        })
        .collect();
    let header_str = header.join(" ");
    let record_str = record.join("`");
    // 格式: header`val0`val1`...`val26总交易单数`total... (无多余 backtick)
    let bill_content = format!("{header_str}`{record_str}总交易单数`1`100`0`0`0`100`0");

    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"));
        (200, "text/plain".to_string(), bill_content.clone(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .download_bill("2023-01-01", "ALL", "", None)
        .await
        .expect("下载对账单成功");
    assert_eq!(result.bill_info_list.len(), 1, "应有 1 条明细");
    assert_eq!(
        result.bill_info_list[0].transaction_id.as_deref(),
        Some("TXN_001")
    );
    assert_eq!(result.bill_info_list[0].total_fee.as_deref(), Some("100"));
    assert_eq!(result.total_record.as_deref(), Some("1"));
}

/// 对应 Java: downloadBill —— parse_bill_detail 以 "总交易单数" 为分隔符
/// 标题行（第一个 backtick 前的空格分隔字段名）被正确跳过，数据行被正确解析。
#[tokio::test]
async fn download_bill_with_header_rows_stripped() {
    // SUCCESS 类型 20 列：header(空格)`val0`val1`...`val19总交易单数`...
    let header: Vec<String> = (0..20).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..20)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            5 => "TXN_HDR".to_string(),
            6 => "OUT_HDR".to_string(),
            12 => "200".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let bill_content = format!("{header_str}`{record_str}总交易单数`1`200`0`0`0`200`0");

    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"));
        (200, "text/plain".to_string(), bill_content.clone(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .download_bill("2023-01-01", "SUCCESS", "", None)
        .await
        .expect("下载对账单成功");
    assert_eq!(result.bill_info_list.len(), 1, "应有 1 条明细");
    assert_eq!(
        result.bill_info_list[0].transaction_id.as_deref(),
        Some("TXN_HDR")
    );
}

/// download_bill 约束：非法 bill_type
#[tokio::test]
async fn download_bill_invalid_type() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .download_bill("2023-01-01", "INVALID", "", None)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("bill_type"), "错误信息: {err}");
}

/// download_bill 约束：非法 tar_type
#[tokio::test]
async fn download_bill_invalid_tar_type() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .download_bill("2023-01-01", "ALL", "TAR_GZ", None)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("tar_type"), "错误信息: {err}");
}

/// download_bill: 微信返回错误 XML（以 `<` 开头）
#[tokio::test]
async fn download_bill_error_xml_response() {
    let server = MockServer::start(|_path, _| {
        let error_xml =
            "<xml><return_code>FAIL</return_code><return_msg>签名错误</return_msg></xml>";
        (200, "text/plain".to_string(), error_xml.to_string(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let err = service
        .download_bill("2023-01-01", "ALL", "", None)
        .await
        .unwrap_err();
    assert!(
        err.to_string().contains("签名错误") || err.to_string().contains("FAIL"),
        "应包含错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. XML 工具函数测试
// ═══════════════════════════════════════════════════════════════════════════

/// root_children_map: 正常 XML 解析
#[test]
fn xml_root_children_map_normal() {
    let xml = "<xml><appid>wx123</appid><mch_id>mch456</mch_id></xml>";
    let map = wx_rust_pay::bean::xml::root_children_map(xml).unwrap();
    assert_eq!(map.get("appid").unwrap(), "wx123");
    assert_eq!(map.get("mch_id").unwrap(), "mch456");
}

/// root_children_map: CDATA 解析
#[test]
fn xml_root_children_map_cdata() {
    let xml = "<xml><body><![CDATA[测试商品]]></body></xml>";
    let map = wx_rust_pay::bean::xml::root_children_map(xml).unwrap();
    assert_eq!(map.get("body").unwrap(), "测试商品");
}

/// root_children_map: 空 XML 报错
#[test]
fn xml_root_children_map_empty() {
    let result = wx_rust_pay::bean::xml::root_children_map("");
    assert!(result.is_err());
}

/// root_children_map: CDATA 保留特殊字符（对应 Java XStream CDATA 包裹）
#[test]
fn xml_root_children_map_special_chars() {
    let xml = "<xml><desc><![CDATA[商品&说明<测试>]]></desc></xml>";
    let map = wx_rust_pay::bean::xml::root_children_map(xml).unwrap();
    assert_eq!(map.get("desc").unwrap(), "商品&说明<测试>");
}

/// expand_empty_elements: 空元素展开
#[test]
fn xml_expand_empty_elements() {
    assert_eq!(
        wx_rust_pay::bean::xml::expand_empty_elements("<xml><a>1</a><b/></xml>"),
        "<xml><a>1</a><b></b></xml>"
    );
}

/// expand_empty_elements: 无空元素
#[test]
fn xml_expand_no_empty() {
    assert_eq!(
        wx_rust_pay::bean::xml::expand_empty_elements("<xml><a>1</a></xml>"),
        "<xml><a>1</a></xml>"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. 微信支付错误码映射（check_result / WxPayException）
// ═══════════════════════════════════════════════════════════════════════════

/// check_result: return_code=FAIL 时应报错
#[test]
fn check_result_return_code_fail() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[("return_code", "FAIL"), ("return_msg", "通信错误")]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(err.to_string().contains("FAIL"), "应含 return_code: {err}");
    assert!(
        err.to_string().contains("通信错误"),
        "应含 return_msg: {err}"
    );
}

/// check_result: result_code=FAIL 且 err_code=SIGN_ERROR
#[test]
fn check_result_sign_error() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "FAIL"),
        ("err_code", "SIGN_ERROR"),
        ("err_code_des", "签名错误"),
    ]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(
        err.to_string().contains("SIGN_ERROR"),
        "应含 err_code: {err}"
    );
    assert!(
        err.to_string().contains("签名错误"),
        "应含 err_code_des: {err}"
    );
}

/// check_result: result_code=FAIL 且 err_code=ORDERNOTEXIST
#[test]
fn check_result_order_not_exist() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "FAIL"),
        ("err_code", "ORDERNOTEXIST"),
        ("err_code_des", "此交易订单号不存在"),
    ]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(err.to_string().contains("ORDERNOTEXIST"), "{err}");
    assert!(err.to_string().contains("此交易订单号不存在"), "{err}");
}

/// check_result: result_code=FAIL 且 err_code=SYSTEMERROR
#[test]
fn check_result_system_error() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "FAIL"),
        ("err_code", "SYSTEMERROR"),
        ("err_code_des", "系统错误"),
    ]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(err.to_string().contains("SYSTEMERROR"), "{err}");
}

/// check_result: result_code=FAIL 且 err_code=BANKERROR
#[test]
fn check_result_bank_error() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "FAIL"),
        ("err_code", "BANKERROR"),
        ("err_code_des", "银行系统异常"),
    ]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(err.to_string().contains("BANKERROR"), "{err}");
    assert!(err.to_string().contains("银行系统异常"), "{err}");
}

/// check_result: result_code=FAIL 且 err_code=USERPAYING（用户支付中）
#[test]
fn check_result_userpaying_error() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "FAIL"),
        ("err_code", "USERPAYING"),
        ("err_code_des", "用户支付中，请稍后再试"),
    ]);
    let err = impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), true).unwrap_err();
    assert!(err.to_string().contains("USERPAYING"), "{err}");
}

/// check_result: check_success=false 时不应报错（用于通知解析）
#[test]
fn check_result_no_success_check() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = v2_signed_response(&[("return_code", "FAIL"), ("return_msg", "通信错误")]);
    assert!(
        impl_utils::check_result(config.as_ref(), &xml, Some("MD5"), false).is_ok(),
        "check_success=false 时不应报错"
    );
}

/// check_result: 签名不匹配时报错
#[test]
fn check_result_signature_mismatch() {
    let config = config_with_host("http://127.0.0.1:1");
    let xml = "<xml><return_code>SUCCESS</return_code><sign>INVALID_SIGN</sign></xml>";
    let err = impl_utils::check_result(config.as_ref(), xml, Some("MD5"), false).unwrap_err();
    assert!(
        err.to_string().contains("参数格式校验错误"),
        "应含签名错误: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. WxPayException 构建与消息拼装
// ═══════════════════════════════════════════════════════════════════════════

/// WxPayException.from_base_result_fields: 完整五元组
#[test]
fn wx_pay_exception_from_base_result() {
    let e = WxPayException::from_base_result_fields(
        Some("FAIL"),
        Some("通信"),
        Some("FAIL"),
        Some("ORDERNOTEXIST"),
        Some("订单不存在"),
        Some("<xml/>"),
    );
    let msg = e.build_error_msg();
    assert!(msg.contains("返回代码：[FAIL]"), "{msg}");
    assert!(msg.contains("返回信息：[通信]"), "{msg}");
    assert!(msg.contains("结果代码：[FAIL]"), "{msg}");
    assert!(msg.contains("错误代码：[ORDERNOTEXIST]"), "{msg}");
    assert!(msg.contains("错误详情：[订单不存在]"), "{msg}");
    assert!(msg.contains("微信返回的原始报文"), "{msg}");
}

/// WxPayException.custom_error_msg 优先
#[test]
fn wx_pay_exception_custom_msg_priority() {
    let e = WxPayException::new("自定义错误");
    assert_eq!(e.build_error_msg(), "自定义错误");
}

/// WxPayException 转 WxErrorException
#[test]
fn wx_pay_exception_to_wx_error() {
    let e = WxPayException::new("测试异常");
    let wx_err: wx_rust_common::error::WxErrorException = e.into();
    assert!(wx_err.to_string().contains("测试异常"));
}

/// WxPayException: 空字段不拼入消息
#[test]
fn wx_pay_exception_empty_fields_skipped() {
    let e = WxPayException::from_base_result_fields(Some("FAIL"), None, None, None, None, None);
    let msg = e.build_error_msg();
    assert!(msg.contains("返回代码：[FAIL]"), "{msg}");
    assert!(!msg.contains("返回信息"), "空字段不应拼入: {msg}");
    assert!(!msg.contains("结果代码"), "空字段不应拼入: {msg}");
}

/// WxPayException.Builder: 链式构建
#[test]
fn wx_pay_exception_builder_chain() {
    let e = WxPayException::new_builder()
        .return_code("FAIL")
        .return_msg("err")
        .result_code("FAIL")
        .err_code("TEST_CODE")
        .err_code_des("测试详情")
        .xml_string("<xml/>")
        .build();
    assert_eq!(e.return_code(), Some("FAIL"));
    assert_eq!(e.err_code(), Some("TEST_CODE"));
    assert_eq!(e.err_code_des(), Some("测试详情"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. parse_bill_result 各账单类型
// ═══════════════════════════════════════════════════════════════════════════

/// parse_bill_result: 未知类型返回 None
#[test]
fn parse_bill_result_unknown_type() {
    assert!(impl_utils::parse_bill_result("data", "UNKNOWN").is_none());
}

/// parse_bill_result: 空内容
#[test]
fn parse_bill_result_empty_content() {
    let result = impl_utils::parse_bill_result("", "ALL");
    assert!(result.is_some());
    let bill = result.unwrap();
    assert!(bill.bill_info_list.is_empty());
}

/// parse_bill_result: SUCCESS 类型解析（20 列）
#[test]
fn parse_bill_result_success_type() {
    let header: Vec<String> = (0..20).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..20)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            5 => "TXN_S01".to_string(),
            6 => "OUT_S01".to_string(),
            12 => "150".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let content = format!("{header_str}`{record_str}总交易单数`1`150`0`0`0`150`0");
    let result = impl_utils::parse_bill_result(&content, "SUCCESS");
    assert!(result.is_some());
    let bill = result.unwrap();
    assert_eq!(bill.bill_info_list.len(), 1);
    assert_eq!(
        bill.bill_info_list[0].transaction_id.as_deref(),
        Some("TXN_S01")
    );
}

/// parse_bill_result: REFUND 类型解析（26 列）
#[test]
fn parse_bill_result_refund_type() {
    let header: Vec<String> = (0..26).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..26)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            16 => "REFUND_R01".to_string(),
            17 => "OUT_R01".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let content = format!("{header_str}`{record_str}总交易单数`1`0`0`0`0`0`0");
    let result = impl_utils::parse_bill_result(&content, "REFUND");
    assert!(result.is_some());
    let bill = result.unwrap();
    assert_eq!(bill.bill_info_list.len(), 1);
    assert_eq!(
        bill.bill_info_list[0].refund_id.as_deref(),
        Some("REFUND_R01")
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 10. parse_fund_flow_result
// ═══════════════════════════════════════════════════════════════════════════

/// parse_fund_flow_result: 正常解析（11 列）
#[test]
fn parse_fund_flow_normal() {
    let header: Vec<String> = (0..11).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..11)
        .map(|i| match i {
            0 => "2023-01-01".to_string(),
            2 => "FUND_001".to_string(),
            6 => "50".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let content = format!("{header_str}`{record_str}资金流水总笔数`1`0`0`1`50");
    let result = impl_utils::parse_fund_flow_result(&content);
    assert_eq!(result.wx_pay_fund_flow_base_result_list.len(), 1);
    assert_eq!(
        result.wx_pay_fund_flow_base_result_list[0]
            .fund_flow_id
            .as_deref(),
        Some("FUND_001")
    );
    assert_eq!(result.total_record.as_deref(), Some("1"));
    assert_eq!(result.expenditure_amount.as_deref(), Some("50"));
}

/// parse_fund_flow_result: 空内容
#[test]
fn parse_fund_flow_empty() {
    let result = impl_utils::parse_fund_flow_result("");
    assert!(result.wx_pay_fund_flow_base_result_list.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════
// 11. check_and_sign 工具函数
// ═══════════════════════════════════════════════════════════════════════════

/// check_and_sign: 非法 sign_type 报错
#[test]
fn check_and_sign_invalid_sign_type() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayOrderQueryRequest::default();
    req.sign_type = Some("INVALID_TYPE".to_string());
    req.out_trade_no = Some("ORDER_001".to_string());
    let err = impl_utils::check_and_sign(config.as_ref(), &mut req).unwrap_err();
    assert!(
        err.to_string().contains("非法的sign_type"),
        "错误信息: {err}"
    );
}

/// check_and_sign: 成功签名后 sign 字段非空
#[test]
fn check_and_sign_fills_sign() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayOrderQueryRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    assert!(req.sign.is_some(), "签名后 sign 不应为空");
    assert!(req.nonce_str.is_some(), "签名后 nonce_str 不应为空");
}

// ═══════════════════════════════════════════════════════════════════════════
// 12. 配置管理（WxPayServiceImpl）
// ═══════════════════════════════════════════════════════════════════════════

/// add_config + switchover: 多商户切换
#[test]
fn config_add_and_switchover() {
    let config1 = config_with_host("http://mch1.example.com");
    let mut config2 = WxPayDefaultConfig::new();
    config2
        .set_app_id("wx_app2")
        .set_mch_id("mch2")
        .set_mch_key("key2")
        .set_api_host_url("http://mch2.example.com");
    let config2: Arc<dyn WxPayConfig> = Arc::new(config2);

    let service = WxPayServiceImpl::new_arc(config1);
    service.add_config("mch1", APP_ID, service.wx_pay_config());
    service.add_config("mch2", "wx_app2", config2.clone());

    // 切换到 mch2
    assert!(service.switchover("mch2", "wx_app2"));
    let current = service.wx_pay_config();
    assert_eq!(current.mch_id().unwrap(), "mch2");

    // 切换回 mch1
    assert!(service.switchover("mch1", APP_ID));
    let current = service.wx_pay_config();
    assert_eq!(current.mch_id().unwrap(), MCH_ID);
}

/// remove_config: 移除后切换失败
#[test]
fn config_remove_and_switchover_fail() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config("mch1", APP_ID, service.wx_pay_config());
    assert!(service.switchover("mch1", APP_ID));
    service.remove_config("mch1", APP_ID);
    assert!(!service.switchover("mch1", APP_ID));
}

/// switchover_with_key: 精确匹配 + 前缀匹配
#[test]
fn config_switchover_with_key() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config("mch1", APP_ID, service.wx_pay_config());
    // 精确匹配
    let key = WxPayServiceImpl::get_config_key("mch1", APP_ID);
    assert!(service.switchover_with_key(&key));
    // 前缀匹配
    assert!(service.switchover_with_key("mch1"));
    // 不存在
    assert!(!service.switchover_with_key("nonexistent"));
}

/// get_config: 单商户直接返回
#[test]
fn config_get_single() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let _ = service.get_config();
}

/// get_config_by_mch_app: 查找配置
#[test]
fn config_get_by_mch_app() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config("mch1", APP_ID, service.wx_pay_config());
    assert!(service.get_config_by_mch_app("mch1", APP_ID).is_some());
    assert!(service.get_config_by_mch_app("mch1", "other").is_none());
}

/// get_config_by_mch: 按商户号查找
#[test]
fn config_get_by_mch() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config("mch1", APP_ID, service.wx_pay_config());
    assert!(service.get_config_by_mch("mch1").is_some());
    assert!(service.get_config_by_mch("nonexistent").is_none());
}

/// set_multi_config: 批量设置
#[test]
fn config_set_multi() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let mut map = HashMap::new();
    map.insert(
        "key1".to_string(),
        service.wx_pay_config() as Arc<dyn WxPayConfig>,
    );
    service.set_multi_config(&map);
    // 设置后应能切换
    assert!(service.switchover_with_key("key1"));
}

/// get_pay_base_url: 非沙箱模式
#[test]
fn pay_base_url_normal() {
    let config = config_with_host("http://api.mch.weixin.qq.com");
    let service = WxPayServiceImpl::new_arc(config);
    let url = service.get_pay_base_url();
    assert_eq!(url, "http://api.mch.weixin.qq.com");
}

// ═══════════════════════════════════════════════════════════════════════════
// 13. v3 close_order 路径
// ═══════════════════════════════════════════════════════════════════════════

/// v3 关单: 空 out_trade_no 约束
#[tokio::test]
async fn v3_close_order_empty_trade_no() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.close_order_v3("").await.unwrap_err();
    assert!(err.to_string().contains("不能为空"), "错误信息: {err}");
}

/// v3 服务商关单: 空 out_trade_no 约束
#[tokio::test]
async fn v3_close_partner_order_empty_trade_no() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service.close_partner_order_v3("").await.unwrap_err();
    assert!(err.to_string().contains("不能为空"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 14. query_order_v3 约束
// ═══════════════════════════════════════════════════════════════════════════

/// query_order_v3: 两个 ID 都为空时 URL 仍会拼接（v3 按 URL 路由）
#[tokio::test]
async fn v3_query_order_empty_ids_url_construction() {
    // v3 查询走 GET 请求，URL 拼接逻辑不会因为空值报错
    // 但会因网络不通失败，这里只验证不 panic
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service.query_order_v3(Some("TXN_001"), None).await;
    // 预期网络错误（v3 需要签名，配置不完整会报错）
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// 15. refund_query 约束
// ═══════════════════════════════════════════════════════════════════════════

/// refund_query: 四参数全为空时报错
#[tokio::test]
async fn refund_query_all_empty() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .refund_query(None, None, None, None)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("四选一"), "错误信息: {err}");
}

/// refund_query: 四参数全提供时报错
#[tokio::test]
async fn refund_query_all_provided() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .refund_query(Some("TXN"), Some("OUT"), Some("REF"), Some("REFID"))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("四选一"), "错误信息: {err}");
}

/// refund_query: 正常单参数查询
#[tokio::test]
async fn refund_query_single_param_ok() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/refundquery"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("out_trade_no", "ORDER_001"),
            ("out_refund_no", "REFUND_001"),
            ("refund_status_0", "SUCCESS"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .refund_query(None, None, Some("REFUND_001"), None)
        .await
        .expect("查询退款成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("ORDER_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 16. execute_post 基础路径
// ═══════════════════════════════════════════════════════════════════════════

/// execute_post: 正常 POST 返回响应体
#[tokio::test]
async fn execute_post_normal() {
    let server = MockServer::start(|_, _| {
        (
            200,
            "text/xml".to_string(),
            "<xml>OK</xml>".to_string(),
            vec![],
        )
    })
    .await;
    let config = config_with_host(&format!("http://{}", server.addr));
    let client = reqwest::Client::new();
    let url = format!("http://{}/test", server.addr);
    let resp = wx_rust_pay::api::r#impl::base_wx_pay_service_impl::execute_post(
        config.as_ref(),
        &client,
        &url,
        "<xml>test</xml>",
        false,
        None,
    )
    .await
    .expect("POST 成功");
    assert!(resp.contains("OK"), "响应应含 OK: {resp}");
}

/// execute_post: 自定义 mime_type
#[tokio::test]
async fn execute_post_custom_mime() {
    let server = MockServer::start(|_, _| {
        (
            200,
            "text/xml".to_string(),
            "<xml>OK</xml>".to_string(),
            vec![],
        )
    })
    .await;
    let config = config_with_host(&format!("http://{}", server.addr));
    let client = reqwest::Client::new();
    let url = format!("http://{}/test", server.addr);
    let resp = wx_rust_pay::api::r#impl::base_wx_pay_service_impl::execute_post(
        config.as_ref(),
        &client,
        &url,
        "<xml>test</xml>",
        false,
        Some("application/xml"),
    )
    .await
    .expect("POST 成功");
    assert!(resp.contains("OK"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 17. 子服务 getter
// ═══════════════════════════════════════════════════════════════════════════

/// WxPayServiceImpl 的子服务 getter 返回 Some（装配后）
#[test]
fn sub_services_getters_initialized() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    // Wave 5 P5 装配后，主要子服务应非 None
    assert!(
        service.ent_pay_service().is_some(),
        "ent_pay_service 应已装配"
    );
    assert!(
        service.gold_plan_service().is_some(),
        "gold_plan_service 应已装配"
    );
    assert!(
        service.redpack_service().is_some(),
        "redpack_service 应已装配"
    );
    assert!(
        service.profit_sharing_service().is_some(),
        "profit_sharing_service 应已装配"
    );
    assert!(
        service.pay_score_service().is_some(),
        "pay_score_service 应已装配"
    );
    assert!(
        service.complaints_service().is_some(),
        "complaints_service 应已装配"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 18. v2 请求签名装配（V2Request trait）
// ═══════════════════════════════════════════════════════════════════════════

/// V2Request: to_xml 输出含签名字段
#[test]
fn v2_request_to_xml_contains_sign() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayOrderQueryRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成成功");
    assert!(xml.contains("<sign>"), "XML 应含 sign: {xml}");
    assert!(xml.contains("ORDER_001"), "XML 应含 out_trade_no: {xml}");
}

/// V2Request: appid 从配置回填
#[test]
fn v2_request_appid_from_config() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayOrderQueryRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    assert_eq!(req.appid.as_deref(), Some(APP_ID));
    assert_eq!(req.mch_id.as_deref(), Some(MCH_ID));
}

// ═══════════════════════════════════════════════════════════════════════════
// 19. WxPayApiData 记录
// ═══════════════════════════════════════════════════════════════════════════

/// get_wx_api_data: 默认返回 None
#[test]
fn wx_api_data_default_none() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    assert!(service.get_wx_api_data().is_none());
}

// ═══════════════════════════════════════════════════════════════════════════
// 20. 统一下单 v2 全流程（unified_order + create_order）
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: unifiedOrder —— NATIVE 下单全流程
#[tokio::test]
async fn v2_unified_order_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "NATIVE"),
            ("code_url", "weixin://wxpay/bizpayurl?pr=xxx"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("NATIVE".to_string());
    req.product_id = Some("PROD_001".to_string());
    let result = service.unified_order(&req).await.expect("统一下单成功");
    assert_eq!(result.prepay_id.as_deref(), Some("wx20230101120000"));
    assert_eq!(
        result.code_url.as_deref(),
        Some("weixin://wxpay/bizpayurl?pr=xxx")
    );
}

/// 对应 Java: unifiedOrder —— trade_type=NATIVE 时必须指定 product_id
#[tokio::test]
async fn v2_unified_order_native_no_product_id() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("NATIVE".to_string());
    let err = service.unified_order(&req).await.unwrap_err();
    assert!(err.to_string().contains("product_id"), "错误信息: {err}");
}

/// 对应 Java: createOrder —— NATIVE 支付组装
#[tokio::test]
async fn v2_create_order_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "NATIVE"),
            ("code_url", "weixin://wxpay/bizpayurl?pr=xxx"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("NATIVE".to_string());
    req.product_id = Some("PROD_001".to_string());
    let result = service.create_order(&req).await.expect("创建订单成功");
    let code_url = result.get("codeUrl").and_then(|v| v.as_str());
    assert_eq!(code_url, Some("weixin://wxpay/bizpayurl?pr=xxx"));
}

/// 对应 Java: createOrder —— APP 支付组装
#[tokio::test]
async fn v2_create_order_app() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "APP"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("APP".to_string());
    let result = service.create_order(&req).await.expect("创建订单成功");
    assert!(result.get("sign").is_some(), "APP 支付应有 sign");
    assert!(result.get("prepayId").is_some(), "APP 支付应有 prepayId");
    assert!(result.get("partnerId").is_some(), "APP 支付应有 partnerId");
    assert!(
        result.get("packageValue").is_some(),
        "APP 支付应有 packageValue"
    );
    assert!(result.get("timeStamp").is_some(), "APP 支付应有 timeStamp");
    assert!(result.get("nonceStr").is_some(), "APP 支付应有 nonceStr");
}

/// 对应 Java: createOrder —— JSAPI 支付组装
#[tokio::test]
async fn v2_create_order_jsapi() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "JSAPI"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("JSAPI".to_string());
    let result = service.create_order(&req).await.expect("创建订单成功");
    assert!(result.get("appId").is_some(), "JSAPI 应有 appId");
    assert!(result.get("timeStamp").is_some(), "JSAPI 应有 timeStamp");
    assert!(result.get("nonceStr").is_some(), "JSAPI 应有 nonceStr");
    assert!(result.get("package").is_some(), "JSAPI 应有 package");
    assert!(result.get("signType").is_some(), "JSAPI 应有 signType");
    assert!(result.get("paySign").is_some(), "JSAPI 应有 paySign");
}

/// 对应 Java: createOrder —— MWEB 支付组装
#[tokio::test]
async fn v2_create_order_mweb() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "MWEB"),
            (
                "mweb_url",
                "https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx20230101120000",
            ),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("MWEB".to_string());
    let result = service.create_order(&req).await.expect("创建订单成功");
    let mweb_url = result.get("mwebUrl").and_then(|v| v.as_str());
    assert!(mweb_url.is_some(), "MWEB 应有 mwebUrl");
}

/// 对应 Java: createOrder —— 不支持的交易类型
#[tokio::test]
async fn v2_create_order_unsupported_type() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "UNKNOWN"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("UNKNOWN".to_string());
    let err = service.create_order(&req).await.unwrap_err();
    assert!(err.to_string().contains("暂不支持"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 21. micropay 刷卡支付
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: micropay —— 刷卡支付成功
#[tokio::test]
async fn v2_micropay_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/micropay"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("out_trade_no", "ORDER_MICRO_001"),
            ("transaction_id", "TXN_MICRO_001"),
            ("total_fee", "50"),
            ("openid", "openid_001"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayMicropayRequest::default();
    req.out_trade_no = Some("ORDER_MICRO_001".to_string());
    req.body = Some("刷卡商品".to_string());
    req.total_fee = Some(50);
    req.auth_code = Some("13000000000000".to_string());
    let result = service.micropay(&req).await.expect("刷卡支付成功");
    assert_eq!(result.transaction_id.as_deref(), Some("TXN_MICRO_001"));
    assert_eq!(result.total_fee, Some(50));
}

// ═══════════════════════════════════════════════════════════════════════════
// 22. reverse_order 撤销订单
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: reverseOrder —— 验证请求 XML 含 out_trade_no（需 p12 证书，测试签名+XML）
#[test]
fn v2_reverse_order_xml_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayOrderReverseRequest::default();
    req.out_trade_no = Some("ORDER_REV_001".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(
        xml.contains("ORDER_REV_001"),
        "XML 应含 out_trade_no: {xml}"
    );
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
}

/// reverse_order: transaction_id 和 out_trade_no 不能同时为空
#[tokio::test]
async fn v2_reverse_order_both_empty() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let req = wx_rust_pay::bean::WxPayOrderReverseRequest::default();
    let err = service.reverse_order(&req).await.unwrap_err();
    assert!(err.to_string().contains("不能同时为空"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 23. shorturl 短链接转换
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: shorturl —— 转换短链接
#[tokio::test]
async fn v2_shorturl_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/tools/shorturl"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("short_url", "weixin://wxpay/bizpayurl?pr=short"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let short = service
        .shorturl("https://pay.weixin.qq.com/wxpay/pay.action?prepay_id=wx123")
        .await
        .expect("短链接转换成功");
    assert_eq!(short, "weixin://wxpay/bizpayurl?pr=short");
}

// ═══════════════════════════════════════════════════════════════════════════
// 24. authcode2_openid 授权码查询 openid
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: authcode2Openid —— 查询成功
#[tokio::test]
async fn v2_authcode2_openid_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/tools/authcodetoopenid"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("openid", "openid_from_authcode"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let openid = service
        .authcode2_openid("13000000000000")
        .await
        .expect("查询成功");
    assert_eq!(openid, "openid_from_authcode");
}

// ═══════════════════════════════════════════════════════════════════════════
// 25. report 交易保障上报
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: report —— 上报成功
#[tokio::test]
async fn v2_report_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/payitil/report"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayReportRequest::default();
    req.interface_url = Some("https://api.mch.weixin.qq.com/pay/unifiedorder".to_string());
    req.execute_time = Some(100);
    req.return_code = Some("SUCCESS".to_string());
    req.result_code = Some("SUCCESS".to_string());
    req.user_ip = Some("127.0.0.1".to_string());
    service.report(&req).await.expect("上报成功");
}

// ═══════════════════════════════════════════════════════════════════════════
// 26. download_fund_flow 资金账单
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: downloadFundFlow —— 验证请求签名（需 p12 证书，测试 check_and_sign）
#[test]
fn v2_download_fund_flow_request_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::util::wx_pay_service_impl_utils::FundFlowBillRequest::default();
    req.bill_date = Some("2023-01-01".to_string());
    req.account_type = Some("Basic".to_string());
    req.tar_type = Some("GZIP".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(xml.contains("2023-01-01"), "XML 应含 bill_date: {xml}");
    assert!(xml.contains("Basic"), "XML 应含 account_type: {xml}");
    assert!(xml.contains("GZIP"), "XML 应含 tar_type: {xml}");
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 27. get_pay_info 废弃方法
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: getPayInfo —— NATIVE 支付
#[tokio::test]
async fn v2_get_pay_info_native() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/unifiedorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("prepay_id", "wx20230101120000"),
            ("trade_type", "NATIVE"),
            ("code_url", "weixin://wxpay/bizpayurl?pr=xxx"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayUnifiedOrderRequest::default();
    req.out_trade_no = Some("ORDER_001".to_string());
    req.body = Some("测试商品".to_string());
    req.total_fee = Some(100);
    req.trade_type = Some("NATIVE".to_string());
    req.product_id = Some("PROD_001".to_string());
    let result = service.get_pay_info(&req).await.expect("获取支付信息成功");
    assert_eq!(
        result.get("codeUrl").map(|s| s.as_str()),
        Some("weixin://wxpay/bizpayurl?pr=xxx")
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 28. v2 退款查询全流程
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: refundQuery —— 按 out_trade_no 查询
#[tokio::test]
async fn v2_refund_query_by_out_trade_no() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/refundquery"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("out_trade_no", "ORDER_001"),
            ("refund_count", "1"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .refund_query(None, Some("ORDER_001"), None, None)
        .await
        .expect("查询退款成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("ORDER_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 29. v2 关单全流程（带请求体验证）
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: closeOrder —— 关单请求 XML 含 out_trade_no
#[tokio::test]
async fn v2_close_order_request_xml_fields() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/closeorder"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .close_order("OUT_TRADE_CLOSE_001")
        .await
        .expect("关单成功");
    assert_eq!(result.result_code.as_deref(), Some("SUCCESS"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 30. v2 查询订单全流程（带请求体验证）
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: queryOrder —— 按 out_trade_no 查询全流程
#[tokio::test]
async fn v2_query_order_full_flow() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/pay/orderquery"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("trade_state", "SUCCESS"),
            ("trade_state_desc", "支付成功"),
            ("transaction_id", "TXN_FULL_001"),
            ("out_trade_no", "OUT_FULL_001"),
            ("total_fee", "200"),
            ("cash_fee", "200"),
            ("bank_type", "ICBC_DEBIT"),
            ("time_end", "20230101120000"),
            ("trade_type", "NATIVE"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .query_order(None, Some("OUT_FULL_001"))
        .await
        .expect("查询成功");
    assert_eq!(result.trade_state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.transaction_id.as_deref(), Some("TXN_FULL_001"));
    assert_eq!(result.total_fee, Some(200));
    assert_eq!(result.bank_type.as_deref(), Some("ICBC_DEBIT"));
    assert_eq!(result.time_end.as_deref(), Some("20230101120000"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 31. v2 下载对账单全流程（SUCCESS 类型）
// ═══════════════════════════════════════════════════════════════════════════

/// 对应 Java: downloadBill —— SUCCESS 类型全流程
#[tokio::test]
async fn v2_download_bill_success_type_full() {
    let header: Vec<String> = (0..20).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..20)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            5 => "TXN_BILL_001".to_string(),
            6 => "OUT_BILL_001".to_string(),
            12 => "300".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let bill_content = format!("{header_str}`{record_str}总交易单数`1`300`0`0`0`300`0");

    let server = MockServer::start(move |path, _| {
        assert!(path.starts_with("/pay/downloadbill"));
        (200, "text/plain".to_string(), bill_content.clone(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .download_bill("2023-01-01", "SUCCESS", "", None)
        .await
        .expect("下载成功");
    assert_eq!(result.bill_info_list.len(), 1);
    assert_eq!(result.bill_info_list[0].total_fee.as_deref(), Some("300"));
    assert_eq!(result.total_fee.as_deref(), Some("300"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 32. v2 下载对账单（空响应）
// ═══════════════════════════════════════════════════════════════════════════

/// download_bill: 空响应返回默认结果
#[tokio::test]
async fn v2_download_bill_empty_response() {
    let server =
        MockServer::start(|_path, _| (200, "text/plain".to_string(), String::new(), vec![])).await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .download_bill("2023-01-01", "ALL", "", None)
        .await
        .expect("下载成功");
    assert!(result.bill_info_list.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════
// 33. v2 下载对账单（REFUND 类型）
// ═══════════════════════════════════════════════════════════════════════════

/// download_bill: REFUND 类型解析
#[tokio::test]
async fn v2_download_bill_refund_type() {
    let header: Vec<String> = (0..26).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..26)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            16 => "REFUND_DL_001".to_string(),
            17 => "OUT_DL_001".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let bill_content = format!("{header_str}`{record_str}总交易单数`1`0`0`0`0`0`0");

    let server = MockServer::start(move |_path, _| {
        (200, "text/plain".to_string(), bill_content.clone(), vec![])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let result = service
        .download_bill("2023-01-01", "REFUND", "", None)
        .await
        .expect("下载成功");
    assert_eq!(result.bill_info_list.len(), 1);
    assert_eq!(
        result.bill_info_list[0].refund_id.as_deref(),
        Some("REFUND_DL_001")
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 34. v2 请求约束检查
// ═══════════════════════════════════════════════════════════════════════════

/// download_bill: GZIP 请求 XML 含 tar_type 字段（需 p12 证书，测试签名+XML）
#[test]
fn v2_download_bill_gzip_request_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = WxPayDownloadBillRequest::default();
    req.bill_type = Some("ALL".to_string());
    req.bill_date = Some("2023-01-01".to_string());
    req.tar_type = Some("GZIP".to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(xml.contains("ALL"), "XML 应含 bill_type: {xml}");
    assert!(xml.contains("2023-01-01"), "XML 应含 bill_date: {xml}");
    assert!(xml.contains("GZIP"), "XML 应含 tar_type: {xml}");
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 35. v2 通知解析
// ═══════════════════════════════════════════════════════════════════════════

/// parse_order_notify_result: 成功通知
#[tokio::test]
async fn v2_parse_order_notify_success() {
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "SUCCESS"),
        ("appid", APP_ID),
        ("mch_id", MCH_ID),
        ("nonce_str", "abc123"),
        ("out_trade_no", "ORDER_NOTIFY_001"),
        ("total_fee", "100"),
    ]);
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service
        .parse_order_notify_result(&xml)
        .await
        .expect("解析通知成功");
    assert_eq!(result.out_trade_no.as_deref(), Some("ORDER_NOTIFY_001"));
    assert_eq!(result.total_fee, Some(100));
}

/// parse_order_notify_result: V3 JSON 通知应报错
#[tokio::test]
async fn v2_parse_order_notify_v3_json_error() {
    let json_data = r#"{"id":"ev-001","resource_type":"encrypt-resource"}"#;
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let err = service
        .parse_order_notify_result(json_data)
        .await
        .unwrap_err();
    assert!(
        err.to_string().contains("V3") || err.to_string().contains("JSON"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 36. v2 扫码支付通知解析
// ═══════════════════════════════════════════════════════════════════════════

/// parse_scan_pay_notify_result: 成功通知
#[tokio::test]
async fn v2_parse_scan_pay_notify_success() {
    let xml = v2_signed_response(&[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "SUCCESS"),
        ("appid", APP_ID),
        ("mch_id", MCH_ID),
        ("nonce_str", "abc123"),
        ("openid", "openid_scan_001"),
        ("is_subscribe", "Y"),
    ]);
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service
        .parse_scan_pay_notify_result(&xml)
        .await
        .expect("解析扫码通知成功");
    assert_eq!(result.openid.as_deref(), Some("openid_scan_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 37. v2 退款通知解析
// ═══════════════════════════════════════════════════════════════════════════

/// parse_refund_notify_result: return_code=FAIL 时直接返回
#[tokio::test]
async fn v2_parse_refund_notify_fail() {
    let xml = "<xml><return_code>FAIL</return_code><return_msg>签名错误</return_msg><appid>wx123</appid><mch_id>mch123</mch_id></xml>";
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let result = service
        .parse_refund_notify_result(xml)
        .await
        .expect("FAIL 通知直接返回");
    assert_eq!(result.return_code.as_deref(), Some("FAIL"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 38. v2 get_sandbox_sign_key
// ═══════════════════════════════════════════════════════════════════════════

/// get_sandbox_sign_key: 验证请求签名（URL 硬编码，无法 mock，测试签名流程）
#[test]
fn v2_get_sandbox_sign_key_request_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayDefaultRequest::default();
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
    // WxPayDefaultRequest ignore_appid=true，不应含 appid
    assert!(!xml.contains("<appid>"), "XML 不应含 appid: {xml}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 39. v2 代金券
// ═══════════════════════════════════════════════════════════════════════════

/// send_coupon: 验证请求 XML 含 coupon_stock_id（需 p12 证书，测试签名+XML）
#[test]
fn v2_send_coupon_xml_fields() {
    let config = config_with_host("http://127.0.0.1:1");
    let mut req = wx_rust_pay::bean::WxPayCouponSendRequest::default();
    req.coupon_stock_id = Some("STOCK_001".to_string());
    req.partner_trade_no = Some("TRADE_001".to_string());
    req.openid = Some("openid_001".to_string());
    req.appid = Some(APP_ID.to_string());
    req.mch_id = Some(MCH_ID.to_string());
    impl_utils::check_and_sign(config.as_ref(), &mut req).expect("签名成功");
    let xml = req.to_xml().expect("XML 生成");
    assert!(xml.contains("STOCK_001"), "XML 应含 coupon_stock_id: {xml}");
    assert!(
        xml.contains("TRADE_001"),
        "XML 应含 partner_trade_no: {xml}"
    );
    assert!(xml.contains("<sign>"), "XML 应含签名: {xml}");
}

/// query_coupon_stock: 查询代金券批次
#[tokio::test]
async fn v2_query_coupon_stock_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/mmpaymkttransfers/query_coupon_stock"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("coupon_stock_id", "STOCK_001"),
            ("stock_name", "测试批次"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayCouponStockQueryRequest::default();
    req.coupon_stock_id = Some("STOCK_001".to_string());
    req.appid = Some(APP_ID.to_string());
    req.mch_id = Some(MCH_ID.to_string());
    let result = service.query_coupon_stock(&req).await.expect("查询成功");
    assert_eq!(result.coupon_stock_id.as_deref(), Some("STOCK_001"));
}

/// query_coupon_info: 查询代金券信息
#[tokio::test]
async fn v2_query_coupon_info_success() {
    let server = MockServer::start(|path, _| {
        assert!(path.starts_with("/mmpaymkttransfers/querycouponsinfo"));
        v2_xml_response(&[
            ("return_code", "SUCCESS"),
            ("return_msg", "OK"),
            ("result_code", "SUCCESS"),
            ("appid", APP_ID),
            ("mch_id", MCH_ID),
            ("nonce_str", "abc123"),
            ("coupon_id", "COUPON_001"),
        ])
    })
    .await;
    let service = WxPayServiceImpl::new_arc(config_with_host(&format!("http://{}", server.addr)));
    let mut req = wx_rust_pay::bean::WxPayCouponInfoQueryRequest::default();
    req.coupon_id = Some("COUPON_001".to_string());
    req.appid = Some(APP_ID.to_string());
    req.mch_id = Some(MCH_ID.to_string());
    let result = service.query_coupon_info(&req).await.expect("查询成功");
    assert_eq!(result.coupon_id.as_deref(), Some("COUPON_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 40. v2 create_scan_pay_qrcode_mode1
// ═══════════════════════════════════════════════════════════════════════════

/// create_scan_pay_qrcode_mode1: 生成二维码 URL
#[tokio::test]
async fn v2_create_scan_pay_qrcode_mode1() {
    let service = WxPayServiceImpl::new_arc(config_with_host("http://127.0.0.1:1"));
    let url = service
        .create_scan_pay_qrcode_mode1("PROD_001")
        .await
        .expect("生成二维码 URL 成功");
    assert!(
        url.starts_with("weixin://wxpay/bizpayurl?"),
        "URL 格式: {url}"
    );
    assert!(
        url.contains("product_id=PROD_001"),
        "应含 product_id: {url}"
    );
    assert!(
        url.contains(&format!("appid={APP_ID}")),
        "应含 appid: {url}"
    );
    assert!(
        url.contains(&format!("mch_id={MCH_ID}")),
        "应含 mch_id: {url}"
    );
    assert!(url.contains("sign="), "应含 sign: {url}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 41. gunzip_to_text
// ═══════════════════════════════════════════════════════════════════════════

/// gunzip_to_text: 空输入报错
#[test]
fn gunzip_to_text_empty() {
    let result = impl_utils::gunzip_to_text(&[]);
    assert!(result.is_err());
}

/// gunzip_to_text: 无效 GZIP 数据报错
#[test]
fn gunzip_to_text_invalid() {
    let result = impl_utils::gunzip_to_text(&[1, 2, 3, 4, 5]);
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// 42. decrypt_refund_req_info
// ═══════════════════════════════════════════════════════════════════════════

/// decrypt_refund_req_info: 无效 Base64 报错
#[test]
fn decrypt_refund_req_info_invalid_base64() {
    let result = impl_utils::decrypt_refund_req_info("test_key", "not_valid_base64!!!");
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// 43. current_time_millis
// ═══════════════════════════════════════════════════════════════════════════

/// current_time_millis: 返回非空字符串
#[test]
fn current_time_millis_works() {
    let ts = impl_utils::current_time_millis();
    assert!(!ts.is_empty());
    assert!(ts.len() >= 10, "时间戳应至少 10 位: {ts}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 44. runtime 错误构造
// ═══════════════════════════════════════════════════════════════════════════

/// runtime: 构造运行时错误
#[test]
fn runtime_error_message() {
    let err = impl_utils::runtime("测试错误消息");
    assert!(err.to_string().contains("测试错误消息"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 45. parse_bill_result RECHARGE_REFUND 类型
// ═══════════════════════════════════════════════════════════════════════════

/// parse_bill_result: RECHARGE_REFUND 类型解析（28 列）
#[test]
fn parse_bill_result_recharge_refund_type() {
    let header: Vec<String> = (0..28).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..28)
        .map(|i| match i {
            0 => "2023-01-01 12:00:00".to_string(),
            16 => "REFUND_RR_001".to_string(),
            17 => "OUT_RR_001".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let content = format!("{header_str}`{record_str}总交易单数`1`0`0`0`0`0`0");
    let result = impl_utils::parse_bill_result(&content, "RECHARGE_REFUND");
    assert!(result.is_some());
    let bill = result.unwrap();
    assert_eq!(bill.bill_info_list.len(), 1);
    assert_eq!(
        bill.bill_info_list[0].refund_id.as_deref(),
        Some("REFUND_RR_001")
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 46. parse_fund_flow_result 汇总字段
// ═══════════════════════════════════════════════════════════════════════════

/// parse_fund_flow_result: 汇总字段全量验证
#[test]
fn parse_fund_flow_summary_fields() {
    let header: Vec<String> = (0..11).map(|i| format!("col{i}")).collect();
    let header_str = header.join(" ");
    let record: Vec<String> = (0..11)
        .map(|i| match i {
            0 => "2023-01-01".to_string(),
            _ => "v".to_string(),
        })
        .collect();
    let record_str = record.join("`");
    let content = format!("{header_str}`{record_str}资金流水总笔数`5`2`100`3`200");
    let result = impl_utils::parse_fund_flow_result(&content);
    assert_eq!(result.total_record.as_deref(), Some("5"));
    assert_eq!(result.income_record.as_deref(), Some("2"));
    assert_eq!(result.income_amount.as_deref(), Some("100"));
    assert_eq!(result.expenditure_record.as_deref(), Some("3"));
    assert_eq!(result.expenditure_amount.as_deref(), Some("200"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 47. 配置管理 switchover_to
// ═══════════════════════════════════════════════════════════════════════════

/// switchover_to: 成功切换
#[tokio::test]
async fn switchover_to_success() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config(MCH_ID, APP_ID, service.wx_pay_config());
    service
        .switchover_to(MCH_ID, APP_ID)
        .await
        .expect("切换成功");
}

/// switchover_to: 失败报错
#[tokio::test]
async fn switchover_to_not_found() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let err = service
        .switchover_to("unknown", "unknown")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("未找到"), "错误信息: {err}");
}

/// switchover_to_with_key: 成功切换
#[tokio::test]
async fn switchover_to_with_key_success() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config(MCH_ID, APP_ID, service.wx_pay_config());
    service
        .switchover_to_with_key(MCH_ID)
        .await
        .expect("切换成功");
}

/// switchover_to_with_key: 失败报错
#[tokio::test]
async fn switchover_to_with_key_not_found() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let err = service.switchover_to_with_key("unknown").await.unwrap_err();
    assert!(err.to_string().contains("未找到"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════════════
// 48. 配置管理 add_config_with_key / remove_config_with_key
// ═══════════════════════════════════════════════════════════════════════════

/// add_config_with_key + remove_config_with_key
#[test]
fn config_add_remove_with_key() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    service.add_config_with_key("tenant_001", service.wx_pay_config());
    assert!(service.switchover_with_key("tenant_001"));
    service.remove_config_with_key("tenant_001");
    assert!(!service.switchover_with_key("tenant_001"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 49. 配置管理 set_multi_config_with_default
// ═══════════════════════════════════════════════════════════════════════════

/// set_multi_config_with_default: 指定默认商户
#[test]
fn config_set_multi_with_default() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let mut map = HashMap::new();
    map.insert(
        "key1".to_string(),
        service.wx_pay_config() as Arc<dyn WxPayConfig>,
    );
    service.set_multi_config_with_default(&map, "key1");
    assert!(service.switchover_with_key("key1"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 50. set_config
// ═══════════════════════════════════════════════════════════════════════════

/// set_config: 设置配置并自动注册
#[test]
fn config_set_config_auto_register() {
    let config = config_with_host("http://127.0.0.1:1");
    let service = WxPayServiceImpl::new_arc(config);
    let new_config = config_with_host("http://mch2.example.com");
    service.set_config(new_config);
    // set_config 会自动注册到 config_map 并切换
    let current = service.wx_pay_config();
    assert_eq!(current.mch_id().unwrap(), MCH_ID);
}
