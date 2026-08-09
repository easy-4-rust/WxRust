//! 对应 Java `me.chanjar.weixin.cp.bean.external.acquisition.WxCpCustomerAcquisitionCreateResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCustomerAcquisitionCreateResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "link", default)]
    pub link: crate::bean::external::acquisition::wx_cp_customer_acquisition_info::Link,
}

impl WxCpCustomerAcquisitionCreateResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpCustomerAcquisitionCreateResult 解析失败: {e}"))
    }
}
