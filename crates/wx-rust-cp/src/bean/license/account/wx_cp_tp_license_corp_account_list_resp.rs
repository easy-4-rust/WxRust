//! 对应 Java `me.chanjar.weixin.cp.bean.license.account.WxCpTpLicenseCorpAccountListResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::license::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseCorpAccountListResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "has_more", default)]
    pub has_more: i32,
    #[serde(rename = "account_list", default)]
    pub order_list:
        Vec<crate::bean::license::wx_cp_tp_license_corp_account::WxCpTpLicenseCorpAccount>,
}

impl WxCpTpLicenseCorpAccountListResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpTpLicenseCorpAccountListResp 解析失败: {e}"))
    }
}
