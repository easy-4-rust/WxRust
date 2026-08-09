//! 镜像 Java `FileUtilsTest`：临时文件创建与图片流转 Base64。
//!
//! Java 测试源：`weixin-java-common/src/test/java/me/chanjar/weixin/common/util/fs/FileUtilsTest.java`

use std::io::Read;
use std::path::PathBuf;

use wx_rust_common::util::fs::FileUtils;

/// 生成唯一临时目录（时间戳，避免并行测试冲突）。
fn unique_tmp_dir(tag: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("wxrust-test-{tag}-{nanos}"))
}

// ---- 镜像 testCreateTmpFile：指定目录、前缀与后缀 ----
#[test]
fn create_tmp_file_with_explicit_dir() {
    let dir = unique_tmp_dir("explicit");
    std::fs::create_dir_all(&dir).unwrap();

    let path = FileUtils::create_tmp_file(b"hello", "pre-", "txt", Some(&dir)).unwrap();

    // Java: File.createTempFile(name, "." + ext, dir)，内容完整写入
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello");
    assert!(
        path.starts_with(&dir),
        "文件应在指定目录，实际 {}",
        path.display()
    );
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    assert!(name.starts_with("pre-"), "前缀应为 pre-，实际 {name}");
    assert!(name.ends_with(".txt"), "后缀应为 .txt，实际 {name}");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

// ---- 镜像 testTestCreateTmpFile：默认临时目录 ----
#[test]
fn create_tmp_file_default_tmp_dir() {
    let content = "默认目录测试内容";
    let path = FileUtils::create_tmp_file(content.as_bytes(), "def-", "tmp", None).unwrap();

    // Rust 默认目录为系统临时目录下的 wxjava-temp（Java 直接用 java.io.tmpdir）
    let expected_dir = std::env::temp_dir().join("wxjava-temp");
    assert!(
        path.starts_with(&expected_dir),
        "默认目录应为 wxjava-temp，实际 {}",
        path.display()
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), content);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn create_tmp_file_unique_names() {
    // Java createTempFile 语义：同名前缀多次创建得到不同文件
    let dir = unique_tmp_dir("unique");
    std::fs::create_dir_all(&dir).unwrap();

    let a = FileUtils::create_tmp_file(b"1", "same-", "bin", Some(&dir)).unwrap();
    let b = FileUtils::create_tmp_file(b"2", "same-", "bin", Some(&dir)).unwrap();

    assert_ne!(a, b, "两次创建应得到不同文件");
    assert_eq!(std::fs::read(&a).unwrap(), b"1");
    assert_eq!(std::fs::read(&b).unwrap(), b"2");

    let _ = std::fs::remove_file(&a);
    let _ = std::fs::remove_file(&b);
    let _ = std::fs::remove_dir(&dir);
}

// ---- 镜像 testImageToBase64ByStream ----
#[test]
fn image_to_base64_known_vector() {
    // 黄金向量：标准 Base64("hello") == "aGVsbG8="（与 Java Base64.getEncoder() 一致）
    let mut reader = std::io::Cursor::new(b"hello".as_slice());
    assert_eq!(
        FileUtils::image_to_base64_by_stream(&mut reader).unwrap(),
        "aGVsbG8="
    );
}

#[test]
fn image_to_base64_empty_stream() {
    let mut reader = std::io::Cursor::new(b"".as_slice());
    assert_eq!(
        FileUtils::image_to_base64_by_stream(&mut reader).unwrap(),
        ""
    );
}

#[test]
fn image_to_base64_read_error_returns_err() {
    // Java：IOException 时打印堆栈并返回 null → Rust 对应 Result::Err
    struct ErrReader;
    impl Read for ErrReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("boom"))
        }
    }
    let mut r = ErrReader;
    assert!(FileUtils::image_to_base64_by_stream(&mut r).is_err());
}
