//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseSimpleOrder.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseSimpleOrder {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
}

impl WxCpTpLicenseSimpleOrder {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpLicenseSimpleOrder 序列化失败: {e}"))
    }
}
