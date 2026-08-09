//! 字节组工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.crypto.ByteGroup`。

/// 字节组，用于按序拼接多个字节数组（微信消息加解密中间步骤）。
#[derive(Debug, Clone, Default)]
pub struct ByteGroup {
    bytes: Vec<u8>,
}

impl ByteGroup {
    /// 构建空字节组。
    pub fn new() -> Self {
        Self::default()
    }

    /// 追加字节数组。
    ///
    /// # 参数
    /// - `bytes`：要追加的字节数组
    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// 返回拼接后的总长度。
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// 返回拼接后的字节数组。
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
