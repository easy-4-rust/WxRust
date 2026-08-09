//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfResourceMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfResourceMsg {
    #[serde(rename = "media_id", default)]
    pub media_id: String,
}
