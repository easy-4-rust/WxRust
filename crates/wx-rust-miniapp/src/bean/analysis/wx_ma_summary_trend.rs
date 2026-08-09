//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaSummaryTrend.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaSummaryTrend {
    #[serde(rename = "refDate", alias = "ref_date", default)]
    pub ref_date: String,
    #[serde(rename = "visitTotal", alias = "visit_total", default)]
    pub visit_total: i64,
    #[serde(rename = "sharePv", alias = "share_pv", default)]
    pub share_pv: i64,
    #[serde(rename = "shareUv", alias = "share_uv", default)]
    pub share_uv: i64,
}
