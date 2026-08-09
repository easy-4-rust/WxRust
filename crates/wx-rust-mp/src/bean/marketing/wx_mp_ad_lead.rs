//! 对应 Java `bean.marketing.WxMpAdLead`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpAdLead {
    #[serde(rename = "click_id", default)]
    pub click_id: String,
    #[serde(rename = "adgroup_id", default)]
    pub adgroup_id: i64,
    #[serde(rename = "adgroup_name", default)]
    pub adgroup_name: String,
    #[serde(rename = "campaign_id", default)]
    pub campaign_id: i64,
    #[serde(rename = "campaign_name", default)]
    pub campaign_name: String,
    #[serde(rename = "agency_id", default)]
    pub agency_id: String,
    #[serde(rename = "agency_name", default)]
    pub agency_name: String,
    #[serde(rename = "leads_info", default)]
    pub leads_info: Vec<WxMpAdLeadInfo>,
}
