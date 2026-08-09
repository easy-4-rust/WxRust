//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUpdateRemarkRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUpdateRemarkRequest {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "remark_company", default)]
    pub remark_company: String,
    #[serde(rename = "remark_mobiles", default)]
    pub remark_mobiles: Vec<String>,
    #[serde(rename = "remark_pic_mediaid", default)]
    pub remark_pic_media_id: String,
}

impl WxCpUpdateRemarkRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpUpdateRemarkRequest 序列化失败: {e}"))
    }
}
