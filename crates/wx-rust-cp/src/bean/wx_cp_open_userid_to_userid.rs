//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpOpenUseridToUserid.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOpenUseridToUserid {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "open_userid", default)]
    pub open_userid: String,
}

impl WxCpOpenUseridToUserid {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpOpenUseridToUserid 解析失败: {e}"))
    }
}
