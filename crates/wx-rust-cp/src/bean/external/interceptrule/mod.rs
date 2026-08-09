//! 对应 Java `me.chanjar.weixin.cp.bean.external/interceptrule` 包（生成）。

pub mod applicable_range;
pub mod wx_cp_intercept_rule;
pub mod wx_cp_intercept_rule_add_request;
pub mod wx_cp_intercept_rule_add_result;
pub mod wx_cp_intercept_rule_info;
pub mod wx_cp_intercept_rule_list;

pub use applicable_range::ApplicableRange;
pub use wx_cp_intercept_rule::ExtraRule;
pub use wx_cp_intercept_rule::WxCpInterceptRule;
pub use wx_cp_intercept_rule_add_request::WxCpInterceptRuleAddRequest;
pub use wx_cp_intercept_rule_add_result::WxCpInterceptRuleAddResult;
pub use wx_cp_intercept_rule_info::Rule;
pub use wx_cp_intercept_rule_info::WxCpInterceptRuleInfo;
pub use wx_cp_intercept_rule_list::WxCpInterceptRuleList;
