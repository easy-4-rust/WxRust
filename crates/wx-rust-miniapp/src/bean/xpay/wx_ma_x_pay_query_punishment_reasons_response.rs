//! 商户被管控原因查询响应。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `query_punishment_reasons`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

/// 单条管控原因及解脱路径。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayRecoverySpecification {
    /// 该条管控原因对应的单据号，可与管控流水通知中的 business_code 关联。
    #[serde(rename = "limitation_case_id", default)]
    pub limitation_case_id: String,
    /// 该条管控原因所属的类型。
    #[serde(rename = "limitation_reason_type", default)]
    pub limitation_reason_type: String,
    /// 该条管控原因的简要描述。
    #[serde(rename = "limitation_reason", default)]
    pub limitation_reason: String,
    /// 该条管控原因的进一步说明。
    #[serde(rename = "limitation_reason_describe", default)]
    pub limitation_reason_describe: String,
    /// 该条管控原因下具体受影响的能力列表。
    #[serde(rename = "relate_limitations", default)]
    pub relate_limitations: Vec<serde_json::Value>,
    /// 未被标准枚举覆盖的管控能力补充说明。
    #[serde(rename = "other_relate_limitations", default)]
    pub other_relate_limitations: String,
    /// 微信支付建议的处理路径。
    #[serde(rename = "recover_way", default)]
    pub recover_way: String,
    /// 解脱路径对应的补充参数（尽调单号、申诉单号等）。
    #[serde(rename = "recover_way_param", default)]
    pub recover_way_param: String,
    /// 微信支付进一步说明页面 URL。
    #[serde(rename = "recover_help_url", default)]
    pub recover_help_url: String,
    /// 该条管控原因对应的管控生效方式。
    #[serde(rename = "limitation_action_type", default)]
    pub limitation_action_type: String,
    /// 处置方式为延迟管控时返回的预计开始时间。
    #[serde(rename = "limitation_start_date", default)]
    pub limitation_start_date: String,
    /// 该条管控原因实际生效的时间。
    #[serde(rename = "limitation_date", default)]
    pub limitation_date: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryPunishmentReasonsResponse {
    /// 错误码，0 表示成功。
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    /// 小程序 AppID。
    #[serde(rename = "appid", default)]
    pub appid: String,
    /// 小程序昵称。
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    /// 微信支付商户号。
    #[serde(rename = "merchant_code", default)]
    pub merchant_code: String,
    /// 商户被管控能力列表。
    #[serde(rename = "limited_functions", default)]
    pub limited_functions: Vec<String>,
    /// 商户其他被管控能力描述。
    #[serde(rename = "other_limited_functions", default)]
    pub other_limited_functions: String,
    /// 被管控原因及解脱路径列表。
    #[serde(rename = "recovery_specifications", default)]
    pub recovery_specifications: Vec<WxMaXPayRecoverySpecification>,
}

impl WxMaXPayQueryPunishmentReasonsResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryPunishmentReasonsResponse 序列化失败: {e}"))
    }
}
