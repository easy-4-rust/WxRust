//! 对应 Java `me.chanjar.weixin.cp.bean.corpgroup.WxCpCorpGroupCorpGetTokenReq.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCorpGroupCorpGetTokenReq {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "business_type", default)]
    pub business_type: i32,
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
}
