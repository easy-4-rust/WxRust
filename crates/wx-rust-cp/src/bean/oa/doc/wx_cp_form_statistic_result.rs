//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpFormStatisticResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFormStatisticResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "statistic_list", default)]
    pub statistic_list: Vec<crate::bean::oa::doc::wx_cp_form_statistic::WxCpFormStatistic>,
}

impl WxCpFormStatisticResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFormStatisticResult 解析失败: {e}"))
    }
}
