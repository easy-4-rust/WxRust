//! 模板消息行业（对应 Java `WxMpTemplateIndustry`）。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 生成后人工修正（行业枚举 Option 化）。

use serde::{Deserialize, Serialize};

use super::WxMpTemplateIndustryEnum;

/// 模板消息行业（获取账号所属行业返回）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpTemplateIndustry {
    /// 主行业。
    #[serde(rename = "primary_industry", skip_serializing_if = "Option::is_none")]
    pub primary_industry: Option<WxMpTemplateIndustryEnum>,
    /// 副行业。
    #[serde(rename = "second_industry", skip_serializing_if = "Option::is_none")]
    pub second_industry: Option<WxMpTemplateIndustryEnum>,
}
