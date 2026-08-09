//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpTagIdListConvertResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpTagIdListConvertResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "items", default)]
    pub items: Vec<Item>,
    #[serde(rename = "invalid_external_tagid_list", default)]
    pub invalid_external_tag_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "external_tagid", default)]
    pub external_tag_id: String,
    #[serde(rename = "open_external_tagid", default)]
    pub open_external_tag_id: String,
}

impl WxCpTpTagIdListConvertResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpTpTagIdListConvertResult 解析失败: {e}"))
    }
}
