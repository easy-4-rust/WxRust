//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.UpdateWaybillGoodsRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateWaybillGoodsRequest {
    #[serde(rename = "waybill_token", default)]
    pub waybill_token: String,
    #[serde(rename = "goods_info", default)]
    pub goods_info: WaybillGoodsInfo,
}

impl UpdateWaybillGoodsRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("UpdateWaybillGoodsRequest 序列化失败: {e}"))
    }
}
