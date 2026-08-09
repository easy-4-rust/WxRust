//! G1 组（核心服务组）接口地址注册。
//!
//! 本组为 Wave 2 核心服务组（用户/消息/素材/客服/统计/代码/物流/安全/设置）
//! 新增的接口地址模块。与 `url_business`/`url_core` 平行，由 Wave 3 统一装配。

//! 模块文件位于 `enums/` 根目录（`url_g1_core.rs`，与任务文件布局一致）；
//! 本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式指回根目录
//! 文件。

#[path = "url_g1_core.rs"]
pub mod url_g1_core;
