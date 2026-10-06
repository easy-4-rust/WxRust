use wx_rust_miniapp::bean::xpay::{WxMaXPayRequestVirtualPaymentRequest, WxMaXPaySigParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例只做本地计算。生产环境从服务端密钥存储读取，不能把密钥传给前端。
    let keys = WxMaXPaySigParams {
        app_key: "demo_app_key".into(),
        session_key: "demo_session_key".into(),
    };
    let request = WxMaXPayRequestVirtualPaymentRequest {
        offer_id: Some("demo_offer".into()),
        buy_quantity: Some(1),
        env: Some(1),
        currency_type: Some("CNY".into()),
        product_id: Some("episode_1".into()),
        goods_price: Some(100),
        out_trade_no: Some("demo_order_1".into()),
        attach: None,
    };
    let frontend_data = request.create_pay_data(&keys)?;
    assert_eq!(frontend_data.mode, "short_series_goods");
    // 将序列化结果交给前端 requestVirtualPayment；不要再重新序列化 signData。
    let _json = serde_json::to_string(&frontend_data)?;
    Ok(())
}
