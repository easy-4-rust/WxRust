//! 临时素材服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaMediaServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::{WxError, WxErrorException, WxRuntimeError};

use crate::api::{WxMaMediaService, WxMaService};
use crate::enums::url_g1_core::media as media_url;

/// Mirrors WxJava HttpResponseProxy.extractFileNameFromContentString (HttpComponents).
///
/// Java prioritizes lowercase \`filename*=utf-8''...\` and otherwise reads the
/// first \`filename="..."\` value. Never silently fabricate a missing filename.
fn extract_java_media_file_name(
    headers: &reqwest::header::HeaderMap,
) -> Result<String, WxErrorException> {
    let disposition = headers
        .get(reqwest::header::CONTENT_DISPOSITION)
        .ok_or_else(|| {
            WxErrorException::from_code(-99, "无法获取到文件名，Content-disposition为空")
        })?;
    let value = disposition
        .to_str()
        .map_err(|error| WxErrorException::from_code(-99, error.to_string()))?;

    if let Some((_, rest)) = value.split_once("filename*=utf-8''") {
        let encoded = rest
            .split(|c: char| c == ';' || c == ',' || c.is_whitespace())
            .next()
            .unwrap_or("");
        return percent_encoding::percent_decode_str(&encoded.replace('+', " "))
            .decode_utf8()
            .map(|name| name.into_owned())
            .map_err(|error| WxErrorException::from_code(-99, error.to_string()));
    }

    if let Some((_, rest)) = value.split_once("filename=\"") {
        if let Some((file_name, _)) = rest.split_once('"') {
            return Ok(file_name.to_owned());
        }
    }

    Err(WxErrorException::from_code(
        -99,
        "无法获取到文件名，header信息有问题",
    ))
}

/// 尚未交付给调用方的临时媒体文件。
///
/// 下载期间由当前 Future 独占；成功返回时才转移清理责任给调用方。
/// Drop 先关闭文件句柄，再删除部分文件和专属目录，以兼容 Windows 文件锁语义。
struct PendingMediaFile {
    path: std::path::PathBuf,
    directory: std::path::PathBuf,
    file: Option<std::fs::File>,
    persisted: bool,
}

impl PendingMediaFile {
    fn create(filename: &str, temp_root: &std::path::Path) -> Result<Self, WxErrorException> {
        use std::fs::{self, OpenOptions};
        use std::time::{SystemTime, UNIX_EPOCH};
        static FILE_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

        // Java uses FilenameUtils.getBaseName/getExtension before File.createTempFile.
        // Strip both separators; do not trust a response header as a filesystem path.
        let leaf = filename.rsplit(['/', '\\']).next().unwrap_or("");
        let (base, extension) = leaf.rsplit_once('.').unwrap_or((leaf, ""));
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default();
        let prefix = if base.chars().count() < 3 {
            now_ms.to_string()
        } else {
            base.chars()
                .map(|ch| match ch {
                    '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0' => '_',
                    _ => ch,
                })
                .collect::<String>()
        };
        let extension = extension
            .chars()
            .map(|ch| match ch {
                '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0' => '_',
                _ => ch,
            })
            .collect::<String>();

        for _ in 0..24 {
            let attempt = FILE_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default();
            let directory = temp_root.join(format!("wxma{}-{nonce}-{attempt}", std::process::id()));
            match fs::create_dir(&directory) {
                Ok(()) => {
                    let path = directory.join(format!("{prefix}{nonce}.{extension}"));
                    match OpenOptions::new().write(true).create_new(true).open(&path) {
                        Ok(file) => {
                            return Ok(Self {
                                path,
                                directory,
                                file: Some(file),
                                persisted: false,
                            });
                        }
                        Err(error) => {
                            let _ = fs::remove_dir(&directory);
                            return Err(WxErrorException::Io(error.to_string()));
                        }
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(WxErrorException::Io(error.to_string())),
            }
        }

        Err(WxErrorException::Io(
            "无法分配唯一的临时素材文件夹".to_string(),
        ))
    }

    fn file_mut(&mut self) -> &mut std::fs::File {
        self.file
            .as_mut()
            .expect("pending media file handle remains open until persist")
    }

    fn persist(mut self) -> std::path::PathBuf {
        let path = self.path.clone();
        self.file.take();
        self.persisted = true;
        path
    }
}

impl Drop for PendingMediaFile {
    fn drop(&mut self) {
        // Close first: Windows cannot remove an open file.
        self.file.take();
        if !self.persisted {
            let _ = std::fs::remove_file(&self.path);
            let _ = std::fs::remove_dir(&self.directory);
        }
    }
}

/// 将媒体响应流写入临时文件。
///
/// Future 在任意 await 被取消时，PendingMediaFile::drop 仍负责关闭和删除部分文件。
/// 只有成功返回 PathBuf 后，临时文件的清理责任才转移给调用方。
async fn download_response_to_temp_file(
    mut response: reqwest::Response,
    filename: &str,
    temp_root: &std::path::Path,
) -> Result<std::path::PathBuf, WxErrorException> {
    use std::io::Write;

    let mut pending = PendingMediaFile::create(filename, temp_root)?;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| WxErrorException::from_code(-99, format!("素材下载失败: {error}")))?
    {
        pending
            .file_mut()
            .write_all(&chunk)
            .map_err(|error| WxErrorException::Io(error.to_string()))?;
    }
    pending
        .file_mut()
        .flush()
        .map_err(|error| WxErrorException::Io(error.to_string()))?;

    Ok(pending.persist())
}

/// 临时素材服务实现。
pub struct WxMaMediaServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaMediaServiceImpl {
    /// 构建临时素材服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 以 multipart（字段名 `media`）上传素材并解析上传结果。
    ///
    /// 对应 Java `uploadMedia`：`wxMaService.upload(url,
    /// CommonUploadParam.fromFile("media", file))` 走执行引擎（token 注入 +
    /// 自动刷新 + 重试），再 `WxMediaUploadResult.fromJson(result)`。
    async fn upload_media_common(
        svc: &dyn WxMaService,
        media_type: &str,
        data: CommonUploadData,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let config = svc.wx_ma_config();
        let url = media_url::media_upload_url(config.as_ref(), media_type);
        let param = CommonUploadParam::new("media", data);
        let response = svc.upload(&url, param).await?;
        serde_json::from_str::<WxMediaUploadResult>(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaMediaService for WxMaMediaServiceImpl {
    async fn upload_media(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `uploadMedia(String, File)`：文件内容 + 文件名（`File.getName()`）
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, content),
        )
        .await
    }

    async fn upload_media_with_stream(
        &self,
        media_type: &str,
        file_type: &str,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `uploadMedia(String, String, InputStream)`：先
        // `FileUtils.createTmpFile(inputStream, UUID, fileType)` 落临时文件再上传；
        // Rust 直接以上传字节表达（ADAPTED），文件名取时间戳 + 随机后缀 + fileType
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let file_name = Some(format!("{now_millis}_{}.{file_type}", rand_suffix()));
        Self::upload_media_common(
            svc.as_ref(),
            media_type,
            CommonUploadData::new(file_name, input),
        )
        .await
    }

    async fn get_media(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getMedia`：`BaseMediaDownloadRequestExecutor` 执行
        // `MEDIA_GET_URL`（`https://api.weixin.qq.com/cgi-bin/media/get`），
        // 查询参数 `media_id=`；响应为 JSON 时视为微信错误报文并抛错，否则
        // 保存到本地临时目录并返回 File（Rust 直接返回文件字节，ADAPTED）。
        let config = svc.wx_ma_config();
        let token = svc.get_access_token().await?;
        let url = media_url::media_get_url(config.as_ref());
        // Match BaseWxMaServiceImpl.executeInternal: custom apiHostUrl / cloud-run
        // replacement must be applied before the request is sent.
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != crate::config::DEFAULT_API_HOST_URL {
            url.replace(crate::config::DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url
        };
        let url = format!("{url}?access_token={token}&media_id={media_id}");
        let resp = svc
            .send_http_request(svc.http_client().get(&url))
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        let is_json = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.starts_with("application/json"))
            .unwrap_or(false);
        if is_json {
            let body = resp
                .text()
                .await
                .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
            let error =
                WxError::from_json_with_type(&body, Some(wx_rust_common::enums::WxType::MiniApp));
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        Ok(bytes.to_vec())
    }

    /// 兼容 Java \`WxMaMediaService.getMedia(String)\` 的文件返回路径。
    ///
    /// 读取实际响应的 Content-Disposition 文件名，在独立 wxma 临时目录
    /// 生成带随机后缀的文件并流式写入响应字节。与原 \`get_media\` 返回字节
    /// 的便捷 API 并存；Rust 调用者负责在不再使用时删除文件和目录。
    async fn get_media_file(&self, media_id: &str) -> Result<std::path::PathBuf, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let token = svc.get_access_token().await?;
        let url = media_url::media_get_url(config.as_ref());
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != crate::config::DEFAULT_API_HOST_URL {
            url.replace(crate::config::DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url
        };
        let url = format!("{url}?access_token={token}&media_id={media_id}");
        let response = svc
            .send_http_request(svc.http_client().get(&url))
            .await
            .map_err(|error| WxErrorException::from_code(-99, format!("素材下载失败: {error}")))?;

        // Java HttpComponents InputStreamResponseHandler rejects HTTP >= 300
        // before parsing a response entity or allocating a temporary file.
        if response.status().as_u16() >= 300 {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(format!(
                "HTTP response status: {}",
                response.status().as_u16()
            ))));
        }

        if response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.starts_with("application/json"))
        {
            let body = response.text().await.map_err(|error| {
                WxErrorException::from_code(-99, format!("素材下载失败: {error}"))
            })?;
            let error =
                WxError::from_json_with_type(&body, Some(wx_rust_common::enums::WxType::MiniApp));
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }

        // Java HttpComponentsResponseProxy fails closed if Content-Disposition
        // is absent or malformed, even if the response contains valid bytes.
        let file_name = extract_java_media_file_name(response.headers())?;
        download_response_to_temp_file(response, &file_name, &std::env::temp_dir()).await
    }
}

/// 生成随机文件名后缀（对应 Java `UUID.randomUUID()` 语义，简化实现）。
fn rand_suffix() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    nanos ^ (std::process::id() as u64).wrapping_mul(0x9E3779B97F4A7C15)
}
#[cfg(test)]
mod phase1_file_download_tests {
    use super::*;
    use reqwest::header::{CONTENT_DISPOSITION, HeaderMap, HeaderValue};

    #[test]
    fn java_content_disposition_quoted_filename() {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_DISPOSITION,
            HeaderValue::from_static("attachment; filename=\"phase1-download.bin\""),
        );
        assert_eq!(
            extract_java_media_file_name(&headers).unwrap(),
            "phase1-download.bin"
        );
    }

    #[test]
    fn java_filename_star_takes_precedence_and_decodes_utf8() {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_DISPOSITION,
            HeaderValue::from_static(
                "attachment; filename=\"fallback.bin\"; filename*=utf-8''%E6%B5%8B%E8%AF%95+file.bin",
            ),
        );
        assert_eq!(
            extract_java_media_file_name(&headers).unwrap(),
            "测试 file.bin"
        );
    }

    #[test]
    fn missing_content_disposition_is_an_explicit_error() {
        let err = extract_java_media_file_name(&HeaderMap::new()).unwrap_err();
        assert_eq!(err.error_code(), Some(-99));
        assert!(err.to_string().contains("Content-disposition为空"));
    }

    struct TestTempRoot(std::path::PathBuf);

    impl TestTempRoot {
        fn new() -> Self {
            use std::time::{SystemTime, UNIX_EPOCH};

            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default();
            let path = std::env::temp_dir().join(format!(
                "wxrust-media-cancel-test-{}-{nonce}",
                std::process::id()
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }

    impl Drop for TestTempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn cancelling_a_partial_media_download_removes_the_file_and_directory() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            use tokio::time::{Duration, sleep, timeout};

            let temp_root = TestTempRoot::new();
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = [0_u8; 1024];
                let _ = stream.read(&mut request).await.unwrap();
                stream
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"partial.bin\"\r\nTransfer-Encoding: chunked\r\nConnection: keep-alive\r\n\r\n5\r\nhello\r\n",
                    )
                    .await
                    .unwrap();
                stream.flush().await.unwrap();
                std::future::pending::<()>().await;
            });

            let response = reqwest::Client::new()
                .get(format!("http://{address}/media"))
                .send()
                .await
                .unwrap();
            let root_path = temp_root.0.clone();
            let download = tokio::spawn(async move {
                download_response_to_temp_file(response, "partial.bin", &root_path).await
            });

            let partial_path = timeout(Duration::from_secs(2), async {
                loop {
                    let candidate = std::fs::read_dir(temp_root.path())
                        .ok()
                        .and_then(|directories| {
                            directories.filter_map(Result::ok).find_map(|directory| {
                                std::fs::read_dir(directory.path()).ok().and_then(|files| {
                                    files.filter_map(Result::ok).find_map(|file| {
                                        let path = file.path();
                                        std::fs::metadata(&path)
                                            .ok()
                                            .filter(|metadata| metadata.len() >= 5)
                                            .map(|_| path)
                                    })
                                })
                            })
                        });
                    if let Some(path) = candidate {
                        break path;
                    }
                    sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .expect("the first response chunk should be written to the temporary file");

            assert_eq!(std::fs::read(&partial_path).unwrap(), b"hello");
            download.abort();
            assert!(download.await.unwrap_err().is_cancelled());
            server.abort();

            let remaining = std::fs::read_dir(temp_root.path())
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            assert!(
                remaining.is_empty(),
                "partial media artifacts remain after cancellation: {remaining:?}"
            );
        });
    }

    #[test]
    fn failed_stream_after_a_partial_chunk_removes_the_file_and_directory() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            use tokio::sync::oneshot;
            use tokio::time::{Duration, sleep, timeout};

            let temp_root = TestTempRoot::new();
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .unwrap();
            let address = listener.local_addr().unwrap();
            let (chunk_sent_tx, chunk_sent_rx) = oneshot::channel();
            let (close_response_tx, close_response_rx) = oneshot::channel();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = [0_u8; 1024];
                let _ = stream.read(&mut request).await.unwrap();
                stream
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"truncated.bin\"\r\nTransfer-Encoding: chunked\r\nConnection: keep-alive\r\n\r\n5\r\nhello\r\n",
                    )
                    .await
                    .unwrap();
                stream.flush().await.unwrap();
                let _ = chunk_sent_tx.send(());
                let _ = close_response_rx.await;
                // Drop the connection without the terminating zero-length chunk.
            });

            let response = reqwest::Client::new()
                .get(format!("http://{address}/media"))
                .send()
                .await
                .unwrap();
            let root_path = temp_root.0.clone();
            let download = tokio::spawn(async move {
                download_response_to_temp_file(response, "truncated.bin", &root_path).await
            });

            chunk_sent_rx.await.unwrap();
            let partial_path = timeout(Duration::from_secs(2), async {
                loop {
                    let candidate = std::fs::read_dir(temp_root.path())
                        .ok()
                        .and_then(|directories| {
                            directories.filter_map(Result::ok).find_map(|directory| {
                                std::fs::read_dir(directory.path()).ok().and_then(|files| {
                                    files.filter_map(Result::ok).find_map(|file| {
                                        let path = file.path();
                                        std::fs::metadata(&path)
                                            .ok()
                                            .filter(|metadata| metadata.len() >= 5)
                                            .map(|_| path)
                                    })
                                })
                            })
                        });
                    if let Some(path) = candidate {
                        break path;
                    }
                    sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .expect("the first response chunk should be written before truncation");

            assert_eq!(std::fs::read(&partial_path).unwrap(), b"hello");
            close_response_tx.send(()).unwrap();
            let result = timeout(Duration::from_secs(2), download)
                .await
                .expect("truncated response should terminate")
                .unwrap();
            assert!(result.is_err(), "truncated chunked response must fail");
            server.abort();

            let remaining = std::fs::read_dir(temp_root.path())
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            assert!(
                remaining.is_empty(),
                "partial media artifacts remain after stream failure: {remaining:?}"
            );
        });
    }

    struct LegacyMediaService;

    #[async_trait]
    impl WxMaMediaService for LegacyMediaService {
        async fn upload_media(
            &self,
            _media_type: &str,
            _file_path: &str,
        ) -> Result<WxMediaUploadResult, WxErrorException> {
            Err(WxErrorException::from_code(-99, "test unsupported"))
        }

        async fn upload_media_with_stream(
            &self,
            _media_type: &str,
            _file_type: &str,
            _input: Vec<u8>,
        ) -> Result<WxMediaUploadResult, WxErrorException> {
            Err(WxErrorException::from_code(-99, "test unsupported"))
        }

        async fn get_media(&self, _media_id: &str) -> Result<Vec<u8>, WxErrorException> {
            Err(WxErrorException::from_code(-99, "test unsupported"))
        }
    }

    #[test]
    fn existing_media_service_implementors_can_omit_the_optional_file_method() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let error = LegacyMediaService
                .get_media_file("media-id")
                .await
                .unwrap_err();
            assert!(error.to_string().contains("不支持"));
        });
    }
}
