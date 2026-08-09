//! 对应 Java `me.chanjar.weixin.open.bean.result.WxFastMaCheckNickameResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaCheckNickameResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "hit_condition", default)]
    pub hit_condition: bool,
    #[serde(rename = "wording", default)]
    pub wording: String,
}
