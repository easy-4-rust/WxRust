//! 对应 Java `me.chanjar.weixin.common.bean.oauth2.WxOAuth2AccessToken`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOAuth2AccessToken {
    /// accessToken
    #[serde(rename = "access_token", default)]
    pub access_token: String,
    /// expiresIn
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,
    /// refreshToken
    #[serde(rename = "refresh_token", default)]
    pub refresh_token: String,
    /// openId
    #[serde(rename = "openid", default)]
    pub open_id: String,
    /// scope
    #[serde(rename = "scope", default)]
    pub scope: String,
    /// snapshotUser
    #[serde(rename = "is_snapshotuser", default)]
    pub snapshot_user: i32,
    /// unionId
    #[serde(rename = "unionid", default)]
    pub union_id: String,
}
