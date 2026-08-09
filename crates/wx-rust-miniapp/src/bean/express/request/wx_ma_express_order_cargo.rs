//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderCargo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderCargo {
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "weight", default)]
    pub weight: f64,
    #[serde(rename = "space_x", default)]
    pub space_length: f64,
    #[serde(rename = "space_y", default)]
    pub space_width: f64,
    #[serde(rename = "space_z", default)]
    pub space_height: f64,
    #[serde(rename = "detail_list", default)]
    pub detail_list: Vec<WxMaExpressOrderCargoDetail>,
}
