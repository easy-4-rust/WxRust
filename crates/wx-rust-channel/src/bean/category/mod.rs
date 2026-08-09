//! 对应 Java `me.chanjar.weixin.channel.bean.category` 包（生成）。

pub mod account_category_response;
pub mod category_and_qualification_list;
pub mod category_detail_result;
pub mod category_qualification;
pub mod category_qualification_response;
pub mod pass_category_info;
pub mod pass_category_response;
pub mod qualification_info;
pub mod relation_category_item;
pub mod relation_category_request;
pub mod relation_category_response;
pub mod shop_category;
pub mod shop_category_response;

pub use account_category_response::AccountCategoryResponse;
pub use category_and_qualification_list::CategoryAndQualificationList;
pub use category_detail_result::Attr;
pub use category_detail_result::BrandInfo;
pub use category_detail_result::CategoryDetailResult;
pub use category_detail_result::CouponRule;
pub use category_detail_result::FeeInfo;
pub use category_detail_result::Info;
pub use category_detail_result::ProductAttr;
pub use category_detail_result::ProductRequirement;
pub use category_detail_result::SizeChart;
pub use category_detail_result::SizeChartItem;
pub use category_qualification::CategoryQualification;
pub use category_qualification_response::CategoryQualificationResponse;
pub use pass_category_info::PassCategoryInfo;
pub use pass_category_response::PassCategoryResponse;
pub use qualification_info::QualificationInfo;
pub use relation_category_item::RelationCategoryItem;
pub use relation_category_request::RelationCategoryRequest;
pub use relation_category_response::RelationCategoryResponse;
pub use shop_category::ShopCategory;
pub use shop_category_response::ShopCategoryResponse;
