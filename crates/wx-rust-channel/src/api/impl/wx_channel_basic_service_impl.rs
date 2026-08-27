//! WxChannelBasicServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelBasicServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_basic_service::WxChannelBasicService;
use crate::bean::address::AddressCodeResponse;
use crate::bean::image::{
    ChannelImageInfo, ChannelImageResponse, QualificationFileResponse, UploadImageResponse,
};
use crate::bean::shop::{
    ShopH5UrlResponse, ShopInfoResponse, ShopQrCodeResponse, ShopTagLinkResponse,
};
use crate::enums::url_basics as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 基础接口服务实现。
pub struct WxChannelBasicServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelBasicServiceImpl {
    /// 构建基础接口服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelBasicService for WxChannelBasicServiceImpl {
    /// 对应 Java `WxChannelBasicServiceImpl.getShopInfo`：
    /// GET `GET_SHOP_INFO`。
    async fn get_shop_info(&self) -> Result<ShopInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.get(url::GET_SHOP_INFO, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBasicServiceImpl.uploadImg(int, String)`：
    /// POST `IMG_UPLOAD_URL?upload_type=1&resp_type=<respType>`，
    /// 请求体 `{"img_url":".."}`，解析 `UploadImageResponse` 后取
    /// `img_info`。
    async fn upload_img(
        &self,
        resp_type: i32,
        img_url: String,
    ) -> Result<ChannelImageInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let url = format!(
            "{}?upload_type=1&resp_type={resp_type}",
            url::IMG_UPLOAD_URL
        );
        let body = build_json(&[("img_url", serde_json::Value::String(img_url))]);
        let response = svc.post(&url, &body).await?;
        let upload_response: UploadImageResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(upload_response.img_info)
    }

    /// 对应 Java `WxChannelBasicServiceImpl.uploadImg(int, File, int, int)`：
    /// multipart 上传 `IMG_UPLOAD_URL?upload_type=0&resp_type=<respType>&height=..&width=..`
    /// （文件字段 `media`，对应 Java `ChannelFileUploadRequestExecutor`）。
    async fn upload_img_with_file(
        &self,
        resp_type: i32,
        file: std::path::PathBuf,
        height: i32,
        width: i32,
    ) -> Result<ChannelImageInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let url = format!(
            "{}?upload_type=0&resp_type={resp_type}&height={height}&width={width}",
            url::IMG_UPLOAD_URL
        );
        let data = CommonUploadData::from_file(&file)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let response = svc
            .upload(&url, CommonUploadParam::new("media", data))
            .await?;
        let upload_response: UploadImageResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(upload_response.img_info)
    }

    /// 对应 Java `WxChannelBasicServiceImpl.uploadQualificationFile`：
    /// multipart 上传 `UPLOAD_QUALIFICATION_FILE`（文件字段 `media`）。
    async fn upload_qualification_file(
        &self,
        file: std::path::PathBuf,
    ) -> Result<QualificationFileResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let data = CommonUploadData::from_file(&file)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let response = svc
            .upload(
                url::UPLOAD_QUALIFICATION_FILE,
                CommonUploadParam::new("media", data),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBasicServiceImpl.getImg`：
    /// GET `GET_IMG_URL?media_id=..`（注入 access_token，对应 Java
    /// `ChannelMediaDownloadRequestExecutor`）。
    ///
    /// 响应为 JSON（下载失败）时解析 `ChannelImageResponse`；否则将图片字节
    /// 写入系统临时目录（对应 Java `createTmpFile`）并以成功响应返回
    /// （Rust bean 无 `file` 字段——Java `@JsonIgnore`，下载路径不对外）。
    async fn get_img(&self, media_id: String) -> Result<ChannelImageResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let access_token = svc.get_access_token().await?;
        let url = format!(
            "{}?media_id={}&access_token={}",
            url::GET_IMG_URL,
            media_id,
            access_token
        );
        let client = svc.http_client();
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?;
        if content_type.starts_with("application/json") {
            let body = String::from_utf8_lossy(&bytes).to_string();
            return serde_json::from_str(&body).map_err(|e| WxErrorException::Serde(e.to_string()));
        }
        // 对应 Java `createTmpFile(baseName, extension, tmpDirFile)`：
        // 以时间戳命名写入系统临时目录（Java 的 `System.currentTimeMillis()`）。
        let file_name = format!("wxjava-channel-{media_id}-{}", now_millis());
        let tmp_file = std::env::temp_dir().join(file_name);
        std::fs::write(&tmp_file, &bytes)
            .map_err(|e| WxErrorException::from_code(-99, format!("写入临时文件失败: {e}")))?;
        let _ = tmp_file;
        Ok(ChannelImageResponse {
            err_code: 0,
            err_msg: "ok".to_string(),
            content_type,
        })
    }

    /// 对应 Java `WxChannelBasicServiceImpl.getAddressCode`：
    /// `{"addr_code": <code>}`（Java 手拼 JSON，`null` 时原样输出
    /// `{"addr_code": null}`）后 POST `GET_ADDRESS_CODE`。
    async fn get_address_code(
        &self,
        code: Option<i32>,
    ) -> Result<AddressCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = match code {
            Some(code) => format!(r#"{{"addr_code": {code}}}"#),
            None => r#"{"addr_code": null}"#.to_string(),
        };
        let response = svc.post(url::GET_ADDRESS_CODE, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBasicServiceImpl.getShopH5Url`：
    /// POST `"{}"` 到 `GET_SHOP_H5URL`。
    async fn get_shop_h5_url(&self) -> Result<ShopH5UrlResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::GET_SHOP_H5URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBasicServiceImpl.getShopQrCode`：
    /// `{"qrcode_type":<qrcodeType>` 后 POST `GET_SHOP_QRCODE`。
    async fn get_shop_qr_code(
        &self,
        qrcode_type: i32,
    ) -> Result<ShopQrCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = format!(r#"{{"qrcode_type":{qrcode_type}}}"#);
        let response = svc.post(url::GET_SHOP_QRCODE, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelBasicServiceImpl.getShopTagLink`：
    /// POST `"{}"` 到 `GET_SHOP_TAGLINK`。
    async fn get_shop_tag_link(&self) -> Result<ShopTagLinkResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::GET_SHOP_TAGLINK, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

/// 当前毫秒时间戳（对应 Java `System.currentTimeMillis()`）。
fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
