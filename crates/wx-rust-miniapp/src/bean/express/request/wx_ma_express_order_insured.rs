//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderInsured.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderInsured {
    #[serde(rename = "use_insured", default)]
    pub use_insured: i32,
    #[serde(rename = "insured_value", default)]
    pub insured_value: i32,
}
