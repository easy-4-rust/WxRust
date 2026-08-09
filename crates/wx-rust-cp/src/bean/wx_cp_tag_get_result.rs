//! 获取标签成员接口响应体。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTagGetResult`（纯 Gson
//! `@SerializedName`，无自定义 adapter）：`errcode`/`errmsg`/`userlist`/
//! `partylist`/`tagname`。Gson 默认省略 null 字段，故以 `Option` +
//! `skip_serializing_if` 表达。

use super::WxCpUser;

/// 获取标签成员接口响应体。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTagGetResult {
    /// 错误码（wire `errcode`）。
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none", default)]
    pub errcode: Option<i32>,
    /// 错误信息（wire `errmsg`）。
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none", default)]
    pub errmsg: Option<String>,
    /// 用户列表（wire `userlist`）。
    #[serde(rename = "userlist", skip_serializing_if = "Option::is_none", default)]
    pub userlist: Option<Vec<WxCpUser>>,
    /// 部门列表（wire `partylist`）。
    #[serde(rename = "partylist", skip_serializing_if = "Option::is_none", default)]
    pub partylist: Option<Vec<i32>>,
    /// 标签名称（wire `tagname`）。
    #[serde(rename = "tagname", skip_serializing_if = "Option::is_none", default)]
    pub tagname: Option<String>,
}

impl WxCpTagGetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTagGetResult 解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTagGetResult 序列化失败: {e}"))
    }
}
