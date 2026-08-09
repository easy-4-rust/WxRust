//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpNewExternalUserIdList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpNewExternalUserIdList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "items", default)]
    pub items: Vec<NewExternalUserIdInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewExternalUserIdInfo {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "new_external_userid", default)]
    pub new_external_user_id: String,
}

impl WxCpNewExternalUserIdList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpNewExternalUserIdList 解析失败: {e}"))
    }
}
