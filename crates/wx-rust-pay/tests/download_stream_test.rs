//! pay 流式对账单下载测试（Task 7：`download_bill_stream` /
//! `wx_rust_common::pipeline::stream::execute_stream`）。
//!
//! httpmock 起真实 TCP 端口，`ReqwestTransport` 直接请求——与 common 侧
//! `execute_stream_test.rs`（手写分块服务器直测传输层）互补，本文件走业务
//! 入口验证：
//! 1. 大 body 流式聚合 == 全量、逐块顺序正确（多次 next）；
//! 2. 非成功状态码（500）→ Err；
//! 3. golden：`download_raw_bill`（原方法，签名/行为不动）与
//!    `download_bill_stream` 聚合对同一 mock 应答逐字节一致；
//! 4. GZIP 语义：流式方法透传原始 gzip 字节（解压由调用方处理，方法文档
//!    约定；flate2 为 pay 既有依赖）。

use std::io::Read;
use std::sync::Arc;

use futures_util::StreamExt;
use httpmock::prelude::*;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;

/// 官方文档样例商户参数（Java SignUtilsTest 同源）。
const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";

/// 构建指向 mock 服务器的支付配置（v2 下载账单仅需 appid/mch_id/mch_key）。
fn config_with_host(host: &str) -> Arc<dyn WxPayConfig> {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_api_host_url(host);
    Arc::new(config)
}

/// 聚合流为字节（顺带断言每个分块均为 Ok）。
async fn collect_stream(
    stream: futures_util::stream::BoxStream<
        'static,
        Result<bytes::Bytes, wx_rust_common::error::WxErrorException>,
    >,
) -> (Vec<bytes::Bytes>, Vec<u8>) {
    let mut items = Vec::new();
    let mut aggregated = Vec::new();
    let mut stream = stream;
    while let Some(item) = stream.next().await {
        let chunk = item.expect("分块读取成功");
        aggregated.extend_from_slice(&chunk);
        items.push(chunk);
    }
    (items, aggregated)
}

/// 大 body（约 1MB）流式下载：聚合等于全量、多次 next 分块接收。
#[tokio::test]
async fn stream_download_aggregates_large_bill_body_in_order() {
    // 账单格式行（Java BaseWxPayServiceImplTest 数据格式）× 12000 行 ≈ 1MB
    let line = "`2018-02-01 04:21:23,`wx2421b1c4370ec43b,`10000100,`,`1000,`50000305742018020103387128253,`201707260201501501005710775,`oUpF8uMuAJO_M2pxb1Q9zNjWeS6o,`JSAPI,`SUCCESS,`CMC,`CNY,`100.00,`0.00,`,`,`15.00,`0.00,`REFUND_SOURCE_RECHARGE_FUNDS,`SUCCESS,`测试商品,`attach-data,`0.01000,`0.60%,`100.00,`0.00,`\n";
    let body = line.repeat(12000);
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/pay/downloadbill");
        then.status(200).body(body.as_bytes());
    });

    let service = WxPayServiceImpl::new_arc(config_with_host(&server.base_url()));
    let stream = service
        .download_bill_stream("20180201", "ALL", "")
        .await
        .expect("流式下载建立");
    let (items, aggregated) = collect_stream(stream).await;

    // 流式证明：约 1MB body 经真 socket 分块接收（多个流项）
    assert!(
        items.len() > 1,
        "大 body 应分多块接收，实际 {} 块",
        items.len()
    );
    // 聚合与全量逐字节一致（顺序正确性的充分证明）
    assert_eq!(aggregated, body.as_bytes());
    // 业务侧确实发出了一次下载请求
    assert_eq!(mock.calls(), 1);
}

/// 非成功状态码（500）：download_bill_stream 返回 Err。
#[tokio::test]
async fn stream_download_error_status_returns_err() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/pay/downloadbill");
        then.status(500).body("internal error");
    });

    let service = WxPayServiceImpl::new_arc(config_with_host(&server.base_url()));
    let err = match service.download_bill_stream("20180201", "ALL", "").await {
        Ok(_) => panic!("500 应返回 Err"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("500"), "错误应含状态码：{err}");
}

/// golden 对照：download_raw_bill（原方法）与流式聚合对同一 mock 应答
/// 逐字节一致；且两次请求报文相同（同一 mock 命中两次）。
#[tokio::test]
async fn golden_stream_matches_download_raw_bill_bytes() {
    let bill_text = "交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,代金券金额,微信退款单号,商户退款单号,退款金额,充值券退款金额,退款类型,退款状态,商品名称,商户数据包,手续费,费率,订单金额,申请退款金额,费率备注\n\
`2018-02-01 04:21:23,`wx2421b1c4370ec43b,`10000100,`,`1000,`50000305742018020103387128253,`201707260201501501005710775,`oUpF8uMuAJO_M2pxb1Q9zNjWeS6o,`JSAPI,`SUCCESS,`CMC,`CNY,`100.00,`0.00,`,`,`15.00,`0.00,`REFUND_SOURCE_RECHARGE_FUNDS,`SUCCESS,`测试商品,`attach-data,`0.01000,`0.60%,`100.00,`0.00,`\n\
总交易单数,应结订单总金额,退款总金额,充值券退款总金额,手续费总金额,订单总金额,申请退款总金额\n\
`1,`100.00,`0.00,`0.00,`0.01000,`100.00,`0.00";
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/pay/downloadbill");
        then.status(200).body(bill_text.as_bytes());
    });

    let service = WxPayServiceImpl::new_arc(config_with_host(&server.base_url()));
    // 原方法（v2 文本通道，resp.text()）
    let raw = service
        .download_raw_bill("20180201", "ALL", "", None)
        .await
        .expect("原方法下载成功");
    // 新流式方法（原始字节通道）
    let stream = service
        .download_bill_stream("20180201", "ALL", "")
        .await
        .expect("流式下载建立");
    let (_, aggregated) = collect_stream(stream).await;

    // 逐字节一致（golden）
    assert_eq!(aggregated, raw.as_bytes());
    // 同一 mock 被两次请求命中（报文同构：同路径/同签名报文）
    assert_eq!(mock.calls(), 2);
}

/// GZIP 语义：流式方法透传原始 gzip 字节（不解压）；调用方以 flate2 解压后
/// 与原方法（tar_type=GZIP 自动解压）的输出一致。
#[tokio::test]
async fn gzip_stream_passes_raw_bytes_and_caller_can_inflate() {
    let bill_text = "交易时间,商户号,微信订单号\n`2018-02-01 04:21:23,`10000100,`50000305742018020103387128253\n总交易单数\n`1";
    let gzipped = {
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(bill_text.as_bytes()).expect("gzip 压缩");
        encoder.finish().expect("完成 gzip")
    };
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/pay/downloadbill");
        then.status(200).body(gzipped.clone());
    });

    let service = WxPayServiceImpl::new_arc(config_with_host(&server.base_url()));
    // 流式：原始 gzip 字节透传
    let stream = service
        .download_bill_stream("20180201", "ALL", "GZIP")
        .await
        .expect("流式下载建立");
    let (_, aggregated) = collect_stream(stream).await;
    assert_eq!(aggregated, gzipped);
    // 调用方解压（文档约定：flate2 已是 pay 依赖）
    let mut inflated = String::new();
    flate2::read::GzDecoder::new(&aggregated[..])
        .read_to_string(&mut inflated)
        .expect("解压流式账单");
    assert_eq!(inflated, bill_text);
    // 原方法（GZIP 自动解压，Java Files.readAllLines + Joiner.on("\n") 语义）
    let raw = service
        .download_raw_bill("20180201", "ALL", "GZIP", None)
        .await
        .expect("原方法 GZIP 下载成功");
    assert_eq!(raw, bill_text);
}
