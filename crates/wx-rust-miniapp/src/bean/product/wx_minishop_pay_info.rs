//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopPayInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopPayInfo {
    #[serde(rename = "pay_method", default)]
    pub pay_method: String,
    #[serde(rename = "prepay_id", default)]
    pub prepay_id: String,
    #[serde(rename = "prepay_time", default)]
    pub prepay_time: String,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "pay_time", default)]
    pub pay_time: String,
}
