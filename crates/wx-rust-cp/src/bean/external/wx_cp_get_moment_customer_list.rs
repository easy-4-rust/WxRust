//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGetMomentCustomerList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetMomentCustomerList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "customer_list", default)]
    pub customer_list: Vec<crate::bean::external::moment::customer_item::CustomerItem>,
}

impl WxCpGetMomentCustomerList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetMomentCustomerList 解析失败: {e}"))
    }
}

impl WxCpGetMomentCustomerList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpGetMomentCustomerList 序列化失败: {e}"))
    }
}
