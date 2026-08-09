//! 小程序代码管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaCodeServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::{WxMaCodeService, WxMaService};
use crate::bean::{
    WxMaCodeAuditStatus, WxMaCodeCommitRequest, WxMaCodeSubmitAuditItem,
    WxMaCodeSubmitAuditRequest, WxMaCodeVersionDistribution, WxMaCodeVersionInfo,
};
use crate::enums::url_g1_core::code as code_url;

/// 小程序代码管理服务实现。
pub struct WxMaCodeServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaCodeServiceImpl {
    /// 构建代码管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 下载接口（对应 Java `BaseMediaDownloadRequestExecutor` 语义）：
    /// 响应 Content-Type 为 JSON 时视为微信错误报文并抛错，否则返回字节。
    async fn download_bytes(svc: &dyn WxMaService, url: &str) -> Result<Vec<u8>, WxErrorException> {
        let resp = svc
            .http_client()
            .get(url)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("二维码下载失败: {e}")))?;
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
                .map_err(|e| WxErrorException::from_code(-99, format!("二维码下载失败: {e}")))?;
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
            .map_err(|e| WxErrorException::from_code(-99, format!("二维码下载失败: {e}")))?;
        Ok(bytes.to_vec())
    }
}

#[async_trait]
impl WxMaCodeService for WxMaCodeServiceImpl {
    async fn commit(&self, commit_request: &WxMaCodeCommitRequest) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `commit`：POST `COMMIT_URL`（`https://api.weixin.qq.com/wxa/commit`）
        let config = svc.wx_ma_config();
        let body = commit_request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&code_url::commit_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn get_qr_code(&self, path: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getQrCode`：`BaseMediaDownloadRequestExecutor` 下载
        // `GET_QRCODE_URL`；path 非空时 `URLEncoder.encode(path, "UTF-8")`
        // 后拼入 `?path=`；返回 bytes 后删除临时文件（Rust 不落盘，直接返回字节）
        let config = svc.wx_ma_config();
        let base = code_url::get_qrcode_url(config.as_ref());
        let url = if !path.is_empty() {
            let encoded =
                percent_encoding::utf8_percent_encode(path, percent_encoding::NON_ALPHANUMERIC)
                    .to_string();
            format!("{base}?path={encoded}")
        } else {
            base
        };
        // Java 执行引擎以 `&`/`?` 区分追加 access_token
        let token = svc.get_access_token().await?;
        let url = if url.contains('?') {
            format!("{url}&access_token={token}")
        } else {
            format!("{url}?access_token={token}")
        };
        Self::download_bytes(svc.as_ref(), &url).await
    }

    async fn get_category(&self) -> Result<Option<Vec<WxMaCodeSubmitAuditItem>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getCategory`：GET `GET_CATEGORY_URL`，解析 `category_list`
        // 数组，字段缺失返回 null → Rust `None`
        let config = svc.wx_ma_config();
        let response = svc
            .get(&code_url::get_category_url(config.as_ref()), "")
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        match json.get("category_list") {
            Some(list) => serde_json::from_value(list.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            None => Ok(None),
        }
    }

    async fn get_page(&self) -> Result<Option<Vec<String>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getPage`：GET `GET_PAGE_URL`，解析 `page_list` 数组，
        // 字段缺失返回 null → Rust `None`
        let config = svc.wx_ma_config();
        let response = svc
            .get(&code_url::get_page_url(config.as_ref()), "")
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        match json.get("page_list") {
            Some(list) => serde_json::from_value(list.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            None => Ok(None),
        }
    }

    async fn submit_audit(
        &self,
        audit_request: &WxMaCodeSubmitAuditRequest,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `submitAudit`：POST `SUBMIT_AUDIT_URL`，`GsonHelper.getLong(json,
        // "auditid")` 取审核编号（Java `long`）
        let config = svc.wx_ma_config();
        let body = audit_request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&code_url::submit_audit_url(config.as_ref()), &body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(json.get("auditid").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    async fn get_audit_status(
        &self,
        audit_id: i64,
    ) -> Result<WxMaCodeAuditStatus, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getAuditStatus`：POST `GET_AUDIT_STATUS_URL`，请求体
        // `{"auditid": ...}`，响应 `WxMaCodeAuditStatus.fromJson`
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "auditid": audit_id }).to_string();
        let response = svc
            .post(&code_url::get_audit_status_url(config.as_ref()), &body)
            .await?;
        WxMaCodeAuditStatus::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_latest_audit_status(&self) -> Result<WxMaCodeAuditStatus, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getLatestAuditStatus`：GET `GET_LATEST_AUDIT_STATUS_URL`
        let config = svc.wx_ma_config();
        let response = svc
            .get(&code_url::get_latest_audit_status_url(config.as_ref()), "")
            .await?;
        WxMaCodeAuditStatus::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn release(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `release`：POST `RELEASE_URL`，请求体 `{}`
        let config = svc.wx_ma_config();
        svc.post(&code_url::release_url(config.as_ref()), "{}")
            .await?;
        Ok(())
    }

    async fn change_visit_status(&self, action: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `changeVisitStatus`：POST `CHANGE_VISIT_STATUS_URL`，
        // 请求体 `{"action": ...}`
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "action": action }).to_string();
        svc.post(&code_url::change_visit_status_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn revert_code_release(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `revertCodeRelease`：GET `REVERT_CODE_RELEASE_URL`
        let config = svc.wx_ma_config();
        svc.get(&code_url::revert_code_release_url(config.as_ref()), "")
            .await?;
        Ok(())
    }

    async fn get_support_version(&self) -> Result<WxMaCodeVersionDistribution, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getSupportVersion`：POST `GET_SUPPORT_VERSION_URL`，请求体 `{}`
        let config = svc.wx_ma_config();
        let response = svc
            .post(&code_url::get_support_version_url(config.as_ref()), "{}")
            .await?;
        WxMaCodeVersionDistribution::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_version_info(&self) -> Result<WxMaCodeVersionInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getVersionInfo`：POST `GET_VERSION_INFO_URL`，请求体 `{}`
        let config = svc.wx_ma_config();
        let response = svc
            .post(&code_url::get_version_info_url(config.as_ref()), "{}")
            .await?;
        WxMaCodeVersionInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_support_version(&self, version: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `setSupportVersion`：POST `SET_SUPPORT_VERSION_URL`，
        // 请求体 `{"version": ...}`
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "version": version }).to_string();
        svc.post(&code_url::set_support_version_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn undo_code_audit(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `undoCodeAudit`：GET `UNDO_CODE_AUDIT_URL`
        let config = svc.wx_ma_config();
        svc.get(&code_url::undo_code_audit_url(config.as_ref()), "")
            .await?;
        Ok(())
    }
}
