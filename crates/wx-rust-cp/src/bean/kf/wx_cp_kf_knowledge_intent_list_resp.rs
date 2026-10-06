/// 企业微信客服知识库IntentListResp数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntentListResp
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfKnowledgeIntentListResp {
    /// 业务错误码。
    #[serde(default)]
    pub errcode: i64,
    /// 业务说明。
    #[serde(default)]
    pub errmsg: String,
    /// 下页游标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// 是否存在下一页。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<i32>,
    /// 问答列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_list: Option<Vec<super::WxCpKfKnowledgeIntent>>,
}
