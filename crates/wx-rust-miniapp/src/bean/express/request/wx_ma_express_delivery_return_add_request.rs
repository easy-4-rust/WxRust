//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressDeliveryReturnAddRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressDeliveryReturnAddRequest {
    #[serde(rename = "shop_order_id", default)]
    pub shop_order_id: String,
    #[serde(rename = "biz_addr", default)]
    pub biz_addr: WxMaExpressOrderPerson,
    #[serde(rename = "user_addr", default)]
    pub user_addr: WxMaExpressOrderPerson,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "order_path", default)]
    pub order_path: String,
    #[serde(rename = "goods_list", default)]
    pub goods_list: Vec<WxMaExpressReturnOrder>,
    #[serde(rename = "order_price", default)]
    pub order_price: i32,
}

impl WxMaExpressDeliveryReturnAddRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaExpressDeliveryReturnAddRequest 序列化失败: {e}"))
    }
}
