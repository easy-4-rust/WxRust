//! JSON 序列化工具（`PLATFORM_NA` 说明）。
//!
//! 对应 Java `me.chanjar.weixin.common.util.json` 包（`WxGsonBuilder` 及手写
//! TypeAdapter）。Java 使用 Gson；WxRust 以 `serde` + `serde_json` 派生替代，
//! 线格式（字段名 camelCase、null 省略）经 golden 夹具验证。
//! 本模块不提供 Gson 兼容实现。
