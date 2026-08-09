//! 对应 Java `bean.guide.WxMpGuideMassedInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideMassedInfo {
    #[serde(rename = "task_id", default)]
    pub task_id: String,
    #[serde(rename = "guide_openid", default)]
    pub openid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "push_time", default)]
    pub push_time: i64,
    #[serde(rename = "finish_time", default)]
    pub finish_time: i64,
    #[serde(rename = "task_name", default)]
    pub task_name: String,
    #[serde(rename = "task_remark", default)]
    pub task_remark: String,
    #[serde(rename = "task_status", default)]
    pub task_status: i32,
    #[serde(rename = "material", default)]
    pub material: Vec<WxMpGuideMaterialInfo>,
    #[serde(rename = "buyer_info", default)]
    pub buyer_infos: Vec<WxMpGuideMassedBuyerInfo>,
}

impl WxMpGuideMassedInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpGuideMassedInfo 解析失败: {e}"))
    }
}
