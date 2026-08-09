//! 订阅消息数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.subscribemsg` 包。

pub mod category_data;
pub mod pub_template_keyword;
pub mod pub_template_title_list_result;
pub mod template_info;

pub use category_data::CategoryData;
pub use pub_template_keyword::PubTemplateKeyword;
pub use pub_template_title_list_result::PubTemplateTitleListResult;
pub use pub_template_title_list_result::TemplateItem;
pub use template_info::TemplateInfo;
