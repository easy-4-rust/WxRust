//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaGrayReleasePlanResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaGrayReleasePlanResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "gray_release_plan", default)]
    pub gray_release_plan: GrayReleasePlanBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrayReleasePlanBean {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_timestamp", default)]
    pub create_timestamp: i64,
    #[serde(rename = "gray_percentage", default)]
    pub gray_percentage: i32,
}
