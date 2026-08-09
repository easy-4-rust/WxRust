//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfBusinessCardMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfBusinessCardMsg {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}
