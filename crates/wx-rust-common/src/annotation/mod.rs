//! 字段校验注解。
//!
//! 对应 Java `me.chanjar.weixin.common.annotation` 包。
//! Java 的 `@Required` 是运行时注解（反射校验）；Rust 侧以编译期派生宏等价物替代，
//! 具体校验行为由 `RequiredField` 特性的实现提供。

/// 标识某个字段是否是必填的。
///
/// 对应 Java `@Required`。在 Rust 中该注解的语义由使用方通过校验函数实现
/// （如构造/反序列化后的校验），不引入运行时反射。
pub trait RequiredField {
    /// 校验所有必填字段是否已设置。
    ///
    /// # 返回
    /// 缺失的字段名列表；全部满足时为空列表。
    fn validate_required(&self) -> Vec<&'static str>;
}
