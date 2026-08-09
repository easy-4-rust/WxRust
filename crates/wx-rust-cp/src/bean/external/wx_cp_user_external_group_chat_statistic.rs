//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserExternalGroupChatStatistic.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalGroupChatStatistic {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "total", default)]
    pub total: i32,
    #[serde(rename = "next_offset", default)]
    pub next_offset: i32,
    #[serde(rename = "items", default)]
    pub item_list: Vec<StatisticItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatisticItem {
    #[serde(rename = "owner", default)]
    pub owner: String,
    #[serde(rename = "data", default)]
    pub item_data: crate::bean::external::wx_cp_user_external_group_chat_statistic::ItemData,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemData {
    #[serde(rename = "new_chat_cnt", default)]
    pub new_chat_cnt: i32,
    #[serde(rename = "chat_total", default)]
    pub chat_total: i32,
    #[serde(rename = "chat_has_msg", default)]
    pub chat_has_msg: i32,
    #[serde(rename = "new_member_cnt", default)]
    pub new_member_cnt: i32,
    #[serde(rename = "member_total", default)]
    pub member_total: i32,
    #[serde(rename = "member_has_msg", default)]
    pub member_has_msg: i32,
    #[serde(rename = "msg_total", default)]
    pub msg_total: i32,
}

impl WxCpUserExternalGroupChatStatistic {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpUserExternalGroupChatStatistic 解析失败: {e}"))
    }
}
