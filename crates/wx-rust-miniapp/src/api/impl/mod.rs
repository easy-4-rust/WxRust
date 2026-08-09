//! 小程序服务实现（对应 Java `cn.binarywang.wx.miniapp.api.impl` 包）。

pub mod base_wx_ma_service_impl;
pub mod wx_ma_service_impl;

pub use wx_ma_service_impl::WxMaServiceImpl;

// 内容服务组（G2）子服务实现注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g2_impls;
pub use g2_impls::*;

// 核心服务组（G1）子服务实现注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g1_impls;
pub use g1_impls::*;

// 电商服务组（G3）子服务实现注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g3_impls;
pub use g3_impls::*;

// 能力服务组（G4）子服务实现注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g4_impls;
pub use g4_impls::*;
