//! 基础服务测试（镜像 Java `BaseWxQidianServiceImplTest` 的有效用例）。
//!
//! Java 测试经 Guice 注入真实配置（`ApiTestModule`）；本文件以 MockServer
//! 承载同一 HTTP 语义：多配置切换、短链、服务器 IP、网络检测、签名校验、
//! access_token 并发单次刷新等。Java 中大量空壳用例（`testGetTicket` 等
//! 无断言）不逐一镜像，以有断言的端到端用例覆盖其语义。

mod common;

use common::{MockServer, dispatch, json, service_with_host, service_with_multi};

use wx_rust_qidian::api::WxQidianService;
use wx_rust_qidian::util::WxQidianConfigStorageHolder;

/// 镜像 `testSwitchover`：切换存在的配置成功并更新持有器；不存在的配置
/// 切换失败。
#[tokio::test]
async fn test_switchover() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_multi(&server.url(""), "another");

    assert!(service.switchover("another"), "切换到 another 成功");
    assert_eq!(WxQidianConfigStorageHolder::get(), "another");
    assert!(!service.switchover("whatever"), "不存在的配置切换失败");
    assert!(
        !service.switchover("default"),
        "default 非 key（key 为 appid）"
    );
}

/// 镜像 `testSwitchoverTo`：切换成功后当前配置可取到 access_token。
#[tokio::test]
async fn test_switchover_to() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_multi(&server.url(""), "another");

    service.switchover_to("another").expect("切换成功");
    assert_eq!(WxQidianConfigStorageHolder::get(), "another");
    let token = service.get_access_token().await.expect("获取 token 成功");
    assert!(!token.is_empty());
    // 切到不存在的配置报错（对应 Java WxRuntimeException）
    assert!(service.switchover_to("whatever").is_err());
}

/// 镜像 `testGetCallbackIP`：解析 ip_list 数组。
#[tokio::test]
async fn test_get_callback_ip() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/getcallbackip") {
            json(r#"{"errcode":0,"errmsg":"ok","ip_list":["101.226.103.0","101.226.62.0"]}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let ip_array = service.get_callback_ip().await.expect("获取 IP 成功");
    assert_eq!(ip_array.len(), 2);
    assert_eq!(ip_array[0], "101.226.103.0");
}

/// 镜像 `testNetCheck`：解析网络检测结果（dns/ping）。
#[tokio::test]
async fn test_net_check() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/callback/check") {
            json(
                r#"{"errcode":0,"errmsg":"ok","dns":[{"ip":"101.226.103.0","real_operator":"CHINANET"}],"ping":[{"ip":"101.226.103.0","from_operator":"CHINANET","package_loss":"0%","time":"2ms"}]}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let result = service
        .net_check("all", "DEFAULT")
        .await
        .expect("网络检测成功");
    assert_eq!(result.dns_infos.len(), 1);
    assert_eq!(result.dns_infos[0].ip, "101.226.103.0");
    assert_eq!(result.ping_infos.len(), 1);
    assert_eq!(result.ping_infos[0].package_loss, "0%");
    // 请求体含 action/check_operator（对应 Java 组装参数）
    assert!(
        server.last_body().contains(r#""action":"all""#),
        "body: {}",
        server.last_body()
    );
    assert!(
        server.last_body().contains(r#""check_operator":"DEFAULT""#),
        "body: {}",
        server.last_body()
    );
}

/// 镜像 `testShortUrl`：POST 短链接口并解析 short_url。
#[tokio::test]
async fn test_short_url() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/shorturl") {
            json(r#"{"errcode":0,"errmsg":"ok","short_url":"https://w.url.cn/s/abc123"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let short_url = service
        .short_url("http://www.baidu.com/test")
        .await
        .expect("短链成功");
    assert_eq!(short_url, "https://w.url.cn/s/abc123");
    // 请求体含 action/long_url（对应 Java 组装参数）
    assert!(
        server.last_body().contains(r#""action":"long2short""#),
        "body: {}",
        server.last_body()
    );
    assert!(
        server
            .last_body()
            .contains(r#""long_url":"http://www.baidu.com/test""#),
        "body: {}",
        server.last_body()
    );
}

/// 镜像 `testShortUrl_with_exceptional_url`：网址含 `&access_token=` 时
/// 报错（对应 Java `@Test(expectedExceptions = WxErrorException.class)`）。
#[tokio::test]
async fn test_short_url_with_exceptional_url() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_host(&server.url(""));
    let result = service
        .short_url("http://www.baidu.com/test?redirect_count=1&access_token=123")
        .await;
    assert!(result.is_err(), "含 access_token 的网址应报错");
    // 未发起任何请求
    assert_eq!(server.request_count(), 0);
}

/// 镜像 `testCheckSignature`：正确/错误签名校验。
#[tokio::test]
async fn test_check_signature() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_host(&server.url(""));
    let timestamp = "1711520394";
    let nonce = "abcdefghijklmn";
    // 期望 = sha1(token + timestamp + nonce 排序拼接，对应 Java SHA1.gen)
    let expected = wx_rust_common::util::crypto::Sha1::digest(&["token123", timestamp, nonce])
        .expect("摘要成功");
    assert!(service.check_signature(timestamp, nonce, &expected));
    assert!(!service.check_signature(timestamp, nonce, "wrong-signature"));
}

/// 镜像 `refreshAccessTokenDuplicatelyTest`：10 线程并发刷新只请求一次
/// token 接口（双检锁去重）。
#[tokio::test]
async fn test_refresh_access_token_duplicately() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_host(&server.url(""));
    // 强制过期（对应 Java `expireAccessToken()`）
    service.config_storage().expire_access_token();

    let mut handles = Vec::new();
    for _ in 0..10 {
        let svc = service.clone();
        handles.push(tokio::spawn(async move {
            svc.get_access_token().await.expect("获取 token 成功")
        }));
    }
    let mut tokens = std::collections::HashSet::new();
    for handle in handles {
        tokens.insert(handle.await.expect("任务完成"));
    }
    // 10 个线程拿到同一个 token（对应 Java `set.size() == 1`）
    assert_eq!(tokens.len(), 1, "并发刷新只产生一个 token");
    assert_eq!(server.token_hits(), 1, "token 接口只请求一次");
}

/// 镜像 `testGetTicket`/`testGetJsapiTicket`：ticket 获取与缓存。
#[tokio::test]
async fn test_get_jsapi_ticket_and_ticket() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_JSAPI","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let jsapi_ticket = service.get_jsapi_ticket().await.expect("获取 jsapi ticket");
    assert_eq!(jsapi_ticket, "TICKET_JSAPI");
    // 缓存命中：再次获取不再请求
    let _ = service.get_jsapi_ticket().await.expect("缓存命中");
    assert_eq!(
        server.path_hits("/cgi-bin/ticket/getticket"),
        1,
        "ticket 只请求一次"
    );
    // 强制刷新（对应 Java `getJsapiTicket(true)` → `expireTicket`）
    let _ = service
        .get_jsapi_ticket_with_force(true)
        .await
        .expect("强制刷新");
    assert_eq!(
        server.path_hits("/cgi-bin/ticket/getticket"),
        2,
        "强制刷新再请求一次"
    );
    // get_ticket 无 force 重载（对应 Java `getTicket(TicketType)`）
    let sdk_ticket = service
        .get_ticket(wx_rust_common::enums::TicketType::Sdk)
        .await
        .expect("sdk ticket");
    assert_eq!(sdk_ticket, "TICKET_JSAPI");
}

/// 镜像 `testCreateJsapiSignature`：签名对象非空（完整签名向量见
/// wx_qidian_jsapi.rs）。
#[tokio::test]
async fn test_create_jsapi_signature_not_null() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_1","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    let signature = service
        .create_jsapi_signature("http://www.baidu.com")
        .await
        .expect("签名成功");
    assert!(!signature.signature.is_empty());
    assert_eq!(signature.app_id, "wxqidian_default");
}

/// 镜像 `testClearQuota`：配额清零请求成功。
#[tokio::test]
async fn test_clear_quota() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/clear_quota") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));
    service
        .clear_quota("wxqidian_default")
        .await
        .expect("清零成功");
    assert!(server.last_path().contains("/cgi-bin/clear_quota"));
    assert!(
        server.last_body().contains(r#""appid":"wxqidian_default""#),
        "body: {}",
        server.last_body()
    );
}

/// 镜像 `testBuildQrConnectUrl`：构造扫码登录 url（redirect_uri 被编码）。
#[tokio::test]
async fn test_build_qr_connect_url() {
    let server = MockServer::start(dispatch(|_path| json("{}"))).await;
    let service = service_with_host(&server.url(""));
    let url = service.build_qr_connect_url(
        "https://example.com/callback?from=wx",
        "snsapi_login",
        "state-1",
    );
    assert!(
        url.starts_with("https://open.weixin.qq.com/connect/qrconnect"),
        "url: {url}"
    );
    assert!(url.contains("appid=wxqidian_default"), "url: {url}");
    assert!(
        url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback%3Ffrom%3Dwx"),
        "redirect_uri 已编码: {url}"
    );
    assert!(url.contains("state=state-1"), "url: {url}");
}
