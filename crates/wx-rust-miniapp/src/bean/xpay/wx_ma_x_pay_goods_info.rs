use serde::{Deserialize, Serialize};

/// 虚拟支付商品发货信息。
/// 对应 Java: cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayGoodsInfo
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaXPayGoodsInfo {
    /// 商品 ID。
    #[serde(rename = "ProductId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 商品数量。
    #[serde(rename = "Quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}
