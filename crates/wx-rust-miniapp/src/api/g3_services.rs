//! 电商服务组（G3）子服务 trait。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api` 包中电商类子服务接口
//! （shop 交易组件 12 个 + 标准版 product/productOrder + 订单管理/发货信息 +
//! 物流退货/即时配送 + 用工关系 + 微信客服）。
//!
//! 模块文件位于 `api/` 根目录（`wx_ma_<域>_service.rs`，与任务文件布局
//! 一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式
//! 指回根目录文件。

#[path = "wx_ma_customservice_work_service.rs"]
pub mod wx_ma_customservice_work_service;
#[path = "wx_ma_employee_relation_service.rs"]
pub mod wx_ma_employee_relation_service;
#[path = "wx_ma_express_delivery_return_service.rs"]
pub mod wx_ma_express_delivery_return_service;
#[path = "wx_ma_immediate_delivery_service.rs"]
pub mod wx_ma_immediate_delivery_service;
#[path = "wx_ma_order_management_service.rs"]
pub mod wx_ma_order_management_service;
#[path = "wx_ma_order_shipping_service.rs"]
pub mod wx_ma_order_shipping_service;
#[path = "wx_ma_product_order_service.rs"]
pub mod wx_ma_product_order_service;
#[path = "wx_ma_product_service.rs"]
pub mod wx_ma_product_service;
#[path = "wx_ma_shop_account_service.rs"]
pub mod wx_ma_shop_account_service;
#[path = "wx_ma_shop_after_sale_service.rs"]
pub mod wx_ma_shop_after_sale_service;
#[path = "wx_ma_shop_audit_service.rs"]
pub mod wx_ma_shop_audit_service;
#[path = "wx_ma_shop_cat_service.rs"]
pub mod wx_ma_shop_cat_service;
#[path = "wx_ma_shop_coupon_service.rs"]
pub mod wx_ma_shop_coupon_service;
#[path = "wx_ma_shop_delivery_service.rs"]
pub mod wx_ma_shop_delivery_service;
#[path = "wx_ma_shop_img_service.rs"]
pub mod wx_ma_shop_img_service;
#[path = "wx_ma_shop_order_service.rs"]
pub mod wx_ma_shop_order_service;
#[path = "wx_ma_shop_pay_service.rs"]
pub mod wx_ma_shop_pay_service;
#[path = "wx_ma_shop_register_service.rs"]
pub mod wx_ma_shop_register_service;
#[path = "wx_ma_shop_sharer_service.rs"]
pub mod wx_ma_shop_sharer_service;
#[path = "wx_ma_shop_spu_service.rs"]
pub mod wx_ma_shop_spu_service;

pub use wx_ma_customservice_work_service::WxMaCustomserviceWorkService;
pub use wx_ma_employee_relation_service::WxMaEmployeeRelationService;
pub use wx_ma_express_delivery_return_service::WxMaExpressDeliveryReturnService;
pub use wx_ma_immediate_delivery_service::WxMaImmediateDeliveryService;
pub use wx_ma_order_management_service::WxMaOrderManagementService;
pub use wx_ma_order_shipping_service::WxMaOrderShippingService;
pub use wx_ma_product_order_service::WxMaProductOrderService;
pub use wx_ma_product_service::WxMaProductService;
pub use wx_ma_shop_account_service::WxMaShopAccountService;
pub use wx_ma_shop_after_sale_service::WxMaShopAfterSaleService;
pub use wx_ma_shop_audit_service::WxMaShopAuditService;
pub use wx_ma_shop_cat_service::WxMaShopCatService;
pub use wx_ma_shop_coupon_service::WxMaShopCouponService;
pub use wx_ma_shop_delivery_service::WxMaShopDeliveryService;
pub use wx_ma_shop_img_service::WxMaShopImgService;
pub use wx_ma_shop_order_service::WxMaShopOrderService;
pub use wx_ma_shop_pay_service::WxMaShopPayService;
pub use wx_ma_shop_register_service::WxMaShopRegisterService;
pub use wx_ma_shop_sharer_service::WxMaShopSharerService;
pub use wx_ma_shop_spu_service::WxMaShopSpuService;
