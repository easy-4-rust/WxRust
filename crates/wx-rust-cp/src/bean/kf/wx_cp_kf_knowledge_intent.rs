/// 企业微信客服知识库Intent数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfKnowledgeIntent {
    /// 分组标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 问答标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    /// 问题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<Question>,
    /// 相似问法。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_questions: Option<SimilarQuestions>,
    /// 答案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<Answer>>,
}

/// 企业微信客服知识库Question数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Question
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Question {
    /// 问题文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// 响应内嵌相似问法。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_questions: Option<SimilarQuestions>,
    /// 响应内嵌答案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<Answer>>,
}

/// 企业微信客服知识库SimilarQuestions数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.SimilarQuestions
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimilarQuestions {
    /// 相似问题列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Question>>,
}

/// 企业微信客服知识库Answer数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Answer
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Answer {
    /// 文本答案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// 附件。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

/// 企业微信客服知识库Attachment数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Attachment
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    /// 附件类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msgtype: Option<String>,
    /// 图片。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// 视频。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// 链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    /// 小程序。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<MiniProgram>,
}

/// 企业微信客服知识库Text数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Text
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Text {
    /// 文本内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 企业微信客服知识库Image数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Image
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    /// 媒体标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 企业微信客服知识库Video数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Video
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    /// 媒体标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 企业微信客服知识库Link数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.Link
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// 标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 图片地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
    /// 说明。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 企业微信客服知识库MiniProgram数据。
/// 对应 Java: me.chanjar.weixin.cp.bean.kf.WxCpKfKnowledgeIntent.MiniProgram
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgram {
    /// 标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 缩略图标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_media_id: Option<String>,
    /// 小程序标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 页面路径。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
}
