//! 枚举/常量分组注册（电商服务组 G3）。
//!
//! 本文件仅聚合 G3 新增的 URL 常量模块，避免直接改写 `enums/mod.rs` 的
//! 既有内容（注册行由各波次追加）。
//!
//! 模块文件位于 `enums/` 根目录（`url_g3_shop.rs`，与任务文件布局一致）；
//! 本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式指回根目录文件。

#[path = "url_g3_shop.rs"]
pub mod url_g3_shop;

pub use url_g3_shop::{
    customservice_work, employee, express_delivery_return, instant_delivery, order_management,
    order_shipping, product, shop_account, shop_aftersale, shop_audit, shop_cat, shop_coupon,
    shop_delivery, shop_img, shop_order, shop_pay, shop_register, shop_sharer, shop_spu,
};
