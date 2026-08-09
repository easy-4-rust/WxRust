//! 对应 Java `bean.guide.WxMpGuideMassed`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideMassed {
    #[serde(rename = "task_id", default)]
    pub task_id: i64,
    #[serde(rename = "openid", default)]
    pub list: Vec<String>,
}
