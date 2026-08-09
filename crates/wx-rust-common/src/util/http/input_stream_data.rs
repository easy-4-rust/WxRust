//! 输入流数据。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.InputStreamData`。

/// 输入流数据（上传媒体文件用）。
///
/// 对应 Java `InputStream + filename`；Rust 侧以字节内容承载。
#[derive(Debug, Clone)]
pub struct InputStreamData {
    /// 文件内容字节
    pub content: Vec<u8>,
    /// 文件名
    pub filename: String,
}

impl InputStreamData {
    /// 构建输入流数据。
    ///
    /// # 参数
    /// - `content`：文件内容字节
    /// - `filename`：文件名
    pub fn new(content: Vec<u8>, filename: impl Into<String>) -> Self {
        Self {
            content,
            filename: filename.into(),
        }
    }
}
