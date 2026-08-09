//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpExternalUserIdList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExternalUserIdList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "external_userid_info", default)]
    pub external_user_id_info: Vec<ExternalUserIdInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalUserIdInfo {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "new_external_userid", default)]
    pub new_external_user_id: String,
}

impl WxCpExternalUserIdList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpExternalUserIdList 解析失败: {e}"))
    }
}
