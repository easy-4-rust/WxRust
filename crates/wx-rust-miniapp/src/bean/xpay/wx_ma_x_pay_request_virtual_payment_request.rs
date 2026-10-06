use serde::{Deserialize, Serialize};

/// 个人主体前端虚拟支付请求。
/// 对应 Java: cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayRequestVirtualPaymentRequest
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaXPayRequestVirtualPaymentRequest {
    /// 支付应用 ID。
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    /// 购买数量。
    #[serde(rename = "buyQuantity", skip_serializing_if = "Option::is_none")]
    pub buy_quantity: Option<i32>,
    /// 环境，个人主体使用正式环境 0。
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<i32>,
    /// 币种，人民币为 CNY。
    #[serde(rename = "currencyType", skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    /// 商品 ID。
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 商品价格，单位分。
    #[serde(rename = "goodsPrice", skip_serializing_if = "Option::is_none")]
    pub goods_price: Option<i32>,
    /// 商户订单号，8 至 32 字符，不以下划线开头。
    #[serde(rename = "outTradeNo", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 透传业务附加信息。
    #[serde(rename = "attach", skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
}

impl WxMaXPayRequestVirtualPaymentRequest {
    /// 根据应用密钥和会话密钥生成前端参数，返回序列化错误或支付数据。
    /// 对应 Java: WxMaXPayRequestVirtualPaymentRequest#createPayData
    pub fn create_pay_data(
        &self,
        sig_params: &super::WxMaXPaySigParams,
    ) -> Result<super::WxMaXPayRequestVirtualPaymentData, wx_rust_common::error::WxErrorException>
    {
        use wx_rust_common::util::SignUtils;
        // 同一份 UTF-8 JSON 同时用于双签名和前端传参，避免重新序列化。
        let sign_data = serde_json::to_string(self)
            .map_err(|e| wx_rust_common::error::WxErrorException::Serde(e.to_string()))?;
        let pay_sig = SignUtils::create_hmac_sha256_sign(
            &format!("requestVirtualPayment&{sign_data}"),
            &sig_params.app_key,
        )
        .to_lowercase();
        let signature =
            SignUtils::create_hmac_sha256_sign(&sign_data, &sig_params.session_key).to_lowercase();
        Ok(super::WxMaXPayRequestVirtualPaymentData {
            mode: "short_series_goods".into(),
            sign_data,
            pay_sig,
            signature,
        })
    }
}
