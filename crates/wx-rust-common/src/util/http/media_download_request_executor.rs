//! 媒体下载请求执行器。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.BaseMediaDownloadRequestExecutor`
//! （及四后端实现，均为 `PLATFORM_NA`）。Rust 以 reqwest 统一实现。

use async_trait::async_trait;

use crate::enums::WxType;
use crate::error::WxErrorException;
use crate::util::http::RequestExecutor;

/// 媒体下载请求执行器。
///
/// 下载媒体文件到字节内容（对应 Java 的 `File` 下载；Rust 返回字节由调用方落盘）。
#[derive(Debug, Clone)]
pub struct MediaDownloadRequestExecutor {
    /// reqwest 客户端
    client: reqwest::Client,
}

impl MediaDownloadRequestExecutor {
    /// 构建下载执行器。
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RequestExecutor<Vec<u8>, String> for MediaDownloadRequestExecutor {
    async fn execute(
        &self,
        uri: &str,
        _data: String,
        _wx_type: WxType,
    ) -> Result<Vec<u8>, WxErrorException> {
        let resp = self.client.get(uri).send().await?;
        let bytes = resp.bytes().await?.to_vec();
        Ok(bytes)
    }
}
