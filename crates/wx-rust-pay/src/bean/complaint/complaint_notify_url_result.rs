//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.ComplaintNotifyUrlResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintNotifyUrlResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "url")]
    pub url: Option<String>,
}
