//! 开放平台（第三方平台）消息 API。
//!
//! 对应 Java `me.chanjar.weixin.open.api` 包。

pub mod r#impl;
pub mod wx_open_component_service;
pub mod wx_open_ma_auth_and_icp_service;
pub mod wx_open_ma_auth_service;
pub mod wx_open_ma_basic_service;
pub mod wx_open_ma_embedded_service;
pub mod wx_open_ma_icp_service;
pub mod wx_open_ma_privacy_service;
pub mod wx_open_ma_shopping_orders_service;
pub mod wx_open_minishop_goods_service;
pub mod wx_open_minishop_service;
pub mod wx_open_service;

pub use wx_open_component_service::WxOpenComponentService;
pub use wx_open_ma_auth_and_icp_service::WxOpenMaAuthAndIcpService;
pub use wx_open_ma_auth_service::WxOpenMaAuthService;
pub use wx_open_ma_basic_service::WxOpenMaBasicService;
pub use wx_open_ma_embedded_service::WxOpenMaEmbeddedService;
pub use wx_open_ma_icp_service::WxOpenMaIcpService;
pub use wx_open_ma_privacy_service::WxOpenMaPrivacyService;
pub use wx_open_ma_shopping_orders_service::WxOpenMaShoppingOrdersService;
pub use wx_open_minishop_goods_service::WxOpenMinishopGoodsService;
pub use wx_open_minishop_service::WxOpenMinishopService;
pub use wx_open_service::WxOpenService;
