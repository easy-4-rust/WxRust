//! 日志异常处理器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.LogExceptionHandler`。

use crate::api::WxErrorExceptionHandler;
use crate::error::WxErrorException;

/// 默认日志异常处理器：把异常记录到日志。
#[derive(Debug, Clone, Default)]
pub struct LogExceptionHandler;

impl WxErrorExceptionHandler for LogExceptionHandler {
    /// 记录错误异常。
    ///
    /// # 参数
    /// - `e`：微信错误异常
    fn handle(&self, e: WxErrorException) {
        tracing::error!("Error happens: {e:?}");
    }
}
