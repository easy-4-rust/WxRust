//! 对应 Java `me.chanjar.weixin.channel.bean.freight.AllFreightCalcMethod.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllFreightCalcMethod {
    #[serde(rename = "freight_calc_method_list", default)]
    pub list: Vec<FreightCalcMethod>,
}
