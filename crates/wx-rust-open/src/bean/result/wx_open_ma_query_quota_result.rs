//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaQueryQuotaResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaQueryQuotaResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "rest", default)]
    pub rest: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
    #[serde(rename = "speedup_rest", default)]
    pub speedup_rest: i32,
    #[serde(rename = "speedup_limit", default)]
    pub speedup_limit: i32,
}
