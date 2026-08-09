//! HTTP 响应代理接口。
//!
//! 对应 Java `me.chanjar.weixin.common.util.http.HttpResponseProxy`。
//! Java 用于封装各 HTTP 后端的响应对象（状态码/头/流）；Rust 中由 reqwest
//! 响应直接承载，接口保留以对齐语义。

/// HTTP 响应代理。
#[derive(Debug, Clone)]
pub struct HttpResponseProxy {
    /// 状态码
    pub status_code: u16,
    /// 响应头
    pub headers: Vec<(String, String)>,
    /// 响应体字节
    pub body: Vec<u8>,
}

impl HttpResponseProxy {
    /// 构建响应代理。
    pub fn new(status_code: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    /// 从 Content-Disposition 头内容中提取文件名。
    ///
    /// 对应 Java `HttpResponseProxy.extractFileNameFromContentString`：
    /// 1. 优先匹配 `filename*=utf-8''...`（URL 解码）
    /// 2. 回退匹配 `filename="..."`（ISO-8859-1 → UTF-8 转换）
    ///
    /// # 参数
    /// - `content`：Content-Disposition 头内容
    ///
    /// # 返回
    /// 提取的文件名。
    ///
    /// # 错误
    /// content 为空或两种模式都未匹配时返回错误。
    pub fn extract_file_name_from_content_string(
        content: &str,
    ) -> Result<String, crate::error::WxErrorException> {
        if content.is_empty() {
            return Err(crate::error::WxErrorException::from_code(
                -1,
                "无法获取到文件名，content为空",
            ));
        }

        // 查找 filename*=utf-8'' 开头的部分
        if let Some(start) = content.find("filename*=utf-8''") {
            let after = &content[start + "filename*=utf-8''".len()..];
            let end = after.find([';', ' ', ',']).unwrap_or(after.len());
            let encoded = &after[..end];
            // URL 解码
            return percent_encoding::percent_decode_str(encoded)
                .decode_utf8()
                .map(|s| s.to_string())
                .map_err(|e| {
                    crate::error::WxErrorException::from_code(-1, format!("文件名解码失败: {e}"))
                });
        }

        // 查找普通 filename="..." 部分
        let marker = "filename=\"";
        if let Some(start) = content.find(marker) {
            let after = &content[start + marker.len()..];
            if let Some(end) = after.find('"') {
                let raw = &after[..end];
                // ISO-8859-1 → UTF-8 转换（对应 Java 行为）
                let bytes: Vec<u8> = raw.chars().map(|c| c as u8).collect();
                return Ok(String::from_utf8_lossy(&bytes).to_string());
            }
        }

        Err(crate::error::WxErrorException::from_code(
            -1,
            "无法获取到文件名，header信息有问题",
        ))
    }
}
