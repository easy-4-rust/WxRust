//! HttpTransport 抽象测试。RUST_OBLIGATION：trait 对象可用性 + MockTransport 零网络。
use wx_rust_common::http::{
    HttpTransport, MockTransport, TransportBody, TransportMethod, TransportRequest,
};

#[tokio::test]
async fn mock_transport_answers_without_network() {
    let t = MockTransport::new(|req| {
        assert_eq!(req.method, TransportMethod::Get);
        let body = format!("{{\"echo\":\"{}\"}}", req.url);
        Ok(wx_rust_common::http::TransportResponse {
            status: 200,
            headers: vec![],
            body: body.into_bytes(),
        })
    });
    let resp = t
        .send(TransportRequest {
            method: TransportMethod::Get,
            url: "https://mock.local/x".into(),
            headers: vec![],
            body: TransportBody::None,
        })
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
    assert!(
        String::from_utf8(resp.body)
            .unwrap()
            .contains("mock.local/x")
    );
}

#[tokio::test]
async fn ok_json_returns_200_with_exact_body() {
    let t = MockTransport::ok_json(r#"{"errcode":0,"errmsg":"ok"}"#);
    let resp = t
        .send(TransportRequest {
            method: TransportMethod::PostJson("{}".into()),
            url: "https://mock.local/post".into(),
            headers: vec![],
            body: TransportBody::None,
        })
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
    assert_eq!(
        resp.body,
        r#"{"errcode":0,"errmsg":"ok"}"#.as_bytes().to_vec()
    );
    assert!(resp.headers.is_empty());
}
