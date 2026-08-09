//! 对应 Java `bean.kefu.result.WxMpKfMsgRecord`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfMsgRecord {
    #[serde(rename = "worker", default)]
    pub worker: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "opercode", default)]
    pub operate_code: i32,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "time", default)]
    pub time: i64,
}
