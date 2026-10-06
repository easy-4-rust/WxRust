//! 对应 Java `com.binarywang.wxjava.store.bean.freight.TemplateListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::OffsetParam;

/// 微信小店 TemplateListParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.TemplateListParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateListParam {
    #[serde(rename = "offset", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
