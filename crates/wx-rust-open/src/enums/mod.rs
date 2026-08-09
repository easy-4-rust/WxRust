//! 开放平台（第三方平台）枚举与 URL 常量。

pub mod url_core;
pub mod url_ma_domain;

pub use url_core::{
    api_authorizer_token_url, api_component_token_url, api_create_preauthcode_url,
    api_get_authorizer_info_url, api_get_authorizer_list_url, api_query_auth_url,
    api_start_push_ticket_url, component_login_page_url, component_mobile_login_page_url,
    get_authorizer_option_url, set_authorizer_option_url,
};
