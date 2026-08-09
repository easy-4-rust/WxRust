//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfServiceUpgradeConfigResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfServiceUpgradeConfigResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "member_range", default)]
    pub member_range: MemberRange,
    #[serde(rename = "groupchat_range", default)]
    pub groupchat_range: GroupchatRange,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberRange {
    #[serde(rename = "userid_list", default)]
    pub userid_list: Vec<String>,
    #[serde(rename = "department_id_list", default)]
    pub department_id_list: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupchatRange {
    #[serde(rename = "chat_id_list", default)]
    pub chat_id_list: Vec<String>,
}

impl WxCpKfServiceUpgradeConfigResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpKfServiceUpgradeConfigResp 解析失败: {e}"))
    }
}
