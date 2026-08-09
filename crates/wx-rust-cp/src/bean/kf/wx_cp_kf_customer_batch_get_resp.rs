//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfCustomerBatchGetResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfCustomerBatchGetResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "customer_list", default)]
    pub customer_list: Vec<crate::bean::wx_cp_user_external_contact_info::ExternalContact>,
    #[serde(rename = "invalid_external_userid", default)]
    pub invalid_external_user_id: Vec<String>,
}

impl WxCpKfCustomerBatchGetResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfCustomerBatchGetResp 解析失败: {e}"))
    }
}
