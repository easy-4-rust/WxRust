//! 模板消息 bean。

pub mod wx_mp_template;
pub mod wx_mp_template_industry;
pub mod wx_mp_template_industry_enum;
pub mod wx_mp_template_message;

pub use wx_mp_template::WxMpTemplate;
pub use wx_mp_template_industry::WxMpTemplateIndustry;
pub use wx_mp_template_industry_enum::WxMpTemplateIndustryEnum;
pub use wx_mp_template_message::{MiniProgram, WxMpTemplateData, WxMpTemplateMessage};
