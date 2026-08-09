//! 微信会话。
//!
//! 对应 Java `me.chanjar.weixin.common.session.WxSession`。

/// 微信会话。
///
/// 承载会话级属性（对应 Java `WxSession` 的 attribute 语义）。
pub trait WxSession: Send + Sync {
    /// 返回会话 ID。
    fn id(&self) -> &str;

    /// 获取属性。
    ///
    /// # 参数
    /// - `name`：属性名
    ///
    /// # 返回
    /// 属性值；不存在时返回 `None`。
    fn get_attribute(&self, name: &str) -> Option<String>;

    /// 设置属性。
    ///
    /// # 参数
    /// - `name`：属性名
    /// - `value`：属性值
    fn set_attribute(&self, name: &str, value: String);

    /// 移除属性。
    ///
    /// # 参数
    /// - `name`：属性名
    fn remove_attribute(&self, name: &str);

    /// 返回所有属性名。
    ///
    /// # 返回
    /// 属性名列表
    fn attribute_names(&self) -> Vec<String>;

    /// 会话是否有效（未过期）。
    fn is_valid(&self) -> bool;

    /// 使会话失效（过期）。
    fn invalidate(&self);
}
