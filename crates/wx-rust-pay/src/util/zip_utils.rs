//! 压缩文件工具。
//!
//! 对应 Java `com.github.binarywang.wxpay.util.ZipUtils`：
//! `unGzip(File file)` 将 gzip 文件解压为去扩展名的同名文件
//! （对账单下载 `gzip` 压缩场景）。
//!
//! 实现：`flate2`（workspace 既有依赖，`download_bill` 的内存解压同源）；
//! 补充 [`gunzip_bytes`] 字节形态（对应 Java `IOUtils.copy(gzis, fos)`
//! 的内存等价，便于流式下载后直接解压）。

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

/// 解压 gzip 字节（Java 无直接等价，为 `IOUtils.copy` 的内存形态）。
///
/// # 参数
/// `gz_bytes`：gzip 压缩数据（对账单下载响应体）
pub fn gunzip_bytes(gz_bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(gz_bytes);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

/// 解压 gzip 文件（对应 Java `ZipUtils.unGzip(File file)`）。
///
/// 输出文件为原路径去掉扩展名（对应 Java
/// `FilenameUtils.removeExtension(file.getAbsolutePath())`），并创建/覆写
/// 该文件后写入解压内容；返回输出文件路径。
pub fn un_gzip_file(file: &Path) -> std::io::Result<PathBuf> {
    let path_str = file.to_string_lossy();
    // 对应 FilenameUtils.removeExtension：仅当 '.' 位于最后一段文件名内时剥除
    let file_name_start = path_str.rfind('/').map(|i| i + 1).unwrap_or(0);
    let dot_in_name = path_str[file_name_start..]
        .rfind('.')
        .map(|i| file_name_start + i);
    let result_file = match dot_in_name {
        Some(idx) => PathBuf::from(&path_str[..idx]),
        None => file.to_path_buf(),
    };

    let compressed = fs::read(file)?;
    let decompressed = gunzip_bytes(&compressed)?;
    fs::write(&result_file, decompressed)?;
    Ok(result_file)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    /// gzip 压缩 → 解压往返（离线）。
    #[test]
    fn gunzip_bytes_roundtrip() {
        let original = b"1234567890,txn,2026-08-24\n".repeat(20);
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&original).unwrap();
        let gz = encoder.finish().unwrap();

        assert_eq!(gunzip_bytes(&gz).unwrap(), original);
    }

    /// 文件形态：`xxx.csv.gz` → `xxx.csv`（对应 Java removeExtension）。
    #[test]
    fn un_gzip_file_strips_extension() {
        let dir = std::env::temp_dir().join(format!("wxrust-zip-{:?}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let gz_path = dir.join("bill.csv.gz");

        let original = b"header\nrow1\n";
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(original).unwrap();
        fs::write(&gz_path, encoder.finish().unwrap()).unwrap();

        let result = un_gzip_file(&gz_path).unwrap();
        assert_eq!(result, dir.join("bill.csv"));
        assert_eq!(fs::read(&result).unwrap(), original);

        fs::remove_dir_all(&dir).ok();
    }
}
