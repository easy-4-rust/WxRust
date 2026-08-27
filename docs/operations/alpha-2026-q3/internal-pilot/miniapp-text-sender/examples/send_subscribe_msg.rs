//! WxRust Alpha 示例：通过 wx-rust-miniapp 发送订阅消息
//!
//! 演示场景：内部通知服务发送订单状态变更的订阅消息。
//! 使用内置 MockServer 模拟微信 API，无需真实服务端。
//!
//! 运行：cargo run --example send_subscribe_msg

use miniapp_text_sender::{
    MockServer, build_order_notify_msg, build_service, wechat_dispatch,
};
use wx_rust_miniapp::api::WxMaService;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    // 启动 mock 服务器（模拟微信 API）
    let server = MockServer::start(wechat_dispatch(|path| {
        if path.contains("subscribe/send") {
            tracing::info!("[MockServer] 收到订阅消息发送请求: {path}");
            return (
                "application/json".to_string(),
                r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
            );
        }
        tracing::info!("[MockServer] 通用应答: {path}");
        (
            "application/json".to_string(),
            r#"{"errcode":0,"errmsg":"ok"}"#.to_string(),
        )
    }))
    .await;

    tracing::info!("MockServer 已启动: {}", server.base_url());

    // 构建小程序服务（指向 mock 服务器）
    let service = build_service(&server.base_url(), "wx_alpha_appid", "alpha_secret");
    tracing::info!("WxMaServiceImpl 构建完成");

    // 构造订阅消息：订单状态变更通知
    let msg = build_order_notify_msg(
        "ox_test_user_001",          // 接收者 openid
        "tpl_order_status_change",   // 模板 ID
        "ORD-20260827-ALPHA-001",    // 订单号
        "已发货，预计明天送达",        // 状态
    );
    tracing::info!("订阅消息已构造: {:?}", msg);

    // 发送订阅消息
    let start = std::time::Instant::now();
    match service.send_subscribe_msg(&msg).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            tracing::info!(
                "订阅消息发送成功! 耗时: {:?}, MockServer 总请求数: {}",
                elapsed,
                server.requests()
            );
            println!("[OK] 订阅消息发送成功, P99 延迟: {:?}", elapsed);
        }
        Err(e) => {
            tracing::error!("订阅消息发送失败: {e}");
            eprintln!("[FAIL] 发送失败: {e}");
            std::process::exit(1);
        }
    }

    // 发送第 2 条消息（验证 token 复用）
    let msg2 = build_order_notify_msg(
        "ox_test_user_002",
        "tpl_order_status_change",
        "ORD-20260827-ALPHA-002",
        "已签收",
    );
    let start2 = std::time::Instant::now();
    match service.send_subscribe_msg(&msg2).await {
        Ok(()) => {
            let elapsed = start2.elapsed();
            tracing::info!("第 2 条消息发送成功, 耗时: {:?}", elapsed);
            println!("[OK] 第 2 条消息发送成功 (token 复用), 延迟: {:?}", elapsed);
        }
        Err(e) => {
            eprintln!("[FAIL] 第 2 条消息发送失败: {e}");
        }
    }

    // 最终统计
    println!("\n=== Day-1 观察数据 ===");
    println!("MockServer 总请求数: {}", server.requests());
    println!("最后请求路径: {}", server.last_path());
    println!("最后请求 body: {}", server.last_body());
    println!("======================");
}
