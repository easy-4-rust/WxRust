//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrMenuResult`。
//!
//! 菜单 OCR 识别结果。

/// 菜单 OCR 识别结果。
///
/// 对应 Java `WxOcrMenuResult`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrMenuResult {
    /// 识别出的菜单项目列表。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<WxOcrMenuItem>,
}

/// 菜单 OCR 识别的单个菜品。
///
/// 对应 Java `WxOcrMenuResult.MenuItems`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrMenuItem {
    /// 菜品名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 菜品价格。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
}

impl WxOcrMenuResult {
    /// 从 JSON 字符串解析。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxOcrMenuResult 解析失败: {e}"))
    }

    /// 序列化为 JSON 字符串。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let result = WxOcrMenuResult {
            items: vec![
                WxOcrMenuItem {
                    name: Some("宫保鸡丁".to_string()),
                    price: Some("38".to_string()),
                },
                WxOcrMenuItem {
                    name: Some("鱼香肉丝".to_string()),
                    price: Some("32".to_string()),
                },
            ],
        };
        let json = result.to_json();
        let parsed = WxOcrMenuResult::from_json(&json).unwrap();
        assert_eq!(result, parsed);
    }

    #[test]
    fn parse_from_api_response() {
        let json = r#"{
            "items": [
                {"name": "红烧肉", "price": "48"},
                {"name": "清蒸鱼", "price": "58"}
            ]
        }"#;
        let result = WxOcrMenuResult::from_json(json).unwrap();
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].name.as_deref(), Some("红烧肉"));
        assert_eq!(result.items[0].price.as_deref(), Some("48"));
        assert_eq!(result.items[1].name.as_deref(), Some("清蒸鱼"));
    }

    #[test]
    fn parse_empty() {
        let json = r#"{"items": []}"#;
        let result = WxOcrMenuResult::from_json(json).unwrap();
        assert!(result.items.is_empty());
    }

    #[test]
    fn parse_minimal() {
        let json = r#"{}"#;
        let result = WxOcrMenuResult::from_json(json).unwrap();
        assert!(result.items.is_empty());
    }
}
