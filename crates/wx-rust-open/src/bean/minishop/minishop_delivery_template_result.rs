//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopDeliveryTemplateResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopDeliveryTemplateResult {
    #[serde(rename = "errCode", default)]
    pub err_code: i32,
    #[serde(rename = "errMsg", default)]
    pub err_msg: String,
    #[serde(rename = "templateList", default)]
    pub template_list: Vec<MinishopDeliveryTemplate>,
}
