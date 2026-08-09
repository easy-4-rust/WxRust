//! 开放平台服务实现（对应 Java `me.chanjar.weixin.open.api.impl` 包）。

pub mod base_wx_open_service_impl;
pub mod minishop_upload_request_executor;
pub mod wx_open_component_service_impl;
pub mod wx_open_ma_auth_and_icp_service_impl;
pub mod wx_open_ma_auth_service_impl;
pub mod wx_open_ma_basic_service_impl;
pub mod wx_open_ma_embedded_service_impl;
pub mod wx_open_ma_icp_service_impl;
pub mod wx_open_ma_privacy_service_impl;
pub mod wx_open_ma_service;
pub mod wx_open_ma_shopping_orders_service_impl;
pub mod wx_open_minishop_goods_service_impl;
pub mod wx_open_minishop_service_impl;
pub mod wx_open_mp_o_auth2_service_impl;
pub mod wx_open_mp_service;
pub mod wx_open_o_auth2_service_impl;
pub mod wx_open_service_impl;

pub use minishop_upload_request_executor::MinishopUploadRequestExecutor;
pub use wx_open_component_service_impl::WxOpenComponentServiceImpl;
pub use wx_open_ma_auth_and_icp_service_impl::WxOpenMaAuthAndIcpServiceImpl;
pub use wx_open_ma_auth_service_impl::WxOpenMaAuthServiceImpl;
pub use wx_open_ma_basic_service_impl::WxOpenMaBasicServiceImpl;
pub use wx_open_ma_embedded_service_impl::WxOpenMaEmbeddedServiceImpl;
pub use wx_open_ma_icp_service_impl::WxOpenMaIcpServiceImpl;
pub use wx_open_ma_privacy_service_impl::WxOpenMaPrivacyServiceImpl;
pub use wx_open_ma_service::{WxOpenMaService, downcast_ma_service};
pub use wx_open_ma_shopping_orders_service_impl::WxOpenMaShoppingOrdersServiceImpl;
pub use wx_open_minishop_goods_service_impl::WxOpenMinishopGoodsServiceImpl;
pub use wx_open_minishop_service_impl::WxOpenMinishopServiceImpl;
pub use wx_open_mp_o_auth2_service_impl::WxOpenMpOAuth2ServiceImpl;
pub use wx_open_mp_service::{WxOpenMpService, downcast_mp_service};
pub use wx_open_o_auth2_service_impl::WxOpenOAuth2ServiceImpl;
pub use wx_open_service_impl::WxOpenServiceImpl;
