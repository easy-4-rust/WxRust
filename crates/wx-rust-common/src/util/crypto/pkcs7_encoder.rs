//! PKCS7 填充编码器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.crypto.PKCS7Encoder`（微信消息加解密专用）。

/// 提供基于 PKCS7 算法的加解密填充（微信 32 字节块）。
pub struct Pkcs7Encoder;

impl Pkcs7Encoder {
    /// 块大小（字节）：微信使用 32
    const BLOCK_SIZE: usize = 32;

    /// 获得对明文进行补位填充的字节。
    ///
    /// # 参数
    /// - `count`：需要进行填充补位操作的明文字节个数
    ///
    /// # 返回
    /// 补齐用的字节数组
    pub fn encode(count: usize) -> Vec<u8> {
        // 计算需要填充的位数
        let amount_to_pad = Self::BLOCK_SIZE - (count % Self::BLOCK_SIZE);
        // 补位所用的字符（数值转 ASCII 字符）
        let pad_chr = (amount_to_pad as u8) as char;
        let mut out = Vec::with_capacity(amount_to_pad);
        for _ in 0..amount_to_pad {
            out.push(pad_chr as u8);
        }
        out
    }

    /// 删除解密后明文的补位字符。
    ///
    /// # 参数
    /// - `decrypted`：解密后的明文
    ///
    /// # 返回
    /// 删除补位字符后的明文
    pub fn decode(decrypted: &[u8]) -> Vec<u8> {
        if decrypted.is_empty() {
            return decrypted.to_vec();
        }
        let pad = decrypted[decrypted.len() - 1] as usize;
        // 非法填充长度（不在 1..=32 范围）时视为无填充
        let pad = if (1..=32).contains(&pad) { pad } else { 0 };
        let len = decrypted.len().saturating_sub(pad);
        decrypted[..len].to_vec()
    }
}
