//! 文件工具。
//!
//! 对应 Java `me.chanjar.weixin.common.util.fs.FileUtils`。

/// 文件工具。
pub struct FileUtils;

impl FileUtils {
    /// 创建临时文件并写入内容。
    ///
    /// # 参数
    /// - `content`：文件内容
    /// - `name`：文件名前缀
    /// - `ext`：扩展名（不含点）
    /// - `tmp_dir`：临时文件夹目录（`None` 时使用系统临时目录下的 `wxjava-temp`）
    ///
    /// # 返回
    /// 临时文件路径。
    pub fn create_tmp_file(
        content: &[u8],
        name: &str,
        ext: &str,
        tmp_dir: Option<&std::path::Path>,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        let dir = match tmp_dir {
            Some(d) => d.to_path_buf(),
            None => {
                let base = std::env::temp_dir();
                let d = base.join("wxjava-temp");
                std::fs::create_dir_all(&d)?;
                d
            }
        };
        // 对应 Java File.createTempFile(name, '.'+ext, dir)
        let mut path = dir.join(format!("{name}{}", std::process::id()));
        // 追加随机后缀避免冲突
        let rand_suffix: String = (0..8)
            .map(|_| {
                let c = rand::random::<u8>() % 36;
                if c < 10 {
                    (b'0' + c) as char
                } else {
                    (b'a' + c - 10) as char
                }
            })
            .collect();
        path.set_file_name(format!("{name}{rand_suffix}.{ext}"));
        std::fs::write(&path, content)?;
        // deleteOnExit 等价：由调用方负责清理；此处不做自动删除（跨平台安全）
        Ok(path)
    }

    /// 将输入流转为 Base64 字符串。
    ///
    /// 对应 Java `FileUtils.imageToBase64ByStream`：读取流全部内容后以标准
    /// Base64 编码返回。Java 读取失败时打印堆栈并返回 `null`，Rust 以
    /// `Result::Err` 表达同一错误路径。
    pub fn image_to_base64_by_stream<R: std::io::Read>(
        reader: &mut R,
    ) -> Result<String, std::io::Error> {
        use base64::Engine;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&data))
    }
}
