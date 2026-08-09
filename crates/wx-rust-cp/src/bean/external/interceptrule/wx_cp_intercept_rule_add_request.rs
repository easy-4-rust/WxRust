//! 对应 Java `me.chanjar.weixin.cp.bean.external.interceptrule.WxCpInterceptRuleAddRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpInterceptRuleAddRequest {
    #[serde(rename = "rule_name", default)]
    pub rule_name: String,
    #[serde(rename = "rule_id", default)]
    pub rule_id: String,
    #[serde(rename = "word_list", default)]
    pub word_list: Vec<String>,
    #[serde(rename = "semantics_list", default)]
    pub semantics_list: Vec<i32>,
    #[serde(rename = "intercept_type", default)]
    pub intercept_type: i32,
    #[serde(rename = "applicable_range", default)]
    pub applicable_range: crate::bean::external::interceptrule::applicable_range::ApplicableRange,
}

impl WxCpInterceptRuleAddRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpInterceptRuleAddRequest 解析失败: {e}"))
    }
}

impl WxCpInterceptRuleAddRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpInterceptRuleAddRequest 序列化失败: {e}"))
    }
}
