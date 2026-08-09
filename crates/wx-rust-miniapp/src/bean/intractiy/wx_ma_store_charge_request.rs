//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaStoreChargeRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaStoreChargeRequest {
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "payMode", default)]
    pub pay_mode: PayMode,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
}
