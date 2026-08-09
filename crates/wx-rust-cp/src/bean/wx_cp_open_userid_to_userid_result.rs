//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpOpenUseridToUseridResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOpenUseridToUseridResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "userid_list", default)]
    pub userid_list: Vec<crate::bean::wx_cp_userid_to_open_userid::WxCpUseridToOpenUserid>,
    #[serde(rename = "invalid_open_userid_list", default)]
    pub invalid_open_userid_list: Vec<String>,
}

impl WxCpOpenUseridToUseridResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpOpenUseridToUseridResult 解析失败: {e}"))
    }
}
