//! 小程序消息 API。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api` 包。

pub mod r#impl;
pub mod wx_ma_service;

pub use wx_ma_service::WxMaService;

// 内容服务组（G2）子服务 trait 注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g2_services;
pub use g2_services::*;

// 核心服务组（G1）子服务 trait 注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g1_services;
pub use g1_services::*;

// 电商服务组（G3）子服务 trait 注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g3_services;
pub use g3_services::*;

// 能力服务组（G4）子服务 trait 注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g4_services;
pub use g4_services::*;
