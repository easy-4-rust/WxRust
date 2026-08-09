//! 对应 Java `me.chanjar.weixin.channel.bean.window` 包（生成）。

pub mod request;
pub mod response;

pub use request::add_window_product_request::AddWindowProductRequest;
pub use request::get_window_product_list_request::GetWindowProductListRequest;
pub use request::window_product_request::WindowProductRequest;
pub use response::get_window_product_list_response::GetWindowProductListResponse;
pub use response::get_window_product_list_response::ProductInfo;
pub use response::get_window_product_response::BannedDetails;
pub use response::get_window_product_response::BranchInfo;
pub use response::get_window_product_response::GetWindowProductResponse;
pub use response::get_window_product_response::LimitDiscountInfo;
pub use response::get_window_product_response::PagePath;
pub use response::get_window_product_response::Product;
