//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfAccountListResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfAccountListResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "account_list", default)]
    pub account_list: Vec<AccountListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountListDTO {
    #[serde(rename = "open_kfid", default)]
    pub open_kfid: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
    #[serde(rename = "manage_privilege", default)]
    pub has_manage_privilege: bool,
}

impl WxCpKfAccountListResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfAccountListResp 解析失败: {e}"))
    }
}
