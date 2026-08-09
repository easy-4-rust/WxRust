//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpOpenKfIdConvertResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpOpenKfIdConvertResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "items", default)]
    pub items: Vec<Item>,
    #[serde(rename = "invalid_open_kfid_list", default)]
    pub invalid_open_kf_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "open_kfid", default)]
    pub open_kf_id: String,
    #[serde(rename = "new_open_kfid", default)]
    pub new_open_kf_id: String,
}

impl WxCpTpOpenKfIdConvertResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpOpenKfIdConvertResult 解析失败: {e}"))
    }
}
