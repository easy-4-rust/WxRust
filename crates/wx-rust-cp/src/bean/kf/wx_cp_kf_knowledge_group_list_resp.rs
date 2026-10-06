/// 企业微信客服知识库GroupListResp数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeGroupListResp
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfKnowledgeGroupListResp {
    /// 业务错误码。
    #[serde(default)]
    pub errcode: i64,
    /// 业务说明。
    #[serde(default)]
    pub errmsg: String,
    /// 下页游标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// 是否存在下一页，保留整数语义。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<i32>,
    /// 分组列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<super::WxCpKfKnowledgeGroup>>,
}
