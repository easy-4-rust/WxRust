//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaVisitTrend.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVisitTrend {
    #[serde(rename = "refDate", alias = "ref_date", default)]
    pub ref_date: String,
    #[serde(rename = "sessionCnt", alias = "session_cnt", default)]
    pub session_cnt: i64,
    #[serde(rename = "visitPv", alias = "visit_pv", default)]
    pub visit_pv: i64,
    #[serde(rename = "visitUv", alias = "visit_uv", default)]
    pub visit_uv: i64,
    #[serde(rename = "visitUvNew", alias = "visit_uv_new", default)]
    pub visit_uv_new: i64,
    #[serde(rename = "stayTimeUv", alias = "stay_time_uv", default)]
    pub stay_time_uv: f32,
    #[serde(rename = "stayTimeSession", alias = "stay_time_session", default)]
    pub stay_time_session: f32,
    #[serde(rename = "visitDepth", alias = "visit_depth", default)]
    pub visit_depth: f32,
}
