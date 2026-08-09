//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopSpuPageRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSpuPageRequest {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "start_create_time", default)]
    pub start_create_time: String,
    #[serde(rename = "end_create_time", default)]
    pub end_create_time: String,
    #[serde(rename = "start_update_time", default)]
    pub start_update_time: String,
    #[serde(rename = "end_update_time", default)]
    pub end_update_time: String,
    #[serde(rename = "need_edit_spu", default)]
    pub need_edit_spu: i32,
    #[serde(rename = "page", default)]
    pub page: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
}
