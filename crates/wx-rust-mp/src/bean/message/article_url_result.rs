//! 图文 url 结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.message.ArticleUrlResult`。

/// 图文 url 结果项。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArticleUrlResultItem {
    /// 图文索引。
    pub article_idx: Option<String>,
    /// 图文 url。
    pub article_url: Option<String>,
}

/// 群发图文 url 检测结果。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArticleUrlResult {
    /// 结果数量。
    pub count: Option<i64>,
    /// 结果列表。
    pub result_list: Vec<ArticleUrlResultItem>,
}
