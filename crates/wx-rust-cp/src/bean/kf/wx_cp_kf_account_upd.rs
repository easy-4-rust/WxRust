//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfAccountUpd.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfAccountUpd {
    #[serde(rename = "open_kfid", default)]
    pub open_kfid: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
}
