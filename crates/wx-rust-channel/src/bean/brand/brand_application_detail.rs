//! 对应 Java `me.chanjar.weixin.channel.bean.brand.BrandApplicationDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandApplicationDetail {
    #[serde(rename = "acceptance_time", default)]
    pub acceptance_time: i64,
    #[serde(rename = "acceptance_certification", default)]
    pub acceptance_certification: Vec<String>,
    #[serde(rename = "acceptance_no", default)]
    pub acceptance_no: String,
}
