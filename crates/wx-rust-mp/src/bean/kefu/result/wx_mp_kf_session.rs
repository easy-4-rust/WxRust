//! 对应 Java `bean.kefu.result.WxMpKfSession`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfSession {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "createtime", default)]
    pub create_time: i64,
    #[serde(rename = "latest_time", default)]
    pub latest_time: i64,
    #[serde(rename = "openid", default)]
    pub openid: String,
}
