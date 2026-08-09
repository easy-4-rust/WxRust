//! 对应 Java `bean.kefu.request.WxMpKfSessionRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfSessionRequest {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
}
