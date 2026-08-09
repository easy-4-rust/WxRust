//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaOrderShippingIsTradeManagedRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingIsTradeManagedRequest {
    #[serde(rename = "appid", default)]
    pub app_id: String,
}
