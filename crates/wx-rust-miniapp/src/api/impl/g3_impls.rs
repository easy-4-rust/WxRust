//! 电商服务组（G3）子服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl` 包中电商类子服务实现
//! （shop 交易组件 12 个 + 标准版 product/productOrder + 订单管理/发货信息 +
//! 物流退货/即时配送 + 用工关系 + 微信客服）。
//!
//! 模块文件位于 `api/impl/` 根目录（`wx_ma_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。

#[path = "wx_ma_customservice_work_service_impl.rs"]
pub mod wx_ma_customservice_work_service_impl;
#[path = "wx_ma_employee_relation_service_impl.rs"]
pub mod wx_ma_employee_relation_service_impl;
#[path = "wx_ma_express_delivery_return_service_impl.rs"]
pub mod wx_ma_express_delivery_return_service_impl;
#[path = "wx_ma_immediate_delivery_service_impl.rs"]
pub mod wx_ma_immediate_delivery_service_impl;
#[path = "wx_ma_order_management_service_impl.rs"]
pub mod wx_ma_order_management_service_impl;
#[path = "wx_ma_order_shipping_service_impl.rs"]
pub mod wx_ma_order_shipping_service_impl;
#[path = "wx_ma_product_order_service_impl.rs"]
pub mod wx_ma_product_order_service_impl;
#[path = "wx_ma_product_service_impl.rs"]
pub mod wx_ma_product_service_impl;
#[path = "wx_ma_shop_account_service_impl.rs"]
pub mod wx_ma_shop_account_service_impl;
#[path = "wx_ma_shop_after_sale_service_impl.rs"]
pub mod wx_ma_shop_after_sale_service_impl;
#[path = "wx_ma_shop_audit_service_impl.rs"]
pub mod wx_ma_shop_audit_service_impl;
#[path = "wx_ma_shop_cat_service_impl.rs"]
pub mod wx_ma_shop_cat_service_impl;
#[path = "wx_ma_shop_coupon_service_impl.rs"]
pub mod wx_ma_shop_coupon_service_impl;
#[path = "wx_ma_shop_delivery_service_impl.rs"]
pub mod wx_ma_shop_delivery_service_impl;
#[path = "wx_ma_shop_img_service_impl.rs"]
pub mod wx_ma_shop_img_service_impl;
#[path = "wx_ma_shop_order_service_impl.rs"]
pub mod wx_ma_shop_order_service_impl;
#[path = "wx_ma_shop_pay_service_impl.rs"]
pub mod wx_ma_shop_pay_service_impl;
#[path = "wx_ma_shop_register_service_impl.rs"]
pub mod wx_ma_shop_register_service_impl;
#[path = "wx_ma_shop_sharer_service_impl.rs"]
pub mod wx_ma_shop_sharer_service_impl;
#[path = "wx_ma_shop_spu_service_impl.rs"]
pub mod wx_ma_shop_spu_service_impl;

pub use wx_ma_customservice_work_service_impl::WxMaCustomserviceWorkServiceImpl;
pub use wx_ma_employee_relation_service_impl::WxMaEmployeeRelationServiceImpl;
pub use wx_ma_express_delivery_return_service_impl::WxMaExpressDeliveryReturnServiceImpl;
pub use wx_ma_immediate_delivery_service_impl::WxMaImmediateDeliveryServiceImpl;
pub use wx_ma_order_management_service_impl::WxMaOrderManagementServiceImpl;
pub use wx_ma_order_shipping_service_impl::WxMaOrderShippingServiceImpl;
pub use wx_ma_product_order_service_impl::WxMaProductOrderServiceImpl;
pub use wx_ma_product_service_impl::WxMaProductServiceImpl;
pub use wx_ma_shop_account_service_impl::WxMaShopAccountServiceImpl;
pub use wx_ma_shop_after_sale_service_impl::WxMaShopAfterSaleServiceImpl;
pub use wx_ma_shop_audit_service_impl::WxMaShopAuditServiceImpl;
pub use wx_ma_shop_cat_service_impl::WxMaShopCatServiceImpl;
pub use wx_ma_shop_coupon_service_impl::WxMaShopCouponServiceImpl;
pub use wx_ma_shop_delivery_service_impl::WxMaShopDeliveryServiceImpl;
pub use wx_ma_shop_img_service_impl::WxMaShopImgServiceImpl;
pub use wx_ma_shop_order_service_impl::WxMaShopOrderServiceImpl;
pub use wx_ma_shop_pay_service_impl::WxMaShopPayServiceImpl;
pub use wx_ma_shop_register_service_impl::WxMaShopRegisterServiceImpl;
pub use wx_ma_shop_sharer_service_impl::WxMaShopSharerServiceImpl;
pub use wx_ma_shop_spu_service_impl::WxMaShopSpuServiceImpl;
