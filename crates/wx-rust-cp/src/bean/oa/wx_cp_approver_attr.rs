//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApproverAttr.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum WxCpApproverAttr {
    #[serde(rename = "1")]
    #[default]
    OneSign,
    #[serde(rename = "2")]
    AllSign,
}
