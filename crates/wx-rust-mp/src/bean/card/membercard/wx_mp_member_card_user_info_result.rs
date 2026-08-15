//! 对应 Java `bean.card.membercard.WxMpMemberCardUserInfoResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMemberCardUserInfoResult {
    #[serde(rename = "errorCode", default)]
    pub error_code: String,
    #[serde(rename = "errorMsg", default)]
    pub error_msg: String,
    #[serde(rename = "openId", default)]
    pub open_id: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "membershipNumber", default)]
    pub membership_number: String,
    #[serde(rename = "bonus", default)]
    pub bonus: i32,
    #[serde(rename = "balance", default)]
    pub balance: f64,
    #[serde(rename = "sex", default)]
    pub sex: String,
    #[serde(rename = "userInfo", default)]
    pub user_info: MemberCardUserInfo,
    #[serde(rename = "userCardStatus", default)]
    pub user_card_status: String,
    #[serde(rename = "hasActive", default)]
    pub has_active: bool,
}

impl WxMpMemberCardUserInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpMemberCardUserInfoResult 解析失败: {e}"))
    }
}
