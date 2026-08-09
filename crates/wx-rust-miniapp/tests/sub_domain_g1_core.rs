#![allow(clippy::field_reassign_with_default)]
//! 小程序 G1 核心服务组集成测试（镜像 Java `WxMaUserServiceImplTest` /
//! `WxMaMsgServiceImplTest` / `WxMaMediaServiceImplTest` /
//! `WxMaKefuServiceImplTest` / `WxMaAnalysisServiceImplTest` /
//! `WxMaCodeServiceImplTest` / `WxMaExpressServiceImplTest` /
//! `WxMaSecurityServiceImplTest` / `WxMaSettingServiceImplTest` 的 HTTP
//! 语义，经 MockServer 验证）。
//!
//! 覆盖：用户、消息、素材、客服、数据分析、代码管理、物流助手、内容安全、
//! 设置 9 个 G1 服务的请求路径 / payload / 响应解析；线格式键名以 bean 的
//! `#[serde(rename)]`（镜像 Java @SerializedName）为准。

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::bean::express::WxMaExpressOrderPerson;
use wx_rust_miniapp::bean::express::request::{
    WxMaExpressAddOrderRequest, WxMaExpressBindAccountRequest, WxMaExpressGetOrderRequest,
};
use wx_rust_miniapp::bean::kefu::WxMaKfAccountRequest;
use wx_rust_miniapp::bean::security::WxMaMsgSecCheckCheckRequest;
use wx_rust_miniapp::bean::{
    WxMaCodeCommitRequest, WxMaCodeSubmitAuditItem, WxMaCodeSubmitAuditRequest, WxMaDomainAction,
    WxMaTemplateData, WxMaUniformMessage,
};
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::message::WxMaKefuMessage;

/// 极简 mock HTTP 服务器：按请求路径返回 (Content-Type, body)，记录
/// 最近一次请求路径（含 query）与请求体、请求计数。
struct MockServer {
    addr: std::net::SocketAddr,
    requests: Arc<AtomicUsize>,
    last_path: Arc<std::sync::Mutex<String>>,
    last_body: Arc<std::sync::Mutex<String>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
}

impl MockServer {
    /// 启动服务器（`handler(path) -> (content_type, body)`）。
    async fn start<F>(handler: F) -> Self
    where
        F: Fn(&str) -> (String, String) + Send + Sync + 'static,
    {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("绑定端口");
        let addr = listener.local_addr().expect("获取地址");
        let requests = Arc::new(AtomicUsize::new(0));
        let last_path = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let requests_clone = requests.clone();
        let last_path_clone = last_path.clone();
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
                requests_clone.fetch_add(1, Ordering::SeqCst);
                let handler = handler.clone();
                let last_path_clone = last_path_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 16384];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求路径（含 query）与请求体（POST 场景）
                    if let Some(path) = request
                        .lines()
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                    {
                        *last_path_clone.lock().unwrap() = path.to_string();
                    }
                    if let Some(idx) = request.find("\r\n\r\n") {
                        let body = request[idx + 4..].to_string();
                        *last_body_clone.lock().unwrap() = body;
                    }
                    let path = request
                        .lines()
                        .next()
                        .map(|l| l.split_whitespace().nth(1).unwrap_or("/").to_string())
                        .unwrap_or_else(|| "/".to_string());
                    let (content_type, body) = handler(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });

        Self {
            addr,
            requests,
            last_path,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }

    fn last_path(&self) -> String {
        self.last_path.lock().unwrap().clone()
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

/// JSON 响应快捷构造。
fn json(body: &str) -> (String, String) {
    ("application/json".to_string(), body.to_string())
}

/// 构建指向 mock 服务器的小程序配置（appid=wxappid, secret=secret）。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let mut config = WxMaDefaultConfig::new("wxappid", "secret");
    config.set_token("token123");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = host.to_string();
    config.set_host_config(host_config);
    Arc::new(config)
}

/// 通用路由 handler：token 请求先应答，业务路径按 contains 分派
/// （子串冲突时更长的路径先判断）。
fn dispatch(
    handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
) -> impl Fn(&str) -> (String, String) + Send + Sync + 'static {
    move |path: &str| {
        if path.contains("/cgi-bin/token") || path.contains("/cgi-bin/stable_token") {
            return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
        }
        handler(path)
    }
}

// ---- 用户域（镜像 Java WxMaUserServiceImplTest） ----

#[tokio::test]
async fn user_js_code_to_session_and_check_user_info() {
    // 镜像 testGetSessionKey：GET /sns/jscode2session 含
    // appid/secret/js_code/grant_type 查询参数（执行引擎追加 access_token），
    // 响应 {openid, session_key, unionid} 解析。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/sns/jscode2session") {
            json(r#"{"openid":"o1","session_key":"sk_1","unionid":"u1"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let user_service = service.user_service().expect("用户服务存在");

    let session = user_service
        .get_session_info("aaa")
        .await
        .expect("登录会话成功");
    assert_eq!(session.openid, "o1");
    assert_eq!(session.session_key, "sk_1");
    assert_eq!(session.unionid, "u1");
    // token 注入（token 请求 + 业务请求共 2 次）
    assert!(server.request_count() >= 2, "token 请求 + 业务请求");
    let path = server.last_path();
    assert!(path.contains("/sns/jscode2session"), "路径: {path}");
    assert!(path.contains("appid=wxappid"), "路径: {path}");
    assert!(path.contains("secret=secret"), "路径: {path}");
    assert!(path.contains("js_code=aaa"), "路径: {path}");
    assert!(
        path.contains("grant_type=authorization_code"),
        "路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");

    // 镜像 testCheckUserInfo（Java golden 值）：sha1(rawData + sessionKey)
    // 十六进制小写与 signature 比较（纯本地计算，无 HTTP）。
    assert!(user_service.check_user_info(
        "HyVFkGl5F5OQWJZZaNzBBg==",
        r#"{"nickName":"Band","gender":1,"language":"zh_CN","city":"Guangzhou","province":"Guangdong","country":"CN","avatarUrl":"http://wx.qlogo.cn/mmopen/vi_32/1vZvI39NWFQ9XM4LtQpFrQJ1xlgZxx3w7bQxKARol6503Iuswjjn6nIGBiaycAjAtpujxyzYsrztuuICqIM5ibXQ/0"}"#,
        "75e81ceda165f4ffa64f4068af58c64b8f54b88c",
    ));
    assert!(!user_service.check_user_info(
        "HyVFkGl5F5OQWJZZaNzBBg==",
        r#"{"nickName":"Band"}"#,
        "75e81ceda165f4ffa64f4068af58c64b8f54b88c",
    ));
}

#[tokio::test]
async fn user_get_phone_number_and_set_user_storage() {
    // 镜像 testGetPhoneInfo：POST /wxa/business/getuserphonenumber
    // 请求体 {"code": ...}，响应 phone_info 解析为 WxMaPhoneNumberInfo
    // （bean rename：phoneNumber/purePhoneNumber/countryCode）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/business/getuserphonenumber") {
            json(
                r#"{"phone_info":{"phoneNumber":"13800138000","purePhoneNumber":"13800138000","countryCode":"86","watermark":{"appid":"wxappid","timestamp":"1700000000"}}}"#,
            )
        } else if path.contains("/wxa/set_user_storage") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let user_service = service.user_service().expect("用户服务存在");

    let phone = user_service
        .get_phone_number("code123")
        .await
        .expect("获取手机号成功")
        .expect("phone_info 存在");
    assert_eq!(phone.phone_number, "13800138000");
    assert_eq!(phone.pure_phone_number, "13800138000");
    assert_eq!(phone.country_code, "86");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["code"], "code123");
    assert!(
        server
            .last_path()
            .contains("/wxa/business/getuserphonenumber")
    );

    // 响应无 phone_info 时返回 None（Java 返回 null）
    let server_none = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/business/getuserphonenumber") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service_none = WxMaServiceImpl::new_arc(config_with_host(&server_none.url("")));
    let none = service_none
        .user_service()
        .expect("用户服务存在")
        .get_phone_number("code456")
        .await
        .expect("请求成功");
    assert!(none.is_none());

    // 镜像 testSetUserStorage：POST /wxa/set_user_storage?appid=&signature=
    // &openid=&sig_method=hmac_sha256，请求体 {"kv_list":[{"key","value"}]}，
    // signature 为请求体 JSON 的 HmacSHA256 签名（十六进制大写）。
    let session_key = "r7BXXKkLb8qrSNn05n0qiA";
    let mut kv_map = std::collections::HashMap::new();
    kv_map.insert("1".to_string(), "2".to_string());
    user_service
        .set_user_storage(&kv_map, session_key, "oX-test")
        .await
        .expect("设置用户数据成功");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["kv_list"][0]["key"], "1");
    assert_eq!(body["kv_list"][0]["value"], "2");
    let expected_params =
        serde_json::json!({ "kv_list": [{"key": "1", "value": "2"}] }).to_string();
    let expected_signature =
        wx_rust_common::util::SignUtils::create_hmac_sha256_sign(&expected_params, session_key);
    let path = server.last_path();
    assert!(path.contains("/wxa/set_user_storage"), "路径: {path}");
    assert!(path.contains("appid=wxappid"), "路径: {path}");
    assert!(path.contains("openid=oX-test"), "路径: {path}");
    assert!(path.contains("sig_method=hmac_sha256"), "路径: {path}");
    assert!(
        path.contains(&format!("signature={expected_signature}")),
        "路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

// ---- 消息域（镜像 Java WxMaMsgServiceImplTest / WxMaKefuMessageTest） ----

#[tokio::test]
async fn msg_send_kefu_msg_and_uniform_msg() {
    // 镜像 testSendKefuMsg + WxMaKefuMessageTest.testTextBuilder 线格式：
    // {"touser","msgtype","text":{"content"}} → POST /cgi-bin/message/custom/send
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/custom/send")
            || path.contains("/cgi-bin/message/wxopen/template/uniform_send")
        {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let msg_service = service.msg_service().expect("消息服务存在");

    let message = WxMaKefuMessage::new_text_builder()
        .to_user("o1")
        .content(
            "欢迎欢迎，热烈欢迎\n换行测试\n超链接:<a href=\"http://www.baidu.com\">Hello World</a>",
        )
        .build();
    assert!(
        msg_service
            .send_kefu_msg(&message)
            .await
            .expect("发送客服消息成功")
    );
    assert!(server.last_path().contains("/cgi-bin/message/custom/send"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["msgtype"], "text");
    assert_eq!(
        body["text"]["content"],
        "欢迎欢迎，热烈欢迎\n换行测试\n超链接:<a href=\"http://www.baidu.com\">Hello World</a>"
    );

    // 镜像 testSendUniformMsg：isMpTemplateMsg=false 时输出
    // {"touser", "weapp_template_msg":{template_id,page,form_id,data,emphasis_keyword}}
    let mut uniform = WxMaUniformMessage {
        is_mp_template_msg: false,
        to_user: Some("o1".to_string()),
        template_id: Some("TEMPLATE_ID".to_string()),
        page: Some("page/page/index".to_string()),
        form_id: Some("FORMID".to_string()),
        emphasis_keyword: Some("keyword1.DATA".to_string()),
        ..Default::default()
    };
    uniform
        .add_data(WxMaTemplateData {
            name: "keyword1".into(),
            value: "339208499".into(),
            color: String::new(),
        })
        .add_data(WxMaTemplateData {
            name: "keyword2".into(),
            value: "2015年01月05日 12:30".into(),
            color: String::new(),
        })
        .add_data(WxMaTemplateData {
            name: "keyword3".into(),
            value: "腾讯微信总部".into(),
            color: String::new(),
        })
        .add_data(WxMaTemplateData {
            name: "keyword4".into(),
            value: "广州市海珠区新港中路397号".into(),
            color: String::new(),
        });
    msg_service
        .send_uniform_msg(&uniform)
        .await
        .expect("发送统一消息成功");
    assert!(
        server
            .last_path()
            .contains("/cgi-bin/message/wxopen/template/uniform_send")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["touser"], "o1");
    assert_eq!(body["weapp_template_msg"]["template_id"], "TEMPLATE_ID");
    assert_eq!(body["weapp_template_msg"]["page"], "page/page/index");
    assert_eq!(body["weapp_template_msg"]["form_id"], "FORMID");
    assert_eq!(
        body["weapp_template_msg"]["emphasis_keyword"],
        "keyword1.DATA"
    );
    assert_eq!(
        body["weapp_template_msg"]["data"]["keyword1"]["value"],
        "339208499"
    );
    assert!(body["weapp_template_msg"]["data"]["keyword4"]["value"].is_string());
}

#[tokio::test]
async fn msg_create_updatable_message_activity_id() {
    // 镜像 testCreateUpdatableMessageActivityId：GET
    // /cgi-bin/message/wxopen/activityid/create，断言 activity_id/expiration_time。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/message/wxopen/activityid/create") {
            json(
                r#"{"activity_id":"1048_4f61uDloWPZl9pAs1dGx07vDiHKZ7FwJ0suohS1iMH5z8zhFktYk4nRqqBY~","expiration_time":1750000000}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let result = service
        .msg_service()
        .expect("消息服务存在")
        .create_updatable_message_activity_id()
        .await
        .expect("创建 activity_id 成功");
    assert_eq!(
        result["activity_id"],
        "1048_4f61uDloWPZl9pAs1dGx07vDiHKZ7FwJ0suohS1iMH5z8zhFktYk4nRqqBY~"
    );
    assert_eq!(result["expiration_time"], 1750000000);
}

// ---- 素材域（镜像 Java WxMaMediaServiceImplTest） ----

#[tokio::test]
async fn media_upload_and_get() {
    // 镜像 testUploadMedia：multipart（字段名 media）上传到
    // /cgi-bin/media/upload?type=image，响应解析
    // （Java WxMediaUploadResult 无 @SerializedName，Gson 默认字段名 →
    // type/mediaId/thumbMediaId/createdAt 线格式，与 bean rename 一致）。
    let media_file = std::env::temp_dir().join("wxma_tmp_media.png");
    std::fs::write(&media_file, b"FAKE_PNG_BYTES").expect("写入临时文件");
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/media/upload") {
            json(r#"{"type":"image","mediaId":"MEDIA_ID","createdAt":1388561359}"#)
        } else if path.contains("/cgi-bin/media/get") {
            (
                "application/octet-stream".to_string(),
                "FAKE_IMAGE_BYTES".to_string(),
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let media_service = service.media_service().expect("素材服务存在");

    let result = media_service
        .upload_media("image", media_file.to_str().expect("路径字符串"))
        .await
        .expect("上传素材成功");
    assert_eq!(result.r#type, "image");
    assert_eq!(result.media_id, "MEDIA_ID");
    assert_eq!(result.created_at, 1388561359);
    let path = server.last_path();
    assert!(
        path.contains("/cgi-bin/media/upload?type=image"),
        "路径: {path}"
    );
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
    // multipart body 含 media 字段与文件名
    let raw = server.last_body();
    assert!(raw.contains("name=\"media\""), "multipart 字段名 media");
    assert!(raw.contains("wxma_tmp_media.png"), "multipart 文件名");

    // 镜像 testGetMedia：GET /cgi-bin/media/get?access_token=&media_id=
    // 返回原始字节（非 JSON 响应）。
    let bytes = media_service
        .get_media("MEDIA_ID")
        .await
        .expect("下载素材成功");
    assert_eq!(bytes, b"FAKE_IMAGE_BYTES");
    let path = server.last_path();
    assert!(path.contains("/cgi-bin/media/get"), "路径: {path}");
    assert!(path.contains("media_id=MEDIA_ID"), "路径: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

// ---- 客服域（镜像 Java WxMaKefuServiceImplTest） ----

#[tokio::test]
async fn kefu_list_session_and_account() {
    // 镜像 testKfList（golden 响应 {"kf_list":[]} → size 0）
    // + testKfSessionGet（getsession?openid=）
    // + testKfAccountAdd（kf_account/kf_nick/kf_pwd 请求体）
    // + testKfSessionCreate（kf_account/openid 请求体）
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/customservice/getkflist") {
            json(r#"{"kf_list":[]}"#)
        } else if path.contains("/customservice/kfsession/getsession") {
            json(r#"{"kf_account":"test@kfaccount","createtime":1700000000}"#)
        } else if path.contains("/customservice/kfaccount/add")
            || path.contains("/customservice/kfsession/create")
        {
            json(r#"{"errcode":0}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let kefu_service = service.kefu_service().expect("客服服务存在");

    let list = kefu_service.kf_list().await.expect("客服列表成功");
    assert_eq!(list.kf_list.len(), 0);

    let session = kefu_service
        .kf_session_get("test_openid")
        .await
        .expect("会话状态成功");
    assert_eq!(session.kf_account, "test@kfaccount");
    assert_eq!(session.create_time, 1700000000);
    let path = server.last_path();
    assert!(
        path.contains("/customservice/kfsession/getsession?openid=test_openid"),
        "路径: {path}"
    );

    let request = WxMaKfAccountRequest {
        kf_account: "test@kfaccount".to_string(),
        kf_nick: "测试客服".to_string(),
        kf_pwd: "password".to_string(),
    };
    assert!(
        kefu_service
            .kf_account_add(&request)
            .await
            .expect("添加客服成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["kf_account"], "test@kfaccount");
    assert_eq!(body["kf_nick"], "测试客服");
    assert_eq!(body["kf_pwd"], "password");

    assert!(
        kefu_service
            .kf_session_create("test_openid", "test@kfaccount")
            .await
            .expect("创建会话成功")
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["kf_account"], "test@kfaccount");
    assert_eq!(body["openid"], "test_openid");
}

// ---- 数据分析域（镜像 Java WxMaAnalysisServiceImplTest） ----

#[tokio::test]
async fn analysis_visit_trend_and_user_portrait() {
    // 镜像 testGetDailyVisitTrend：POST
    // /datacube/getweanalysisappiddailyvisittrend，请求体
    // {"begin_date","end_date"}（yyyyMMdd），响应 list 解析
    // （bean rename：refDate/sessionCnt/visitPv）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/datacube/getweanalysisappiddailyvisittrend") {
            json(
                r#"{"list":[{"refDate":"20240101","sessionCnt":100,"visitPv":200,"visitUv":150,"visitUvNew":120,"stayTimeUv":1.5,"stayTimeSession":2.5,"visitDepth":3.5}]}"#,
            )
        } else if path.contains("/datacube/getweanalysisappiduserportrait") {
            json(
                r#"{"ref_date":"20240101-20240107","visit_uv_new":{"province":[{"name":"北京","value":10},{"name":"广东","value":20}]},"visit_uv":{"city":[{"name":"广州","value":5}]}}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let analysis_service = service.analysis_service().expect("数据分析服务存在");

    let trends = analysis_service
        .get_daily_visit_trend("20240101", "20240101")
        .await
        .expect("日访问趋势成功")
        .expect("list 存在");
    assert_eq!(trends.len(), 1);
    assert_eq!(trends[0].ref_date, "20240101");
    assert_eq!(trends[0].session_cnt, 100);
    assert_eq!(trends[0].visit_pv, 200);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["begin_date"], "20240101");
    assert_eq!(body["end_date"], "20240101");

    // 镜像 testGetUserPortrait：adapter 线格式 ref_date/visit_uv_new
    let portrait = analysis_service
        .get_user_portrait("20231225", "20240107")
        .await
        .expect("用户画像成功");
    assert_eq!(portrait.ref_date, "20240101-20240107");
    let province = portrait
        .visit_uv_new
        .as_ref()
        .expect("新增用户画像存在")
        .province
        .clone();
    assert_eq!(province.get("北京"), Some(&10));
    assert_eq!(province.get("广东"), Some(&20));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["begin_date"], "20231225");
    assert_eq!(body["end_date"], "20240107");
}

// ---- 代码管理域（镜像 Java WxMaCodeServiceImplTest） ----

#[tokio::test]
async fn code_commit_submit_audit_status_and_release() {
    // 镜像 testCommit（template_id/user_version/user_desc 请求体）
    // + testSubmitAudit（golden auditid 421937937，assertTrue(auditId > 0)）
    // + testGetAuditStatus（响应 auditId/status 解析）
    // + testRelease（请求体 {}）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/commit") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else if path.contains("/wxa/submit_audit") {
            json(r#"{"auditid":421937937}"#)
        } else if path.contains("/wxa/get_auditstatus") {
            json(
                r#"{"auditId":421937937,"status":0,"reason":"审核通过","screenshot":"","user_version":"v0.1.0","user_desc":"init","submit_audit_time":"2024-01-01 12:00:00"}"#,
            )
        } else if path.contains("/wxa/release") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let code_service = service.code_service().expect("代码服务存在");

    let commit = WxMaCodeCommitRequest {
        template_id: 6,
        user_version: "v0.1.0".to_string(),
        user_desc: "init".to_string(),
        ext_config: None,
    };
    code_service.commit(&commit).await.expect("上传代码成功");
    assert!(server.last_path().contains("/wxa/commit"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["template_id"], 6);
    assert_eq!(body["user_version"], "v0.1.0");
    assert_eq!(body["user_desc"], "init");

    let audit = WxMaCodeSubmitAuditRequest {
        item_list: vec![WxMaCodeSubmitAuditItem {
            address: "pages/logs/logs".to_string(),
            tag: "工具 效率".to_string(),
            first_class: "工具".to_string(),
            first_id: 287,
            second_class: "效率".to_string(),
            second_id: 616,
            title: "日志".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let audit_id = code_service
        .submit_audit(&audit)
        .await
        .expect("提交审核成功");
    assert!(audit_id > 0);
    assert_eq!(audit_id, 421937937);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["item_list"][0]["address"], "pages/logs/logs");
    assert_eq!(body["item_list"][0]["first_id"], 287);
    assert_eq!(body["item_list"][0]["second_id"], 616);
    assert_eq!(body["item_list"][0]["title"], "日志");

    let status = code_service
        .get_audit_status(421937937)
        .await
        .expect("审核状态成功");
    assert_eq!(status.audit_id, 421937937);
    assert_eq!(status.status, 0);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["auditid"], 421937937);

    code_service.release().await.expect("发布成功");
    assert_eq!(server.last_body(), "{}");
}

#[tokio::test]
async fn code_get_qr_code_bytes() {
    // 镜像 testGetQrCode：GET /wxa/get_qrcode（path 为空时仅 access_token），
    // 非 JSON 响应返回字节且非空。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/get_qrcode") {
            (
                "application/octet-stream".to_string(),
                "QR_CODE_BYTES".to_string(),
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let bytes = service
        .code_service()
        .expect("代码服务存在")
        .get_qr_code("")
        .await
        .expect("获取体验二维码成功");
    assert!(!bytes.is_empty());
    assert_eq!(bytes, b"QR_CODE_BYTES");
    let path = server.last_path();
    assert!(path.contains("/wxa/get_qrcode"), "路径: {path}");
    assert!(path.contains("access_token=MOCK_TOKEN"), "路径: {path}");
}

// ---- 物流助手域（镜像 Java WxMaExpressServiceImplTest） ----

#[tokio::test]
async fn express_account_quota_add_order_and_get_order() {
    // 镜像 testGetAllAccount（响应 list 数组，键 biz_id/delivery_id/quota_num）
    // + testGetQuota（响应解析 WxMaExpressAccount → quotaNum）
    // + testAddOrder（add_source/order_id/delivery_id/sender 请求体 + 响应解析）
    // + testGetOrder（order_id/waybill_id 请求体 + 响应解析）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/express/business/account/getall") {
            json(
                r##"{"list":[{"biz_id":"test_biz_id","delivery_id":"YUNDA","create_time":1574850455,"update_time":1574850456,"status_code":0,"alias":"alias","remark_wrong_msg":"","remark_content":"remark","quota_num":100,"quota_update_time":0,"service_type":[{"service_type":1,"service_name":"标准快递"}]}]}"##,
            )
        } else if path.contains("/cgi-bin/express/business/quota/get") {
            json(r#"{"quota_num":50,"delivery_id":"YUNDA","biz_id":"test_biz_id"}"#)
        } else if path.contains("/cgi-bin/express/business/order/add") {
            json(
                r#"{"errcode":0,"errmsg":"ok","order_id":"test201911271429004","waybill_id":"waybill_1","print_html":"<p>test</p>","waybill_data":[{"k":"v"}],"order_status":1}"#,
            )
        } else if path.contains("/cgi-bin/express/business/order/get") {
            json(
                r#"{"errcode":0,"errmsg":"ok","order_id":"test201911271429000","waybill_id":"test201911271429000_1574836404_waybill_id","order_status":2}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let express_service = service.express_service().expect("物流服务存在");

    let accounts = express_service
        .get_all_account()
        .await
        .expect("账号列表成功");
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].delivery_id, "YUNDA");
    assert_eq!(accounts[0].biz_id, "test_biz_id");
    assert_eq!(accounts[0].quota_num, 100);

    // 镜像 testGetQuota：POST /cgi-bin/express/business/quota/get
    let bind = WxMaExpressBindAccountRequest {
        r#type: "bind".to_string(),
        biz_id: "test_biz_id".to_string(),
        delivery_id: "YUNDA".to_string(),
        password: "password".to_string(),
        remark_content: "####".to_string(),
    };
    let quota = express_service
        .get_quota(&bind)
        .await
        .expect("获取余额成功");
    assert_eq!(quota, 50);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["delivery_id"], "YUNDA");
    assert_eq!(body["biz_id"], "test_biz_id");

    // 镜像 testAddOrder：POST /cgi-bin/express/business/order/add
    let add_order = WxMaExpressAddOrderRequest {
        add_source: 0,
        order_id: "test201911271429004".to_string(),
        openid: "oAg_-0PDUPuLbQw9V9kXE9OkU-Vo".to_string(),
        delivery_id: "TEST".to_string(),
        biz_id: "test_biz_id".to_string(),
        custom_remark: "".to_string(),
        sender: WxMaExpressOrderPerson {
            name: "张三".to_string(),
            mobile: "177****9809".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let order_result = express_service
        .add_order(&add_order)
        .await
        .expect("生成运单成功");
    assert_eq!(order_result.order_id, "test201911271429004");
    assert_eq!(order_result.waybill_id, "waybill_1");
    assert_eq!(order_result.order_status, 1);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["order_id"], "test201911271429004");
    assert_eq!(body["delivery_id"], "TEST");
    assert_eq!(body["sender"]["name"], "张三");

    // 镜像 testGetOrder：POST /cgi-bin/express/business/order/get
    let get_order = WxMaExpressGetOrderRequest {
        order_id: "test201911271429000".to_string(),
        delivery_id: "TEST".to_string(),
        waybill_id: "test201911271429000_1574836404_waybill_id".to_string(),
        openid: "oAg_-0PDUPuLbQw9V9kXE9OkU-Vo".to_string(),
    };
    let order_result = express_service
        .get_order(&get_order)
        .await
        .expect("获取运单成功");
    assert_eq!(order_result.order_id, "test201911271429000");
    assert_eq!(
        order_result.waybill_id,
        "test201911271429000_1574836404_waybill_id"
    );
    assert_eq!(order_result.order_status, 2);
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["order_id"], "test201911271429000");
    assert_eq!(
        body["waybill_id"],
        "test201911271429000_1574836404_waybill_id"
    );
}

// ---- 内容安全域（镜像 Java WxMaSecurityServiceImplTest） ----

#[tokio::test]
async fn security_msg_check_and_image_check() {
    // 镜像 testCheckMessage：POST /wxa/msg_sec_check，请求体 {"content":...}
    // 成功即返回 true（对应 secData 中 "hello world!" → true 分支）
    // + testCheckMessage2（v2 请求体 version/openid/scene/content →
    // 响应 WxMaMsgSecCheckCheckResponse 解析）
    // + testCheckImage：multipart（字段 media）上传 /wxa/img_sec_check。
    let image_file = std::env::temp_dir().join("wxma_tmp_img.png");
    std::fs::write(&image_file, b"FAKE_PNG_BYTES").expect("写入临时文件");
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/msg_sec_check") {
            json(
                r#"{"errcode":0,"errmsg":"ok","result":{"suggest":"pass","label":"100"},"trace_id":"trace_1","detail":[]}"#,
            )
        } else if path.contains("/wxa/img_sec_check") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let security_service = service.security_service().expect("安全服务存在");

    assert!(
        security_service
            .check_message("hello world!")
            .await
            .expect("文本检测成功")
    );
    assert!(server.last_path().contains("/wxa/msg_sec_check"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["content"], "hello world!");

    let request = WxMaMsgSecCheckCheckRequest {
        version: "2".to_string(),
        openid: "xxx".to_string(),
        scene: 1,
        content: "特3456书yuuo莞6543李zxcz蒜7782法fgnv级".to_string(),
        ..Default::default()
    };
    let response = security_service
        .check_message_with_request(&request)
        .await
        .expect("v2 文本检测成功");
    assert_eq!(response.result.suggest, "pass");
    assert_eq!(response.trace_id, "trace_1");
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["version"], "2");
    assert_eq!(body["openid"], "xxx");
    assert_eq!(body["scene"], 1);
    assert_eq!(body["content"], "特3456书yuuo莞6543李zxcz蒜7782法fgnv级");

    assert!(
        security_service
            .check_image(image_file.to_str().expect("路径字符串"))
            .await
            .expect("图片检测成功")
    );
    assert!(server.last_path().contains("/wxa/img_sec_check"));
    let raw = server.last_body();
    assert!(raw.contains("name=\"media\""), "multipart 字段名 media");
    assert!(raw.contains("wxma_tmp_img.png"), "multipart 文件名");
}

// ---- 设置域（镜像 Java WxMaSettingServiceImplTest） ----

#[tokio::test]
async fn setting_bind_tester_unbind_and_modify_domain() {
    // 镜像 testBindTester：POST /wxa/bind_tester，请求体 {"wechatid":...}
    // + testUnbindTester：POST /wxa/unbind_tester
    // + testModifyDomain：POST /wxa/modify_domain，请求/响应 WxMaDomainAction
    //（线格式键 requestdomain/wsrequestdomain/uploaddomain/downloaddomain/
    // webviewdomain 与 Java @SerializedName 一致）。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/bind_tester") || path.contains("/wxa/unbind_tester") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else if path.contains("/wxa/modify_domain") {
            json(
                r#"{"action":"get","requestdomain":["https://example.com"],"wsrequestdomain":[],"uploaddomain":[],"downloaddomain":[],"webviewdomain":[]}"#,
            )
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let setting_service = service.setting_service().expect("设置服务存在");

    setting_service
        .bind_tester("WeChatId")
        .await
        .expect("绑定体验者成功");
    assert!(server.last_path().contains("/wxa/bind_tester"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["wechatid"], "WeChatId");

    setting_service
        .unbind_tester("WeChatId")
        .await
        .expect("解绑体验者成功");
    assert!(server.last_path().contains("/wxa/unbind_tester"));
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["wechatid"], "WeChatId");

    let domain_action = WxMaDomainAction {
        action: "get".to_string(),
        ..Default::default()
    };
    let result = setting_service
        .modify_domain(&domain_action)
        .await
        .expect("操作域名成功");
    assert_eq!(result.action, "get");
    assert_eq!(
        result.request_domain,
        vec!["https://example.com".to_string()]
    );
    let body: serde_json::Value = serde_json::from_str(&server.last_body()).expect("请求体 JSON");
    assert_eq!(body["action"], "get");
}

// ---- 用户域补充：checkSessionKey（镜像 Java testCheckSessionKey） ----

#[tokio::test]
async fn user_check_session_key_signature() {
    // Java testCheckSessionKey：GET /wxa/checksessionkey?openid=&signature=
    // &sig_method=hmac_sha256，signature 为 openid 的 HmacSHA256（key 为
    // session_key，十六进制大写）；请求成功恒返回 true。
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/wxa/checksessionkey") {
            json(r#"{"errcode":0,"errmsg":"ok"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let ok = service
        .user_service()
        .expect("用户服务存在")
        .check_session_key("test_openid", "sk_1")
        .await
        .expect("检查登录态成功");
    assert!(ok);
    assert!(server.request_count() >= 2);
    let path = server.last_path();
    assert!(path.contains("/wxa/checksessionkey"), "路径: {path}");
    assert!(path.contains("openid=test_openid"), "路径: {path}");
    assert!(path.contains("sig_method=hmac_sha256"), "路径: {path}");
    let expected_signature =
        wx_rust_common::util::SignUtils::create_hmac_sha256_sign("test_openid", "sk_1");
    assert!(
        path.contains(&format!("signature={expected_signature}")),
        "路径: {path}"
    );
}
