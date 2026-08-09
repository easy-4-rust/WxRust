//! 对应 Java `bean.result.WxMpSemanticQueryResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpSemanticQueryResult {
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "semantic", default)]
    pub semantic: String,
    #[serde(rename = "result", default)]
    pub result: String,
    #[serde(rename = "answer", default)]
    pub answer: String,
    #[serde(rename = "text", default)]
    pub text: String,
}
