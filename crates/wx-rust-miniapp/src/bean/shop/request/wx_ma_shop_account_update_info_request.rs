//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAccountUpdateInfoRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAccountUpdateInfoRequest {
    #[serde(rename = "service_agent_path", default)]
    pub service_agent_path: String,
    #[serde(rename = "service_agent_phone", default)]
    pub service_agent_phone: String,
}
