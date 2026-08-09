//! 随机字符串工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.RandomUtils`。

/// 随机字符串常量（与 Java `RANDOM_STR` 一致）。
const RANDOM_STR: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// 随机字符串工具。
pub struct RandomUtils;

impl RandomUtils {
    /// 生成 16 位随机字符串（字母数字混合）。
    ///
    /// # 返回
    /// 16 位随机字符串
    pub fn get_random_str() -> String {
        (0..16)
            .map(|_| {
                let idx = rand::random_range(0..RANDOM_STR.len());
                RANDOM_STR[idx] as char
            })
            .collect()
    }
}
