//! 对应 Java `me.chanjar.weixin.cp.bean.external.msg.Location.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(rename = "latitude", default)]
    pub latitude: String,
    #[serde(rename = "longitude", default)]
    pub longitude: String,
    #[serde(rename = "name", default)]
    pub name: String,
}
