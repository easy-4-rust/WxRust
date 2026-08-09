//! 获取「客户数据统计」企业汇总数据响应。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfGetCorpStatisticResp`
//! （继承 `WxCpBaseResp`，纯 Gson `@SerializedName`）：`statistic_list`
//! 统计数据列表，每项含 `stat_time`（当日 0 点时间戳）与 `statistic`
//! （一天的统计数据）。风格与已生成的
//! `WxCpKfGetServicerStatisticResp` 一致（字段默认值，不省略 null）。

/// 获取「客户数据统计」企业汇总数据响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfGetCorpStatisticResp {
    /// 错误码（对应 Java `WxCpBaseResp.errcode`）。
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    /// 错误信息（对应 Java `WxCpBaseResp.errmsg`）。
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    /// 统计数据列表（wire `statistic_list`）。
    #[serde(rename = "statistic_list", default)]
    pub statistic_list: Vec<StatisticList>,
}

impl WxCpKfGetCorpStatisticResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfGetCorpStatisticResp 解析失败: {e}"))
    }

    /// 序列化为 JSON。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpKfGetCorpStatisticResp 序列化失败: {e}"))
    }
}

/// 单日统计项（对应 Java `WxCpKfGetCorpStatisticResp.StatisticList`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatisticList {
    /// 数据统计日期，为当日 0 点的时间戳（wire `stat_time`）。
    #[serde(rename = "stat_time", default)]
    pub stat_time: i64,
    /// 一天的统计数据（wire `statistic`）。
    #[serde(rename = "statistic", default)]
    pub statistic: Statistic,
}

/// 一天的统计数据（对应 Java `WxCpKfGetCorpStatisticResp.Statistic`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Statistic {
    /// 咨询会话数（wire `session_cnt`）。
    #[serde(rename = "session_cnt", default)]
    pub session_cnt: i32,
    /// 咨询客户数（wire `customer_cnt`）。
    #[serde(rename = "customer_cnt", default)]
    pub customer_cnt: i32,
    /// 咨询消息总数（wire `customer_msg_cnt`）。
    #[serde(rename = "customer_msg_cnt", default)]
    pub customer_msg_cnt: i32,
    /// 升级服务客户数（wire `upgrade_service_customer_cnt`）。
    #[serde(rename = "upgrade_service_customer_cnt", default)]
    pub upgrade_service_customer_cnt: i32,
    /// 智能回复会话数（wire `ai_session_reply_cnt`）。
    #[serde(rename = "ai_session_reply_cnt", default)]
    pub ai_session_reply_cnt: i32,
    /// 转人工率（wire `ai_transfer_rate`）。
    #[serde(rename = "ai_transfer_rate", default)]
    pub ai_transfer_rate: f32,
    /// 知识命中率（wire `ai_knowledge_hit_rate`）。
    #[serde(rename = "ai_knowledge_hit_rate", default)]
    pub ai_knowledge_hit_rate: f32,
    /// 被拒收消息的客户数（wire `msg_rejected_customer_cnt`）。
    #[serde(rename = "msg_rejected_customer_cnt", default)]
    pub msg_rejected_customer_cnt: i32,
}
