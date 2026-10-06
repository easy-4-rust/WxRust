//! 对应 Java `com.binarywang.wxjava.store.bean.audit.CategoryAuditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{CategoryBrand, CatsV2};

/// 微信小店 CategoryAuditInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.audit.CategoryAuditInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryAuditInfo {
    #[serde(rename = "level1", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1: Option<i64>,
    #[serde(rename = "level2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<i64>,
    #[serde(rename = "level3", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<i64>,
    #[serde(rename = "cats_v2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cats_v2: Option<Vec<CatsV2>>,
    #[serde(rename = "certificate", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<String>>,
    #[serde(rename = "baobeihan", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baobeihan: Option<Vec<String>>,
    #[serde(rename = "jingyingzhengming", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jingyingzhengming: Option<Vec<String>>,
    #[serde(rename = "daihuokoubei", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daihuokoubei: Option<Vec<String>>,
    #[serde(rename = "ruzhuzhizhi", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruzhuzhizhi: Option<Vec<String>>,
    #[serde(rename = "jingyingliushui", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jingyingliushui: Option<Vec<String>>,
    #[serde(rename = "buchongcailiao", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buchongcailiao: Option<Vec<String>>,
    #[serde(rename = "jingyingpingtai", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jingyingpingtai: Option<String>,
    #[serde(rename = "zhanghaomingcheng", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zhanghaomingcheng: Option<String>,
    #[serde(rename = "brand_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_list: Option<Vec<CategoryBrand>>,
}
