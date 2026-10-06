use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use wx_rust_channel::api::{WxChannelService, r#impl::WxChannelServiceImpl};
use wx_rust_channel::config::{WxChannelConfig, r#impl::WxChannelDefaultConfig};
use wx_rust_store::api::{WxStoreService, r#impl::WxStoreServiceImpl};
use wx_rust_store::config::{WxStoreConfig, r#impl::WxStoreDefaultConfig};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let host = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut business_requests = Vec::new();
        for _ in 0..4 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut bytes = Vec::new();
            let mut chunk = [0u8; 4096];
            let request = loop {
                let count = socket.read(&mut chunk).await.unwrap();
                assert!(count > 0, "请求提前关闭");
                bytes.extend_from_slice(&chunk[..count]);
                let text = String::from_utf8(bytes.clone()).unwrap();
                if let Some((headers, body)) = text.split_once("\r\n\r\n") {
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (key, value) = line.split_once(':')?;
                            key.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if body.len() >= length {
                        break text;
                    }
                }
            };
            let (headers, request_body) = request.split_once("\r\n\r\n").unwrap();
            let first_line = headers.lines().next().unwrap();
            let body = if first_line.starts_with("GET /channel-token") {
                r#"{"access_token":"old_token","expires_in":7200}"#
            } else if first_line.starts_with("GET /store-token") {
                r#"{"access_token":"new_token","expires_in":7200}"#
            } else {
                assert!(first_line.starts_with("GET /channels/ec/basics/info/get?"));
                assert_eq!(request_body, "");
                business_requests.push(first_line.to_string());
                r#"{"errcode":0,"info":{"nickname":"same-shop"}}"#
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        }
        business_requests
    });
    let old_config = WxChannelDefaultConfig::new("old_app", "old_secret");
    old_config.set_api_host_url(&host);
    old_config.set_access_token_url(&format!("{host}/channel-token"));
    let new_config = WxStoreDefaultConfig::new("new_app", "new_secret");
    new_config.set_api_host_url(&host);
    new_config.set_access_token_url(&format!("{host}/store-token"));
    let channel = WxChannelServiceImpl::new_arc(Arc::new(old_config));
    let store = WxStoreServiceImpl::new_arc(Arc::new(new_config));
    assert!(channel.order_service().is_some());
    assert!(store.order_service().is_some());
    let old_basic = channel.basic_service().unwrap();
    let new_basic = store.basic_service().unwrap();
    let (old_response, new_response) =
        tokio::join!(old_basic.get_shop_info(), new_basic.get_shop_info());
    let old_response = old_response.unwrap();
    let new_response = new_response.unwrap();
    assert_eq!(old_response.err_code, new_response.err_code);
    assert_eq!(
        Some(old_response.info.nickname),
        new_response.info.unwrap().nickname
    );
    let requests = server.await.unwrap();
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .any(|line| line.contains("access_token=old_token"))
    );
    assert!(
        requests
            .iter()
            .any(|line| line.contains("access_token=new_token"))
    );
    let old = wx_rust_channel::bean::base::AddressInfo::default();
    // 新旧模型类型独立，业务方显式决定旧默认值的转换方式。
    let new = wx_rust_store::bean::base::AddressInfo {
        user_name: Some(old.user_name.clone()),
        tel_number: Some(old.tel_number.clone()),
        ..Default::default()
    };
    assert_eq!(Some(old.user_name), new.user_name);
}
