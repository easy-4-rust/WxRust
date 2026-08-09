//! 对应 Java `me.chanjar.weixin.common.bean.WxOAuth2UserInfo`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOAuth2UserInfo {
    /// openid
    #[serde(rename = "openid", default)]
    pub openid: String,
    /// nickname
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    /// sex
    #[serde(rename = "sex", default)]
    pub sex: i32,
    /// city
    #[serde(rename = "city", default)]
    pub city: String,
    /// province
    #[serde(rename = "province", default)]
    pub province: String,
    /// country
    #[serde(rename = "country", default)]
    pub country: String,
    /// headImgUrl
    #[serde(rename = "headimgurl", default)]
    pub head_img_url: String,
    /// unionId
    #[serde(rename = "unionid", default)]
    pub union_id: String,
    /// privileges
    #[serde(rename = "privilege", default)]
    pub privileges: Vec<String>,
}
