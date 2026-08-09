//! 枚举/常量分组注册（内容服务组 G2）。
//!
//! 本文件仅聚合 G2 新增的 URL 常量模块，避免直接改写 `enums/mod.rs` 的
//! 既有内容（注册行由各波次追加）。
//!
//! 模块文件位于 `enums/` 根目录（`url_g2_content.rs`，与任务文件布局
//! 一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式
//! 指回根目录文件。

#[path = "url_g2_content.rs"]
pub mod url_g2_content;
