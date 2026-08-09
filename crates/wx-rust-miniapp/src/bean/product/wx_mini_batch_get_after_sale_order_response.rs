//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMiniBatchGetAfterSaleOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniBatchGetAfterSaleOrderResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "after_sale_order_list", default)]
    pub after_sale_order_list: Vec<WxMiniAfterSaleOrder>,
}
