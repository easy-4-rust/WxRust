//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthSubmitParamInvoiceElectronic.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthSubmitParamInvoiceElectronic {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}
