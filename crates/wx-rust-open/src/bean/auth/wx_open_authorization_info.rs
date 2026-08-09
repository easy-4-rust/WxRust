//! 授权信息（query_auth / get_authorizer_info 的 authorization_info）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.auth.WxOpenAuthorizationInfo`。
//! 由 `WxOpenAuthorizationInfoGsonAdapter` 驱动解析（snake_case 键 +
//! `func_info` 数组扁平化为 `List<Integer>`），与字段名直映不同，
//! 故人工迁移：Rust 以 serde rename + 自定义反序列化函数表达同一线格式。

use serde::Deserialize;

/// 授权信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenAuthorizationInfo {
    /// 授权方 appid。
    #[serde(rename = "authorizer_appid", default)]
    pub authorizer_appid: Option<String>,
    /// 授权方 access_token。
    #[serde(rename = "authorizer_access_token", default)]
    pub authorizer_access_token: Option<String>,
    /// 有效期（秒）。
    #[serde(rename = "expires_in", default)]
    pub expires_in: Option<i32>,
    /// 授权方 refresh_token。
    #[serde(rename = "authorizer_refresh_token", default)]
    pub authorizer_refresh_token: Option<String>,
    /// 授权给开发者的权限集列表（对应 Java `funcInfo`：
    /// `[{"funcscope_category":{"id":N}},...]` → `[N,...]` 扁平化）。
    #[serde(rename = "func_info", default, deserialize_with = "de_func_info")]
    pub func_info: Vec<i32>,
}

/// 解析 `[{"funcscope_category":{"id":N}},...]` → `Vec<i32>`
/// （对应 Java adapter 的 func_info 扁平化；非对象/缺 id 项跳过）。
fn de_func_info<'de, D>(d: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = <Vec<serde_json::Value> as Deserialize>::deserialize(d)
        .map_err(serde::de::Error::custom)?;
    Ok(v.iter()
        .filter_map(|item| {
            item.get("funcscope_category")
                .and_then(|c| c.get("id"))
                .and_then(|x| x.as_i64())
                .map(|x| x as i32)
        })
        .collect())
}
