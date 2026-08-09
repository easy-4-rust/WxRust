//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.WaybillGoodsInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillGoodsInfo {
    #[serde(rename = "detail_list", default)]
    pub goods_item_list: Vec<GoodsItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsItem {
    #[serde(rename = "goods_name", default)]
    pub goods_name: String,
    #[serde(rename = "goods_img_url", default)]
    pub goods_img_url: String,
    #[serde(rename = "goods_desc", default)]
    pub goods_desc: String,
}

impl WaybillGoodsInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WaybillGoodsInfo 解析失败: {e}"))
    }
}
