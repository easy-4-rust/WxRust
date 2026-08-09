//! 对应 Java `bean.result.WxMpChangeOpenid`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpChangeOpenid {
    #[serde(rename = "oriOpenid", default)]
    pub ori_openid: String,
    #[serde(rename = "newOpenid", default)]
    pub new_openid: String,
    #[serde(rename = "errMsg", default)]
    pub err_msg: String,
}
