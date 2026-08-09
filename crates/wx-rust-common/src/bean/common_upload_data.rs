//! 通用文件上传数据。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.CommonUploadData`。

/// 通用文件上传数据。
///
/// 承载上传文件的文件名、内容与长度；内容以字节切片形式持有
/// （对应 Java `InputStream`，Rust 侧由调用方提供 `Vec<u8>`）。
#[derive(Debug, Clone)]
pub struct CommonUploadData {
    /// 文件名，如 `1.jpg`
    pub file_name: Option<String>,

    /// 文件内容
    pub content: Vec<u8>,

    /// 文件内容长度（字节数）
    pub length: u64,
}

impl CommonUploadData {
    /// 从字节内容构建上传数据。
    ///
    /// # 参数
    /// - `file_name`：文件名（可为 `None`）
    /// - `content`：文件内容字节
    pub fn new(file_name: Option<String>, content: Vec<u8>) -> Self {
        let length = content.len() as u64;
        Self {
            file_name,
            content,
            length,
        }
    }

    /// 从文件路径构建上传数据（读取整个文件到内存）。
    ///
    /// # 参数
    /// - `file`：文件路径
    ///
    /// # 返回
    /// 上传数据；读取失败时返回错误。
    pub fn from_file(file: &std::path::Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read(file)?;
        let file_name = file.file_name().map(|n| n.to_string_lossy().into_owned());
        Ok(Self::new(file_name, content))
    }
}
