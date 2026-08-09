//! 微信错误异常处理器。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxErrorExceptionHandler`。

use crate::error::WxErrorException;

/// 微信错误异常处理器回调接口。
///
/// 用于消息路由等场景中统一处理异步执行时产生的错误。
pub trait WxErrorExceptionHandler: Send + Sync {
    /// 处理微信错误异常。
    ///
    /// # 参数
    /// - `e`：微信错误异常
    fn handle(&self, e: WxErrorException);
}
