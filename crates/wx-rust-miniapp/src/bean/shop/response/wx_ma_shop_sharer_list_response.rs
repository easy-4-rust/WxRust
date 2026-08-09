//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopSharerListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSharerListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "sharers", default)]
    pub sharers: Vec<SharerInfo>,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerInfo {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "invited_time", default)]
    pub invited_time: i64,
    #[serde(rename = "bind_time", default)]
    pub bind_time: i64,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
}
