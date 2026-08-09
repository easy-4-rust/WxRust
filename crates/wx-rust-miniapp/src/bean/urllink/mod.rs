//! 对应 Java `cn.binarywang.wx.miniapp.bean.urllink` 包（生成）。

pub mod cloud_base;
pub mod generate_url_link_request;
pub mod request;
pub mod response;

pub use cloud_base::CloudBase;
pub use generate_url_link_request::GenerateUrlLinkRequest;
pub use request::query_url_link_request::QueryUrlLinkRequest;
pub use response::query_url_link_response::QueryUrlLinkResponse;
pub use response::query_url_link_response::UrlLinkInfo;
