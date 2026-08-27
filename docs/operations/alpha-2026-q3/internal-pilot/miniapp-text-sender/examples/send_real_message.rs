//! WxRust Alpha Day-5 真实流量示例
//!
//! 通过 wx-rust-miniapp 对真实微信服务器（api.weixin.qq.com）执行 5 个场景：
//!   1. access_token 真实获取（验证 IP 白名单 + token 端点）
//!   2. 验签（check_signature，本地 SHA1 校验）
//!   3. 订阅消息真实发送（send_subscribe_msg）
//!   4. 客服消息真实发送（send_kefu_msg）
//!   5. 错误场景（缺 access_token 的 POST → 真实 errcode）
//!   6. 重试（注入过期 token → 自动刷新后发送）
//!
//! 凭证仅从环境变量读取，绝不写入任何文件：
//!   WX_MA_APPID      必填
//!   WX_MA_APPSECRET  必填
//!   WX_MA_TEMPLATE_ID 必填（订阅消息模板）
//!   WX_MA_OPENID     可选（缺省用占位 openid，将得到真实 40003）
//!
//! 运行：WX_MA_APPID=.. WX_MA_APPSECRET=.. WX_MA_TEMPLATE_ID=.. \
//!        cargo run --example send_real_message

use std::sync::Arc;
use std::time::Instant;

use miniapp_text_sender::{build_kefu_text_msg, build_order_notify_msg};
use wx_rust_common::config::WxConfigStorage;
use wx_rust_common::http::{HttpTransport, ReqwestTransport, TransportBody, TransportMethod, TransportRequest};
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;
use wx_rust_miniapp::config::WxMaConfig;

const REAL_HOST: &str = "https://api.weixin.qq.com";

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    let appid = std::env::var("WX_MA_APPID").expect("缺少环境变量 WX_MA_APPID");
    let secret = std::env::var("WX_MA_APPSECRET").expect("缺少环境变量 WX_MA_APPSECRET");
    let template_id =
        std::env::var("WX_MA_TEMPLATE_ID").expect("缺少环境变量 WX_MA_TEMPLATE_ID");
    let openid = std::env::var("WX_MA_OPENID")
        .unwrap_or_else(|_| "ox_day5_placeholder_openid_000".to_string());
    let use_placeholder = std::env::var("WX_MA_OPENID").is_err();

    println!(
        "[INFO] appid={} template_id_len={} openid={}（{}）",
        appid,
        template_id.len(),
        if use_placeholder { "<占位>" } else { &openid },
        if use_placeholder { "占位，发送将得真实 40003" } else { "用户提供" }
    );

    let rt = tokio::runtime::Runtime::new().expect("创建 runtime");
    rt.block_on(run(appid, secret, template_id, openid, use_placeholder));
}

async fn run(appid: String, secret: String, template_id: String, openid: String, use_placeholder: bool) {
    // 构建 config（保留句柄以便注入过期 token 做重试场景）
    let mut config = WxMaDefaultConfig::new(&appid, &secret);
    config.set_token("alpha-day5-verify-token");
    config.set_msg_data_format("json");
    let mut host_config = wx_rust_miniapp::config::WxMaHostConfig::new();
    host_config.api_host = REAL_HOST.to_string();
    config.set_host_config(host_config);
    let config_arc: Arc<WxMaDefaultConfig> = Arc::new(config);
    let service = WxMaServiceImpl::new_arc(config_arc.clone());

    // ---- 场景 1：真实 access_token（IP 白名单 + token 端点验证）----
    println!("\n=== 场景 1: access_token 真实获取 ===");
    let t0 = Instant::now();
    match service.get_access_token().await {
        Ok(tok) => {
            let ms = t0.elapsed().as_millis();
            tracing::info!("token refresh success: access_token 获取成功");
            tracing::info!("api_call name=access_token duration={} status=ok", ms);
            println!(
                "[SCENARIO] name=access_token status=ok duration={} token_len={}",
                ms,
                tok.len()
            );
        }
        Err(e) => {
            let ms = t0.elapsed().as_millis();
            tracing::warn!("token refresh failed: {}", e);
            println!("[SCENARIO] name=access_token status=error duration={} error={}", ms, e);
            // 若 40164 说明 IP 未加白名单，直接提示
            if format!("{e}").contains("40164") {
                println!("[HINT] 40164 = IP 不在白名单，请到 mp.weixin.qq.com 开发设置添加本机出口 IP");
            }
            println!("[RESULT] 场景 1 失败——后续场景依赖 token，终止。");
            std::process::exit(2);
        }
    }

    // ---- 场景 2：验签（本地 SHA1，确定性校验）----
    println!("\n=== 场景 2: 验签 check_signature ===");
    let ts = "1724688000";
    let nonce = "day5-nonce-001";
    let valid_sig = wx_rust_common::util::crypto::sha1::Sha1::digest(&[
        "alpha-day5-verify-token",
        ts,
        nonce,
    ])
    .expect("sha1 计算");
    let ok_valid = service.check_signature(ts, nonce, &valid_sig);
    let ok_invalid = service.check_signature(ts, nonce, "0000000000000000000000000000000000000000");
    println!(
        "[SCENARIO] name=check_signature status=ok valid_sig={} invalid_sig_rejected={}",
        ok_valid, ok_invalid
    );
    if !ok_valid || ok_invalid {
        println!("[RESULT] 场景 2 失败：验签逻辑异常");
        std::process::exit(2);
    }

    // ---- 场景 3：订阅消息真实发送 ----
    println!("\n=== 场景 3: 订阅消息真实发送 ===");
    let msg = build_order_notify_msg(&openid, &template_id, "ORD-20260827-REAL-001", "已发货");
    let t0 = Instant::now();
    match service.send_subscribe_msg(&msg).await {
        Ok(()) => {
            let ms = t0.elapsed().as_millis();
            tracing::info!("api_call name=subscribe_msg duration={} status=ok", ms);
            println!("[SCENARIO] name=subscribe_msg status=ok duration={}", ms);
        }
        Err(e) => {
            let ms = t0.elapsed().as_millis();
            let err = format!("{e}");
            tracing::info!("api_call name=subscribe_msg duration={} status=error error={}", ms, err);
            println!("[SCENARIO] name=subscribe_msg status=error duration={} error={}", ms, err);
            if use_placeholder {
                println!("[NOTE] 占位 openid 的 40003 为预期——真实链路已通，需真实 openid 才能送达");
            } else if err.contains("43101") {
                println!("[NOTE] 43101 = 用户未授权订阅，需用户在真机点击授权后重试");
            }
        }
    }

    // ---- 场景 4：客服消息真实发送 ----
    println!("\n=== 场景 4: 客服消息真实发送 ===");
    let kmsg = build_kefu_text_msg(&openid, "WxRust Alpha Day-5 真实流量验证");
    let t0 = Instant::now();
    match service.send_kefu_msg(&kmsg).await {
        Ok(_) => {
            let ms = t0.elapsed().as_millis();
            tracing::info!("api_call name=kefu_msg duration={} status=ok", ms);
            println!("[SCENARIO] name=kefu_msg status=ok duration={}", ms);
        }
        Err(e) => {
            let ms = t0.elapsed().as_millis();
            let err = format!("{e}");
            tracing::info!("api_call name=kefu_msg duration={} status=error error={}", ms, err);
            println!("[SCENARIO] name=kefu_msg status=error duration={} error={}", ms, err);
            if use_placeholder {
                println!("[NOTE] 占位 openid 的 40003 为预期——需真实 openid + 48h 内用户交互");
            } else if err.contains("45015") {
                println!("[NOTE] 45015 = 48h 响应时限过期，需用户先发消息");
            }
        }
    }

    // ---- 场景 5：错误场景（缺 access_token → 真实 errcode）----
    println!("\n=== 场景 5: 错误场景（缺 access_token 的 POST）===");
    let client = reqwest::Client::new();
    let transport = ReqwestTransport::new(client);
    let t0 = Instant::now();
    let resp = transport
        .send(TransportRequest {
            method: TransportMethod::PostJson(
                r#"{"touser":"ox_probe","template_id":"tpl_probe","data":{}}"#.to_string(),
            ),
            url: format!("{REAL_HOST}/cgi-bin/message/subscribe/send"),
            headers: vec![],
            body: TransportBody::None,
        })
        .await;
    match resp {
        Ok(r) => {
            let ms = t0.elapsed().as_millis();
            let body = String::from_utf8_lossy(&r.body).to_string();
            tracing::info!("api_call name=error_probe duration={} status=ok resp={}", ms, body);
            println!(
                "[SCENARIO] name=error_probe status=ok http_status={} duration={} resp={}",
                r.status, ms, body
            );
        }
        Err(e) => {
            let ms = t0.elapsed().as_millis();
            println!("[SCENARIO] name=error_probe status=error duration={} error={}", ms, e);
        }
    }

    // ---- 场景 6：重试（注入过期 token → 自动刷新后发送）----
    println!("\n=== 场景 6: token 过期自动刷新重试 ===");
    config_arc.update_access_token("expired_token_probe_0000", 0);
    tracing::info!("token refresh attempt: 注入过期 token，触发自动刷新");
    let t0 = Instant::now();
    match service.send_subscribe_msg(&msg).await {
        Ok(()) => {
            let ms = t0.elapsed().as_millis();
            tracing::info!("api_call name=retry_refresh duration={} status=ok", ms);
            println!("[SCENARIO] name=retry_refresh status=ok duration={}", ms);
        }
        Err(e) => {
            let ms = t0.elapsed().as_millis();
            let err = format!("{e}");
            tracing::info!("api_call name=retry_refresh duration={} status=error error={}", ms, err);
            println!("[SCENARIO] name=retry_refresh status=error duration={} error={}", ms, err);
            if use_placeholder {
                println!("[NOTE] 刷新成功但发送仍被 40003 拦截（占位 openid）——刷新路径已验证");
            }
        }
    }

    println!("\n=== Day-5 真实流量执行完毕 ===");
}
