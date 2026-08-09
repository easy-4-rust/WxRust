//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpUserDetail.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpUserDetail {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "gender", default)]
    pub gender: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
    #[serde(rename = "qr_code", default)]
    pub qr_code: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "biz_mail", default)]
    pub biz_mail: String,
    #[serde(rename = "address", default)]
    pub address: String,
}

impl WxCpTpUserDetail {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpUserDetail 解析失败: {e}"))
    }
}

impl WxCpTpUserDetail {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpUserDetail 序列化失败: {e}"))
    }
}
