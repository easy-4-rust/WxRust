//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentWindowProductListParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductListParam {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
