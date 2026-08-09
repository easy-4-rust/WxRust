//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaShowItemResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaShowItemResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "is_open", default)]
    pub is_open: i32,
    #[serde(rename = "can_open", default)]
    pub can_open: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "headimg", default)]
    pub headimg: String,
}
