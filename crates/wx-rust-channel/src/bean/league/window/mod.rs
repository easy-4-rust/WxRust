//! 对应 Java `me.chanjar.weixin.channel.bean.league/window` 包（生成）。

pub mod auth_info;
pub mod auth_info_response;
pub mod auth_status_response;
pub mod product_search_param;
pub mod window_product_list_response;
pub mod window_product_param;
pub mod window_product_response;

pub use auth_info::AuthInfo;
pub use auth_info_response::AuthInfoResponse;
pub use auth_status_response::AuthStatusResponse;
pub use product_search_param::ProductSearchParam;
pub use window_product_list_response::ItemKey;
pub use window_product_list_response::WindowProductListResponse;
pub use window_product_param::WindowProductParam;
pub use window_product_response::ProductDetail;
pub use window_product_response::WindowProductResponse;
