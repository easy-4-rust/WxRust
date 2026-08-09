//! 对应 Java `me.chanjar.weixin.cp.bean.external.interceptrule.WxCpInterceptRuleList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpInterceptRuleList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "rule_list", default)]
    pub rule_list: Vec<Rule>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    #[serde(rename = "rule_id", default)]
    pub rule_id: String,
    #[serde(rename = "rule_name", default)]
    pub rule_name: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}

impl WxCpInterceptRuleList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpInterceptRuleList 解析失败: {e}"))
    }
}
