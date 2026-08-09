//! 智能对话服务门面。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.WxAispeechService` +
//! `WxAispeechServiceImpl`。Java 的接口只有 4 个访问器（dialog/knowledge
//! 子服务、配置存储），执行引擎（`executeDialogPost`/`executeKnowledgeGet`/
//! `Post`/`Put`/`Delete`/`MultipartPost` + 请求头签名注入）位于具体实现类
//! `WxAispeechServiceImpl` 的 protected 方法；Rust 以 trait 默认实现表达
//! 同一契约（与 miniapp/mp 模块同一设计原则），子服务通过
//! `Weak<dyn WxAispeechService>` 调用执行引擎。
//!
//! 说明：
//! - Java `HttpComponentsClientBuilder`（代理构建器）以 reqwest 客户端承载
//!   （`PLATFORM_NA`：HttpClient 专属后端）。
//! - Java `toBody(Object)`（Gson 序列化）由调用方先序列化为 JSON 字符串，
//!   `None` 等价于 Java `toBody(null)` 的 `"{}"`。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::{WxAispeechDialogService, WxAispeechKnowledgeService};
use crate::config::WxAispeechConfigStorage;
use crate::util::WxAispeechSignUtil;

/// 智能对话服务门面。
#[async_trait]
pub trait WxAispeechService: Send + Sync {
    /// 当前配置存储（对应 Java `getConfigStorage()`）。
    fn config_storage(&self) -> Arc<dyn WxAispeechConfigStorage>;

    /// 设置配置存储（对应 Java `setConfigStorage`，并重建 HTTP 客户端）。
    fn set_config_storage(&self, config_storage: Arc<dyn WxAispeechConfigStorage>);

    /// HTTP 客户端（reqwest，克隆廉价）。
    fn http_client(&self) -> reqwest::Client;

    // ---- 子服务（对应 Java `getDialogService()`/`getKnowledgeService()`；
    // 默认返回 None，由 WxAispeechServiceImpl 覆写为装配后的实例） ----

    /// 对话机器人服务。
    fn dialog_service(&self) -> Option<Arc<dyn WxAispeechDialogService>> {
        None
    }

    /// 知识库助理服务。
    fn knowledge_service(&self) -> Option<Arc<dyn WxAispeechKnowledgeService>> {
        None
    }

    // ---- 执行引擎（对应 Java `WxAispeechServiceImpl` protected 方法） ----

    /// 对话 API POST 请求（对应 Java `executeDialogPost`）。
    ///
    /// 注入请求头：`request_id`/`timestamp`/`nonce`/`sign`（MD5 链式签名，
    /// 见 `WxAispeechSignUtil::calc_dialog_sign`）与 `X-APPID`（appid 为空时
    /// 回落到配置 appid）或 `X-OPENAI-TOKEN`（为空时报错）。
    async fn execute_dialog_post(
        &self,
        path: &str,
        request_body: Option<&str>,
        with_open_token: bool,
        appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let body = to_body(request_body);
        let request_id = uuid::Uuid::new_v4().to_string();
        let timestamp = now_seconds();
        let nonce = random_nonce();
        let config = self.config_storage();
        let sign = WxAispeechSignUtil::calc_dialog_sign(config.token(), timestamp, &nonce, &body);
        let resolved_appid = match appid {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => config.appid().unwrap_or("").to_string(),
        };

        let mut request = self
            .http_client()
            .post(format!("{}{path}", config.dialog_api_base_url()));
        request = request
            .header("request_id", &request_id)
            .header("timestamp", timestamp.to_string())
            .header("nonce", &nonce)
            .header("sign", sign)
            .header(reqwest::header::CONTENT_TYPE, "application/json");
        request = if with_open_token {
            match config.open_ai_token() {
                Some(open_ai_token) if !open_ai_token.is_empty() => {
                    request.header("X-OPENAI-TOKEN", open_ai_token)
                }
                _ => {
                    return Err(WxErrorException::from_code(
                        -99,
                        "X-OPENAI-TOKEN不能为空，请先调用getAccessToken或手动设置",
                    ));
                }
            }
        } else {
            if resolved_appid.is_empty() {
                return Err(WxErrorException::from_code(-99, "X-APPID不能为空"));
            }
            request.header("X-APPID", resolved_appid)
        };

        let resp = request
            .body(body)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }

    /// 知识库 API GET 请求（对应 Java `executeKnowledgeGet`）。
    ///
    /// `query_params` 中 value 为 null 的项跳过（对应 Java URIBuilder
    /// 判空逻辑）；注入 `X-APPID`/`X-Request-ID`/`X-Timestamp`/`X-Nonce`/
    /// `X-Signature` 头（HmacSHA256 签名）。
    async fn execute_knowledge_get(
        &self,
        path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        let mut url = format!("{}{path}", config.knowledge_api_base_url());
        if let Some(params) = query_params {
            let mut first = !url.contains('?');
            for (key, value) in params {
                if value.is_empty() {
                    continue;
                }
                url.push_str(if first { "?" } else { "&" });
                first = false;
                url.push_str(key);
                url.push('=');
                url.push_str(value);
            }
        }
        let headers = build_knowledge_headers(&*config, "")?;
        let resp = self
            .http_client()
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }

    /// 知识库 API POST 请求（对应 Java `executeKnowledgePost`）。
    async fn execute_knowledge_post(
        &self,
        path: &str,
        request_body: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let body = to_body(request_body);
        let config = self.config_storage();
        let headers = build_knowledge_headers(&*config, &body)?;
        let resp = self
            .http_client()
            .post(format!("{}{path}", config.knowledge_api_base_url()))
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }

    /// 知识库 API PUT 请求（对应 Java `executeKnowledgePut`）。
    async fn execute_knowledge_put(
        &self,
        path: &str,
        request_body: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let body = to_body(request_body);
        let config = self.config_storage();
        let headers = build_knowledge_headers(&*config, &body)?;
        let resp = self
            .http_client()
            .put(format!("{}{path}", config.knowledge_api_base_url()))
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }

    /// 知识库 API DELETE 请求（对应 Java `executeKnowledgeDelete`）。
    async fn execute_knowledge_delete(&self, path: &str) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        let headers = build_knowledge_headers(&*config, "")?;
        let resp = self
            .http_client()
            .delete(format!("{}{path}", config.knowledge_api_base_url()))
            .headers(headers)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }

    /// 知识库 API 文件上传（multipart/form-data，对应 Java
    /// `executeKnowledgeMultipartPost`）。
    ///
    /// 表单字段：`file`（二进制）+ 可选的 `title`/`description`（text/plain，
    /// UTF-8）与 `metadata`（application/json）。Java `File` 以字节 + 文件名
    /// 承载（ADAPTED）。
    async fn execute_knowledge_multipart_post(
        &self,
        path: &str,
        file_name: &str,
        file_bytes: &[u8],
        title: Option<&str>,
        description: Option<&str>,
        metadata: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let config = self.config_storage();
        let headers = build_knowledge_headers(&*config, "")?;

        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes.to_vec())
                .file_name(file_name.to_string())
                .mime_str("application/octet-stream")
                .map_err(|e| WxErrorException::Http(e.to_string()))?,
        );
        if let Some(title) = title.filter(|t| !t.is_empty()) {
            form = form.text("title", title.to_string());
        }
        if let Some(description) = description.filter(|d| !d.is_empty()) {
            form = form.text("description", description.to_string());
        }
        if let Some(metadata) = metadata.filter(|m| !m.is_empty()) {
            form = form.part(
                "metadata",
                reqwest::multipart::Part::text(metadata.to_string())
                    .mime_str("application/json")
                    .map_err(|e| WxErrorException::Http(e.to_string()))?,
            );
        }

        let resp = self
            .http_client()
            .post(format!("{}{path}", config.knowledge_api_base_url()))
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        execute_request(resp).await
    }
}

/// 请求体序列化（对应 Java `WxAispeechServiceImpl.toBody`）：
/// `null` → `"{}"`；字符串原样透传（调用方已用 serde 序列化）。
fn to_body(request_body: Option<&str>) -> String {
    match request_body {
        Some(body) => body.to_string(),
        None => "{}".to_string(),
    }
}

/// 构造知识库请求头（对应 Java `enrichKnowledgeHeaders`）。
///
/// 要求配置 appid 与 secretKey 均非空；签名载荷为
/// `timestamp\nnonce\nrequestId\nbody`（body 为空串时仍参与签名）。
fn build_knowledge_headers(
    config: &dyn WxAispeechConfigStorage,
    body: &str,
) -> Result<reqwest::header::HeaderMap, WxErrorException> {
    if config.appid().is_none() || config.appid().unwrap().is_empty() {
        return Err(WxErrorException::from_code(
            -99,
            "知识助理请求需要配置appid",
        ));
    }
    if config.secret_key().is_none() || config.secret_key().unwrap().is_empty() {
        return Err(WxErrorException::from_code(
            -99,
            "知识助理请求需要配置secretKey",
        ));
    }

    let request_id = uuid::Uuid::new_v4().to_string();
    let timestamp = now_seconds();
    let nonce = random_nonce();
    let signature = WxAispeechSignUtil::calc_knowledge_signature(
        config.secret_key(),
        timestamp,
        &nonce,
        &request_id,
        body,
    );

    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(appid) = config.appid() {
        headers.insert(
            "X-APPID",
            reqwest::header::HeaderValue::from_str(appid).expect("appid 非空"),
        );
    }
    headers.insert(
        "X-Request-ID",
        reqwest::header::HeaderValue::from_str(&request_id).expect("request_id 合法"),
    );
    headers.insert(
        "X-Timestamp",
        reqwest::header::HeaderValue::from_str(&timestamp.to_string()).expect("timestamp 合法"),
    );
    headers.insert(
        "X-Nonce",
        reqwest::header::HeaderValue::from_str(&nonce).expect("nonce 合法"),
    );
    headers.insert(
        "X-Signature",
        reqwest::header::HeaderValue::from_str(&signature).expect("signature 合法"),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    Ok(headers)
}

/// 执行请求并读取响应（对应 Java `executeRequest`）。
///
/// 2xx 返回响应体；非 2xx 抛 `WxErrorException`（错误码为 HTTP 状态码，
/// 错误信息为响应体，对应 Java `WxError.builder().errorCode(statusCode)
/// .errorMsg(body)`）。
async fn execute_request(resp: reqwest::Response) -> Result<String, WxErrorException> {
    let status_code = resp.status().as_u16() as i32;
    let body = resp
        .text()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    if (200..300).contains(&status_code) {
        return Ok(body);
    }
    Err(WxErrorException::Wx(
        wx_rust_common::error::WxErrorError::new(WxError::new(status_code, body)),
    ))
}

/// 当前时间戳（秒，对应 Java `System.currentTimeMillis() / 1000`）。
fn now_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// 随机 nonce（对应 Java `randomNonce`：uuid 去横线取前 16 位）。
fn random_nonce() -> String {
    uuid::Uuid::new_v4().to_string().replace('-', "")[..16].to_string()
}
