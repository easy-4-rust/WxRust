/// 企业微信客服知识库Group数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeGroup
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfKnowledgeGroup {
    /// 分组标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 分组名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 默认分组标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i32>,
}
