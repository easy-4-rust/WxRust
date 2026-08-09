//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocGetDataRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocGetDataRequest {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
}

impl WxCpDocGetDataRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocGetDataRequest 解析失败: {e}"))
    }
}

impl WxCpDocGetDataRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocGetDataRequest 序列化失败: {e}"))
    }
}
