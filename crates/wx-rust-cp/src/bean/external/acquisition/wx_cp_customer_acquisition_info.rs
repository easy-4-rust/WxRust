//! 对应 Java `me.chanjar.weixin.cp.bean.external.acquisition.WxCpCustomerAcquisitionInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCustomerAcquisitionInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "link", default)]
    pub link: Link,
    #[serde(rename = "range", default)]
    pub range: Range,
    #[serde(rename = "skip_verify", default)]
    pub skip_verify: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "link_id", default)]
    pub link_id: String,
    #[serde(rename = "link_name", default)]
    pub link_name: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Range {
    #[serde(rename = "user_list", default)]
    pub user_list: Vec<String>,
    #[serde(rename = "department_list", default)]
    pub department_list: Vec<String>,
}

impl WxCpCustomerAcquisitionInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpCustomerAcquisitionInfo 解析失败: {e}"))
    }
}

impl WxCpCustomerAcquisitionInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpCustomerAcquisitionInfo 序列化失败: {e}"))
    }
}
