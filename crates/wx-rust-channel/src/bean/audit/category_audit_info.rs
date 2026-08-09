//! 对应 Java `me.chanjar.weixin.channel.bean.audit.CategoryAuditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryAuditInfo {
    #[serde(rename = "level1", default)]
    pub level1: i64,
    #[serde(rename = "level2", default)]
    pub level2: i64,
    #[serde(rename = "level3", default)]
    pub level3: i64,
    #[serde(rename = "cats_v2", default)]
    pub cats_v2: Vec<CatsV2>,
    #[serde(rename = "certificate", default)]
    pub certificates: Vec<String>,
    #[serde(rename = "baobeihan", default)]
    pub baobeihan: Vec<String>,
    #[serde(rename = "jingyingzhengming", default)]
    pub jingyingzhengming: Vec<String>,
    #[serde(rename = "daihuokoubei", default)]
    pub daihuokoubei: Vec<String>,
    #[serde(rename = "ruzhuzhizhi", default)]
    pub ruzhuzhizhi: Vec<String>,
    #[serde(rename = "jingyingliushui", default)]
    pub jingyingliushui: Vec<String>,
    #[serde(rename = "buchongcailiao", default)]
    pub buchongcailiao: Vec<String>,
    #[serde(rename = "jingyingpingtai", default)]
    pub jingyingpingtai: String,
    #[serde(rename = "zhanghaomingcheng", default)]
    pub zhanghaomingcheng: String,
    #[serde(rename = "brand_list", default)]
    pub brand_list: Vec<CategoryBrand>,
}
