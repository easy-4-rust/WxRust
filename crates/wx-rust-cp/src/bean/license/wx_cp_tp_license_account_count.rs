//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseAccountCount.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseAccountCount {
    #[serde(rename = "base_count", default)]
    pub base_count: i32,
    #[serde(rename = "external_contact_count", default)]
    pub external_contact_count: i32,
}

impl WxCpTpLicenseAccountCount {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpTpLicenseAccountCount 序列化失败: {e}"))
    }
}
