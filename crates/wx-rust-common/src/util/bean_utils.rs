//! bean 操作工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.BeanUtils`。
//! Java 通过反射检查 `@Required` 字段；Rust 侧以 [`crate::annotation::RequiredField`]
//! trait 在编译期/显式校验替代反射。

use crate::annotation::RequiredField;
use crate::error::{WxErrorException, WxRuntimeError};

/// bean 操作工具。
pub struct BeanUtils;

impl BeanUtils {
    /// 检查 bean 里标记为必填的字段是否为空，为空则返回错误。
    ///
    /// 对应 Java `checkRequiredFields`：收集所有缺失的必填字段，
    /// 任一缺失即报错（错误信息列出缺失字段）。
    ///
    /// # 参数
    /// - `bean`：要检查的 bean 对象
    ///
    /// # 返回
    /// 校验通过返回 `Ok(())`；存在缺失字段返回错误。
    pub fn check_required_fields<T: RequiredField>(bean: &T) -> Result<(), WxErrorException> {
        let missing = bean.validate_required();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(WxErrorException::Runtime(WxRuntimeError::new(format!(
                "必填字段缺失：{}",
                missing.join(", ")
            ))))
        }
    }
}
