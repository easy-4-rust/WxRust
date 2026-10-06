use serde::{Deserialize, Serialize};

/// 前端 requestVirtualPayment 的参数，不包含用于签名的密钥。
/// 对应 Java: cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayRequestVirtualPaymentData
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WxMaXPayRequestVirtualPaymentData {
    /// 支付模式。
    pub mode: String,
    /// 原始签名 JSON。
    pub sign_data: String,
    /// 应用密钥签名。
    pub pay_sig: String,
    /// 会话密钥签名。
    pub signature: String,
}
