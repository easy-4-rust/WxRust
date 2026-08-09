//! 对应 Java `me.chanjar.weixin.cp.bean.corpgroup.WxCpCorpGroupCorpListAppShareInfoResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCorpGroupCorpListAppShareInfoResp {
    #[serde(rename = "ending", default)]
    pub ending: i32,
    #[serde(rename = "corp_list", default)]
    pub corp_list: Vec<crate::bean::corpgroup::wx_cp_corp_group_corp::WxCpCorpGroupCorp>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}
