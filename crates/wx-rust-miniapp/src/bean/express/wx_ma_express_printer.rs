//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.WxMaExpressPrinter.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressPrinter {
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "openid", default)]
    pub openid: Vec<String>,
    #[serde(rename = "tagid_list", default)]
    pub tagid_list: Vec<String>,
}

impl WxMaExpressPrinter {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaExpressPrinter 解析失败: {e}"))
    }
}
