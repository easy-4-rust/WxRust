//! 对应 Java `me.chanjar.weixin.cp.bean.external.acquisition.WxCpCustomerAcquisitionRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCustomerAcquisitionRequest {
    #[serde(rename = "link_id", default)]
    pub link_id: String,
    #[serde(rename = "link_name", default)]
    pub link_name: String,
    #[serde(rename = "range", default)]
    pub range: crate::bean::external::acquisition::wx_cp_customer_acquisition_info::Range,
    #[serde(rename = "skip_verify", default)]
    pub skip_verify: bool,
}

impl WxCpCustomerAcquisitionRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpCustomerAcquisitionRequest 序列化失败: {e}"))
    }
}
