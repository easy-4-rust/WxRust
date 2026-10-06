//! 对应 Java `com.binarywang.wxjava.store.bean.freight.AllFreightCalcMethod.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FreightCalcMethod;

/// 微信小店 AllFreightCalcMethod 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.AllFreightCalcMethod.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllFreightCalcMethod {
    #[serde(rename = "freight_calc_method_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FreightCalcMethod>>,
}
