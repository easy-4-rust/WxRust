//! 任务卡片按钮。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.taskcard.TaskCardButton`。Java 字段
//! 全为可空（Lombok @Builder/@Data）；Rust 以 `Option` 表达，`is_bold` 仅在
//! 显式设置时输出（对齐 Gson 省略 null 的线格式）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaskCardButton {
    /// 按钮 key 值（对应 Java `key`）。
    #[serde(rename = "key", default)]
    pub key: Option<String>,
    /// 按钮名称（对应 Java `name`）。
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// 点击按钮后显示的名称（对应 Java `replace_name`）。
    #[serde(rename = "replace_name", default)]
    pub replace_name: Option<String>,
    /// 按钮文字颜色（对应 Java `color`）。
    #[serde(rename = "color", default)]
    pub color: Option<String>,
    /// 是否加粗（对应 Java `is_bold`）。
    #[serde(rename = "is_bold", default)]
    pub bold: Option<bool>,
}

impl TaskCardButton {
    /// 构建任务卡片按钮。
    pub fn new(
        key: impl Into<String>,
        name: impl Into<String>,
        replace_name: Option<String>,
        color: Option<String>,
        bold: Option<bool>,
    ) -> Self {
        Self {
            key: Some(key.into()),
            name: Some(name.into()),
            replace_name,
            color,
            bold,
        }
    }
}
