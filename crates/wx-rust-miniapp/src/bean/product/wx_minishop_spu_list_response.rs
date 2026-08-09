//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopSpuListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopSpuListResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data", default)]
    pub data: serde_json::Value,
    #[serde(rename = "total_num", default)]
    pub total_num: i64,
    #[serde(rename = "spus", default)]
    pub spus: Vec<WxMinishopSpu>,
}
