//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenIcpVerifyTaskResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenIcpVerifyTaskResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "is_finish", default)]
    pub finish: bool,
    #[serde(rename = "face_status", default)]
    pub face_status: i32,
    #[serde(rename = "along_with_auth_result", default)]
    pub along_with_auth_result: i32,
}
