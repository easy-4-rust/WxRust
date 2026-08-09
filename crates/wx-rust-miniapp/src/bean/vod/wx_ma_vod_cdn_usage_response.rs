//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodCdnUsageResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodCdnUsageResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data_interval", default)]
    pub data_interval: i32,
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<DataItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataItem {
    #[serde(rename = "value", default)]
    pub value: i64,
    #[serde(rename = "time", default)]
    pub time: i64,
}
