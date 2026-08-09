//! 对应 Java `me.chanjar.weixin.cp.bean.external.moment.VisibleRange.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VisibleRange {
    #[serde(rename = "sender_list", default)]
    pub sender_list: crate::bean::external::moment::sender_list::SenderList,
    #[serde(rename = "external_contact_list", default)]
    pub external_contact_list:
        crate::bean::external::moment::external_contact_list::ExternalContactList,
}
