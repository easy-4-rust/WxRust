//! Real WxRust MiniApp HTTP adapter for the pinned WxJava differential.
use std::error::Error;
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;
use wx_rust_miniapp::api::r#impl::WxMaServiceImpl;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

type AdapterResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
type HttpResponseParts = (u16, String, String, Vec<(String, String)>);
fn decode_hex(value: &str) -> AdapterResult<Vec<u8>> {
    if value.len() % 2 != 0 {
        return Err("hex payload must have even length".into());
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for index in (0..value.len()).step_by(2) {
        bytes.push(u8::from_str_radix(&value[index..index + 2], 16)?);
    }
    Ok(bytes)
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0f) as usize] as char);
    }
    result
}

fn response_parts(value: &Value) -> AdapterResult<HttpResponseParts> {
    if let Some(body) = value.as_str() {
        return Ok((
            200,
            body.to_owned(),
            "application/json; charset=UTF-8".to_string(),
            Vec::new(),
        ));
    }
    let status = value["status"].as_u64().ok_or("response status missing")? as u16;
    let body = value["body"]
        .as_str()
        .ok_or("response body missing")?
        .to_owned();
    let content_type = value["content_type"]
        .as_str()
        .unwrap_or("application/json; charset=UTF-8")
        .to_string();
    let response_headers = value["response_headers"]
        .as_object()
        .map(|headers| {
            headers
                .iter()
                .map(|(name, value)| {
                    Ok((
                        name.clone(),
                        value
                            .as_str()
                            .ok_or("response header value must be a string")?
                            .to_owned(),
                    ))
                })
                .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()
        })
        .transpose()?
        .unwrap_or_default();
    Ok((status, body, content_type, response_headers))
}

struct Request {
    method: String,
    target: String,
    content_type: Option<String>,
    headers: Vec<(String, String)>,
    body: String,
    body_hex: String,
    stream: TcpStream,
}

async fn read_request(stream: TcpStream) -> Result<Request, Box<dyn Error + Send + Sync>> {
    let mut reader = BufReader::new(stream);
    let mut start_line = String::new();
    reader.read_line(&mut start_line).await?;
    let mut parts = start_line.split_whitespace();
    let method = parts.next().ok_or("HTTP method missing")?.to_string();
    let target = parts
        .next()
        .ok_or("HTTP request target missing")?
        .to_string();

    let mut content_type = None;
    let mut content_length = 0usize;
    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line).await?;
        if read == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if name.eq_ignore_ascii_case("content-type") {
                content_type = Some(value.clone());
            } else if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse()?;
            }
            headers.push((name, value));
        }
    }

    let mut bytes = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut bytes).await?;
    }
    let body_hex = to_hex(&bytes);
    let body = String::from_utf8_lossy(&bytes).into_owned();
    Ok(Request {
        method,
        target,
        content_type,
        headers,
        body,
        body_hex,
        stream: reader.into_inner(),
    })
}

async fn write_http_response(
    mut stream: TcpStream,
    status: u16,
    content_type: &str,
    response_headers: &[(String, String)],
    body: &str,
) -> io::Result<()> {
    let bytes = body.as_bytes();
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Test Response",
    };
    let mut header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
        bytes.len()
    );
    for (name, value) in response_headers {
        header.push_str(&format!("{name}: {value}\r\n"));
    }
    header.push_str("\r\n");
    stream.write_all(header.as_bytes()).await?;
    stream.write_all(bytes).await?;
    stream.flush().await
}

async fn serve(
    listener: TcpListener,
    input: Value,
    trace: Arc<Mutex<Vec<Value>>>,
) -> Result<(usize, usize), Box<dyn Error + Send + Sync>> {
    let token_path = input["token_path"].as_str().ok_or("token_path missing")?;
    let token_responses = input["token_responses"]
        .as_array()
        .ok_or("token_responses missing")?
        .iter()
        .map(response_parts)
        .collect::<Result<Vec<_>, _>>()?;
    let api_responses = input["api_responses"]
        .as_array()
        .ok_or("api_responses missing")?
        .iter()
        .map(response_parts)
        .collect::<Result<Vec<_>, _>>()?;

    let mut sequence = 0usize;
    let mut token_calls = 0usize;
    let mut api_calls = 0usize;

    // A short idle timeout makes an absent replay observable instead of hanging the runner.
    let expected_requests = token_responses.len() + api_responses.len();
    for _ in 0..expected_requests {
        let accepted = timeout(Duration::from_secs(3), listener.accept()).await;
        let (stream, _) = match accepted {
            Ok(result) => result?,
            Err(_) => break,
        };
        let request = read_request(stream).await?;
        sequence += 1;
        let request_path = request.target.split('?').next().unwrap_or_default();
        let kind = if request_path == token_path {
            "token"
        } else {
            "api"
        };
        let (response_status, response_body, response_content_type, response_headers) =
            if kind == "token" {
                let index = token_calls;
                token_calls += 1;
                token_responses.get(index).cloned().unwrap_or_else(|| {
                    (
                        200,
                        r#"{"errcode":-1,"errmsg":"unexpected extra token request"}"#.to_string(),
                        "application/json; charset=UTF-8".to_string(),
                        Vec::new(),
                    )
                })
            } else {
                let index = api_calls;
                api_calls += 1;
                api_responses.get(index).cloned().unwrap_or_else(|| {
                    (
                        200,
                        r#"{"errcode":-1,"errmsg":"unexpected extra API request"}"#.to_string(),
                        "application/json; charset=UTF-8".to_string(),
                        Vec::new(),
                    )
                })
            };
        let mut custom_headers = serde_json::Map::new();
        if let Some(expected_headers) = input["default_headers"].as_object() {
            for name in expected_headers.keys() {
                let values = request
                    .headers
                    .iter()
                    .filter(|(actual, _)| actual.eq_ignore_ascii_case(name))
                    .map(|(_, value)| value.clone())
                    .collect::<Vec<_>>();
                custom_headers.insert(name.to_ascii_lowercase(), json!(values));
            }
        }
        if let Some(dynamic) = input["dynamic_headers"].as_object() {
            let name = dynamic["name"]
                .as_str()
                .ok_or("dynamic header name missing")?;
            let values = request
                .headers
                .iter()
                .filter(|(actual, _)| actual.eq_ignore_ascii_case(name))
                .map(|(_, value)| value.clone())
                .collect::<Vec<_>>();
            custom_headers.insert(name.to_ascii_lowercase(), json!(values));
        }

        trace
            .lock()
            .map_err(|_| "request trace mutex poisoned")?
            .push(json!({
                "sequence": sequence,
                "kind": kind,
                "method": request.method,
                "target": request.target,
                "content_type": request.content_type,
                "custom_headers": custom_headers,
                "body": request.body,
                "body_hex": request.body_hex,
                "response_status": response_status,
                "response_content_type": response_content_type,
                "response_headers": response_headers.iter().cloned().collect::<std::collections::BTreeMap<_, _>>(),
                "response_body": response_body,
                "response_body_hex": to_hex(response_body.as_bytes())
            }));
        write_http_response(
            request.stream,
            response_status,
            &response_content_type,
            &response_headers,
            &response_body,
        )
        .await?;
    }
    Ok((token_calls, api_calls))
}

fn execute_fixture(fixture: &Value) -> AdapterResult<Value> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        let input = &fixture["input"];
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let base_url = format!("http://127.0.0.1:{}", address.port());
        let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
        let server_trace = Arc::clone(&trace);
        let server_input = input.clone();
        let server = tokio::spawn(async move { serve(listener, server_input, server_trace).await });

        let config = WxMaDefaultConfig::new(
            input["app_id"].as_str().ok_or("app_id missing")?,
            input["app_secret"].as_str().ok_or("app_secret missing")?,
        );
        config.set_api_host_url(&base_url);
        let token_url = format!(
            "{}{}?grant_type=client_credential&appid=%s&secret=%s",
            base_url,
            input["token_path"].as_str().ok_or("token_path missing")?
        );
        config.set_access_token_url(&token_url);
        let mut default_headers = reqwest::header::HeaderMap::new();
        if let Some(headers) = input["default_headers"].as_object() {
            for (name, value) in headers {
                let name = reqwest::header::HeaderName::from_bytes(name.as_bytes())?;
                let value = reqwest::header::HeaderValue::from_bytes(
                    value
                        .as_str()
                        .ok_or("default header value must be a string")?
                        .as_bytes(),
                )?;
                default_headers.insert(name, value);
            }
        }
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;
        let mut interceptors: Vec<wx_rust_common::http::HttpRequestInterceptor> = Vec::new();
        if let Some(dynamic) = input["dynamic_headers"].as_object() {
            let name = reqwest::header::HeaderName::from_bytes(
                dynamic["name"].as_str().ok_or("dynamic header name missing")?.as_bytes()
            )?;
            let token_value = reqwest::header::HeaderValue::from_str(
                dynamic["token_value"].as_str().ok_or("dynamic token header value missing")?
            )?;
            let api_value = reqwest::header::HeaderValue::from_str(
                dynamic["api_value"].as_str().ok_or("dynamic api header value missing")?
            )?;
            let token_path = input["token_path"].as_str().ok_or("token_path missing")?.to_owned();
            interceptors.push(Arc::new(move |request: &mut reqwest::Request| {
                let value = if request.url().path() == token_path {
                    token_value.clone()
                } else {
                    api_value.clone()
                };
                request.headers_mut().insert(name.clone(), value);
                Ok(())
            }));
        }
        let service = WxMaServiceImpl::new_arc_with_http_client_and_interceptors(
            Arc::new(config), client, interceptors
        );

        let api_url = format!(
            "https://api.weixin.qq.com{}?{}",
            input["api_path"].as_str().ok_or("api_path missing")?,
            input["api_query"].as_str().ok_or("api_query missing")?
        );
        let operation = fixture["operation"].as_str();
        let call: Result<String, WxErrorException> = if operation == Some("miniapp.http.media-download-file") {
            let media_service = service.media_service().ok_or("MiniApp media service unavailable")?;
            match media_service.get_media_file(
                input["media_id"].as_str().ok_or("media_id missing")?
            ).await {
                Ok(path) => {
                    let file_name = path.file_name()
                        .and_then(|value| value.to_str())
                        .ok_or("downloaded file name is not UTF-8")?.to_owned();
                    let temp_dir_name = path.parent()
                        .and_then(|parent| parent.file_name())
                        .and_then(|value| value.to_str())
                        .ok_or("downloaded temporary directory missing")?.to_owned();
                    let file_exists = path.is_file();
                    let bytes = fs::read(&path)?;
                    fs::remove_file(&path)?;
                    if let Some(parent) = path.parent() {
                        fs::remove_dir(parent)?;
                    }
                    Ok(json!({
                        "file_name": file_name,
                        "bytes_hex": to_hex(&bytes),
                        "file_exists": file_exists,
                        "temp_dir_name": temp_dir_name
                    }).to_string())
                }
                Err(error) => Err(error),
            }
        } else if operation == Some("miniapp.http.media-download") {
            let media_service = service.media_service().ok_or("MiniApp media service unavailable")?;
            match media_service.get_media(
                input["media_id"].as_str().ok_or("media_id missing")?
            ).await {
                Ok(bytes) => Ok(json!({
                    "file_name": null,
                    "bytes_hex": to_hex(&bytes)
                }).to_string()),
                Err(error) => Err(error),
            }
        } else if matches!(
            operation,
            Some("miniapp.http.media-upload" | "miniapp.http.common-upload")
        ) {
            let bytes = decode_hex(input["file_bytes_hex"].as_str().ok_or("file_bytes_hex missing")?)?;
            let file_name = input["file_name"].as_str().ok_or("file_name missing")?.to_owned();
            if operation == Some("miniapp.http.media-upload") {
                let nonce = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|value| value.as_nanos())
                    .unwrap_or_default();
                let temp_dir = std::env::temp_dir().join(format!(
                    "wxrust-phase1-{}-{nonce}", std::process::id()
                ));
                fs::create_dir(&temp_dir)?;
                let file_path = temp_dir.join(&file_name);
                fs::write(&file_path, &bytes)?;
                let media_service = service.media_service().ok_or("MiniApp media service unavailable")?;
                let upload_result = media_service.upload_media(
                    input["media_type"].as_str().ok_or("media_type missing")?,
                    file_path.to_str().ok_or("temporary upload path is not UTF-8")?,
                ).await;
                let _ = fs::remove_file(&file_path);
                let _ = fs::remove_dir(&temp_dir);
                match upload_result {
                    Ok(result) => serde_json::to_string(&result)
                        .map_err(|error| WxErrorException::Serde(error.to_string())),
                    Err(error) => Err(error),
                }
            } else {
                let fields = input["form_fields"].as_object()
                    .ok_or("form_fields missing")?
                    .iter()
                    .map(|(name, value)| {
                        Ok((name.clone(), value.as_str().ok_or("form field value must be a string")?.to_owned()))
                    })
                    .collect::<Result<std::collections::HashMap<_, _>, Box<dyn Error + Send + Sync>>>()?;
                let data = CommonUploadData::new(Some(file_name), bytes);
                let param = CommonUploadParam::with_form_fields("media", data, fields);
                service.upload(&api_url, param).await
            }
        } else if matches!(
            operation,
            Some("miniapp.http.get-query-order" | "miniapp.http.get-custom-client-headers" | "miniapp.http.get-dynamic-interceptor")
        ) {
            service
                .get(
                    &api_url,
                    input["query_param"].as_str().ok_or("query_param missing")?,
                )
                .await
        } else {
            service
                .post(
                    &api_url,
                    input["api_body"].as_str().ok_or("api_body missing")?,
                )
                .await
        };
        let (token_calls, api_calls) = timeout(Duration::from_secs(8), server)
            .await
            .map_err(|_| "loopback server did not finish")??
            .map_err(|error| format!("loopback server failed: {error}"))?;

        let request_trace = trace
            .lock()
            .map_err(|_| "request trace mutex poisoned")?
            .clone();
        let mut result = match call {
            Ok(response_body) => json!({
                "kind": "value",
                "value": {
                    "response_body": response_body,
                    "request_trace": request_trace,
                    "token_calls": token_calls,
                    "api_calls": api_calls
                }
            }),
            Err(error) => {
                let api_response = request_trace
                    .iter()
                    .rev()
                    .find(|entry| entry["kind"] == "api");
                let category = if matches!(&error, WxErrorException::Runtime(_)) {
                    api_response.and_then(|entry| {
                        if entry["response_status"].as_u64().unwrap_or(200) >= 300 {
                            Some("http_status")
                        } else if entry["response_body"].as_str() == Some("") {
                            Some("empty_response_runtime")
                        } else {
                            None
                        }
                    })
                } else if error.wx_error().is_some_and(|value| {
                    value.error_code == -99
                        && value.error_msg.as_deref() == Some("无法获取到文件名，Content-disposition为空")
                }) {
                    Some("missing_content_disposition")
                } else if error.wx_error().is_some_and(|value| {
                    value.error_code == -99 && value.error_msg.as_deref() == Some("无响应内容")
                }) {
                    Some("empty_response_business")
                } else if matches!(&error, WxErrorException::Wx(_))
                    && api_response.is_some_and(|entry| entry["response_body"].as_str() == Some(""))
                {
                    Some("business_error")
                } else {
                    None
                };
                let error_type = match &error {
                    WxErrorException::Runtime(_) => "WxErrorException::Runtime",
                    WxErrorException::Wx(_) => "WxErrorException::Wx",
                    WxErrorException::Http(_) => "WxErrorException::Http",
                    WxErrorException::Io(_) => "WxErrorException::Io",
                    WxErrorException::Serde(_) => "WxErrorException::Serde",
                };
                let mut result = json!({
                    "kind": "exception",
                    "type": error_type,
                    "message": error.to_string(),
                    "debug": format!("{error:?}"),
                    "request_trace": request_trace,
                    "token_calls": token_calls,
                    "api_calls": api_calls
                });
                if let Some(category) = category {
                    result["category"] = json!(category);
                }
                if let Some(error_code) = error.error_code() {
                    result["error_code"] = json!(error_code);
                }
                result
            }
        };
        // Keep the result protocol explicit even if an adapter returns an unexpected shape.
        if !result.is_object() {
            result = json!({"kind":"adapter_error","message":"non-object result"});
        }
        Ok::<Value, Box<dyn Error + Send + Sync>>(result)
    })
}

fn main() -> AdapterResult<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or("fixture JSONL path required")?;
    let content = fs::read_to_string(path)?;
    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        let fixture: Value = serde_json::from_str(line)?;
        let result = execute_fixture(&fixture)?;
        let row = json!({
            "id": fixture["id"],
            "operation": fixture["operation"],
            "result": result
        });
        println!("{}", serde_json::to_string(&row)?);
    }
    Ok(())
}
