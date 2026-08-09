//! 授权方基本信息。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.auth.WxOpenAuthorizerInfo`。
//! 由 `WxOpenAuthorizerInfoGsonAdapter` 驱动解析（snake_case 键、
//! `service_type_info`/`verify_type_info` 为 `{"id":N}` 扁平为 Integer、
//! `MiniProgramInfo` 键为大驼峰），与字段名直映不同，故人工迁移：
//! Rust 以 serde rename + 自定义反序列化函数表达同一线格式。

use std::collections::HashMap;

use serde::Deserialize;

/// 授权方基本信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenAuthorizerInfo {
    /// 授权方昵称。
    #[serde(rename = "nick_name", default)]
    pub nick_name: Option<String>,
    /// 授权方头像。
    #[serde(rename = "head_img", default)]
    pub head_img: Option<String>,
    /// 公众号类型（对应 Java `serviceTypeInfo`，`{"id":N}` 扁平化）。
    #[serde(rename = "service_type_info", default, deserialize_with = "de_id")]
    pub service_type_info: Option<i32>,
    /// 认证类型（对应 Java `verifyTypeInfo`，`{"id":N}` 扁平化）。
    #[serde(rename = "verify_type_info", default, deserialize_with = "de_id")]
    pub verify_type_info: Option<i32>,
    /// 授权方主体名称。
    #[serde(rename = "user_name", default)]
    pub user_name: Option<String>,
    /// 授权方主体名称（认证主体）。
    #[serde(rename = "principal_name", default)]
    pub principal_name: Option<String>,
    /// 业务信息（对应 Java `businessInfo`）。
    #[serde(rename = "business_info", default)]
    pub business_info: Option<HashMap<String, i32>>,
    /// 授权方账号的账号名称。
    #[serde(rename = "alias", default)]
    pub alias: Option<String>,
    /// 二维码图片的URL。
    #[serde(rename = "qrcode_url", default)]
    pub qrcode_url: Option<String>,
    /// 帐号状态：1 正常 / 14 已注销 / 16 已封禁 / 18 已告警 / 19 已冻结。
    #[serde(rename = "account_status", default)]
    pub account_status: Option<i32>,
    /// 账号介绍。
    #[serde(rename = "signature", default)]
    pub signature: Option<String>,
    /// 小程序信息（授权方为小程序时返回）。
    #[serde(rename = "MiniProgramInfo", default)]
    pub mini_program_info: Option<MiniProgramInfo>,
    /// 小程序注册方式（0 普通 / 2 复用公众号创建 / 6 法人扫脸 / 13 试用 /
    /// 15 联盟 / 16 个人 / 17 个人交易 / 19 试用转正 / 22 复用商户号 / 23 复用商户号转正）。
    #[serde(rename = "register_type", default)]
    pub register_type: Option<i32>,
    /// 小程序基础配置信息。
    #[serde(rename = "basic_config", default)]
    pub basic_config: Option<BasicConfig>,
}

/// 解析 `{"id": N}` 或裸数值 N → `Option<i32>`
/// （对应 Java adapter 的 service_type_info/verify_type_info 扁平化）。
fn de_id<'de, D>(d: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: serde_json::Value =
        serde_json::Value::deserialize(d).map_err(serde::de::Error::custom)?;
    Ok(match v {
        serde_json::Value::Object(m) => m.get("id").and_then(|x| x.as_i64()).map(|x| x as i32),
        serde_json::Value::Number(n) => n.as_i64().map(|x| x as i32),
        _ => None,
    })
}

/// 小程序信息（对应 Java 内嵌类 `WxOpenAuthorizerInfo.MiniProgramInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgramInfo {
    /// 小程序访问状态。
    #[serde(rename = "visit_status", default)]
    pub visit_status: Option<i32>,
    /// 小程序已设置的各个服务器域名。
    #[serde(default)]
    pub network: Option<Network>,
    /// 小程序已设置的类目。
    #[serde(default)]
    pub categories: Option<Vec<Category>>,
}

/// 小程序类目（对应 Java 内嵌类 `MiniProgramInfo.Category`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Category {
    #[serde(default)]
    pub first: Option<String>,
    #[serde(default)]
    pub second: Option<String>,
}

/// 小程序服务器域名（对应 Java 内嵌类 `MiniProgramInfo.Network`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Network {
    #[serde(rename = "RequestDomain", default)]
    pub request_domain: Option<Vec<String>>,
    #[serde(rename = "WsRequestDomain", default)]
    pub ws_request_domain: Option<Vec<String>>,
    #[serde(rename = "UploadDomain", default)]
    pub upload_domain: Option<Vec<String>>,
    #[serde(rename = "DownloadDomain", default)]
    pub download_domain: Option<Vec<String>>,
    #[serde(rename = "BizDomain", default)]
    pub biz_domain: Option<Vec<String>>,
}

/// 小程序基础配置（对应 Java 内嵌类 `WxOpenAuthorizerInfo.BasicConfig`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BasicConfig {
    #[serde(rename = "is_phone_configured", default)]
    pub is_phone_configured: Option<bool>,
    #[serde(rename = "is_email_configured", default)]
    pub is_email_configured: Option<bool>,
}
