//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfServicerListResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfServicerListResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "servicer_list", default)]
    pub servicer_list: Vec<WxCpKfServicerStatus>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfServicerStatus {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

impl WxCpKfServicerListResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfServicerListResp 解析失败: {e}"))
    }
}
