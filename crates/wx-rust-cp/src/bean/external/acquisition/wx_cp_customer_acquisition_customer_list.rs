//! 对应 Java `me.chanjar.weixin.cp.bean.external.acquisition.WxCpCustomerAcquisitionCustomerList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCustomerAcquisitionCustomerList {
    #[serde(rename = "customer_list", default)]
    pub customer_list: Vec<Customer>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    #[serde(rename = "external_userid", default)]
    pub external_userid: String,
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "chat_status", default)]
    pub chat_status: i32,
    #[serde(rename = "state", default)]
    pub state: String,
}

impl WxCpCustomerAcquisitionCustomerList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpCustomerAcquisitionCustomerList 解析失败: {e}"))
    }
}

impl WxCpCustomerAcquisitionCustomerList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpCustomerAcquisitionCustomerList 序列化失败: {e}"))
    }
}
