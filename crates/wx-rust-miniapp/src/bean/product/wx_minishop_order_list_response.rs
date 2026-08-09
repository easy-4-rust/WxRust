//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopOrderListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopOrderListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "orders", default)]
    pub orders: Vec<WxMinishopOrderResult>,
    #[serde(rename = "total_num", default)]
    pub total_num: i64,
}
