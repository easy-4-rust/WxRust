//! 对应 Java `me.chanjar.weixin.channel.bean.home/banner` 包（生成）。

pub mod banner_apply_detail;
pub mod banner_apply_info;
pub mod banner_apply_param;
pub mod banner_apply_response;
pub mod banner_get_response;
pub mod banner_info;
pub mod banner_item;
pub mod banner_item_detail;
pub mod banner_item_finder;
pub mod banner_item_official_account;
pub mod banner_item_product;

pub use banner_apply_detail::BannerApplyDetail;
pub use banner_apply_info::BannerApplyInfo;
pub use banner_apply_param::BannerApplyParam;
pub use banner_apply_response::BannerApplyResponse;
pub use banner_get_response::BannerGetResponse;
pub use banner_info::BannerInfo;
pub use banner_item::BannerItem;
pub use banner_item_detail::BannerItemDetail;
pub use banner_item_finder::BannerItemFinder;
pub use banner_item_official_account::BannerItemOfficialAccount;
pub use banner_item_product::BannerItemProduct;
