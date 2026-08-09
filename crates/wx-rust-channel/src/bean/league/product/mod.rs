//! 对应 Java `me.chanjar.weixin.channel.bean.league/product` 包（生成）。

pub mod batch_add_param;
pub mod batch_add_response;
pub mod product_delete_param;
pub mod product_detail_param;
pub mod product_detail_response;
pub mod product_list_param;
pub mod product_list_response;
pub mod product_update_param;
pub mod product_update_response;

pub use batch_add_param::BatchAddParam;
pub use batch_add_param::Product;
pub use batch_add_response::BatchAddResponse;
pub use batch_add_response::ResultInfo;
pub use product_delete_param::ProductDeleteParam;
pub use product_detail_param::ProductDetailParam;
pub use product_detail_response::ExclusiveInfo;
pub use product_detail_response::ExtInfo;
pub use product_detail_response::Item;
pub use product_detail_response::ProductDetailResponse;
pub use product_list_param::ProductListParam;
pub use product_list_response::ProductListResponse;
pub use product_update_param::ProductUpdateParam;
pub use product_update_response::ProductUpdateResponse;
