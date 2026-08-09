//! 对应 Java `me.chanjar.weixin.cp.bean.hr.WxCpHrEmployeeFieldInfoResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpHrEmployeeFieldInfoResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "group_list", default)]
    pub group_list: Vec<FieldGroup>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldGroup {
    #[serde(rename = "group_id", default)]
    pub group_id: i32,
    #[serde(rename = "group_name", default)]
    pub group_name: String,
    #[serde(rename = "field_list", default)]
    pub field_list: Vec<crate::bean::hr::wx_cp_hr_employee_field_info::WxCpHrEmployeeFieldInfo>,
}

impl WxCpHrEmployeeFieldInfoResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpHrEmployeeFieldInfoResp 解析失败: {e}"))
    }
}
