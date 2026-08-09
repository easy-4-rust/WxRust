#![allow(clippy::field_reassign_with_default)]
//! miniapp G4 能力服务组补充集成测试（MockServer 模式，镜像 Java 测试语义）。
//!
//! 对应 Java `weixin-java-miniapp/src/test/java/cn/binarywang/wx/miniapp/api/impl/`
//! 下 `WxMaOcrServiceImplTest` 的 `MockTest` 内嵌用例（idCard/bankCard/driving
//! 等 URL 版响应 JSON 逐字镜像）与 `weixin-java-mp` 同族 `WxMpImgProcServiceImplTest`
//! 的 `mockTest` 内嵌用例（qrcode/superResolution/aiCrop 响应 JSON）。
//!
//! 与 Java 语义对齐的断言重点：
//! - OCR 六方法均为 **POST**，imgUrl 经 `URLEncoder.encode`（等价于 Rust
//!   `url::form_urlencoded::byte_serialize`）后作为 `?img_url=` query 参数，
//!   请求体为空（Java `post(url, (String) null)`）。
//! - `super_resolution` 为 **GET**（Java `get(url, null)`）。
//! - `ai_crop` 的 `ratios` 为 Java `String.format` 直填（不编码）。

use std::sync::Arc;
use std::sync::atomic::Ordering;

use wx_rust_common::bean::ocr::{
    WxOcrBankCardResult, WxOcrCommResult, WxOcrDrivingLicenseResult, WxOcrDrivingResult,
    WxOcrIdCardResult,
};
use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 极简 mock HTTP 服务器：按请求路径返回固定响应，记录最近一次请求行与请求体。
struct MockServer {
    addr: std::net::SocketAddr,
    last_request_line: Arc<std::sync::Mutex<String>>,
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
        let last_request_line = Arc::new(std::sync::Mutex::new(String::new()));
        let last_body = Arc::new(std::sync::Mutex::new(String::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handler = Arc::new(handler);

        let last_request_line_clone = last_request_line.clone();
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
                let last_request_line_clone = last_request_line_clone.clone();
                let last_body_clone = last_body_clone.clone();
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};
                    let mut buf = [0u8; 65536];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    // 记录请求行（含 query 串，如
                    // "POST /cv/ocr/idcard?img_url=..&access_token=.. HTTP/1.1"）
                    if let Some(line) = request.lines().next() {
                        *last_request_line_clone.lock().unwrap() = line.to_string();
                    }
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
            last_request_line,
            last_body,
            stop,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn last_request_line(&self) -> String {
        self.last_request_line.lock().unwrap().clone()
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

/// 构建指向 mock 服务器的小程序配置。
fn config_with_host(host: &str) -> Arc<dyn WxMaConfig> {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
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

/// 从请求行中提取 query 参数（`?key=value&...`）。
fn query_param(request_line: &str, key: &str) -> Option<String> {
    let query = request_line.split('?').nth(1)?;
    let query = query.split(' ').next()?;
    for pair in query.split('&') {
        let mut it = pair.splitn(2, '=');
        if it.next() == Some(key) {
            return it.next().map(|s| s.to_string());
        }
    }
    None
}

/// Java `URLEncoder.encode("http://img/x.jpg", UTF_8)` 的结果（Rust 侧
/// `url::form_urlencoded::byte_serialize` 同语义：`:`/`/` 编码为 `%3A`/`%2F`）。
const ENCODED_IMG_URL: &str = "http%3A%2F%2Fimg%2Fx.jpg";

// ---- OCR（镜像 Java WxMaOcrServiceImplTest.MockTest.testIdCard / testBankCard） ----

#[tokio::test]
async fn ocr_id_card_post_with_encoded_img_url_query() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/ocr/idcard") {
            // 镜像 Java MockTest.testIdCard 的 returnJson
            r#"{"type":"Back","name":"张三","id":"110101199909090099","valid_date":"20110101-20210201"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let ocr = service.ocr_service().expect("OCR 服务存在");

    let result: WxOcrIdCardResult = ocr
        .ocr_id_card("http://img/x.jpg")
        .await
        .expect("身份证识别成功");
    // 响应解析（对应 Java `WxOcrIdCardResult.fromJson`）
    assert_eq!(result.r#type, "Back");
    assert_eq!(result.name, "张三");
    assert_eq!(result.id, "110101199909090099");
    assert_eq!(result.valid_date, "20110101-20210201");

    // Java 语义：POST + URLEncoder 编码后的 img_url 作 query 参数 + 空请求体
    let request_line = server.last_request_line();
    assert!(
        request_line.starts_with("POST "),
        "应为 POST：{request_line}"
    );
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );
    assert!(server.last_body().is_empty(), "请求体应为空");
}

#[tokio::test]
async fn ocr_bank_card_and_driving() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/ocr/bankcard") {
            // 镜像 Java MockTest.testBankCard 的 returnJson
            r#"{"number":"24234234345234"}"#.to_string()
        } else if path.contains("/cv/ocr/driving") {
            // 镜像 Java MockTest.testDriving 的 returnJson（精简注释）
            r#"{"errcode":0,"errmsg":"ok","plate_num":"粤xxxxx","vehicle_type":"小型普通客车","owner":"东莞市xxxxx机械厂","img_size":{"w":3120,"h":4208}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let ocr = service.ocr_service().expect("OCR 服务存在");

    let bank: WxOcrBankCardResult = ocr
        .ocr_bank_card("http://img/x.jpg")
        .await
        .expect("银行卡识别成功");
    assert_eq!(bank.number, "24234234345234");
    let request_line = server.last_request_line();
    assert!(request_line.starts_with("POST "));
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );

    let driving: WxOcrDrivingResult = ocr
        .ocr_driving("http://img/x.jpg")
        .await
        .expect("行驶证识别成功");
    assert_eq!(driving.plate_num, "粤xxxxx");
    assert_eq!(driving.vehicle_type, "小型普通客车");
    assert_eq!(driving.img_size.w, 3120);
}

#[tokio::test]
async fn ocr_driving_license_and_comm() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/ocr/drivinglicense") {
            // 镜像 Java MockTest.testDrivingLicense 的 returnJson（精简注释）
            r#"{"errcode":0,"errmsg":"ok","id_num":"660601xxxxxxxx1234","name":"张三","sex":"男","nationality":"中国","car_class":"C1","valid_from":"2018-07-06","valid_to":"2020-07-01","official_seal":"xx市公安局公安交通管理局"}"#.to_string()
        } else if path.contains("/cv/ocr/comm") {
            // 镜像 Java MockTest.testComm 的 returnJson（精简注释）
            r#"{"errcode":0,"errmsg":"ok","items":[{"text":"腾讯"},{"text":"微信团队"}],"img_size":{"w":1280,"h":720}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let ocr = service.ocr_service().expect("OCR 服务存在");

    let license: WxOcrDrivingLicenseResult = ocr
        .ocr_driving_license("http://img/x.jpg")
        .await
        .expect("驾驶证识别成功");
    assert_eq!(license.id_num, "660601xxxxxxxx1234");
    assert_eq!(license.name, "张三");
    let request_line = server.last_request_line();
    assert!(request_line.starts_with("POST "));
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );

    let comm: WxOcrCommResult = ocr
        .ocr_comm("http://img/x.jpg")
        .await
        .expect("通用识别成功");
    assert_eq!(comm.items.len(), 2);
    assert_eq!(comm.items[0].text, "腾讯");
    assert_eq!(comm.img_size.w, 1280);
}

// ---- 图像处理（镜像 Java WxMpImgProcServiceImplTest.mockTest） ----

#[tokio::test]
async fn img_proc_qrcode_post() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/img/qrcode") {
            // 镜像 Java mockTest.testQrCode 的 returnJson（精简）
            r#"{"errcode":0,"errmsg":"ok","img_size":{"w":1000,"h":900},"code_results":[{"type_name":"QR_CODE","data":"https://www.qq.com","pos":{"left_top":{"x":585,"y":378},"right_top":{"x":828,"y":378},"right_bottom":{"x":828,"y":618},"left_bottom":{"x":585,"y":618}}}]}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let img_proc = service.img_proc_service().expect("图片处理服务存在");

    let result = img_proc
        .qrcode("http://img/x.jpg")
        .await
        .expect("二维码识别成功");
    // 响应解析（对应 Java `WxImgProcQrCodeResult.fromJson`）
    assert_eq!(result.img_size.w, 1000);
    assert_eq!(result.code_results.len(), 1);
    assert_eq!(result.code_results[0].type_name, "QR_CODE");
    assert_eq!(result.code_results[0].data, "https://www.qq.com");
    assert_eq!(result.code_results[0].pos.left_top.x, 585);

    // Java 语义：POST（`post(String.format(QRCODE, imgUrl), "")`）+ 编码 query + 空请求体
    let request_line = server.last_request_line();
    assert!(
        request_line.starts_with("POST "),
        "应为 POST：{request_line}"
    );
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );
    assert!(server.last_body().is_empty(), "请求体应为空");
}

#[tokio::test]
async fn img_proc_super_resolution_get() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/img/superresolution") {
            // 镜像 Java mockTest.testSuperResolution 的 returnJson
            r#"{"errcode":0,"errmsg":"ok","media_id":"6WXsIXkG7lXuDLspD9xfm5dsvHzb0EFl0li6ySxi92ap8Vl3zZoD9DpOyNudeJGB"}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let img_proc = service.img_proc_service().expect("图片处理服务存在");

    let result = img_proc
        .super_resolution("http://img/x.jpg")
        .await
        .expect("图片高清化成功");
    // 响应解析（对应 Java `WxImgProcSuperResolutionResult.fromJson`）
    assert_eq!(
        result.media_id,
        "6WXsIXkG7lXuDLspD9xfm5dsvHzb0EFl0li6ySxi92ap8Vl3zZoD9DpOyNudeJGB"
    );

    // Java 语义：GET（`get(String.format(SUPER_RESOLUTION, imgUrl), null)`）+ 编码 query
    let request_line = server.last_request_line();
    assert!(request_line.starts_with("GET "), "应为 GET：{request_line}");
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );
}

#[tokio::test]
async fn img_proc_ai_crop_post_with_ratios() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cv/img/aicrop") {
            // 镜像 Java mockTest.testAiCrop 的 returnJson（精简）
            r#"{"errcode":0,"errmsg":"ok","results":[{"crop_left":112,"crop_top":0,"crop_right":839,"crop_bottom":727}],"img_size":{"w":966,"h":728}}"#.to_string()
        } else {
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string()
        }
    }))
    .await;
    let service =
        wx_rust_miniapp::api::r#impl::WxMaServiceImpl::new_arc(config_with_host(&server.url("")));
    let img_proc = service.img_proc_service().expect("图片处理服务存在");

    // ratio 非 0：对应 Java `aiCrop(imgUrl, ratios)`，ratios 以十进制字符串直填
    let result = img_proc
        .ai_crop("http://img/x.jpg", 2.35)
        .await
        .expect("智能裁剪成功");
    // 响应解析（对应 Java `WxImgProcAiCropResult.fromJson`）
    assert_eq!(result.img_size.w, 966);
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].crop_left, 112);
    assert_eq!(result.results[0].crop_bottom, 727);

    // Java 语义：POST + 空请求体；img_url 编码、ratios 不编码（String.format 直填）
    let request_line = server.last_request_line();
    assert!(
        request_line.starts_with("POST "),
        "应为 POST：{request_line}"
    );
    assert_eq!(
        query_param(&request_line, "img_url").as_deref(),
        Some(ENCODED_IMG_URL)
    );
    assert_eq!(
        query_param(&request_line, "ratios").as_deref(),
        Some("2.35")
    );
    assert!(server.last_body().is_empty(), "请求体应为空");

    // ratio 0.0：对应 Java `aiCrop(imgUrl)` 默认空 ratios（`StringUtils.isEmpty` → ""）
    let result = img_proc
        .ai_crop("http://img/x.jpg", 0.0)
        .await
        .expect("默认比例智能裁剪成功");
    assert_eq!(result.results[0].crop_top, 0);
    let request_line = server.last_request_line();
    assert_eq!(query_param(&request_line, "ratios").as_deref(), Some(""));
}
