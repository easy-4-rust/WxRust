//! 对应 Java `com.binarywang.wxjava.store.bean.freight.TemplateAddParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FreightTemplate;

/// 微信小店 TemplateAddParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.TemplateAddParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateAddParam {
    #[serde(rename = "freight_template", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<FreightTemplate>,
}
