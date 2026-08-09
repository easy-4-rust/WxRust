//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserTransferCustomerReq.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserTransferCustomerReq {
    #[serde(rename = "handover_userid", default)]
    pub hand_over_userid: String,
    #[serde(rename = "takeover_userid", default)]
    pub take_over_userid: String,
    #[serde(rename = "external_userid", default)]
    pub external_userid: Vec<String>,
    #[serde(rename = "transfer_success_msg", default)]
    pub transfer_msg: String,
}

impl WxCpUserTransferCustomerReq {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpUserTransferCustomerReq 序列化失败: {e}"))
    }
}
