//! 对应 Java `cn.binarywang.wx.miniapp.bean.safety.response.WxMaUserSafetyRiskRankResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::safety::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaUserSafetyRiskRankResponse {
    #[serde(rename = "unoin_id", default)]
    pub unoin_id: i64,
    #[serde(rename = "risk_rank", default)]
    pub risk_rank: i32,
}

impl WxMaUserSafetyRiskRankResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMaUserSafetyRiskRankResponse 解析失败: {e}"))
    }
}
