//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaCodeLineColor.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。
//! Java `@NoArgsConstructor` 初始化 `r = "0", g = "0", b = "0"`（三字段默认黑色）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeLineColor {
    #[serde(rename = "r", default = "default_zero")]
    pub r: String,
    #[serde(rename = "g", default = "default_zero")]
    pub g: String,
    #[serde(rename = "b", default = "default_zero")]
    pub b: String,
}

impl Default for WxMaCodeLineColor {
    /// 对应 Java `@NoArgsConstructor`：r/g/b 默认 "0"。
    fn default() -> Self {
        Self {
            r: "0".to_string(),
            g: "0".to_string(),
            b: "0".to_string(),
        }
    }
}

fn default_zero() -> String {
    "0".to_string()
}
