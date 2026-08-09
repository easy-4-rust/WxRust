//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.WxMaExpressAccount.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressAccount {
    #[serde(rename = "biz_id", default)]
    pub biz_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "status_code", default)]
    pub status_code: i32,
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "remark_wrong_msg", default)]
    pub remark_wrong_msg: String,
    #[serde(rename = "remark_content", default)]
    pub remark_content: String,
    #[serde(rename = "quota_num", default)]
    pub quota_num: i32,
    #[serde(rename = "quota_update_time", default)]
    pub quota_update_time: i32,
    #[serde(rename = "service_type", default)]
    pub service_type: Vec<ServiceType>,
}

impl WxMaExpressAccount {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaExpressAccount 解析失败: {e}"))
    }
}
