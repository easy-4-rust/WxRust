//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpConvertTmpExternalUserIdResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpConvertTmpExternalUserIdResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "invalid_tmp_external_userid_list", default)]
    pub results: Vec<Results>,
    #[serde(rename = "invalid_tmp_external_userid_list", default)]
    pub invalid_tmp_external_user_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Results {
    #[serde(rename = "tmp_external_userid", default)]
    pub tmp_external_user_id: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

impl WxCpTpConvertTmpExternalUserIdResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpTpConvertTmpExternalUserIdResult 解析失败: {e}"))
    }
}
