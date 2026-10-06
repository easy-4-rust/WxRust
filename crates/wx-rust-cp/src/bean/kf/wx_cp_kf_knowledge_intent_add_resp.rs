/// 企业微信客服知识库IntentAddResp数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntentAddResp
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfKnowledgeIntentAddResp {
    /// 业务错误码。
    #[serde(default)]
    pub errcode: i64,
    /// 业务说明。
    #[serde(default)]
    pub errmsg: String,
    /// 新增问答标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
}
