//! 微信用户信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpUser`。线格式由
//! `WxMpUserGsonAdapter` 决定：`openid`/`headimgurl`/`unionid`/`tagid_list`/`privilege` 等。

use serde::{Deserialize, Serialize};

/// 从 0/1 或 true/false 解析布尔（对应 Java `WxBooleanTypeAdapter`）。
pub fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(v.map(|v| match v {
        serde_json::Value::Bool(b) => b,
        serde_json::Value::Number(n) => n.as_i64().map(|i| i != 0).unwrap_or(false),
        serde_json::Value::String(s) => !s.is_empty() && s != "0",
        _ => false,
    }))
}

/// 微信用户信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpUser {
    /// 用户是否订阅该公众号标识（0/1）。
    #[serde(default, deserialize_with = "deserialize_bool_from_int")]
    pub subscribe: Option<bool>,
    /// 用户的标识，对当前公众号唯一。
    #[serde(rename = "openid", default)]
    pub open_id: String,
    /// 昵称（2021年12月27日之后不再输出）。
    #[serde(default)]
    pub nickname: String,
    /// 用户的语言，简体中文为 zh_CN。
    #[serde(default)]
    pub language: String,
    /// 用户头像（2021年12月27日之后不再输出）。
    #[serde(rename = "headimgurl", default)]
    pub head_img_url: String,
    /// 用户关注时间（Unix 秒）。
    #[serde(rename = "subscribe_time", default)]
    pub subscribe_time: Option<i64>,
    /// 用户统一标识（绑定开放平台账号后出现）。
    #[serde(rename = "unionid", default)]
    pub union_id: String,
    /// 公众号运营者对粉丝的备注。
    #[serde(default)]
    pub remark: String,
    /// 用户所在的分组 ID。
    #[serde(rename = "groupid", default)]
    pub group_id: Option<i32>,
    /// 用户被打上的标签 ID 列表。
    #[serde(rename = "tagid_list", default)]
    pub tag_ids: Vec<i64>,
    /// 用户特权信息数组。
    #[serde(rename = "privilege", default)]
    pub privileges: Vec<String>,
    /// 用户关注的渠道来源。
    #[serde(rename = "subscribe_scene", default)]
    pub subscribe_scene: String,
    /// 二维码扫码场景（开发者自定义）。
    #[serde(rename = "qr_scene", default)]
    pub qr_scene: String,
    /// 二维码扫码场景描述（开发者自定义）。
    #[serde(rename = "qr_scene_str", default)]
    pub qr_scene_str: String,
}

impl WxMpUser {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("用户信息解析失败: {e}"))
    }

    /// 从批量获取用户信息 JSON 构建列表（对应 Java `fromJsonList`，取 `user_info_list`）。
    pub fn from_json_list(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("用户信息列表解析失败: {e}"))?;
        let list = value
            .get("user_info_list")
            .ok_or_else(|| "缺少 user_info_list 字段".to_string())?;
        serde_json::from_value(list.clone()).map_err(|e| format!("用户信息列表解析失败: {e}"))
    }
}
