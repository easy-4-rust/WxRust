use serde::{Deserialize, Serialize};

/// 虚拟支付微信订单信息。
/// 对应 Java: cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayWeChatPayInfo
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaXPayWeChatPayInfo {
    /// 微信支付订单号。
    #[serde(rename = "MchOrderNo", skip_serializing_if = "Option::is_none")]
    pub mch_order_no: Option<String>,
}
