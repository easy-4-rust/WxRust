//! 获取标签成员接口响应体（第三方代开发）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpTagGetResult`：Java 中继承
//! `WxCpTagGetResult` 且无新增字段，仅提供静态 `deserialize(String)`（与
//! 父类 `fromJson` 行为一致）。Rust 以类型别名表达继承关系，`from_json`
//! 即 `WxCpTagGetResult::from_json`。

/// 类型别名：`WxCpTpTagGetResult = WxCpTagGetResult`（对应 Java 继承）。
pub type WxCpTpTagGetResult = super::WxCpTagGetResult;
