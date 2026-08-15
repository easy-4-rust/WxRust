#![allow(clippy::field_reassign_with_default)]
//! mp 子域服务测试（镜像 Java `WxMpUserTagServiceTest` / `WxMpStoreServiceTest` /
//! `WxMpCommentServiceTest` / `WxMpDataCubeServiceTest` / `WxMpWifiServiceTest` /
//! `WxMpDraftServiceTest` / `WxMpFreePublishServiceTest` / `WxMpDeviceServiceTest` /
//! `WxMpMassMessageServiceTest` 的 HTTP 语义，经 MockServer 验证）。
//!
//! 覆盖：用户标签、黑名单、门店、评论、数据统计、Wi-Fi、草稿箱、发布、
//! 设备、群发消息 10 个子服务的请求路径 / payload / 响应解析。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_mp::api::WxMpService;
use wx_rust_mp::bean::material::WxMpNewsArticle;
use wx_rust_mp::bean::{
    WxMpMassNews, WxMpMassOpenIdsMessage, WxMpMassPreviewMessage, WxMpMassTagMessage, WxMpMassVideo,
};
use wx_rust_mp::config::WxMpConfigStorage;
use wx_rust_mp::config::r#impl::WxMpDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> body`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
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
                    // 记录请求体（POST 场景）
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let body = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });

        Self {
            addr,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
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

/// 构建指向 mock 服务器的公众号配置。
fn config_with_host(host: &str) -> Arc<dyn WxMpConfigStorage> {
    let mut config = WxMpDefaultConfig::new("wxappid", "secret");
    config
        .set_token("token123")
        .set_aes_key("abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG");
    let mut host_config = wx_rust_mp::config::WxMpHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由 handler：token 请求 + 各子域响应。
fn dispatch(
    handler: impl Fn(&str) -> String + Send + Sync + 'static,
) -> impl Fn(&str) -> String + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/stable_token") {
            return r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#.to_string();
        }
        handler(path)
    }
}

// ---- 用户标签（镜像 Java WxUserTagServiceTest） ----

#[tokio::test]
async fn user_tag_create_and_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/tags/create") {
            r#"{"tag":{"id":100,"name":"测试标签"}}"#.to_string()
        } else if path.contains("/cgi-bin/tags/get") {
            r#"{"tags":[{"id":100,"name":"测试标签","count":42}]}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let tag_service = service.user_tag_service().expect("标签服务存在");

    let tag = tag_service
        .tag_create("测试标签")
        .await
        .expect("创建标签成功");
    assert_eq!(tag.id, 100);
    assert_eq!(tag.name, "测试标签");
    // Java 语义：{"tag":{"name":...}}
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["tag"]["name"], "测试标签");

    let tags = tag_service.tag_get().await.expect("获取标签成功");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].count, 42);
}

#[tokio::test]
async fn user_tag_update_delete_and_batch() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/user/tag/get") {
            r#"{"count":2,"data":{"openid":["o1","o2"]},"next_openid":"next"}"#.to_string()
        } else if path.contains("/cgi-bin/tags/getidlist") {
            r#"{"tagid_list":[1,2,3]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let tag_service = service.user_tag_service().expect("标签服务存在");

    assert!(tag_service.tag_update(100, "改名").await.expect("更新成功"));
    assert!(tag_service.tag_delete(100).await.expect("删除成功"));
    assert!(
        tag_service
            .batch_tagging(100, &["o1", "o2"])
            .await
            .expect("打标签成功")
    );
    assert!(
        tag_service
            .batch_untagging(100, &["o1"])
            .await
            .expect("取消标签成功")
    );

    let users = tag_service
        .tag_list_user(100, "next")
        .await
        .expect("标签用户列表成功");
    assert_eq!(users.count, 2);
    assert_eq!(users.data.openid_list.len(), 2);
    assert_eq!(users.next_openid, "next");

    let ids = tag_service
        .user_tag_list("o1")
        .await
        .expect("用户标签列表成功");
    assert_eq!(ids, vec![1, 2, 3]);
}

// ---- 黑名单（镜像 Java WxMpUserBlacklistServiceTest） ----

#[tokio::test]
async fn user_blacklist_get_and_push() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/tags/members/getblacklist") {
            r#"{"total":2,"count":2,"data":{"openid":["o1","o2"]},"next_openid":""}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let blacklist_service = service.user_blacklist_service().expect("黑名单服务存在");

    let result = blacklist_service
        .get_blacklist("")
        .await
        .expect("获取黑名单成功");
    assert_eq!(result.total, 2);
    assert_eq!(result.openid_list, vec!["o1", "o2"]);
    assert_eq!(result.next_openid, "");

    blacklist_service
        .push_to_blacklist(&["o3".to_string()])
        .await
        .expect("拉黑成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["openid_list"][0], "o3");

    blacklist_service
        .pull_from_blacklist(&["o3".to_string()])
        .await
        .expect("取消拉黑成功");
}

// ---- 门店（镜像 Java WxMpStoreServiceTest） ----

#[tokio::test]
async fn store_add_get_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/poi/getpoilist") {
            r#"{"total_count":1,"business_list":[{"base_info":{"business_name":"测试门店"}}]}"#.to_string()
        } else if path.contains("/cgi-bin/poi/getpoi") {
            r#"{"business":{"base_info":{"business_name":"测试门店","province":"广东省","city":"深圳市"}}}"#.to_string()
        } else if path.contains("/cgi-bin/poi/getwxcategory") {
            r#"{"category_list":["美食","购物"]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let store_service = service.store_service().expect("门店服务存在");

    let mut info = wx_rust_mp::bean::store::WxMpStoreBaseInfo::default();
    info.business_name = "测试门店".to_string();
    store_service.add(&info).await.expect("添加门店成功");
    // Java 语义：{"business":{"base_info":{...}}}
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["business"]["base_info"]["business_name"], "测试门店");

    let got = store_service.get("POI_ID").await.expect("获取门店成功");
    assert_eq!(got.business_name, "测试门店");
    assert_eq!(got.province, "广东省");

    let list = store_service.list(0, 10).await.expect("门店列表成功");
    assert_eq!(list.total_count, 1);
    assert_eq!(list.business_list.len(), 1);

    let categories = store_service.list_categories().await.expect("门店类目成功");
    assert_eq!(categories, vec!["美食", "购物"]);
}

// ---- 评论（镜像 Java WxMpCommentServiceTest） ----

#[tokio::test]
async fn comment_open_list_and_mark() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/comment/list") {
            r#"{"total":1,"comment":[{"user_comment_id":100,"openid":"o1","create_time":"1700000000","content":"好文","comment_type":0}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let comment_service = service.comment_service().expect("评论服务存在");

    comment_service
        .open("MSG_1", Some(0))
        .await
        .expect("打开评论成功");
    comment_service
        .close("MSG_1", Some(0))
        .await
        .expect("关闭评论成功");

    let list = comment_service
        .list("MSG_1", Some(0), 0, 10, 0)
        .await
        .expect("评论列表成功");
    assert_eq!(list.total, 1);
    assert_eq!(list.comment[0].content, "好文");

    comment_service
        .mark_elect("MSG_1", Some(0), 100)
        .await
        .expect("精选评论成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["user_comment_id"], 100);

    comment_service
        .reply_add("MSG_1", Some(0), 100, "感谢")
        .await
        .expect("回复成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["content"], "感谢");
}

// ---- 数据统计（镜像 Java WxMpDataCubeServiceTest） ----

#[tokio::test]
async fn datacube_user_summary_and_article_total() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/datacube/getusersummary") {
            r#"{"list":[{"ref_date":"2024-01-01","user_source":0,"new_user":10,"cancel_user":1}]}"#.to_string()
        } else if path.contains("/datacube/getarticletotal") {
            r#"{"list":[{"msgid":"12003_3","title":"标题","url":"https://x","details":[],"user_source":0}]}"#.to_string()
        } else {
            r#"{"list":[]}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let data_cube_service = service.data_cube_service().expect("数据统计服务存在");

    let summary = data_cube_service
        .get_user_summary("2024-01-01", "2024-01-01")
        .await
        .expect("用户分析数据成功");
    assert_eq!(summary.len(), 1);
    assert_eq!(summary[0].new_user, 10);
    assert_eq!(summary[0].ref_date, "2024-01-01");
    // Java 语义：{"begin_date":...,"end_date":...}
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["begin_date"], "2024-01-01");
    assert_eq!(body["end_date"], "2024-01-01");

    let totals = data_cube_service
        .get_article_total("2024-01-01", "2024-01-07")
        .await
        .expect("图文分析数据成功");
    assert_eq!(totals.len(), 1);
    assert_eq!(totals[0].title, "标题");
}

// ---- Wi-Fi（镜像 Java WxMpWifiServiceTest） ----

#[tokio::test]
async fn wifi_list_and_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/bizwifi/shop/list") {
            r#"{"totalcount":1,"pageindex":1,"pagecount":1,"records":[{"shop_id":100,"shop_name":"店","ssid":"WIFI1"}]}"#.to_string()
        } else if path.contains("/bizwifi/shop/get") {
            r#"{"shop_name":"店","ssid":"WIFI1","protocol_type":1,"ap_count":2}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let wifi_service = service.wifi_service().expect("Wi-Fi 服务存在");

    let list = wifi_service
        .list_shop(1, 10)
        .await
        .expect("门店 Wi-Fi 列表成功");
    assert_eq!(list.total_count, 1);
    assert_eq!(list.records[0].shop_name, "店");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["pageindex"], 1);
    assert_eq!(body["pagesize"], 10);

    let info = wifi_service
        .get_shop_wifi_info(100)
        .await
        .expect("门店 Wi-Fi 信息成功");
    assert_eq!(info.ssid, "WIFI1");
    assert_eq!(info.ap_count, 2);

    assert!(
        wifi_service
            .update_shop_wifi_info(100, "WIFI1", "WIFI2", Some("pwd"))
            .await
            .expect("更新成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["old_ssid"], "WIFI1");
    assert_eq!(body["password"], "pwd");
}

// ---- 草稿箱（镜像 Java WxMpDraftServiceTest） ----

#[tokio::test]
async fn draft_add_and_list() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/draft/add") {
            r#"{"media_id":"DRAFT_MEDIA_ID"}"#.to_string()
        } else if path.contains("/cgi-bin/draft/batchget") {
            r#"{"total_count":1,"item_count":1,"item":[{"media_id":"DRAFT_MEDIA_ID","content":{"news_item":[]},"update_time":1700000000}]}"#.to_string()
        } else if path.contains("/cgi-bin/draft/count") {
            r#"{"total_count":3}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let draft_service = service.draft_service().expect("草稿箱服务存在");

    let mut add = wx_rust_mp::bean::draft::WxMpAddDraft::default();
    add.articles = Vec::new();
    let media_id = draft_service.add_draft(&add).await.expect("新建草稿成功");
    assert_eq!(media_id, "DRAFT_MEDIA_ID");

    let list = draft_service.list_draft(0, 10).await.expect("草稿列表成功");
    assert_eq!(list.total_count, 1);
    assert_eq!(list.items[0].media_id, "DRAFT_MEDIA_ID");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["no_content"], 0);

    let count = draft_service.count_draft().await.expect("草稿数量成功");
    assert_eq!(count, 3);
}

// ---- 发布（镜像 Java WxMpFreePublishServiceTest） ----

#[tokio::test]
async fn free_publish_submit_and_status() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/freepublish/submit") {
            r#"{"publish_id":"100"}"#.to_string()
        } else if path.contains("/cgi-bin/freepublish/get") {
            r#"{"publish_id":"100","publish_status":0,"article_id":"ARTICLE_1","article_detail":{"count":1,"item":[{"idx":1,"article_url":"https://x"}]},"fail_idx":[]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let publish_service = service.free_publish_service().expect("发布服务存在");

    let publish_id = publish_service
        .submit("DRAFT_MEDIA_ID")
        .await
        .expect("发布成功");
    assert_eq!(publish_id, "100");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["media_id"], "DRAFT_MEDIA_ID");

    let status = publish_service
        .get_push_status("100")
        .await
        .expect("发布状态成功");
    assert_eq!(status.publish_status, 0);
    assert_eq!(status.article_detail.item[0].article_url, "https://x");
}

// ---- 设备（镜像 Java WxMpDeviceServiceTest） ----

#[tokio::test]
async fn device_qr_code_and_bind() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/device/getqrcode") {
            r#"{"deviceid":"DEVICE_1","qrticket":"TICKET","devicelicence":"LIC","base_resp":{"base_info":{"device_type":"gh_xxx","device_id":"DEVICE_1"},"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else if path.contains("/device/bind") {
            r#"{"base_resp":{"base_info":{"device_type":"gh_xxx","device_id":"DEVICE_1"},"errcode":0,"errmsg":"ok"}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let device_service = service.device_service().expect("设备服务存在");

    let qr = device_service
        .get_qr_code("PRODUCT_1")
        .await
        .expect("设备二维码成功");
    assert_eq!(qr.qr_ticket, "TICKET");
    assert_eq!(qr.base_resp.err_code, 0);

    let mut bind = wx_rust_mp::bean::device::WxDeviceBind::default();
    bind.ticket = "TICKET".to_string();
    bind.device_id = "DEVICE_1".to_string();
    bind.open_id = "o1".to_string();
    let result = device_service.bind(&bind).await.expect("绑定成功");
    assert_eq!(result.base_resp.err_code, 0);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["device_id"], "DEVICE_1");
    assert_eq!(body["openid"], "o1");
}

// ---- 群发消息（镜像 Java WxMpMassMessageServiceTest） ----

#[tokio::test]
async fn mass_open_ids_and_tag_send() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/mass/sendall")
            || path.contains("/cgi-bin/message/mass/send")
        {
            r#"{"errcode":0,"errmsg":"ok","msg_id":34182,"msg_data_id":206227730}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let mass_service = service.mass_message_service().expect("群发服务存在");

    // openid 列表群发（text）
    let mut open_ids = WxMpMassOpenIdsMessage::default();
    open_ids.to_users = vec!["o1".to_string(), "o2".to_string()];
    open_ids.msg_type = "text".to_string();
    open_ids.content = "hello".to_string();
    let result = mass_service
        .mass_open_ids_message_send(&open_ids)
        .await
        .expect("群发成功");
    assert_eq!(result.msg_id, "34182");
    // Java adapter 线格式：touser 数组 + text 分支
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"][0], "o1");
    assert_eq!(body["msgtype"], "text");
    assert_eq!(body["text"]["content"], "hello");

    // 标签群发（mpnews）
    let mut tag = WxMpMassTagMessage::default();
    tag.tag_id = 100;
    tag.msg_type = "mpnews".to_string();
    tag.media_id = "MEDIA_1".to_string();
    let result = mass_service
        .mass_group_message_send(&tag)
        .await
        .expect("标签群发成功");
    assert_eq!(result.msg_data_id, "206227730");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["filter"]["is_to_all"], false);
    assert_eq!(body["filter"]["tag_id"], 100);
    assert_eq!(body["mpnews"]["media_id"], "MEDIA_1");
    assert_eq!(body["send_ignore_reprint"], 0);
}

#[tokio::test]
async fn mass_news_upload_and_preview() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/uploadnews") {
            r#"{"type":"news","media_id":"NEWS_MEDIA_ID","created_at":1388561359}"#.to_string()
        } else if path.contains("/cgi-bin/message/mass/preview") {
            r#"{"errcode":0,"errmsg":"ok","msg_id":34182}"#.to_string()
        } else if path.contains("/cgi-bin/media/uploadvideo") {
            r#"{"type":"video","media_id":"VIDEO_MEDIA_ID","created_at":1388561359}"#.to_string()
        } else {
            "{}".to_string()
        }
    }))
    .await;
    let service =
        wx_rust_mp::api::r#impl::WxMpServiceImpl::new_arc(config_with_host(&server.url("")));
    let mass_service = service.mass_message_service().expect("群发服务存在");

    // 图文上传：adapter 线格式 articles 数组
    let mut news = WxMpMassNews::default();
    let mut article = WxMpNewsArticle::default();
    article.title = "标题".to_string();
    article.content = "内容".to_string();
    article.thumb_media_id = "THUMB".to_string();
    article.show_cover_pic = true;
    article.author = "作者".to_string();
    news.articles.push(article);
    let result = mass_service
        .mass_news_upload(&news)
        .await
        .expect("图文上传成功");
    assert_eq!(result.media_id, "NEWS_MEDIA_ID");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["articles"][0]["title"], "标题");
    assert_eq!(body["articles"][0]["show_cover_pic"], "1");
    assert_eq!(body["articles"][0]["author"], "作者");

    // 视频上传
    let mut video = WxMpMassVideo::default();
    video.media_id = "VIDEO_1".to_string();
    video.title = "视频".to_string();
    video.description = "描述".to_string();
    let result = mass_service
        .mass_video_upload(&video)
        .await
        .expect("视频上传成功");
    assert_eq!(result.media_id, "VIDEO_MEDIA_ID");

    // 预览
    let mut preview = WxMpMassPreviewMessage::default();
    preview.to_wx_user_openid = "o1".to_string();
    preview.msg_type = "text".to_string();
    preview.content = "预览内容".to_string();
    let result = mass_service
        .mass_message_preview(&preview)
        .await
        .expect("预览成功");
    assert_eq!(result.msg_id, "34182");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["text"]["content"], "预览内容");
}
