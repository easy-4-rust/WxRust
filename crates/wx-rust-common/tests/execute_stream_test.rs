//! `execute_stream` 流式下载测试（Task 7）。
//!
//! RUST_OBLIGATION：大文件可流式——响应体以分块 [`bytes::Bytes`] 流交付，
//! 调用方不 forced 在传输层聚合全量 body。
//!
//! 测试自带极简 TCP HTTP 服务器（tokio net/io-util 为 common 既有依赖），
//! 精确控制分块写出时机（块间 flush + sleep），与 pay 侧 httpmock 真端口
//! 流式测试（`wx-rust-pay/tests/download_stream_test.rs`）互补：
//! - 本文件直测 [`wx_rust_common::pipeline::stream::execute_stream`] 与
//!   [`wx_rust_common::http::ReqwestTransport::send_stream`]；
//! - pay 侧走业务入口（`download_bill_stream`）的 golden 对照。

use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wx_rust_common::http::{ReqwestTransport, TransportBody, TransportMethod, TransportRequest};
use wx_rust_common::pipeline::stream::execute_stream;

/// 极简 HTTP 服务器：接受单个连接，读完整请求（头 + Content-Length 体），
/// 按给定状态行应答并把 `chunks` 逐块写出（块间 flush + 20ms sleep——
/// 确保客户端以多个流项观察到分块），随后关闭连接。
///
/// 返回 (base_url, 收到的原始请求文本)。
async fn spawn_chunk_server(
    status_line: &str,
    chunks: Vec<Vec<u8>>,
) -> (String, Arc<std::sync::Mutex<String>>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("绑定端口");
    let addr = listener.local_addr().expect("获取地址");
    let status_line = status_line.to_string();
    let request_log = Arc::new(std::sync::Mutex::new(String::new()));
    let request_log_clone = request_log.clone();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("接受连接");
        // 读到请求头结束，并按 Content-Length 读完整请求体
        let mut raw = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            let n = socket.read(&mut buf).await.expect("读取请求");
            if n == 0 {
                break;
            }
            raw.extend_from_slice(&buf[..n]);
            let text = String::from_utf8_lossy(&raw).into_owned();
            if let Some((head, _)) = text.split_once("\r\n\r\n") {
                let len = head
                    .lines()
                    .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
                    .and_then(|l| l.split_once(':'))
                    .and_then(|(_, v)| v.trim().parse::<usize>().ok())
                    .unwrap_or(0);
                if raw.len() >= head.len() + 4 + len {
                    break;
                }
            }
        }
        let text = String::from_utf8_lossy(&raw).into_owned();
        *request_log_clone.lock().unwrap() = text;
        // 应答：Content-Length 固定总长，body 分块写出
        let total: usize = chunks.iter().map(|c| c.len()).sum();
        let head = format!(
            "{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {total}\r\nConnection: close\r\n\r\n"
        );
        socket.write_all(head.as_bytes()).await.expect("写响应头");
        for chunk in &chunks {
            socket.write_all(chunk).await.expect("写响应块");
            socket.flush().await.expect("flush 响应块");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        socket.shutdown().await.expect("关闭连接");
    });
    (format!("http://{addr}"), request_log)
}

/// 分块流：逐块顺序拼接正确、聚合等于全量 body。
#[tokio::test]
async fn stream_chunks_arrive_in_order_and_aggregate_to_full_body() {
    // 三个可辨识分块（首字节各异），32KB 块 + 块间 20ms——客户端必然分多次读到
    let chunk_a = vec![b'A'; 32 * 1024];
    let chunk_b = vec![b'B'; 32 * 1024];
    let chunk_c = vec![b'C'; 32 * 1024];
    let mut full = Vec::new();
    for c in [&chunk_a, &chunk_b, &chunk_c] {
        full.extend_from_slice(c);
    }
    let (base, request_log) =
        spawn_chunk_server("HTTP/1.1 200 OK", vec![chunk_a, chunk_b, chunk_c]).await;

    let transport = ReqwestTransport::new(reqwest::Client::new());
    let mut stream = execute_stream(
        &transport,
        TransportRequest {
            method: TransportMethod::Get,
            url: format!("{base}/pay/downloadbill"),
            headers: vec![],
            body: TransportBody::None,
        },
    )
    .await
    .expect("流式请求建立");

    let mut items: Vec<bytes::Bytes> = Vec::new();
    while let Some(item) = stream.next().await {
        items.push(item.expect("分块读取成功"));
    }
    // 流式证明：分块写出（块间 sleep）必然产生多个流项
    assert!(items.len() > 1, "应观察到多个分块，实际 {} 块", items.len());
    // 顺序证明：流项按到达序拼接后与全量 body 逐字节一致
    let mut aggregated = Vec::new();
    for item in &items {
        aggregated.extend_from_slice(item);
    }
    assert_eq!(aggregated, full);
    // 首块以 'A' 开头、末块以 'C' 结尾（块序非乱序的直观断言）
    assert_eq!(items[0][0], b'A');
    assert_eq!(
        items[items.len() - 1][items[items.len() - 1].len() - 1],
        b'C'
    );
    // 请求侧：GET 打到指定路径
    let request = request_log.lock().unwrap().clone();
    assert!(request.starts_with("GET /pay/downloadbill "), "{request}");
}

/// 非成功状态码（500）：execute_stream 直接返回 Err（不产出流）。
#[tokio::test]
async fn non_success_status_returns_err() {
    let (base, _) =
        spawn_chunk_server("HTTP/1.1 500 Internal Server Error", vec![b"boom".to_vec()]).await;
    let transport = ReqwestTransport::new(reqwest::Client::new());
    let err = match execute_stream(
        &transport,
        TransportRequest {
            method: TransportMethod::Get,
            url: format!("{base}/pay/downloadbill"),
            headers: vec![],
            body: TransportBody::None,
        },
    )
    .await
    {
        Ok(_) => panic!("500 应返回 Err"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("500"), "错误应含状态码：{err}");
}

/// POST + 文本体：请求体经流式通道原样送达（send_stream 与 send 同一请求构造）。
#[tokio::test]
async fn post_body_reaches_server_via_stream() {
    let (base, request_log) =
        spawn_chunk_server("HTTP/1.1 200 OK", vec![b"<xml>ok</xml>".to_vec()]).await;
    let transport = ReqwestTransport::new(reqwest::Client::new());
    let mut stream = execute_stream(
        &transport,
        TransportRequest {
            method: TransportMethod::PostXml("<xml>bill</xml>".to_string()),
            url: format!("{base}/pay/downloadbill"),
            headers: vec![],
            body: TransportBody::None,
        },
    )
    .await
    .expect("流式请求建立");
    let mut aggregated = Vec::new();
    while let Some(item) = stream.next().await {
        aggregated.extend_from_slice(&item.expect("分块读取成功"));
    }
    assert_eq!(aggregated, b"<xml>ok</xml>");
    let request = request_log.lock().unwrap().clone();
    assert!(request.starts_with("POST /pay/downloadbill "), "{request}");
    assert!(request.ends_with("<xml>bill</xml>"), "{request}");
}
