//! 对应 Java `bean.datacube.WxDataCubeMsgResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDataCubeMsgResult {
    #[serde(rename = "ref_hour", default)]
    pub ref_hour: i32,
    #[serde(rename = "msg_type", default)]
    pub msg_type: i32,
    #[serde(rename = "msg_user", default)]
    pub msg_user: i32,
    #[serde(rename = "msg_count", default)]
    pub msg_count: i32,
    #[serde(rename = "count_interval", default)]
    pub count_interval: i32,
    #[serde(rename = "int_page_read_count", default)]
    pub int_page_read_count: i32,
    #[serde(rename = "ori_page_read_user", default)]
    pub ori_page_read_user: i32,
}
