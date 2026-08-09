//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfGetServicerStatisticRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfGetServicerStatisticRequest {
    #[serde(rename = "open_kfid", default)]
    pub open_kf_id: String,
    #[serde(rename = "servicer_userid", default)]
    pub servicer_userid: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}
