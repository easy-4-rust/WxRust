//! 对应 Java `me.chanjar.weixin.cp.bean.external.moment.CustomerItem.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomerItem {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}
