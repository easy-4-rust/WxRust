//! Wave 2 G4 能力服务组接口地址注册。
//!
//! 本组（G4）新增的能力类子服务 URL 函数统一收敛于 `url_g4_ability`，
//! 对应 Java `WxMaApiUrlConstants` 的能力类子域常量。

#[path = "url_g4_ability.rs"]
pub mod url_g4_ability;

pub use url_g4_ability::{
    cloud, complaint, device_subscribe, face, img_proc, intracity, invoice, live, marketing, ocr,
    promotion, qrcode_jump, vod, xpay,
};
