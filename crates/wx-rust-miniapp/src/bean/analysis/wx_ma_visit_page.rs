//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaVisitPage.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVisitPage {
    #[serde(rename = "pagePath", alias = "page_path", default)]
    pub page_path: String,
    #[serde(rename = "pageVisitPv", alias = "page_visit_pv", default)]
    pub page_visit_pv: i64,
    #[serde(rename = "pageVisitUv", alias = "page_visit_uv", default)]
    pub page_visit_uv: i64,
    #[serde(rename = "pageStayTimePv", alias = "page_staytime_pv", default)]
    pub page_stay_time_pv: f32,
    #[serde(rename = "entryPagePv", alias = "entrypage_pv", default)]
    pub entry_page_pv: i64,
    #[serde(rename = "exitPagePv", alias = "exitpage_pv", default)]
    pub exit_page_pv: i64,
    #[serde(rename = "pageSharePv", alias = "page_share_pv", default)]
    pub page_share_pv: i64,
    #[serde(rename = "pageShareUv", alias = "page_share_uv", default)]
    pub page_share_uv: i64,
}
