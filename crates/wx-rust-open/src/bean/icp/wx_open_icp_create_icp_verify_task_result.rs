//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenIcpCreateIcpVerifyTaskResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenIcpCreateIcpVerifyTaskResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "task_id", default)]
    pub task_id: String,
    #[serde(rename = "verify_url", default)]
    pub verify_url: String,
}
