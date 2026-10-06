use std::sync::Arc;
use wx_rust_store::api::{WxStoreService, r#impl::WxStoreServiceImpl};
use wx_rust_store::config::{WxStoreConfig, r#impl::WxStoreDefaultConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let host = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        for index in 0..2 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut bytes = [0u8; 8192];
            let count = socket.read(&mut bytes).await.unwrap();
            let request = String::from_utf8_lossy(&bytes[..count]);
            let body = if index == 0 {
                assert!(request.starts_with("GET /token"));
                r#"{"access_token":"consumer_token","expires_in":7200}"#
            } else {
                assert!(request.contains("/channels/ec/basics/info/get"));
                assert!(request.contains("access_token=consumer_token"));
                r#"{"errcode":0,"info":{"nickname":"fixture-shop"}}"#
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        }
    });
    let config = WxStoreDefaultConfig::new("fixture_app", "fixture_secret");
    config.set_api_host_url(&host);
    config.set_access_token_url(&format!("{host}/token"));
    let service = WxStoreServiceImpl::new_arc(Arc::new(config));
    assert_eq!(service.get_access_token().await.unwrap(), "consumer_token");
    let shop = service
        .basic_service()
        .unwrap()
        .get_shop_info()
        .await
        .unwrap();
    assert_eq!(shop.err_code, 0);
    assert_eq!(shop.info.unwrap().nickname.as_deref(), Some("fixture-shop"));
    server.await.unwrap();
}
