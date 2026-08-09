//! 企业微信标签。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTag`，线格式以
//! `util/json/WxCpTagGsonAdapter` 为准：`tagid`/`tagname` 两个字符串字段，
//! null 省略（Java `addPropertyIfNotNull`）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTag {
    /// 标签 id（wire `tagid`）。
    #[serde(rename = "tagid", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// 标签名（wire `tagname`）。
    #[serde(rename = "tagname", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
}

impl WxCpTag {
    /// 新建标签（对应 Java `@AllArgsConstructor` 构造）。
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            name: Some(name.into()),
        }
    }

    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTag 解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTag 序列化失败: {e}"))
    }
}
