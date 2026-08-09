//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthQueryResultDispatchInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthQueryResultDispatchInfo {
    #[serde(rename = "provider", default)]
    pub provider: String,
    #[serde(rename = "contact", default)]
    pub contact: String,
    #[serde(rename = "dispatch_time", default)]
    pub dispatch_time: i64,
}
