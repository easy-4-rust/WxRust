//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderCargoDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderCargoDetail {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "count", default)]
    pub count: i32,
}
