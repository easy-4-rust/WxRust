//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseBaseAccount.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseBaseAccount {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

impl WxCpTpLicenseBaseAccount {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpLicenseBaseAccount 序列化失败: {e}"))
    }
}
