//! 对应 Java `bean.guide.WxMpGuideBuyerInfoList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideBuyerInfoList {
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
    #[serde(rename = "list", default)]
    pub list: Vec<WxMpGuideBuyerInfo>,
}
