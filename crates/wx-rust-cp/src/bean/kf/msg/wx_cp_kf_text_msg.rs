//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfTextMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfTextMsg {
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "menu_id", default)]
    pub menu_id: String,
}
