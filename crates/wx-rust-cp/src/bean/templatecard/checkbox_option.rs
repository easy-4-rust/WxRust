//! 选项（模板卡片）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.CheckboxOption`。Java
//! 字段全为可空（Lombok @Builder/@Data）；Rust 以 `Option` 表达，`is_checked`
//! 仅在显式设置时输出（对齐 Gson 省略 null 的线格式）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckboxOption {
    /// 选项 id（对应 Java `id`）。
    #[serde(rename = "id", default)]
    pub id: Option<String>,
    /// 选项文案（对应 Java `text`）。
    #[serde(rename = "text", default)]
    pub text: Option<String>,
    /// 是否默认选中（对应 Java `is_checked`）。
    #[serde(rename = "is_checked", default)]
    pub is_checked: Option<bool>,
}

impl CheckboxOption {
    /// 构建选项。
    pub fn new(id: impl Into<String>, text: impl Into<String>, is_checked: Option<bool>) -> Self {
        Self {
            id: Some(id.into()),
            text: Some(text.into()),
            is_checked,
        }
    }
}
