//! 临时素材接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaMediaService`。

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::error::WxErrorException;

/// 临时素材接口。
#[async_trait]
pub trait WxMaMediaService: Send + Sync {
    /// 新增临时素材（对应 Java `uploadMedia(String, File)`）。
    ///
    /// Java 以 `File` 传参；Rust 以文件路径传参（ADAPTED）。
    async fn upload_media(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 新增临时素材（对应 Java `uploadMedia(String, String, InputStream)`）。
    ///
    /// Java 先以 `fileType` 创建临时文件（`FileUtils.createTmpFile`）再上传；
    /// Rust 直接以上传字节表达（ADAPTED：无 InputStream 类型）。
    async fn upload_media_with_stream(
        &self,
        media_type: &str,
        file_type: &str,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 获取临时素材（对应 Java `getMedia(String)`）。
    ///
    /// Java 将文件保存到本地临时目录并返回 `File`；Rust 返回文件字节
    /// `Vec<u8>`（ADAPTED）。
    async fn get_media(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 获取与 Java \`getMedia(String)\` 文件返回语义对应的临时文件路径。
    ///
    /// 读取 Content-Disposition、创建独立 wxma 临时目录，并分块保存内容。
    /// 与现有字节接口并存。Rust 调用方负责及时清理返回的文件及其父目录
    /// （与 Java 的 JVM deleteOnExit 生命周期不同，必须显式处置）。
    /// 默认返回明确的不支持错误，避免要求既有下游 Trait 实现者新增方法；
    /// 内置 MiniApp 实现覆盖此方法。
    async fn get_media_file(
        &self,
        _media_id: &str,
    ) -> Result<std::path::PathBuf, WxErrorException> {
        Err(WxErrorException::from_code(
            -99,
            "当前 WxMaMediaService 实现不支持文件式素材下载",
        ))
    }
}
