use wx_rust_miniapp::bean::xpay::{WxMaXPayRequestVirtualPaymentRequest, WxMaXPaySigParams};
use wx_rust_miniapp::message::WxMaMessage;

#[test]
fn virtual_payment_matches_upstream_utf8_signatures() {
    let request = WxMaXPayRequestVirtualPaymentRequest {
        offer_id: Some("1450019686".into()),
        buy_quantity: Some(1),
        env: Some(0),
        currency_type: Some("CNY".into()),
        product_id: Some("product_001".into()),
        goods_price: Some(100),
        out_trade_no: Some("order12345".into()),
        attach: Some("attach中文".into()),
    };
    let params = WxMaXPaySigParams {
        app_key: "app_key_123".into(),
        session_key: "session_key_123".into(),
    };
    let data = request.create_pay_data(&params).unwrap();
    assert_eq!(data.mode, "short_series_goods");
    assert_eq!(
        data.sign_data,
        r#"{"offerId":"1450019686","buyQuantity":1,"env":0,"currencyType":"CNY","productId":"product_001","goodsPrice":100,"outTradeNo":"order12345","attach":"attach中文"}"#
    );
    assert_eq!(
        data.pay_sig,
        "52b0abda3c933b0273d328b5c5102ee9ab9249309474ddcdf5e9ce0f80b23532"
    );
    assert_eq!(
        data.signature,
        "602f76d9cadf36c232f7f6c1faa5fe295b0f955d188049997d0df699c0619c86"
    );
    let output = serde_json::to_value(&data).unwrap();
    assert_eq!(output["signData"], data.sign_data);
    assert!(output.get("appKey").is_none());
    let without_attach = WxMaXPayRequestVirtualPaymentRequest {
        attach: None,
        ..request
    };
    assert!(
        !without_attach
            .create_pay_data(&params)
            .unwrap()
            .sign_data
            .contains("attach")
    );
}

#[test]
fn goods_notification_json_and_xml_preserve_nested_fields() {
    let json = r#"{"Event":"xpay_goods_deliver_notify","OutTradeNo":"order12345","WeChatPayInfo":{"MchOrderNo":"wx_order_123"},"GoodsInfo":{"ProductId":"product_001","Quantity":2},"RetryTimes":0}"#;
    let xml = "<xml><Event>xpay_goods_deliver_notify</Event><OutTradeNo>order12345</OutTradeNo><WeChatPayInfo><MchOrderNo>wx_order_123</MchOrderNo></WeChatPayInfo><GoodsInfo><ProductId>product_001</ProductId><Quantity>2</Quantity></GoodsInfo><RetryTimes>0</RetryTimes></xml>";
    for message in [
        WxMaMessage::from_json(json).unwrap(),
        WxMaMessage::from_xml(xml).unwrap(),
    ] {
        assert_eq!(message.out_trade_no.as_deref(), Some("order12345"));
        assert_eq!(
            message
                .we_chat_pay_info
                .as_ref()
                .unwrap()
                .mch_order_no
                .as_deref(),
            Some("wx_order_123")
        );
        assert_eq!(
            message.goods_info.as_ref().unwrap().product_id.as_deref(),
            Some("product_001")
        );
        assert_eq!(message.goods_info.as_ref().unwrap().quantity, Some(2));
        assert_eq!(message.retry_times, Some(0));
        let roundtrip = WxMaMessage::from_xml(&message.to_xml()).unwrap();
        assert_eq!(roundtrip.goods_info, message.goods_info);
        assert_eq!(roundtrip.we_chat_pay_info, message.we_chat_pay_info);
    }
    assert!(WxMaMessage::from_json("{}").unwrap().goods_info.is_none());
    assert!(
        WxMaMessage::from_xml("<xml><Event>old</Event></xml>")
            .unwrap()
            .out_trade_no
            .is_none()
    );
}
