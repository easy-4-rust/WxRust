//! 对应 Java `bean.kefu.request.WxMpKfAccountRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfAccountRequest {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "nickname", default)]
    pub nick_name: String,
    #[serde(rename = "invite_wx", default)]
    pub invite_wx: String,
}
